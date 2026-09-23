// src/llm.rs
// OpenRouter client: streaming chat completions with retries, and the model list.
use crate::config::Config;
use crate::data::Topic;
use crate::model::LearningModule;
use crate::{parsing, prompts};
use anyhow::{Context, Result, anyhow, bail};
use futures_util::StreamExt;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const CHAT_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
const MODELS_URL: &str = "https://openrouter.ai/api/v1/models";
const MAX_ATTEMPTS: u32 = 3;
const MAX_RETRY_DELAY: Duration = Duration::from_secs(10);

#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct Message {
    role: &'static str,
    content: String,
}

/// An OpenRouter model
#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub is_free: bool,
}

/// A parsed server-sent event line of a streaming completion
#[derive(Debug, PartialEq)]
enum StreamEvent {
    /// Generated text
    Delta(String),
    /// The stream is complete
    Done,
    /// An error reported inside the stream
    Error(String),
    /// Comments, keep-alives and events without content
    Ignore,
}

fn error_message(value: &serde_json::Value) -> Option<String> {
    let error = value.get("error")?;
    Some(error.get("message").and_then(|m| m.as_str()).map(str::to_string).unwrap_or_else(|| error.to_string()))
}

fn parse_stream_line(line: &str) -> StreamEvent {
    let Some(data) = line.trim().strip_prefix("data:") else {
        return StreamEvent::Ignore;
    };
    let data = data.trim();
    if data == "[DONE]" {
        return StreamEvent::Done;
    }
    let Ok(value) = serde_json::from_str::<serde_json::Value>(data) else {
        return StreamEvent::Ignore;
    };
    if let Some(message) = error_message(&value) {
        return StreamEvent::Error(message);
    }
    match value.pointer("/choices/0/delta/content").and_then(|c| c.as_str()) {
        Some(text) if !text.is_empty() => StreamEvent::Delta(text.to_string()),
        _ => StreamEvent::Ignore,
    }
}

fn is_retryable(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

fn retry_delay(attempt: u32, retry_after: Option<&str>) -> Duration {
    retry_after
        .and_then(|value| value.trim().parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(1 << attempt))
        .min(MAX_RETRY_DELAY)
}

// LLM client for generating learning content
#[derive(Clone)]
pub struct LlmClient {
    client: Client,
    api_key: String,
}

impl LlmClient {
    pub fn new(api_key: String) -> Self {
        // Generation can be slow, but a stalled connection must not hang forever
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(15))
            .read_timeout(Duration::from_secs(90))
            .build()
            .unwrap_or_else(|err| {
                tracing::warn!("Failed to build HTTP client with timeouts: {}", err);
                Client::new()
            });
        Self { client, api_key }
    }

    /// Lists the models available on OpenRouter, free models first
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        #[derive(Deserialize)]
        struct Pricing {
            prompt: Option<String>,
            completion: Option<String>,
        }
        #[derive(Deserialize)]
        struct ApiModel {
            id: String,
            name: Option<String>,
            pricing: Option<Pricing>,
        }
        #[derive(Deserialize)]
        struct ModelList {
            data: Vec<ApiModel>,
        }

        let response = self
            .client
            .get(MODELS_URL)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .context("Failed to fetch the model list")?;
        let status = response.status();
        if !status.is_success() {
            bail!("Fetching the model list failed ({})", status);
        }
        let list: ModelList = response.json().await.context("Invalid model list")?;

        let is_zero = |price: &Option<String>| price.as_deref().is_some_and(|p| p.parse::<f64>() == Ok(0.0));
        let mut models: Vec<Model> = list
            .data
            .into_iter()
            .map(|m| Model {
                is_free: m.id.ends_with(":free")
                    || m.pricing.as_ref().is_some_and(|p| is_zero(&p.prompt) && is_zero(&p.completion)),
                name: m.name.unwrap_or_else(|| m.id.clone()),
                id: m.id,
            })
            .collect();
        models.sort_by(|a, b| b.is_free.cmp(&a.is_free).then_with(|| a.id.cmp(&b.id)));
        Ok(models)
    }

    /// Generates a learning module for a topic. `on_text` receives the text as it streams in.
    pub async fn generate_learning_module(
        &self,
        topic: &Topic,
        level: u8,
        config: &Config,
        on_text: impl FnMut(&str) + Send,
    ) -> Result<LearningModule> {
        let prompt = prompts::learning_module(topic, level, config);
        let response = self.complete_streaming(&config.model, prompt, on_text).await?;
        Ok(module_from_response(&topic.topic, &response))
    }

    /// Runs a streaming completion. Retries on rate limits and server errors
    /// before any text has been received.
    pub async fn complete_streaming(
        &self,
        model: &str,
        prompt: String,
        mut on_text: impl FnMut(&str) + Send,
    ) -> Result<String> {
        if self.api_key.is_empty() {
            bail!("OpenRouter API key is not set. Please set the OPENROUTER_API_KEY environment variable.");
        }

        let request = ChatRequest { model, messages: vec![Message { role: "user", content: prompt }], stream: true };

        let mut attempt = 0;
        let response = loop {
            attempt += 1;
            let result = self
                .client
                .post(CHAT_URL)
                .bearer_auth(&self.api_key)
                .header("X-Title", "RustMentor")
                .json(&request)
                .send()
                .await;

            match result {
                Ok(response) if response.status().is_success() => break response,
                Ok(response) => {
                    let status = response.status();
                    let retry_after = response
                        .headers()
                        .get(reqwest::header::RETRY_AFTER)
                        .and_then(|v| v.to_str().ok())
                        .map(str::to_string);
                    let body = response.text().await.unwrap_or_default();
                    if attempt < MAX_ATTEMPTS && is_retryable(status) {
                        let delay = retry_delay(attempt, retry_after.as_deref());
                        tracing::warn!("OpenRouter returned {}, retrying in {:?}", status, delay);
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    let message = serde_json::from_str::<serde_json::Value>(&body)
                        .ok()
                        .and_then(|v| error_message(&v))
                        .unwrap_or(body);
                    bail!("OpenRouter API request failed ({}): {}", status, message);
                }
                Err(err) if attempt < MAX_ATTEMPTS && err.is_connect() => {
                    let delay = retry_delay(attempt, None);
                    tracing::warn!("Connection to OpenRouter failed ({}), retrying in {:?}", err, delay);
                    tokio::time::sleep(delay).await;
                }
                Err(err) => return Err(anyhow!(err).context("Failed to reach OpenRouter")),
            }
        };

        let mut text = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        let mut stream = response.bytes_stream();
        'stream: while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("The response stream was interrupted")?;
            buffer.extend_from_slice(&chunk);
            while let Some(newline) = buffer.iter().position(|&b| b == b'\n') {
                let line: Vec<u8> = buffer.drain(..=newline).collect();
                match parse_stream_line(&String::from_utf8_lossy(&line)) {
                    StreamEvent::Delta(delta) => {
                        text.push_str(&delta);
                        on_text(&delta);
                    }
                    StreamEvent::Done => break 'stream,
                    StreamEvent::Error(message) => bail!("OpenRouter reported an error: {}", message),
                    StreamEvent::Ignore => {}
                }
            }
        }

        if text.trim().is_empty() {
            bail!("The model returned an empty response");
        }
        Ok(text)
    }
}

/// Builds a learning module from the model's response. If the response doesn't follow
/// the expected format, the raw text is shown as the explanation.
pub fn module_from_response(topic: &str, response: &str) -> LearningModule {
    match parsing::parse_module_response(response) {
        Ok(parsed) => LearningModule {
            topic: topic.to_string(),
            explanation: parsed.explanation,
            code_snippets: parsed.code_snippets,
            exercises: parsed.exercises,
            additional_resources: None, // Added by the App
        },
        Err(err) => {
            tracing::warn!("Failed to parse the learning module response: {}", err);
            tracing::debug!("Unparseable response: {}", response);
            LearningModule {
                topic: topic.to_string(),
                explanation: format!(
                    "> The response didn't follow the expected format, so it is shown as is.\n\n{}",
                    response.trim()
                ),
                code_snippets: vec![],
                exercises: vec![],
                additional_resources: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_stream_lines() {
        assert_eq!(
            parse_stream_line(r#"data: {"choices":[{"delta":{"content":"Hi"}}]}"#),
            StreamEvent::Delta("Hi".to_string())
        );
        assert_eq!(parse_stream_line("data: [DONE]"), StreamEvent::Done);
        assert_eq!(parse_stream_line(": OPENROUTER PROCESSING"), StreamEvent::Ignore);
        assert_eq!(parse_stream_line(""), StreamEvent::Ignore);
        assert_eq!(parse_stream_line(r#"data: {"choices":[{"delta":{"role":"assistant"}}]}"#), StreamEvent::Ignore);
        assert_eq!(
            parse_stream_line(r#"data: {"error":{"message":"Rate limited","code":429}}"#),
            StreamEvent::Error("Rate limited".to_string())
        );
    }

    #[test]
    fn retry_delay_is_bounded() {
        assert_eq!(retry_delay(1, None), Duration::from_secs(2));
        assert_eq!(retry_delay(1, Some("3")), Duration::from_secs(3));
        assert_eq!(retry_delay(1, Some("3600")), MAX_RETRY_DELAY);
        assert!(is_retryable(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable(StatusCode::BAD_GATEWAY));
        assert!(!is_retryable(StatusCode::UNAUTHORIZED));
    }

    #[test]
    fn unparseable_module_shows_raw_text() {
        let module = module_from_response("Topic", "just some text");
        assert!(module.explanation.contains("just some text"));
        assert!(module.code_snippets.is_empty());
    }
}
