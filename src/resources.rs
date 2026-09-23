// src/resources.rs
// Links to additional learning resources for a topic.
use crate::config::LearningResources;
use crate::model::{AdditionalResources, Resource, ResourceGroup};

const STOP_WORDS: &[&str] = &["rust", "the", "and", "for", "with", "library", "from", "into", "your"];
const MAX_KEYWORDS: usize = 3;

fn resource(title: impl Into<String>, url: impl Into<String>, description: impl Into<String>) -> Resource {
    Resource { title: title.into(), url: url.into(), description: description.into() }
}

/// Extracts search keywords from a topic like "Library: std::collections::HashMap"
pub fn keywords(topic: &str) -> Vec<String> {
    let mut keywords: Vec<String> = Vec::new();
    for word in topic.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if word.len() > 3
            && !STOP_WORDS.contains(&word.to_lowercase().as_str())
            && !keywords.iter().any(|k| k.eq_ignore_ascii_case(word))
        {
            keywords.push(word.to_string());
        }
    }
    // Prefer the most specific (last) words, e.g. "HashMap" over "collections"
    keywords.reverse();
    keywords.truncate(MAX_KEYWORDS);
    keywords
}

/// Builds the resource links enabled in the settings
pub fn for_topic(topic: &str, settings: &LearningResources) -> Option<AdditionalResources> {
    let keywords = keywords(topic);
    let mut groups = Vec::new();

    if settings.show_official_docs {
        let mut resources = vec![
            resource(
                "Rust Standard Library Documentation",
                "https://doc.rust-lang.org/std/",
                "Official documentation for the Rust standard library",
            ),
            resource(
                "The Rust Programming Language Book",
                "https://doc.rust-lang.org/book/",
                "Comprehensive guide to the Rust programming language",
            ),
            resource(
                "Rust By Example",
                "https://doc.rust-lang.org/rust-by-example/",
                "Collection of runnable examples that illustrate various Rust concepts",
            ),
        ];
        resources.extend(keywords.iter().map(|k| {
            resource(
                format!("Rust Documentation Search: {}", k),
                format!("https://doc.rust-lang.org/std/?search={}", k),
                format!("Search results for '{}' in the Rust documentation", k),
            )
        }));
        groups.push(ResourceGroup { title: "Official Documentation", resources });
    }

    if settings.show_community_resources {
        let mut resources = vec![
            resource(
                "Rust Users Forum",
                "https://users.rust-lang.org/",
                "Official forum for Rust users to ask questions and share knowledge",
            ),
            resource("Rust Subreddit", "https://www.reddit.com/r/rust/", "Reddit community for Rust developers"),
        ];
        resources.extend(keywords.iter().map(|k| {
            resource(
                format!("Stack Overflow: Rust + {}", k),
                format!("https://stackoverflow.com/questions/tagged/rust?q={}", k),
                format!("Stack Overflow questions about Rust and {}", k),
            )
        }));
        groups.push(ResourceGroup { title: "Community Resources", resources });
    }

    if settings.show_crates_io {
        let mut resources = vec![resource(
            "Crates.io - The Rust Package Registry",
            "https://crates.io/",
            "The official Rust package registry",
        )];
        resources.extend(keywords.iter().map(|k| {
            resource(
                format!("Crates.io Search: {}", k),
                format!("https://crates.io/search?q={}", k),
                format!("Rust packages related to {}", k),
            )
        }));
        groups.push(ResourceGroup { title: "Crates.io Packages", resources });
    }

    if settings.show_github_repos {
        let mut resources = vec![resource(
            "Rust Language GitHub Repository",
            "https://github.com/rust-lang/rust",
            "The official Rust language repository",
        )];
        resources.extend(keywords.iter().map(|k| {
            resource(
                format!("GitHub: Rust + {}", k),
                format!("https://github.com/search?q=language%3Arust+{}", k),
                format!("GitHub repositories related to Rust and {}", k),
            )
        }));
        groups.push(ResourceGroup { title: "GitHub Repositories", resources });
    }

    if groups.is_empty() { None } else { Some(AdditionalResources { groups }) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_specific_keywords() {
        assert_eq!(keywords("Library: std::collections::HashMap"), ["HashMap", "collections"]);
        assert_eq!(keywords("Error Handling: Recoverable Errors with Result"), ["Result", "Errors", "Recoverable"]);
        assert!(keywords("The Rust").is_empty());
    }

    #[test]
    fn respects_settings() {
        let mut settings = LearningResources::default();
        assert_eq!(for_topic("Traits", &settings).unwrap().groups.len(), 4);
        settings.show_official_docs = false;
        settings.show_community_resources = false;
        settings.show_crates_io = false;
        settings.show_github_repos = false;
        assert!(for_topic("Traits", &settings).is_none());
    }
}
