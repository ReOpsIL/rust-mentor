// src/question_generator.rs
use crate::app::LearningGoal;
use crate::llm::LlmClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a question type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestionType {
    Binary,     // Yes/No questions
    Multiple,   // Multiple choice questions (1-4 or a-d)
}

impl fmt::Display for QuestionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuestionType::Binary => write!(f, "Yes/No"),
            QuestionType::Multiple => write!(f, "Multiple Choice"),
        }
    }
}

/// Represents an answer option for multiple choice questions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerOption {
    pub id: String,       // "1", "2", "3", "4" or "a", "b", "c", "d"
    pub text: String,     // The answer text
}

/// Represents a question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: usize,
    pub text: String,
    pub question_type: QuestionType,
    pub options: Vec<AnswerOption>,  // Empty for binary questions
    pub selected_answer: Option<String>, // The user's selected answer
}

/// Represents a set of questions for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionSet {
    pub topic: String,
    pub questions: Vec<Question>,
    pub current_question_index: usize,
}

impl QuestionSet {
    pub fn new(topic: String, questions: Vec<Question>) -> Self {
        Self {
            topic,
            questions,
            current_question_index: 0,
        }
    }

    pub fn current_question(&self) -> Option<&Question> {
        self.questions.get(self.current_question_index)
    }

    pub fn current_question_mut(&mut self) -> Option<&mut Question> {
        self.questions.get_mut(self.current_question_index)
    }

    pub fn next_question(&mut self) -> Option<&Question> {
        if self.current_question_index < self.questions.len() - 1 {
            self.current_question_index += 1;
            self.current_question()
        } else {
            None
        }
    }

    pub fn previous_question(&mut self) -> Option<&Question> {
        if self.current_question_index > 0 {
            self.current_question_index -= 1;
            self.current_question()
        } else {
            None
        }
    }

    pub fn is_complete(&self) -> bool {
        self.questions.iter().all(|q| q.selected_answer.is_some())
    }

    pub fn progress(&self) -> (usize, usize) {
        let answered = self.questions.iter().filter(|q| q.selected_answer.is_some()).count();
        (answered, self.questions.len())
    }
}

/// Represents the application to be generated based on user answers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedApplication {
    pub name: String,
    pub description: String,
    pub features: Vec<String>,
    pub code_snippets: Vec<crate::prompt_response::CodeSnippet>,
}

/// The question generator module
#[derive(Clone)]
pub struct QuestionGenerator {
    llm_client: LlmClient,
}

impl QuestionGenerator {
    pub fn new(llm_client: LlmClient) -> Self {
        Self { llm_client }
    }

    /// Generate a set of questions for a specific topic
    pub async fn generate_questions(&self, topic: &str, learning_goal: &LearningGoal, num_questions: usize) -> Result<QuestionSet> {
        // Create a prompt for the LLM to generate questions
        let prompt = self.create_questions_prompt(topic, learning_goal, num_questions);
        
        // Call the LLM API
        let response = self.llm_client.call_openrouter_api(prompt).await?;
        
        // Parse the response into a QuestionSet
        let questions = self.parse_questions_response(response, topic)?;
        
        Ok(QuestionSet::new(topic.to_string(), questions))
    }

    /// Create a prompt for generating questions
    fn create_questions_prompt(&self, topic: &str, learning_goal: &LearningGoal, num_questions: usize) -> String {
        format!(
            r#"You are RustMentor, an AI assistant specialized in teaching Rust programming.

I need you to generate {num_questions} questions about {topic} in the context of {learning_goal}.

Generate a mix of binary (yes/no) and multiple choice questions (with 4 options each).
The questions will eventualy guide an LLM how to create a sample application related to the user requested topic and as part of the user learning goal. The questions should be innovative, can relate to cool features, sub subjects etc.. bu all related to the learned topic and learning goal.

For each question, provide:
1. The question text
2. The question type (binary or multiple)
3. For multiple choice questions, provide 4 options labeled 1-4

Format your response as follows:

<<<question:1>>>
[QUESTION TEXT]
[TYPE: binary/multiple]
[OPTIONS (only for multiple choice):
1. Option 1
2. Option 2
3. Option 3
4. Option 4]
<<<end>>>

<<<question:2>>>
...

Make sure the questions are relevant to {topic} and {learning_goal}, and cover different aspects of the topic.
The questions should help assess the user's understanding of the topic and guide the generation of a Rust application.
"#
        )
    }

    /// Parse the LLM response into a list of questions
    fn parse_questions_response(&self, response: String, _topic: &str) -> Result<Vec<Question>> {
        let mut questions = Vec::new();
        let mut current_question = String::new();
        let mut current_type = QuestionType::Binary;
        let mut current_options = Vec::new();
        let mut in_question = false;
        let mut in_options = false;

        for line in response.lines() {
            let line = line.trim();
            
            if line.starts_with("<<<question:") && line.ends_with(">>>") {
                // Start of a new question - first save the previous question if it exists
                if !current_question.is_empty() {
                    questions.push(Question {
                        id: questions.len(),
                        text: current_question.trim().to_string(),
                        question_type: current_type.clone(),
                        options: current_options.clone(),
                        selected_answer: None,
                    });
                }
                
                // Reset for the new question
                in_question = true;
                in_options = false;
                current_question = String::new();
                current_type = QuestionType::Binary;
                current_options = Vec::new();
            } else if line.starts_with("<<<end>>>") {
                // End of the current question
                if !current_question.is_empty() {
                    questions.push(Question {
                        id: questions.len(),
                        text: current_question.trim().to_string(),
                        question_type: current_type.clone(),
                        options: current_options.clone(),
                        selected_answer: None,
                    });
                }
                
                // Reset state
                in_question = false;
                in_options = false;
                current_question = String::new();
                current_type = QuestionType::Binary;
                current_options = Vec::new();
            } else if in_question {
                if line.starts_with("[TYPE:") {
                    // Parse the question type
                    let type_str = line.replace("[TYPE:", "").replace("]", "").trim().to_lowercase();
                    current_type = if type_str.contains("multiple") {
                        QuestionType::Multiple
                    } else {
                        QuestionType::Binary
                    };
                } else if line.starts_with("[OPTIONS") {
                    // Start of options section
                    in_options = true;
                } else if line.ends_with("]") && in_options {
                    // End of options section
                    in_options = false;
                } else if in_options && current_type == QuestionType::Multiple {
                    // Parse option lines - look for patterns like "1. Text" or "a. Text"
                    if let Some(pos) = line.find('.') {
                        let id = line[..pos].trim();
                        let text = line[pos + 1..].trim();
                        
                        if !id.is_empty() && !text.is_empty() {
                            current_options.push(AnswerOption {
                                id: id.to_string(),
                                text: text.to_string(),
                            });
                        }
                    }
                } else if !line.starts_with("[") && !in_options {
                    // This is part of the question text
                    if !current_question.is_empty() {
                        current_question.push(' ');
                    }
                    current_question.push_str(line);
                }
            }
        }

        // Handle the last question if we ended inside one
        if in_question && !current_question.is_empty() {
            questions.push(Question {
                id: questions.len(),
                text: current_question.trim().to_string(),
                question_type: current_type,
                options: current_options,
                selected_answer: None,
            });
        }

        // Ensure we have at least one question
        if questions.is_empty() {
            return Err(anyhow::anyhow!("No valid questions found in response"));
        }

        Ok(questions)
    }

    /// Generate an application based on user answers
    pub async fn generate_application(&self, question_set: &QuestionSet) -> Result<GeneratedApplication> {
        // Create a prompt for the LLM to generate an application
        let prompt = self.create_application_prompt(question_set);
        
        // Call the LLM API
        let response = self.llm_client.call_openrouter_api(prompt).await?;
        
        // Parse the response into a GeneratedApplication
        self.parse_application_response(response, &question_set.topic)
    }

    /// Create a prompt for generating an application
    fn create_application_prompt(&self, question_set: &QuestionSet) -> String {
        let mut prompt = format!(
            r#"You are RustMentor, an AI assistant specialized in teaching Rust programming.

Based on the following questions and answers about {}, I need you to generate a Rust application that demonstrates the concepts covered.

"#,
            question_set.topic
        );

        // Add each question and its answer to the prompt
        for question in &question_set.questions {
            prompt.push_str(&format!("Question: {}\n", question.text));
            
            if let Some(answer) = &question.selected_answer {
                match question.question_type {
                    QuestionType::Binary => {
                        prompt.push_str(&format!("Answer: {}\n\n", answer));
                    }
                    QuestionType::Multiple => {
                        // Find the selected option
                        if let Some(option) = question.options.iter().find(|opt| &opt.id == answer) {
                            prompt.push_str(&format!("Answer: {} ({})\n\n", answer, option.text));
                        } else {
                            prompt.push_str(&format!("Answer: {}\n\n", answer));
                        }
                    }
                }
            }
        }

        prompt.push_str(r#"
Generate a Rust application that:
1. Is relevant to the topic and the user's answers
2. Demonstrates the concepts covered in the questions
3. Is functional and can be compiled and run

Format your response as follows:

<<<application_name>>>
[NAME OF THE APPLICATION]
<<<end>>>

<<<application_description>>>
[DESCRIPTION OF THE APPLICATION]
<<<end>>>

<<<application_features>>>
- [FEATURE 1]
- [FEATURE 2]
- ...
<<<end>>>

<<<code_snippet:Main Code>>>
[MAIN CODE OF THE APPLICATION]
<<<end>>>

<<<code_snippet:Additional Module 1>>>
[CODE FOR ADDITIONAL MODULE]
<<<end>>>

You can include more code snippets as needed.
"#);

        prompt
    }

    /// Parse the LLM response into a GeneratedApplication
    fn parse_application_response(&self, response: String, topic: &str) -> Result<GeneratedApplication> {
        let mut name = String::new();
        let mut description = String::new();
        let mut features = Vec::new();
        let mut code_snippets = Vec::new();
        
        let mut current_section = String::new();
        let mut current_content = String::new();
        let mut current_title = String::new();
        
        for line in response.lines() {
            let line = line.trim();
            
            if line.starts_with("<<<application_name>>>") {
                current_section = "name".to_string();
                current_content = String::new();
            } else if line.starts_with("<<<application_description>>>") {
                current_section = "description".to_string();
                current_content = String::new();
            } else if line.starts_with("<<<application_features>>>") {
                current_section = "features".to_string();
                current_content = String::new();
            } else if line.starts_with("<<<code_snippet:") {
                if !current_title.is_empty() && !current_content.is_empty() && current_section == "code_snippet" {
                    code_snippets.push(crate::prompt_response::CodeSnippet {
                        title: current_title.clone(),
                        description: String::new(), // We don't have descriptions in this format
                        code: current_content.clone(),
                    });
                }
                
                current_section = "code_snippet".to_string();
                current_content = String::new();
                
                // Extract the snippet title
                if let Some(title_part) = line.split(':').nth(1) {
                    current_title = title_part.trim_end_matches(">>>").trim().to_string();
                }
            } else if line.starts_with("<<<end>>>") {
                match current_section.as_str() {
                    "name" => name = current_content.trim().to_string(),
                    "description" => description = current_content.trim().to_string(),
                    "features" => {
                        // Parse features (one per line, starting with - or *)
                        features = current_content
                            .lines()
                            .filter(|l| l.trim().starts_with('-') || l.trim().starts_with('*'))
                            .map(|l| l.trim_start_matches('-').trim_start_matches('*').trim().to_string())
                            .collect();
                    }
                    "code_snippet" => {
                        if !current_title.is_empty() && !current_content.is_empty() {
                            code_snippets.push(crate::prompt_response::CodeSnippet {
                                title: current_title.clone(),
                                description: String::new(), // We don't have descriptions in this format
                                code: current_content.clone(),
                            });
                        }
                        current_title = String::new();
                    }
                    _ => {}
                }
                
                current_section = String::new();
                current_content = String::new();
            } else if !current_section.is_empty() {
                current_content.push_str(line);
                current_content.push('\n');
            }
        }
        
        // If name is empty, use a default
        if name.is_empty() {
            name = format!("Rust {} Application", topic);
        }
        
        Ok(GeneratedApplication {
            name,
            description,
            features,
            code_snippets,
        })
    }
}