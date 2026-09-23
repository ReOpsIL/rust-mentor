// src/model.rs
// Domain types shared by the LLM layer, the app state and the UI.

#[derive(Debug, Clone, PartialEq)]
pub struct CodeSnippet {
    pub title: String,
    pub description: String,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exercise {
    pub name: String,
    pub description: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct LearningModule {
    pub topic: String,
    pub explanation: String,
    pub code_snippets: Vec<CodeSnippet>,
    pub exercises: Vec<Exercise>,
    pub additional_resources: Option<AdditionalResources>,
}

#[derive(Debug, Clone)]
pub struct AdditionalResources {
    pub groups: Vec<ResourceGroup>,
}

#[derive(Debug, Clone)]
pub struct ResourceGroup {
    pub title: &'static str,
    pub resources: Vec<Resource>,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub title: String,
    pub url: String,
    pub description: String,
}
