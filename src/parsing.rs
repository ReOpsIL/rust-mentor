// src/parsing.rs
// Parsers for the delimited text formats the prompts ask the LLM to produce.
//
// The marker format (`<<<section: title>>>`) is used instead of JSON output because many
// (free) OpenRouter models don't support `response_format`, and code inside JSON strings
// is a frequent source of escaping errors. The parsers are deliberately lenient.
use crate::model::{CodeSnippet, Exercise};
use crate::question_generator::{AnswerOption, GeneratedApplication, Question, QuestionType};
use anyhow::{Result, bail};
use regex::Regex;
use std::sync::LazyLock;

// Matches answer option lines like "(1) Text" or "(y) Text"
static OPTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*\(([a-zA-Z0-9])\)\s*(.*)$").expect("valid option regex"));

/// The parsed learning module response
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleResponse {
    pub explanation: String,
    pub code_snippets: Vec<CodeSnippet>,
    pub exercises: Vec<Exercise>,
}

/// Whether a line is a Markdown code fence (```` ``` ```` or ```` ```rust ````)
fn is_fence(line: &str) -> bool {
    line.trim_start().starts_with("```")
}

/// Extracts the title from a marker line like `<<<code_snippet 1: Title>>>`
fn marker_title(line: &str) -> String {
    let inner = line.trim().trim_start_matches('<').trim_end_matches('>');
    match inner.split_once(':') {
        Some((_, title)) => title.trim().trim_matches(|c| c == '[' || c == ']').trim().to_string(),
        None => String::new(),
    }
}

/// Extracts a description from lines like `// code snippet: text` or `# code snippet: text`
fn description_line<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let trimmed = line.trim_start();
    let rest = trimmed.strip_prefix("//").or_else(|| trimmed.strip_prefix('#'))?.trim_start();
    let rest = rest.strip_prefix(key)?.trim_start();
    Some(rest.strip_prefix(':').unwrap_or(rest).trim())
}

/// Trims leading/trailing blank lines but keeps indentation
fn trim_block(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(lines.len());
    let end = lines.iter().rposition(|l| !l.trim().is_empty()).map_or(start, |i| i + 1);
    lines[start..end].join("\n")
}

enum ModuleSection {
    None,
    Explanation,
    Snippet,
    Exercise,
}

/// Parses a learning module response with `<<<explanation: …>>>`, `<<<code_snippet n: …>>>`
/// and `<<<exercise n: …>>>` sections
pub fn parse_module_response(response: &str) -> Result<ModuleResponse> {
    let mut explanation = String::new();
    let mut code_snippets: Vec<CodeSnippet> = Vec::new();
    let mut exercises: Vec<Exercise> = Vec::new();
    let mut section = ModuleSection::None;
    let mut found_marker = false;

    for line in response.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<<<explanation") {
            found_marker = true;
            section = ModuleSection::Explanation;
            let title = marker_title(trimmed);
            if !title.is_empty() {
                explanation.push_str(&format!("# {}\n", title));
            }
        } else if trimmed.starts_with("<<<code_snippet") {
            found_marker = true;
            section = ModuleSection::Snippet;
            code_snippets.push(CodeSnippet {
                title: marker_title(trimmed),
                description: String::new(),
                code: String::new(),
            });
        } else if trimmed.starts_with("<<<exercise") {
            found_marker = true;
            section = ModuleSection::Exercise;
            exercises.push(Exercise { name: marker_title(trimmed), description: String::new(), code: String::new() });
        } else {
            match section {
                ModuleSection::None => {}
                ModuleSection::Explanation => {
                    explanation.push_str(line);
                    explanation.push('\n');
                }
                ModuleSection::Snippet => {
                    let Some(snippet) = code_snippets.last_mut() else { continue };
                    if is_fence(line) {
                        continue;
                    }
                    match description_line(line, "code snippet") {
                        Some(description) if snippet.description.is_empty() && snippet.code.trim().is_empty() => {
                            snippet.description = description.to_string();
                        }
                        _ => {
                            snippet.code.push_str(line);
                            snippet.code.push('\n');
                        }
                    }
                }
                ModuleSection::Exercise => {
                    let Some(exercise) = exercises.last_mut() else { continue };
                    if is_fence(line) {
                        continue;
                    }
                    match description_line(line, "exercise description") {
                        Some(description) if exercise.description.is_empty() && exercise.code.trim().is_empty() => {
                            exercise.description = description.to_string();
                        }
                        _ => {
                            exercise.code.push_str(line);
                            exercise.code.push('\n');
                        }
                    }
                }
            }
        }
    }

    if !found_marker {
        bail!("No section markers found in the response");
    }

    for snippet in &mut code_snippets {
        snippet.code = trim_block(&snippet.code);
        if snippet.title.is_empty() {
            snippet.title = "Code example".to_string();
        }
    }
    code_snippets.retain(|s| !s.code.is_empty());

    for exercise in &mut exercises {
        exercise.code = trim_block(&exercise.code);
        if exercise.name.is_empty() {
            exercise.name = "Exercise".to_string();
        }
    }
    exercises.retain(|e| !e.code.is_empty() || !e.description.is_empty());

    Ok(ModuleResponse { explanation: trim_block(&explanation), code_snippets, exercises })
}

/// Parses the questions response (`<<<question:n>>> … <<<end>>>` blocks)
pub fn parse_questions_response(response: &str) -> Result<Vec<Question>> {
    struct Draft {
        text: String,
        question_type: QuestionType,
        options: Vec<AnswerOption>,
    }

    fn finish(draft: Draft, questions: &mut Vec<Question>) {
        let text = draft.text.trim().to_string();
        if text.is_empty() {
            return;
        }
        // A multiple choice question without options can only be answered yes/no
        let question_type = if draft.question_type == QuestionType::Multiple && draft.options.len() < 2 {
            QuestionType::Binary
        } else {
            draft.question_type
        };
        let options = if question_type == QuestionType::Binary { Vec::new() } else { draft.options };
        questions.push(Question { id: questions.len(), text, question_type, options, selected_answer: None });
    }

    let mut questions = Vec::new();
    let mut current: Option<Draft> = None;
    let mut in_options = false;

    for line in response.lines() {
        let line = line.trim();

        if line.starts_with("<<<question") {
            if let Some(draft) = current.take() {
                finish(draft, &mut questions);
            }
            current = Some(Draft { text: String::new(), question_type: QuestionType::Binary, options: Vec::new() });
            in_options = false;
        } else if line.starts_with("<<<end") {
            if let Some(draft) = current.take() {
                finish(draft, &mut questions);
            }
            in_options = false;
        } else if let Some(draft) = current.as_mut() {
            if let Some(type_str) = line.strip_prefix("[TYPE:") {
                draft.question_type = if type_str.to_lowercase().contains("multiple") {
                    QuestionType::Multiple
                } else {
                    QuestionType::Binary
                };
            } else if line.starts_with("[OPTIONS") || line.starts_with("[YESNO") {
                in_options = true;
            } else if in_options || OPTION_RE.is_match(line) {
                // Option lines like "(1) Text"; the last one may carry the closing "]"
                let closes_section = line.ends_with(']');
                let option_line = line.strip_suffix(']').unwrap_or(line);
                if let Some(cap) = OPTION_RE.captures(option_line) {
                    let id = cap[1].to_string();
                    let text = cap[2].trim().to_string();
                    let is_yes_no = matches!(id.as_str(), "y" | "Y" | "n" | "N");
                    if !text.is_empty() && !is_yes_no {
                        draft.options.push(AnswerOption { id, text });
                    }
                }
                if closes_section {
                    in_options = false;
                }
            } else if !line.is_empty() && !line.starts_with('[') {
                // Part of the question text; strip a "Question:" label if present
                let text = line.strip_prefix("Question:").unwrap_or(line).trim();
                if !draft.text.is_empty() {
                    draft.text.push(' ');
                }
                draft.text.push_str(text);
            }
        }
    }

    if let Some(draft) = current.take() {
        finish(draft, &mut questions);
    }

    if questions.is_empty() {
        bail!("No valid questions found in the response");
    }
    Ok(questions)
}

/// Parses the application response (`<<<application_name>>>`, `<<<code_snippet:…>>>` … `<<<end>>>`)
pub fn parse_application_response(response: &str, topic: &str) -> Result<GeneratedApplication> {
    #[derive(PartialEq)]
    enum Section {
        None,
        Name,
        Description,
        Features,
        Code,
    }

    let mut name = String::new();
    let mut description = String::new();
    let mut features = Vec::new();
    let mut code_snippets = Vec::new();
    let mut section = Section::None;
    let mut content = String::new();
    let mut title = String::new();

    let mut close = |section: &Section, content: &str, title: &str| match section {
        Section::Name => name = content.trim().to_string(),
        Section::Description => description = content.trim().to_string(),
        Section::Features => {
            features = content
                .lines()
                .map(str::trim)
                .filter(|l| l.starts_with('-') || l.starts_with('*'))
                .map(|l| l.trim_start_matches(['-', '*']).trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
        }
        Section::Code => {
            let code = trim_block(content);
            if !code.is_empty() {
                code_snippets.push(CodeSnippet {
                    title: if title.is_empty() { "Code".to_string() } else { title.to_string() },
                    description: String::new(),
                    code,
                });
            }
        }
        Section::None => {}
    };

    for line in response.lines() {
        let trimmed = line.trim();
        let new_section = if trimmed.starts_with("<<<application_name") {
            Some(Section::Name)
        } else if trimmed.starts_with("<<<application_description") {
            Some(Section::Description)
        } else if trimmed.starts_with("<<<application_features") {
            Some(Section::Features)
        } else if trimmed.starts_with("<<<code_snippet") {
            Some(Section::Code)
        } else if trimmed.starts_with("<<<end") {
            Some(Section::None)
        } else {
            None
        };

        match new_section {
            Some(next) => {
                close(&section, &content, &title);
                content.clear();
                title = if next == Section::Code { marker_title(trimmed) } else { String::new() };
                section = next;
            }
            None if section != Section::None => {
                if section == Section::Code && is_fence(line) {
                    continue;
                }
                // Keep indentation: this may be code
                content.push_str(line);
                content.push('\n');
            }
            None => {}
        }
    }
    close(&section, &content, &title);

    if name.is_empty() && code_snippets.is_empty() {
        bail!("No application sections found in the response");
    }
    if name.is_empty() {
        name = format!("Rust {} Application", topic);
    }

    Ok(GeneratedApplication { name, description, features, code_snippets })
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODULE_FIXTURE: &str = include_str!("../tests/fixtures/module_response.txt");
    const QUESTIONS_FIXTURE: &str = include_str!("../tests/fixtures/questions_response.txt");
    const APPLICATION_FIXTURE: &str = include_str!("../tests/fixtures/application_response.txt");

    #[test]
    fn parses_module_fixture() {
        let module = parse_module_response(MODULE_FIXTURE).unwrap();
        assert!(module.explanation.starts_with("# Ownership in Rust"));
        // Code fences inside the explanation are kept (it is Markdown)
        assert!(module.explanation.contains("```rust"));

        assert_eq!(module.code_snippets.len(), 2);
        let first = &module.code_snippets[0];
        assert_eq!(first.title, "Moving a String");
        assert_eq!(first.description, "Shows how ownership moves between variables");
        assert!(first.code.starts_with("fn main() {"));
        assert!(first.code.contains("    let s1 = String::from(\"hello\");"), "indentation kept");
        assert!(!first.code.contains("```"), "fences removed from code");
        // Titles containing colons are kept whole
        assert_eq!(module.code_snippets[1].title, "Borrowing: references");

        assert_eq!(module.exercises.len(), 2);
        assert_eq!(module.exercises[0].name, "Fix the move");
        assert_eq!(module.exercises[0].description, "Make this program compile without cloning");
        assert!(module.exercises[1].code.contains("fn longest"));
    }

    #[test]
    fn module_without_markers_is_an_error() {
        assert!(parse_module_response("Sorry, I can't help with that.").is_err());
    }

    #[test]
    fn module_accepts_hash_descriptions() {
        let response = "<<<explanation: T>>>\ntext\n<<<code_snippet 1: A>>>\n# code snippet: desc\nlet x = 1;\n";
        let module = parse_module_response(response).unwrap();
        assert_eq!(module.code_snippets[0].description, "desc");
        assert_eq!(module.code_snippets[0].code, "let x = 1;");
    }

    #[test]
    fn parses_questions_fixture() {
        let questions = parse_questions_response(QUESTIONS_FIXTURE).unwrap();
        assert_eq!(questions.len(), 4);

        let q = &questions[0];
        assert_eq!(q.question_type, QuestionType::Multiple);
        assert_eq!(q.text, "Imagine an app that helps people learn. Which would you rather build?");
        let ids: Vec<_> = q.options.iter().map(|o| o.id.as_str()).collect();
        assert_eq!(ids, ["1", "2", "3", "4"]);
        assert_eq!(q.options[3].text, "A text adventure with a procedurally generated world");

        assert_eq!(questions[1].question_type, QuestionType::Binary);
        assert!(questions[1].options.is_empty());

        // Options without the [OPTIONS header and "(1)Text" without a space
        assert_eq!(questions[2].options.len(), 3);
        assert_eq!(questions[2].options[0].text, "Terminal UI");

        // "multiple" without options falls back to yes/no
        assert_eq!(questions[3].question_type, QuestionType::Binary);
    }

    #[test]
    fn questions_without_markers_is_an_error() {
        assert!(parse_questions_response("no markers here").is_err());
    }

    #[test]
    fn parses_application_fixture() {
        let app = parse_application_response(APPLICATION_FIXTURE, "Ownership").unwrap();
        assert_eq!(app.name, "Borrow Checker Quest");
        assert!(app.description.starts_with("A terminal game"));
        assert_eq!(app.features, ["Levels that teach moves and borrows", "Score tracking"]);
        assert_eq!(app.code_snippets.len(), 2);
        assert_eq!(app.code_snippets[0].title, "Main Code");
        assert!(app.code_snippets[0].code.contains("    println!"), "indentation kept");
        assert!(!app.code_snippets[0].code.contains("```"));
        assert_eq!(app.code_snippets[1].title, "Additional Module 1");
    }

    #[test]
    fn application_without_sections_is_an_error() {
        assert!(parse_application_response("nothing useful", "t").is_err());
    }
}
