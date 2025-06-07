// src/llm.rs
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::app::LearningModule;
use crate::data::Topic;

// OpenRouter API request structure
#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

// OpenRouter API response structure
#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

// LLM client for generating learning content
#[derive(Clone)]
pub struct LlmClient {
    client: Client,
    api_key: String,
}

impl LlmClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    // Generate a learning module based on a topic and user level
    pub async fn generate_learning_module(&self, topic: &Topic, level: u8) -> Result<LearningModule> {
        // Check if API key is available
        if self.api_key.is_empty() {
            anyhow::bail!("OpenRouter API key is not set. Please set the OPENROUTER_API_KEY environment variable.");
        }

        // Create the prompt for the LLM
        let prompt = self.create_prompt(topic, level);

        // Call the OpenRouter API
        let response = self.call_openrouter_api(prompt).await?;

        // Parse the response into a LearningModule
        self.parse_response(response, topic)
    }

    // Create a prompt for the LLM based on the topic and level
    fn create_prompt(&self, topic: &Topic, level: u8) -> String {
        let level_description = match level {
            1 => "Absolute Beginner",
            2 => "Beginner",
            3 => "Early Intermediate",
            4 => "Intermediate",
            5 => "Solid Intermediate",
            6 => "Advanced Intermediate",
            7 => "Early Advanced",
            8 => "Advanced",
            9 => "Very Advanced",
            10 => "Expert",
            _ => "Intermediate",
        };

        format!(
            "You are an expert Rust programming language tutor. Create a learning module about '{}' for a {} level Rust programmer.\n\n\
            The learning module should include:\n\
            1. A clear explanation of the topic\n\
            2. At least two code examples demonstrating the concept\n\
            3. Two practice exercises\n\n\
            Format your response as a JSON object with the following structure:\n\
            {{\n\
              \"explanation\": \"Your explanation here...\",\n\
              \"code_snippets\": [\"First code example\", \"Second code example\"],\n\
              \"exercises\": [\"First exercise\", \"Second exercise\"]\n\
            }}\n\n\
            Make sure your explanation and examples are appropriate for a {} level programmer.\n\
            The source of this topic is: {}",
            topic.topic, level_description, level_description, topic.source
        )
    }

    // Call the OpenRouter API with the prompt
    async fn call_openrouter_api(&self, prompt: String) -> Result<String> {
        // Create the request body
        let request = OpenRouterRequest {
            model: "anthropic/claude-3-opus:beta".to_string(), // You can change this to a different model if needed
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
        };

        // Make the API call
        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        // Check if the request was successful
        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("OpenRouter API request failed: {}", error_text);
        }

        // Parse the response
        let response_data: OpenRouterResponse = response.json().await?;

        // Extract the content from the response
        if let Some(choice) = response_data.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            anyhow::bail!("No content in OpenRouter API response");
        }
    }

    // Parse the LLM response into a LearningModule
    fn parse_response(&self, response: String, topic: &Topic) -> Result<LearningModule> {
        // Try to parse the response as JSON
        match serde_json::from_str::<serde_json::Value>(&response) {
            Ok(json) => {
                // Extract the fields from the JSON
                let explanation = json["explanation"]
                    .as_str()
                    .unwrap_or("No explanation provided")
                    .to_string();

                let code_snippets = json["code_snippets"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_else(|| vec!["// No code examples provided".to_string()]);

                let exercises = json["exercises"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_else(|| vec!["No exercises provided".to_string()]);

                // Create and return the LearningModule
                Ok(LearningModule {
                    topic: topic.topic.clone(),
                    explanation,
                    code_snippets,
                    exercises,
                })
            },
            Err(e) => {
                // If JSON parsing fails, try to extract content in a more forgiving way
                tracing::warn!("Failed to parse LLM response as JSON: {}", e);

                // Create a simple module with the raw response
                Ok(LearningModule {
                    topic: topic.topic.clone(),
                    explanation: format!("The AI generated a response that couldn't be parsed correctly. Here's the raw response:\n\n{}", response),
                    code_snippets: vec!["// No code examples could be extracted".to_string()],
                    exercises: vec!["No exercises could be extracted".to_string()],
                })
            }
        }
    }
}
