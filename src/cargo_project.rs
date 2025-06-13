use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use chrono::Local;
use anyhow::{Result, Context};

use crate::app::LearningModule;
use crate::question_generator::GeneratedApplication;

/// Creates a Cargo project for a learning module
pub fn create_cargo_project(module: &LearningModule, level: u8) -> Result<PathBuf> {
    // Create directory name in the format [topic]_[level]_[date]
    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let topic_slug = module.topic
        .replace(|c| !char::is_alphanumeric(c), "_").to_lowercase();

    let dir_name = format!("{}_{}_{}",
        topic_slug,
        level,
        current_date
    );

    // Create the directory
    let project_dir = PathBuf::from(&dir_name);
    fs::create_dir_all(&project_dir).context("Failed to create project directory")?;

    // Initialize Cargo project
    initialize_cargo_project(&project_dir, topic_slug.as_str())?;

    // Create markdown file with explanation
    create_explanation_file(&project_dir, module)?;

    // Create docs folder and save explanation as markdown file
    create_docs_explanation_file(&project_dir, module)?;

    // Create Rust files for code snippets
    create_code_snippet_files(&project_dir, module)?;

    // Create Rust files for exercises
    create_exercise_files(&project_dir, module)?;

    Ok(project_dir)
}

/// Initializes a new Cargo project in the given directory
fn initialize_cargo_project(project_dir: &Path, project_name: &str) -> Result<()> {
    let cargo_cmd = Command::new("cargo")
        .current_dir(project_dir)
        .arg("init")
        .arg("--name")
        .arg(project_name.to_lowercase().replace(' ', "_"))
        .output()
        .context("Failed to execute cargo init command")?;

    if !cargo_cmd.status.success() {
        let error = String::from_utf8_lossy(&cargo_cmd.stderr);
        anyhow::bail!("Failed to initialize Cargo project: {}", error);
    }

    Ok(())
}

/// Creates a markdown file with the explanation content
fn create_explanation_file(project_dir: &Path, module: &LearningModule) -> Result<()> {
    let explanation_path = project_dir.join("README.md");
    let content = format!("# {}\n\n{}", module.topic, module.explanation);
    fs::write(explanation_path, content).context("Failed to write explanation file")?;

    Ok(())
}

/// Creates a docs folder and saves the explanation as a markdown file
fn create_docs_explanation_file(project_dir: &Path, module: &LearningModule) -> Result<()> {
    // Create docs directory
    let docs_dir = project_dir.join("docs");
    fs::create_dir_all(&docs_dir).context("Failed to create docs directory")?;

    // Create markdown file with explanation
    let explanation_path = docs_dir.join("explanation.md");
    let content = format!("# {}\n\n{}", module.topic, module.explanation);
    fs::write(explanation_path, content).context("Failed to write explanation markdown file")?;

    Ok(())
}

/// Creates Rust files for each code snippet
fn create_code_snippet_files(project_dir: &Path, module: &LearningModule) -> Result<()> {
    let examples_dir = project_dir.join("examples");
    fs::create_dir_all(&examples_dir).context("Failed to create examples directory")?;

    for (i, snippet) in module.code_snippets.iter().enumerate() {
        let file_name = format!("{}.rs", sanitize_filename(&snippet.title, i + 1));
        let file_path = examples_dir.join(file_name);

        let content = format!("// {}\n// {}\n\n{}", 
            snippet.title, 
            snippet.description, 
            snippet.code
        );

        fs::write(file_path, content).context("Failed to write code snippet file")?;
    }

    Ok(())
}

/// Creates Rust files for each exercise
fn create_exercise_files(project_dir: &Path, module: &LearningModule) -> Result<()> {
    let exercises_dir = project_dir.join("src").join("bin");
    fs::create_dir_all(&exercises_dir).context("Failed to create exercises directory")?;

    for (i, exercise) in module.exercises.iter().enumerate() {
        let file_name = format!("{}.rs", sanitize_filename(&exercise.name, i + 1));
        let file_path = exercises_dir.join(file_name);

        let content = format!("// {}\n// {}\n\n{}", 
            exercise.name, 
            exercise.description, 
            exercise.code
        );

        fs::write(file_path, content).context("Failed to write exercise file")?;
    }

    // Update Cargo.toml to include the exercises as binaries
    update_cargo_toml(project_dir, module)?;

    Ok(())
}

/// Updates the Cargo.toml file to include the exercises as binaries
fn update_cargo_toml(project_dir: &Path, module: &LearningModule) -> Result<()> {
    let cargo_toml_path = project_dir.join("Cargo.toml");
    let mut cargo_toml = fs::read_to_string(&cargo_toml_path).context("Failed to read Cargo.toml")?;

    // Add [[bin]] sections for each exercise
    cargo_toml.push_str("\n");
    for (i, exercise) in module.exercises.iter().enumerate() {
        let bin_name = sanitize_filename(&exercise.name, i + 1);
        cargo_toml.push_str(&format!("[[bin]]\nname = \"{}\"\npath = \"src/bin/{}.rs\"\n\n", 
            bin_name, 
            bin_name
        ));
    }

    fs::write(cargo_toml_path, cargo_toml).context("Failed to write updated Cargo.toml")?;

    Ok(())
}

/// Sanitizes a filename by removing invalid characters and ensuring it's a valid Rust identifier
fn sanitize_filename(name: &str, fallback_index: usize) -> String {
    let sanitized = name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect::<String>();

    if sanitized.is_empty() || sanitized.chars().next().unwrap().is_numeric() {
        format!("exercise_{}", fallback_index)
    } else {
        sanitized
    }
}

/// Creates a Cargo project from a generated application
pub fn create_application_project(application: &GeneratedApplication) -> Result<PathBuf> {
    // Create directory name in the format [app_name]_[date]
    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let app_name_slug = application.name
        .replace(|c| !char::is_alphanumeric(c), "_").to_lowercase();

    let dir_name = format!("{}_{}", app_name_slug, current_date);

    // Create the directory
    let project_dir = PathBuf::from(&dir_name);
    fs::create_dir_all(&project_dir).context("Failed to create project directory")?;

    // Initialize Cargo project
    initialize_cargo_project(&project_dir, &app_name_slug)?;

    // Create README.md with application description
    let readme_path = project_dir.join("README.md");
    let readme_content = format!("# {}\n\n{}\n\n## Features\n\n{}", 
        application.name, 
        application.description,
        application.features.iter().map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n")
    );
    fs::write(readme_path, readme_content).context("Failed to write README.md file")?;

    // Create Rust files for code snippets
    for (i, snippet) in application.code_snippets.iter().enumerate() {
        let is_main = i == 0 || snippet.title.to_lowercase().contains("main");

        if is_main {
            // Main code goes to src/main.rs
            let main_path = project_dir.join("src").join("main.rs");
            fs::write(main_path, &snippet.code).context("Failed to write main.rs file")?;
        } else {
            // Other snippets go to src/lib.rs or src/bin/
            let file_name = sanitize_filename(&snippet.title, i + 1);

            if file_name.contains("lib") || snippet.title.to_lowercase().contains("lib") {
                // Library code goes to src/lib.rs
                let lib_path = project_dir.join("src").join("lib.rs");
                fs::write(lib_path, &snippet.code).context("Failed to write lib.rs file")?;
            } else {
                // Other code goes to src/bin/
                let bin_dir = project_dir.join("src").join("bin");
                fs::create_dir_all(&bin_dir).context("Failed to create bin directory")?;

                let bin_path = bin_dir.join(format!("{}.rs", file_name));
                fs::write(bin_path, &snippet.code).context("Failed to write binary file")?;

                // Update Cargo.toml to include the binary
                update_cargo_toml_for_bin(&project_dir, &file_name)?;
            }
        }
    }

    Ok(project_dir)
}

/// Updates the Cargo.toml file to include a binary
fn update_cargo_toml_for_bin(project_dir: &Path, bin_name: &str) -> Result<()> {
    let cargo_toml_path = project_dir.join("Cargo.toml");
    let mut cargo_toml = fs::read_to_string(&cargo_toml_path).context("Failed to read Cargo.toml")?;

    // Add [[bin]] section for the binary
    cargo_toml.push_str(&format!("\n[[bin]]\nname = \"{}\"\npath = \"src/bin/{}.rs\"\n", 
        bin_name, 
        bin_name
    ));

    fs::write(cargo_toml_path, cargo_toml).context("Failed to write updated Cargo.toml")?;

    Ok(())
}
