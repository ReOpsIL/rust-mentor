// src/data.rs
use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;

// Structs for rust_by_example_index.json
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Topic {
    pub topic: String,
    pub source: String,
    pub min_level: u8,
}

// Structs for rust_by_example_index_full.json
#[derive(Debug, Deserialize, Serialize)]
pub struct RustByExampleFull {
    pub book: RustByExampleBook,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RustByExampleBook {
    pub title: String,
    pub url: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chapter {
    pub chapter_number: u8,
    pub title: String,
    pub sections: Vec<Section>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_level: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    pub section_number: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub min_level: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_sections: Option<Vec<SubSection>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubSection {
    pub section_number: String,
    pub title: String,
    pub min_level: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_sections: Option<Vec<SubSection>>,
}

// Structs for the_rust_programming_language.json
#[derive(Debug, Deserialize, Serialize)]
pub struct RustProgrammingLanguage {
    pub book: RustProgrammingLanguageBook,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RustProgrammingLanguageBook {
    pub title: String,
    pub url: String,
    pub introduction: String,
    pub chapters: Vec<Chapter>,
    pub appendices: Vec<Appendix>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Appendix {
    pub appendix_letter: String,
    pub title: String,
    pub min_level: u8,
}

// Function to load the simple topic index

// Function to load the full Rust by Example index
pub fn load_rust_by_example_full() -> Result<RustByExampleFull> {
    let file_content = fs::read_to_string("data/rust_by_example_index_full.json")?;
    let rbe_full: RustByExampleFull = serde_json::from_str(&file_content)?;
    Ok(rbe_full)
}

// Function to load The Rust Programming Language book
pub fn load_rust_programming_language() -> Result<RustProgrammingLanguage> {
    let file_content = fs::read_to_string("data/the_rust_programming_language.json")?;
    let rust_book: RustProgrammingLanguage = serde_json::from_str(&file_content)?;
    Ok(rust_book)
}

// Function to get a random topic based on user's level
pub fn get_random_topic_for_level(level: u8) -> Result<Topic> {
    // Load both data sources
    let rbe_full = load_rust_by_example_full()?;
    let rust_book = load_rust_programming_language()?;

    // Filter topics that are appropriate for the user's level
    let mut suitable_topics: Vec<Topic> = Vec::new();

    // Add topics from rust_by_example_full.json
    for chapter in &rbe_full.book.chapters {
        for section in &chapter.sections {
            if section.min_level <= level {
                suitable_topics.push(Topic {
                    topic: format!("{}: {}", chapter.title, section.title),
                    source: format!("RBE {}", section.section_number),
                    min_level: section.min_level,
                });
            }
        }
    }

    // Add topics from the_rust_programming_language.json
    for chapter in &rust_book.book.chapters {
        for section in &chapter.sections {
            if section.min_level <= level {
                suitable_topics.push(Topic {
                    topic: format!("{}: {}", chapter.title, section.title),
                    source: format!("The Book Ch {}", section.section_number),
                    min_level: section.min_level,
                });
            }
        }
    }

    // Select a random topic from the suitable ones
    if suitable_topics.is_empty() {
        anyhow::bail!("No suitable topics found for level {}", level);
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..suitable_topics.len());
    Ok(suitable_topics[random_index].clone())
}
