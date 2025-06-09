// src/llm.rs
use crate::app::LearningModule;
use crate::data::Topic;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::prompt_response::{CodeSnippet, Exercise, PromptResponse};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub id: String,
    pub name: String,
}


impl LlmClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn list_models(&self) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
        let url = "https://openrouter.ai/api/v1/models";

        let resp = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        let json: serde_json::Value = resp.json().await?;
        let models: Vec<Model> = serde_json::from_value(json["data"].clone())?;

        Ok(models)
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

        // Save the response to a file for debugging
        //let current_datetime = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        //let filename = format!("response_{}.txt", current_datetime);
        //std::fs::write(&filename, &response)?;
        // Parse the response into a LearningModule
        self.parse_response(response, topic)
    }

    // Create a prompt for the LLM based on the topic, level, and customization options
    // In your struct impl
    fn create_prompt(&self, topic: &Topic, level: u8) -> String {
        let config = Config::load().unwrap();
        let customization = config.content_customization;

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

        // Get customization settings as text
        let complexity_text = match customization.code_complexity {
            crate::config::CodeComplexity::Simple => "simple and straightforward",
            crate::config::CodeComplexity::Moderate => "moderately complex",
            crate::config::CodeComplexity::Complex => "complex and advanced",
        };

        let verbosity_text = match customization.explanation_verbosity {
            crate::config::ExplanationVerbosity::Concise => "concise and to-the-point",
            crate::config::ExplanationVerbosity::Moderate => "moderately detailed",
            crate::config::ExplanationVerbosity::Detailed => "highly detailed and comprehensive",
        };

        let focus_instruction = match customization.focus_area {
            crate::config::FocusArea::Concepts => "Focus more on explaining concepts than on code examples or exercises.",
            crate::config::FocusArea::CodeExamples => "Focus more on providing code examples than on concepts or exercises.",
            crate::config::FocusArea::Exercises => "Focus more on providing exercises than on concepts or code examples.",
            crate::config::FocusArea::Balanced => "Provide a balanced mix of concepts, code examples, and exercises.",
        };

        // The refined prompt is much more explicit and strict.
        format!(
            r#"
You are an expert Rust programming language tutor and a specialist in generating structured data.
Your task is to create a learning module about the topic '{topic}' for a Rust programmer at the '{level_description}' level.

**Output Formatting Rules:**
- Your output should be *only* the text of the prompt described above.
- Do not include any conversational text or explanations outside of the prompt itself.
- Structure the entire output clearly using the following delimiters.
- Ensure the "explanation",  "exercise descriptions" sections are valid Markdown.
- Ensure "exercise code"  sections are valid RUST language code.
- Output Structure Structure: 

    ```
    <<<explanation: [explanation title]>>>
    [Detailed explanation for the topic ...]

    <<<code_snippet 1: [code snippets title 1]>>>
    // code snippet: [ code snippet description 1]
    [ The actual example code snippet 1 ... ]

    <<<code_snippet 2: [code snippet title 2]>>>
    // code snippet: [ code snippet description 2]
    [ The actual example code snippet 2 ... ]

    <<<code_snippet n: [code snippet title n]>>>
    // code snippet: [ code snippet description n]
    [ The actual example code snippet n ... ]

    <<<exercise 1: [ exercise name 1 ]>>>
    // exercise description: [  exercise description 1 ]
    [ The actual exercise 1 code ... ]

    <<<exercise 2: [ exercise name 2 ]>>>
    // exercise description: [ exercise description 2] 
    [ The actual exercise 2 code ... ]

    <<<exercise n: [ exercise name n ]>>>
    // exercise description: [ exercise description n] 
    [ The actual exercise n code ... ]

    ```

**Content Guidelines:**
-   `explanation`: Provide a {verbosity_text} explanation of the topic, tailored to the '{level_description}' level.
-   `code_snippets`: Provide several complete, runnable, and well-commented Rust code examples. The code should be {complexity_text}, appropriate for the target level.
-   `exercises`: Provide several distinct practice exercises. They should be clear problem statements that allow the user to apply the concepts from the explanation and code snippets.
-   {focus_instruction}

**Request:**
Generate the learning module for topic '{topic}' at the '{level_description}' level, following all rules above.
Source of this topic: {source}
"#,
            topic = topic.topic,
            level_description = level_description,
            source = topic.source,
            verbosity_text = verbosity_text,
            complexity_text = complexity_text,
            focus_instruction = focus_instruction
        )
    }

    // Call the OpenRouter API with the prompt
    async fn call_openrouter_api(&self, prompt: String) -> Result<String> {
        // Create the request body
        let model_id = Config::load().unwrap().model;
        let request = OpenRouterRequest {
            model: model_id, // You can change this to a different model if needed
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

        let prompt_res = PromptResponse::parse_response(response.clone());
        match prompt_res {
            Ok(prompt_res) => {
                // Now you have a strongly-typed struct.
                // You can add extra checks here if you want (e.g., ensure vecs are not empty).
                Ok(LearningModule {
                    topic: topic.topic.clone(),
                    explanation: prompt_res.explanation,
                    code_snippets: if prompt_res.code_snippets.is_empty() {
                        vec![CodeSnippet {
                            title: "No code examples provided".to_string(),
                            description: "// No code examples provided".to_string(),
                            code: "// No code examples provided".to_string(),
                        }]
                    } else {
                        prompt_res.code_snippets
                    },
                    exercises: if prompt_res.exercises.is_empty() {
                        vec![Exercise {
                            name: "No exercises provided".to_string(),
                            description: "// No exercises provided".to_string(),
                            code: "// No exercises provided".to_string(),
                            }
                        ]
                    } else {
                        prompt_res.exercises
                    },
                    additional_resources: None, // Will be populated by the App when displayed
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
                    code_snippets:  vec![CodeSnippet {
                        title: "Error - No code examples extracted.".to_string(),
                        description: "// No code examples extracted".to_string(),
                        code: "// No code examples extracted".to_string(),
                    }],
                    exercises: vec![Exercise {
                        name: "Error - No exercises extracted from LLM response".to_string(),
                        description: "// No exercises ...".to_string(),
                        code: "// No code provided".to_string(),
                    }],
                    additional_resources: None,
                })
            }
        }
    }
}
