// src/prompts.rs
// Prompt templates. The output formats must stay in sync with `parsing.rs`.
use crate::config::{CodeComplexity, Config, ExplanationVerbosity, FocusArea, LearningGoal, QuestionStyle};
use crate::data::Topic;
use crate::question_generator::{QuestionSet, QuestionType};

pub fn level_description(level: u8) -> &'static str {
    match level {
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
    }
}

/// Prompt for a learning module about `topic`
pub fn learning_module(topic: &Topic, level: u8, config: &Config) -> String {
    let customization = &config.content_customization;
    let level_description = level_description(level);

    let complexity_text = match customization.code_complexity {
        CodeComplexity::Simple => "simple and straightforward",
        CodeComplexity::Moderate => "moderately complex",
        CodeComplexity::Complex => "complex and advanced",
    };

    let verbosity_text = match customization.explanation_verbosity {
        ExplanationVerbosity::Concise => "concise and to-the-point",
        ExplanationVerbosity::Moderate => "moderately detailed",
        ExplanationVerbosity::Detailed => "highly detailed and comprehensive",
    };

    let focus_instruction = match customization.focus_area {
        FocusArea::Concepts => "Focus more on explaining concepts than on code examples or exercises.",
        FocusArea::CodeExamples => "Focus more on providing code examples than on concepts or exercises.",
        FocusArea::Exercises => "Focus more on providing exercises than on concepts or code examples.",
        FocusArea::Balanced => "Provide a balanced mix of concepts, code examples, and exercises.",
    };

    let learning_goal = customization.learning_goal;

    format!(
        r#"
You are an expert Rust programming language tutor and a specialist in generating structured content.
Your task is to create a learning module about the topic '{topic}' for a Rust programmer at the '{level_description}' level (level {level} of 10), with examples from the '{learning_goal}' domain.

**Output Formatting Rules:**
- Output *only* the module, structured with the delimiters shown below. No conversational text before or after it.
- Each delimiter line starts with `<<<` and ends with `>>>` and stands on its own line.
- The explanation is Markdown and may contain fenced Rust code blocks.
- Code snippet and exercise sections contain only valid Rust code (the first line is a `//` description comment).
- Output structure:

<<<explanation: [explanation title]>>>
[Markdown explanation of the topic ...]

<<<code_snippet 1: [code snippet title 1]>>>
// code snippet: [code snippet description 1]
[The actual example code snippet 1 ...]

<<<code_snippet 2: [code snippet title 2]>>>
// code snippet: [code snippet description 2]
[The actual example code snippet 2 ...]

<<<exercise 1: [exercise name 1]>>>
// exercise description: [exercise description 1]
[Starter code for exercise 1, with `todo!()` where the learner should write code ...]

<<<exercise 2: [exercise name 2]>>>
// exercise description: [exercise description 2]
[Starter code for exercise 2 ...]

**Content Guidelines:**
- Explanation: {verbosity_text}, tailored to the '{level_description}' level, using '{learning_goal}' as the application domain.
- Code snippets: several complete, runnable, well-commented Rust programs (each with a `main` function). The code should be {complexity_text}, appropriate for the target level.
- Exercises: several distinct practice exercises with clear problem statements and compilable starter code.
- {focus_instruction}

**Request:**
Generate the learning module for topic '{topic}' at the '{level_description}' level, following all rules above.
Source of this topic: {source}
"#,
        topic = topic.topic,
        source = topic.source,
    )
}

/// Prompt for the questions that shape the generated application
pub fn questions(
    topic: &str,
    level: u8,
    learning_goal: LearningGoal,
    style: QuestionStyle,
    num_questions: usize,
) -> String {
    let level_description = level_description(level);
    let style_instruction = match style {
        QuestionStyle::Mixed => {
            "Mix **binary (yes/no)** and **multiple choice** (with 4 imaginative options) questions."
        }
        QuestionStyle::YesNo => "Ask **only binary (yes/no)** questions (`[TYPE: binary]`).",
        QuestionStyle::MultipleChoice => {
            "Ask **only multiple choice** questions (`[TYPE: multiple]`), each with 4 imaginative options."
        }
    };

    format!(
        r#"
You are **RustMentor**, an AI assistant specialized in teaching Rust programming through hands-on application development.

Your task is to generate {num_questions} creative and engaging questions about `{topic}` in the context of `{learning_goal}`, for a learner at the '{level_description}' level (level {level} of 10).
These questions do not assess knowledge. They **explore the learner's preferences, goals, and inspirations**, and their answers guide the generation of a **unique Rust application** that practices `{topic}` at an appropriate difficulty.

The questions should cover:
* The desired application type or use case
* Preferred features or modules
* Relevant subtopics or technologies
* Possible styles, formats, or interaction modes
* Innovative or unexpected directions the app could take

Rules:
* {style_instruction}
* A question offering several alternatives must be multiple choice, never yes/no.
* Multiple choice options are labeled (1) (2) (3) (4); yes/no questions list (Y) Yes and (N) No.
* Keep ideas within the boundaries of `{topic}` and `{learning_goal}`, and realistic for the learner's level.

**Output only the questions, formatted exactly like this:**

<<<question:1>>>
[question text only - without options]
[TYPE: multiple]
[OPTIONS:
(1) Option 1
(2) Option 2
(3) Option 3
(4) Option 4]
<<<end>>>

<<<question:2>>>
[question text only - without options]
[TYPE: binary]
[YESNO:
(Y) Yes
(N) No]
<<<end>>>
"#
    )
}

/// Prompt for an application based on the answered questions
pub fn application(question_set: &QuestionSet, level: u8) -> String {
    let mut answers = String::new();
    for question in &question_set.questions {
        answers.push_str(&format!("Question: {}\n", question.text));
        if let Some(answer) = &question.selected_answer {
            let option_text = match question.question_type {
                QuestionType::Binary => None,
                QuestionType::Multiple => question.options.iter().find(|opt| &opt.id == answer).map(|o| &o.text),
            };
            match option_text {
                Some(text) => answers.push_str(&format!("Answer: {} ({})\n\n", answer, text)),
                None => answers.push_str(&format!("Answer: {}\n\n", answer)),
            }
        }
    }

    format!(
        r#"You are RustMentor, an AI assistant specialized in teaching Rust programming.

Based on the following questions and answers about {topic}, generate a Rust application that practices the topic for a learner at the '{level_description}' level (level {level} of 10).

{answers}
Generate a Rust application that:
1. Is relevant to the topic and the user's answers
2. Demonstrates the concepts covered in the questions
3. Compiles and runs with only the standard library (no external crates)
4. Fits in a single file, src/main.rs

Format your response exactly as follows, with no text outside the sections:

<<<application_name>>>
[NAME OF THE APPLICATION]
<<<end>>>

<<<application_description>>>
[DESCRIPTION OF THE APPLICATION]
<<<end>>>

<<<application_features>>>
- [FEATURE 1]
- [FEATURE 2]
<<<end>>>

<<<code_snippet:Main Code>>>
[THE WHOLE APPLICATION: CONTENTS OF src/main.rs]
<<<end>>>
"#,
        topic = question_set.topic,
        level_description = level_description(level),
    )
}
