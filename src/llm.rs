// src/llm.rs
use crate::app::LearningModule;
use crate::data::Topic;
use anyhow::Result;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Debug)]
pub struct RawLearningModule {
    pub explanation: String,
    // Use `default` to prevent panics if the key is missing, although the new prompt makes this unlikely.
    #[serde(default = "default_vec_string")]
    pub code_snippets: Vec<String>,
    #[serde(default = "default_vec_string")]
    pub exercises: Vec<String>,
}

fn default_vec_string() -> Vec<String> {
    Vec::new()
}

impl LlmClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    // Generate a learning module based on a topic and user level
    pub async fn generate_learning_module(
        &self,
        topic: &Topic,
        level: u8,
    ) -> Result<LearningModule> {
        // Check if API key is available
        if self.api_key.is_empty() {
            anyhow::bail!(
                "OpenRouter API key is not set. Please set the OPENROUTER_API_KEY environment variable."
            );
        }

        // Create the prompt for the LLM
        let prompt = self.create_prompt(topic, level);

        // Call the OpenRouter API
        let response = self.call_openrouter_api(prompt).await?;

        // Parse the response into a LearningModule
        self.parse_response(response, topic)
    }

    // Create a prompt for the LLM based on the topic and level
    // In your struct impl
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

        // The refined prompt is much more explicit and strict.
        format!(
            r#"
You are an expert Rust programming language tutor and a specialist in generating structured data.
Your task is to create a learning module about the topic '{topic}' for a Rust programmer at the '{level_description}' level.

**Formatting Rules:**
1.  Your ENTIRE response MUST be a single, raw JSON object.
2.  Do NOT include any introductory text, concluding remarks, or explanations outside of the JSON structure.
3.  Do NOT wrap the JSON in markdown code blocks (like ```json ... ```).
4.  Your response must start with `{{` and end with `}}`.
5.  Ensure all strings within the JSON are properly escaped (e.g., use `\\n` for newlines, `\\"` for quotes, `\\\\` for backslashes). This is critical for the code snippets.

**JSON Schema:**
You must adhere strictly to the following JSON structure:
{{
  "explanation": "string",
  "code_snippets": ["string", "string", ...],
  "exercises": ["string", "string", ...]
}}

**Content Guidelines:**
-   `explanation`: Provide a clear and concise explanation of the topic, tailored to the '{level_description}' level.
-   `code_snippets`: Provide at least two complete, runnable, and well-commented Rust code examples. The code's complexity must be appropriate for the target level.
-   `exercises`: Provide two distinct practice exercises. They should be clear problem statements that allow the user to apply the concepts from the explanation and code snippets.

**Request:**
Generate the learning module for topic '{topic}' at the '{level_description}' level, following all rules above.
Source of this topic: {source}
"#,
            topic = topic.topic,
            level_description = level_description,
            source = topic.source
        )
    }

    // Call the OpenRouter API with the prompt
    async fn call_openrouter_api(&self, prompt: String) -> Result<String> {
        // Create the request body
        let request = OpenRouterRequest {
            model: "google/gemini-2.5-pro-preview".to_string(), // You can change this to a different model if needed
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        // Make the API call
        let response = self
            .client
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
    // Assuming RawLearningModule is defined as above
    // And LearningModule is the struct you want to create

    pub(crate) fn parse_response(&self, response: String, topic: &Topic) -> Result<LearningModule> {
        // Check if the response is wrapped in a code block and extract the JSON
        let json_str = if response.trim_start().starts_with("```json")
            && response.trim_end().ends_with("```")
        {
            // Extract the content between the code block markers
            let start_idx = response.find('{').unwrap_or(0);
            let end_idx = response.rfind('}').unwrap_or(response.len());

            if start_idx < end_idx {
                &response[start_idx..=end_idx]
            } else {
                &response
            }
        } else {
            &response
        };
        
        // Try to parse the response directly into our target struct
        match serde_json::from_str::<RawLearningModule>(&*json_str) {
            Ok(raw_module) => {
                // Now you have a strongly-typed struct.
                // You can add extra checks here if you want (e.g., ensure vecs are not empty).
                Ok(LearningModule {
                    topic: topic.topic.clone(),
                    explanation: raw_module.explanation,
                    code_snippets: if raw_module.code_snippets.is_empty() {
                        vec!["// No code examples provided".to_string()]
                    } else {
                        raw_module.code_snippets
                    },
                    exercises: if raw_module.exercises.is_empty() {
                        vec!["No exercises provided".to_string()]
                    } else {
                        raw_module.exercises
                    },
                })
            }
            Err(e) => {
                // This fallback will now be triggered far less often.
                tracing::warn!("Failed to parse LLM response as JSON: {}", e);
                tracing::debug!("Problematic response body: {}", response); // Log the body for debugging

                // Your existing fallback logic is fine.
                Ok(LearningModule {
                    topic: topic.topic.clone(),
                    explanation: format!(
                        "The AI generated a response that couldn't be parsed correctly. Here's the raw response:\n\n{}",
                        response
                    ),
                    code_snippets: vec!["// No code examples could be extracted".to_string()],
                    exercises: vec!["No exercises could be extracted".to_string()],
                })
            }
        }
    }

    #[cfg(test)]
    pub fn test_parse_response_with_code_block(&self) -> Result<()> {
        // Test response with code block markers
        let response = r#"```json
{
  "explanation": "Test explanation",
  "code_snippets": ["Test code snippet"],
  "exercises": ["Test exercise"]
}
```"#;

        // Create a dummy topic
        let topic = Topic {
            topic: "Test Topic".to_string(),
            source: "Test Source".to_string(),
            min_level: 5,
        };

        // Call parse_response
        let result = self.parse_response(response.to_string(), &topic)?;

        // Verify the result
        assert_eq!(result.topic, "Test Topic");
        assert_eq!(result.explanation, "Test explanation");
        assert_eq!(result.code_snippets.len(), 1);
        assert_eq!(result.code_snippets[0], "Test code snippet");
        assert_eq!(result.exercises.len(), 1);
        assert_eq!(result.exercises[0], "Test exercise");

        Ok(())
    }
}
