// src/question_generator.rs
use crate::config::{LearningGoal, QuestionStyle};
use crate::llm::LlmClient;
use crate::model::CodeSnippet;
use crate::{parsing, prompts};
use anyhow::Result;

/// Represents a question type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuestionType {
    Binary,   // Yes/No questions
    Multiple, // Multiple choice questions (1-4)
}

/// Represents an answer option for multiple choice questions
#[derive(Debug, Clone, PartialEq)]
pub struct AnswerOption {
    pub id: String, // "1", "2", "3", "4"
    pub text: String,
}

/// Represents a question
#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    pub id: usize,
    pub text: String,
    pub question_type: QuestionType,
    pub options: Vec<AnswerOption>,      // Empty for binary questions
    pub selected_answer: Option<String>, // The user's selected answer
}

impl Question {
    /// Records the answer for a key press. Returns false if the key is not a valid answer.
    /// Multiple choice accepts the option id or a-d for options 1-4.
    pub fn answer(&mut self, key: char) -> bool {
        let answer = match self.question_type {
            QuestionType::Binary => match key.to_ascii_lowercase() {
                'y' => Some("Yes".to_string()),
                'n' => Some("No".to_string()),
                _ => None,
            },
            QuestionType::Multiple => {
                let letter_index = ('a'..='z').position(|c| c == key.to_ascii_lowercase());
                self.options
                    .iter()
                    .enumerate()
                    .find(|(i, option)| {
                        option.id.eq_ignore_ascii_case(&key.to_string())
                            || (option.id.chars().all(|c| c.is_ascii_digit()) && letter_index == Some(*i))
                    })
                    .map(|(_, option)| option.id.clone())
            }
        };
        match answer {
            Some(answer) => {
                self.selected_answer = Some(answer);
                true
            }
            None => false,
        }
    }
}

/// Represents a set of questions for a specific topic
#[derive(Debug, Clone)]
pub struct QuestionSet {
    pub topic: String,
    pub questions: Vec<Question>,
    pub current_question_index: usize,
}

impl QuestionSet {
    pub fn new(topic: String, questions: Vec<Question>) -> Self {
        Self { topic, questions, current_question_index: 0 }
    }

    pub fn current_question(&self) -> Option<&Question> {
        self.questions.get(self.current_question_index)
    }

    pub fn current_question_mut(&mut self) -> Option<&mut Question> {
        self.questions.get_mut(self.current_question_index)
    }

    pub fn next_question(&mut self) {
        if self.current_question_index + 1 < self.questions.len() {
            self.current_question_index += 1;
        }
    }

    pub fn previous_question(&mut self) {
        self.current_question_index = self.current_question_index.saturating_sub(1);
    }

    /// Moves to the first unanswered question after the current one, if any
    pub fn advance_to_unanswered(&mut self) {
        let len = self.questions.len();
        for offset in 1..len {
            let index = (self.current_question_index + offset) % len;
            if self.questions[index].selected_answer.is_none() {
                self.current_question_index = index;
                return;
            }
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

/// Represents the application generated from the user's answers
#[derive(Debug, Clone)]
pub struct GeneratedApplication {
    pub name: String,
    pub description: String,
    pub features: Vec<String>,
    pub code_snippets: Vec<CodeSnippet>,
}

/// Parameters for generating questions
pub struct QuestionRequest {
    pub model: String,
    pub topic: String,
    pub level: u8,
    pub learning_goal: LearningGoal,
    pub style: QuestionStyle,
    pub num_questions: usize,
}

/// Generates questions and applications with the LLM
#[derive(Clone)]
pub struct QuestionGenerator {
    llm_client: LlmClient,
}

impl QuestionGenerator {
    pub fn new(llm_client: LlmClient) -> Self {
        Self { llm_client }
    }

    /// Generate a set of questions for a specific topic
    pub async fn generate_questions(
        &self,
        request: QuestionRequest,
        on_text: impl FnMut(&str) + Send,
    ) -> Result<QuestionSet> {
        let prompt = prompts::questions(
            &request.topic,
            request.level,
            request.learning_goal,
            request.style,
            request.num_questions,
        );
        let response = self.llm_client.complete_streaming(&request.model, prompt, on_text).await?;
        let questions = parsing::parse_questions_response(&response).inspect_err(|_| {
            tracing::debug!("Unparseable questions response: {}", response);
        })?;
        Ok(QuestionSet::new(request.topic, questions))
    }

    /// Generate an application based on user answers
    pub async fn generate_application(
        &self,
        model: &str,
        question_set: &QuestionSet,
        level: u8,
        on_text: impl FnMut(&str) + Send,
    ) -> Result<GeneratedApplication> {
        let prompt = prompts::application(question_set, level);
        let response = self.llm_client.complete_streaming(model, prompt, on_text).await?;
        parsing::parse_application_response(&response, &question_set.topic).inspect_err(|_| {
            tracing::debug!("Unparseable application response: {}", response);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn multiple_choice() -> Question {
        Question {
            id: 0,
            text: "Pick".to_string(),
            question_type: QuestionType::Multiple,
            options: (1..=4).map(|i| AnswerOption { id: i.to_string(), text: format!("Option {}", i) }).collect(),
            selected_answer: None,
        }
    }

    #[test]
    fn multiple_choice_accepts_numbers_and_letters() {
        let mut q = multiple_choice();
        assert!(q.answer('3'));
        assert_eq!(q.selected_answer.as_deref(), Some("3"));
        assert!(q.answer('b'));
        assert_eq!(q.selected_answer.as_deref(), Some("2"));
        assert!(!q.answer('5'));
        assert!(!q.answer('e'));
        assert_eq!(q.selected_answer.as_deref(), Some("2"));
    }

    #[test]
    fn binary_accepts_yes_and_no() {
        let mut q = Question { question_type: QuestionType::Binary, options: vec![], ..multiple_choice() };
        assert!(q.answer('Y'));
        assert_eq!(q.selected_answer.as_deref(), Some("Yes"));
        assert!(q.answer('n'));
        assert_eq!(q.selected_answer.as_deref(), Some("No"));
        assert!(!q.answer('1'));
    }

    #[test]
    fn advances_to_next_unanswered_question() {
        let mut set = QuestionSet::new("t".to_string(), vec![multiple_choice(), multiple_choice(), multiple_choice()]);
        set.questions[1].selected_answer = Some("1".to_string());
        set.advance_to_unanswered();
        assert_eq!(set.current_question_index, 2);
        set.questions[2].selected_answer = Some("1".to_string());
        set.advance_to_unanswered();
        assert_eq!(set.current_question_index, 0);
    }
}
