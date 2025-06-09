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

// Structs for rust_library_index.json
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LibraryTopic {
    pub library_name: String,
    pub description: String,
    pub programmer_level: u8,
    pub programmer_level_description: String,
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

// Function to load the rust_library_index.json file
pub fn load_rust_library_index() -> Result<Vec<LibraryTopic>> {
    let file_content = fs::read_to_string("data/rust_library_index.json")?;
    let library_topics: Vec<LibraryTopic> = serde_json::from_str(&file_content)?;
    Ok(library_topics)
}

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

// Function to get a random topic based on user's level and selected index
pub fn get_random_topic_for_level(level: u8, index_type: &crate::app::IndexType) -> Result<Topic> {
    use crate::app::IndexType;

    let mut rng = rand::thread_rng();

    match index_type {
        IndexType::RustLibrary => {
            // Get topics from rust_library_index.json
            let library_topics = load_rust_library_index()?;

            // Filter topics that are appropriate for the user's level
            let suitable_topics: Vec<&LibraryTopic> = library_topics
                .iter()
                .filter(|topic| topic.programmer_level <= level)
                .collect();

            if suitable_topics.is_empty() {
                anyhow::bail!("No suitable library topics found for level {}", level);
            }

            // Select a random topic
            let random_index = rng.gen_range(0..suitable_topics.len());
            let selected_topic = suitable_topics[random_index];

            Ok(Topic {
                topic: format!("Library: {}", selected_topic.library_name),
                source: format!("Rust Library: {}", selected_topic.library_name),
                min_level: selected_topic.programmer_level,
            })
        },
        IndexType::RustByExample => {
            // Get topics from rust_by_example_full.json
            let rbe_full = load_rust_by_example_full()?;

            // Filter topics that are appropriate for the user's level
            let mut suitable_topics: Vec<Topic> = Vec::new();

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

            if suitable_topics.is_empty() {
                anyhow::bail!("No suitable Rust By Example topics found for level {}", level);
            }

            // Select a random topic
            let random_index = rng.gen_range(0..suitable_topics.len());
            Ok(suitable_topics[random_index].clone())
        },
        IndexType::RustProgrammingLanguage => {
            // Get topics from the_rust_programming_language.json
            let rust_book = load_rust_programming_language()?;

            // Filter topics that are appropriate for the user's level
            let mut suitable_topics: Vec<Topic> = Vec::new();

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

            if suitable_topics.is_empty() {
                anyhow::bail!("No suitable Rust Programming Language topics found for level {}", level);
            }

            // Select a random topic
            let random_index = rng.gen_range(0..suitable_topics.len());
            Ok(suitable_topics[random_index].clone())
        },
        IndexType::Random => {
            // Randomly select one of the three index types
            let random_index_type = match rng.gen_range(0..3) {
                0 => IndexType::RustLibrary,
                1 => IndexType::RustByExample,
                _ => IndexType::RustProgrammingLanguage,
            };

            // Recursively call this function with the randomly selected index type
            get_random_topic_for_level(level, &random_index_type)
        },
    }
}
