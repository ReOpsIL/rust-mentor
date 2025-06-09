use std::fs;
use std::error::Error;

// Define the structs from prompt_response.rs
#[derive(Debug, Clone)]
struct PromptResponse {
    explanation: String,
    code_snippets: Vec<CodeSnippet>,
    exercises: Vec<Exercise>,
}

#[derive(Debug, Clone)]
struct CodeSnippet {
    title: String,
    description: String,
    code: String,
}

#[derive(Debug, Clone)]
struct Exercise {
    name: String,
    description: String,
    code: String,
}

// Define the parse_response function
impl PromptResponse {
    fn parse_response(response: String) -> Result<Self, String> {
        let mut explanation = String::new();
        let mut code_snippets = Vec::new();
        let mut exercises = Vec::new();

        let mut current_section = String::new();
        let mut current_code_snippet = CodeSnippet {
            title: String::new(),
            description: String::new(),
            code: String::new(),
        };
        let mut current_exercise = Exercise {
            name: String::new(),
            description: String::new(),
            code: String::new(),
        };

        for line in response.lines() {
            if line.starts_with("<<<explanation:") {
                current_section = "explanation".to_string();
                // Extract the explanation title
                let title_part = line.trim_start_matches("<<<explanation:").trim_matches('>').trim();
                explanation.push_str(title_part);
                explanation.push('\n');
            } else if line.starts_with("<<<code_snippet") {
                if !current_code_snippet.title.is_empty() && !current_code_snippet.code.is_empty() {
                    code_snippets.push(current_code_snippet.clone());
                    current_code_snippet = CodeSnippet {
                        title: String::new(),
                        description: String::new(),
                        code: String::new(),
                    };
                }
                current_section = "code_snippet".to_string();

                // Extract the code snippet title, handling the numbering
                if let Some(title_part) = line.split(':').nth(1) {
                    current_code_snippet.title = title_part.trim_matches('>').trim().to_string();
                }
            } else if line.starts_with("<<<exercise") {
                if !current_exercise.name.is_empty() && !current_exercise.code.is_empty() {
                    exercises.push(current_exercise.clone());
                    current_exercise = Exercise {
                        name: String::new(),
                        description: String::new(),
                        code: String::new(),
                    };
                }
                current_section = "exercise".to_string();

                // Extract the exercise name, handling the numbering
                if let Some(name_part) = line.split(':').nth(1) {
                    current_exercise.name = name_part.trim_matches('>').trim().to_string();
                }
            } else if line.starts_with("# code snippet:") {
                if current_section == "code_snippet" {
                    current_code_snippet.description = line.trim_start_matches("# code snippet:").trim().to_string();
                }
            } else if line.starts_with("# exercise description:") {
                if current_section == "exercise" {
                    current_exercise.description = line.trim_start_matches("# exercise description:").trim().to_string();
                }
            } else {
                match current_section.as_str() {
                    "explanation" => {
                        explanation.push_str(line);
                        explanation.push('\n');
                    },
                    "code_snippet" => {
                        current_code_snippet.code.push_str(line);
                        current_code_snippet.code.push('\n');
                    },
                    "exercise" => {
                        current_exercise.code.push_str(line);
                        current_exercise.code.push('\n');
                    },
                    _ => (),
                }
            }
        }

        // Push the last code snippet or exercise if they are not empty
        if !current_code_snippet.title.is_empty() && !current_code_snippet.code.is_empty() {
            code_snippets.push(current_code_snippet);
        }
        if !current_exercise.name.is_empty() && !current_exercise.code.is_empty() {
            exercises.push(current_exercise);
        }

        Ok(PromptResponse {
            explanation,
            code_snippets,
            exercises,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the example response file
    let response = fs::read_to_string("response_2025-06-08_10-59-53.txt")?;

    // Parse the response
    match PromptResponse::parse_response(response) {
        Ok(prompt_response) => {
            // Print the parsed response
            println!("Explanation: {}", prompt_response.explanation);
            println!("\nCode Snippets:");
            for (i, snippet) in prompt_response.code_snippets.iter().enumerate() {
                println!("  Snippet {}: {}", i + 1, snippet.title);
                println!("  Description: {}", snippet.description);
                println!("  Code length: {} characters", snippet.code.len());
                println!("  First 100 chars of code: {}", &snippet.code[..100.min(snippet.code.len())]);
                println!();
            }
            println!("\nExercises:");
            for (i, exercise) in prompt_response.exercises.iter().enumerate() {
                println!("  Exercise {}: {}", i + 1, exercise.name);
                println!("  Description: {}", exercise.description);
                println!("  Code length: {} characters", exercise.code.len());
                println!("  First 100 chars of code: {}", &exercise.code[..100.min(exercise.code.len())]);
                println!();
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error parsing response: {}", e);
            Err(e.into())
        }
    }
}
