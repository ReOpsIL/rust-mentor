// src/data.rs
use crate::app::IndexType;
use anyhow::Result;
use rand::seq::{IndexedRandom, SliceRandom};
use serde::{Deserialize, Serialize};

// The indexes are embedded so the binary works from any directory
const RUST_LIBRARY_INDEX: &str = include_str!("../data/rust_library_index.json");
const RUST_BY_EXAMPLE_INDEX: &str = include_str!("../data/rust_by_example_index_full.json");
const RUST_PROGRAMMING_LANGUAGE_INDEX: &str = include_str!("../data/the_rust_programming_language.json");

// Structs for rust_by_example_index.json
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
    let library_topics: Vec<LibraryTopic> = serde_json::from_str(RUST_LIBRARY_INDEX)?;
    Ok(library_topics)
}

// Function to load the full Rust by Example index
pub fn load_rust_by_example_full() -> Result<RustByExampleFull> {
    let rbe_full: RustByExampleFull = serde_json::from_str(RUST_BY_EXAMPLE_INDEX)?;
    Ok(rbe_full)
}

// Function to load The Rust Programming Language book
pub fn load_rust_programming_language() -> Result<RustProgrammingLanguage> {
    let rust_book: RustProgrammingLanguage = serde_json::from_str(RUST_PROGRAMMING_LANGUAGE_INDEX)?;
    Ok(rust_book)
}

/// Book sections at or below `level`, as topics
fn book_topics(chapters: &[Chapter], level: u8, source_prefix: &str) -> Vec<Topic> {
    chapters
        .iter()
        .flat_map(|chapter| {
            chapter.sections.iter().filter(|section| section.min_level <= level).map(move |section| Topic {
                topic: format!("{}: {}", chapter.title, section.title),
                source: format!("{} {}", source_prefix, section.section_number),
                min_level: section.min_level,
            })
        })
        .collect()
}

/// All topics of an index that suit the user's level, in index order.
/// For `Random`, the topics of all indexes are combined.
pub fn topics_for_level(level: u8, index_type: &IndexType) -> Result<Vec<Topic>> {
    Ok(match index_type {
        IndexType::RustLibrary => load_rust_library_index()?
            .into_iter()
            .filter(|topic| topic.programmer_level <= level)
            .map(|topic| Topic {
                topic: format!("Library: {}", topic.library_name),
                source: format!("Rust Library: {}", topic.library_name),
                min_level: topic.programmer_level,
            })
            .collect(),
        IndexType::RustByExample => book_topics(&load_rust_by_example_full()?.book.chapters, level, "RBE"),
        IndexType::RustProgrammingLanguage => {
            book_topics(&load_rust_programming_language()?.book.chapters, level, "The Book Ch")
        }
        IndexType::Random => {
            let mut topics = Vec::new();
            for index_type in CONCRETE_INDEXES {
                topics.extend(topics_for_level(level, &index_type)?);
            }
            topics
        }
    })
}

const CONCRETE_INDEXES: [IndexType; 3] =
    [IndexType::RustLibrary, IndexType::RustByExample, IndexType::RustProgrammingLanguage];

/// A random topic that suits the user's level. For `Random`, a random index is picked
/// first (so the large library index doesn't dominate); indexes without topics for
/// the level are skipped.
pub fn get_random_topic_for_level(level: u8, index_type: &IndexType) -> Result<Topic> {
    let mut rng = rand::rng();
    let mut index_types = match index_type {
        IndexType::Random => CONCRETE_INDEXES.to_vec(),
        other => vec![*other],
    };
    index_types.shuffle(&mut rng);

    for index_type in &index_types {
        if let Some(topic) = topics_for_level(level, index_type)?.choose(&mut rng) {
            return Ok(topic.clone());
        }
    }
    anyhow::bail!("No suitable topics found for level {}", level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_indexes_parse() {
        assert!(!load_rust_library_index().unwrap().is_empty());
        assert!(!load_rust_by_example_full().unwrap().book.chapters.is_empty());
        assert!(!load_rust_programming_language().unwrap().book.chapters.is_empty());
    }

    #[test]
    fn topics_respect_level() {
        for index in [IndexType::RustByExample, IndexType::RustProgrammingLanguage, IndexType::Random] {
            for _ in 0..20 {
                let topic = get_random_topic_for_level(3, &index).unwrap();
                assert!(topic.min_level <= 3);
            }
        }
        for _ in 0..20 {
            assert!(get_random_topic_for_level(1, &IndexType::Random).is_ok());
        }
        assert!(get_random_topic_for_level(1, &IndexType::RustLibrary).is_err());
    }

    #[test]
    fn lists_topics_for_level() {
        let beginner = topics_for_level(1, &IndexType::RustProgrammingLanguage).unwrap();
        let expert = topics_for_level(10, &IndexType::RustProgrammingLanguage).unwrap();
        assert!(!beginner.is_empty());
        assert!(expert.len() > beginner.len());
        assert!(beginner.iter().all(|t| t.min_level <= 1));

        let all = topics_for_level(10, &IndexType::Random).unwrap();
        let sum: usize = CONCRETE_INDEXES.iter().map(|i| topics_for_level(10, i).unwrap().len()).sum();
        assert_eq!(all.len(), sum);
    }
}
