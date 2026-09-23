// src/cargo_project.rs
// Writes learning modules and generated applications to disk as Cargo projects.
use crate::model::{CodeSnippet, LearningModule};
use crate::question_generator::GeneratedApplication;
use anyhow::{Context, Result};
use chrono::Local;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Creates a Cargo project for a learning module in `base_dir`:
/// README with the explanation, code snippets as examples, exercises as binaries
pub fn create_cargo_project(base_dir: &Path, module: &LearningModule, level: u8) -> Result<PathBuf> {
    let crate_name = slugify(&module.topic);
    let project_dir = create_unique_dir(base_dir, &format!("{}_{}", crate_name, level))?;
    write_manifest(&project_dir, &crate_name)?;

    let mut used = HashSet::new();
    let examples: Vec<String> = module
        .code_snippets
        .iter()
        .enumerate()
        .map(|(i, snippet)| unique_name(&snippet.title, &format!("example_{}", i + 1), &mut used))
        .collect();
    let exercises: Vec<String> = module
        .exercises
        .iter()
        .enumerate()
        .map(|(i, exercise)| unique_name(&exercise.name, &format!("exercise_{}", i + 1), &mut used))
        .collect();

    for (snippet, name) in module.code_snippets.iter().zip(&examples) {
        let content = format!("// {}\n// {}\n\n{}", snippet.title, snippet.description, with_main(&snippet.code));
        write_file(&project_dir.join("examples").join(format!("{}.rs", name)), &content)?;
    }
    for (exercise, name) in module.exercises.iter().zip(&exercises) {
        let content = format!("// {}\n// {}\n\n{}", exercise.name, exercise.description, with_main(&exercise.code));
        write_file(&project_dir.join("src").join("bin").join(format!("{}.rs", name)), &content)?;
    }

    let mut readme = format!("# {}\n\n{}\n", module.topic, module.explanation);
    if !examples.is_empty() {
        readme.push_str("\n## Examples\n\n");
        for (snippet, name) in module.code_snippets.iter().zip(&examples) {
            readme.push_str(&format!("- {}: `cargo run --example {}`\n", snippet.title, name));
        }
    }
    if !exercises.is_empty() {
        readme.push_str("\n## Exercises\n\n");
        for (exercise, name) in module.exercises.iter().zip(&exercises) {
            readme.push_str(&format!("- {}: `cargo run --bin {}` ({})\n", exercise.name, name, exercise.description));
        }
    }
    write_file(&project_dir.join("README.md"), &readme)?;

    let main = format!(
        "fn main() {{\n    println!(\"{}\");\n    println!(\"See README.md for the examples and exercises.\");\n}}\n",
        module.topic.replace('\\', "\\\\").replace('"', "\\\"")
    );
    write_file(&project_dir.join("src").join("main.rs"), &main)?;

    Ok(project_dir)
}

/// Creates a Cargo project from a generated application in `base_dir`.
/// The first snippet (or the one titled "main") becomes `src/main.rs`; others become examples.
pub fn create_application_project(base_dir: &Path, application: &GeneratedApplication) -> Result<PathBuf> {
    let crate_name = slugify(&application.name);
    let project_dir = create_unique_dir(base_dir, &crate_name)?;
    write_manifest(&project_dir, &crate_name)?;

    let features = application.features.iter().map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n");
    let readme = format!("# {}\n\n{}\n\n## Features\n\n{}\n", application.name, application.description, features);
    write_file(&project_dir.join("README.md"), &readme)?;

    let main_index =
        application.code_snippets.iter().position(|s| s.title.to_lowercase().contains("main")).unwrap_or(0);
    let mut used = HashSet::new();
    let mut wrote_main = false;
    for (i, snippet) in application.code_snippets.iter().enumerate() {
        if i == main_index {
            write_file(&project_dir.join("src").join("main.rs"), &with_main(&snippet.code))?;
            wrote_main = true;
        } else {
            let name = unique_name(&snippet.title, &format!("example_{}", i + 1), &mut used);
            write_snippet_example(&project_dir, &name, snippet)?;
        }
    }
    if !wrote_main {
        write_file(&project_dir.join("src").join("main.rs"), "fn main() {}\n")?;
    }

    Ok(project_dir)
}

fn write_snippet_example(project_dir: &Path, name: &str, snippet: &CodeSnippet) -> Result<()> {
    let content = format!("// {}\n\n{}", snippet.title, with_main(&snippet.code));
    write_file(&project_dir.join("examples").join(format!("{}.rs", name)), &content)
}

fn write_manifest(project_dir: &Path, crate_name: &str) -> Result<()> {
    let manifest =
        format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n", crate_name);
    write_file(&project_dir.join("Cargo.toml"), &manifest)?;
    write_file(&project_dir.join(".gitignore"), "/target\n")
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).with_context(|| format!("Failed to create {}", dir.display()))?;
    }
    fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))
}

/// Adds an empty `main` to code without one, so every example and exercise compiles as a target
fn with_main(code: &str) -> String {
    if code.contains("fn main") {
        format!("{}\n", code.trim_end())
    } else {
        format!("{}\n\nfn main() {{}}\n", code.trim_end())
    }
}

/// Turns a title into a lowercase identifier usable as a directory, crate and target name
fn slugify(title: &str) -> String {
    let slug = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_");

    // Crate names must not be empty or start with a digit
    if slug.is_empty() || slug.starts_with(|c: char| c.is_ascii_digit()) { format!("rm_{}", slug) } else { slug }
}

/// A target name derived from `title` that is not in `used` yet
fn unique_name(title: &str, fallback: &str, used: &mut HashSet<String>) -> String {
    let mut name = slugify(title);
    if name.trim_start_matches("rm_").is_empty() {
        name = fallback.to_string();
    }
    if name.len() > 50 {
        name.truncate(50);
        name = name.trim_end_matches('_').to_string();
    }
    let base = name.clone();
    let mut counter = 1;
    while !used.insert(name.clone()) {
        counter += 1;
        name = format!("{}_{}", base, counter);
    }
    name
}

/// Creates a new directory `[base]_[timestamp]` in `parent`, adding a counter if it already exists
fn create_unique_dir(parent: &Path, base: &str) -> Result<PathBuf> {
    let timestamp = Local::now().format("%Y-%m-%d_%H%M%S");
    let mut project_dir = parent.join(format!("{}_{}", base, timestamp));
    let mut counter = 1;
    while project_dir.exists() {
        counter += 1;
        project_dir = parent.join(format!("{}_{}_{}", base, timestamp, counter));
    }
    fs::create_dir_all(&project_dir)
        .with_context(|| format!("Failed to create project directory {}", project_dir.display()))?;
    Ok(project_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Exercise;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rust-mentor-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn slugify_makes_valid_crate_names() {
        assert_eq!(slugify("Library: std::collections::HashMap"), "library_std_collections_hashmap");
        assert_eq!(slugify("Hello World: Comments"), "hello_world_comments");
        assert_eq!(slugify("1.2 Formatted print"), "rm_1_2_formatted_print");
        assert_eq!(slugify("!!!"), "rm_");
    }

    #[test]
    fn unique_names_do_not_collide() {
        let mut used = HashSet::new();
        assert_eq!(unique_name("Intro", "x", &mut used), "intro");
        assert_eq!(unique_name("Intro", "x", &mut used), "intro_2");
        assert_eq!(unique_name("???", "example_3", &mut used), "example_3");
    }

    #[test]
    fn writes_module_project() {
        let dir = temp_dir("module");
        let module = LearningModule {
            topic: "Ownership: Moves".to_string(),
            explanation: "Values have owners.".to_string(),
            code_snippets: vec![
                CodeSnippet {
                    title: "Move".to_string(),
                    description: "d".to_string(),
                    code: "fn main() {}".to_string(),
                },
                CodeSnippet { title: "Move".to_string(), description: "d".to_string(), code: "fn f() {}".to_string() },
            ],
            exercises: vec![Exercise {
                name: "Fix it".to_string(),
                description: "d".to_string(),
                code: "fn g() {}".to_string(),
            }],
            additional_resources: None,
        };
        let project = create_cargo_project(&dir, &module, 3).unwrap();

        assert!(project.join("Cargo.toml").exists());
        assert!(project.join("src/main.rs").exists());
        assert!(project.join("examples/move.rs").exists());
        let second = fs::read_to_string(project.join("examples/move_2.rs")).unwrap();
        assert!(second.contains("fn main() {}"), "main added to snippet without one");
        assert!(project.join("src/bin/fix_it.rs").exists());
        let readme = fs::read_to_string(project.join("README.md")).unwrap();
        assert!(readme.contains("cargo run --example move_2"));

        // A second project for the same topic gets its own directory
        let again = create_cargo_project(&dir, &module, 3).unwrap();
        assert_ne!(project, again);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn writes_application_project() {
        let dir = temp_dir("application");
        let application = GeneratedApplication {
            name: "Quest".to_string(),
            description: "A game".to_string(),
            features: vec!["Fun".to_string()],
            code_snippets: vec![
                CodeSnippet { title: "Helpers".to_string(), description: String::new(), code: "fn h() {}".to_string() },
                CodeSnippet {
                    title: "Main Code".to_string(),
                    description: String::new(),
                    code: "fn main() {}".to_string(),
                },
            ],
        };
        let project = create_application_project(&dir, &application).unwrap();
        assert_eq!(fs::read_to_string(project.join("src/main.rs")).unwrap(), "fn main() {}\n");
        assert!(project.join("examples/helpers.rs").exists());
        fs::remove_dir_all(&dir).unwrap();
    }
}
