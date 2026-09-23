// src/settings.rs
// Declarative description of the settings screen: sections and their items.
use crate::config::{Config, Cycle, MAX_QUESTIONS, MIN_QUESTIONS};
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, PartialEq, Display, EnumIter)]
pub enum SettingsSection {
    #[strum(to_string = "Learning Resources")]
    LearningResources,
    #[strum(to_string = "Content Customization")]
    ContentCustomization,
    #[strum(to_string = "Learning Goals")]
    LearningGoals,
    #[strum(to_string = "Question Generator")]
    QuestionGenerator,
    #[strum(to_string = "AI Model")]
    Model,
}

/// What happens when the user changes a setting
pub enum SettingAction {
    /// Change the value in place
    Change(fn(&mut Config, bool)),
    /// Open the model picker
    PickModel,
}

pub struct SettingItem {
    pub label: &'static str,
    pub value: fn(&Config) -> String,
    pub action: SettingAction,
}

fn checkbox(checked: bool) -> String {
    if checked { "X".to_string() } else { " ".to_string() }
}

static LEARNING_RESOURCES: [SettingItem; 4] = [
    SettingItem {
        label: "Show Official Documentation",
        value: |c| checkbox(c.learning_resources.show_official_docs),
        action: SettingAction::Change(|c, _| {
            c.learning_resources.show_official_docs = !c.learning_resources.show_official_docs
        }),
    },
    SettingItem {
        label: "Show Community Resources",
        value: |c| checkbox(c.learning_resources.show_community_resources),
        action: SettingAction::Change(|c, _| {
            c.learning_resources.show_community_resources = !c.learning_resources.show_community_resources
        }),
    },
    SettingItem {
        label: "Show Crates.io Links",
        value: |c| checkbox(c.learning_resources.show_crates_io),
        action: SettingAction::Change(|c, _| {
            c.learning_resources.show_crates_io = !c.learning_resources.show_crates_io
        }),
    },
    SettingItem {
        label: "Show GitHub Repositories",
        value: |c| checkbox(c.learning_resources.show_github_repos),
        action: SettingAction::Change(|c, _| {
            c.learning_resources.show_github_repos = !c.learning_resources.show_github_repos
        }),
    },
];

static CONTENT_CUSTOMIZATION: [SettingItem; 3] = [
    SettingItem {
        label: "Code Complexity",
        value: |c| c.content_customization.code_complexity.to_string(),
        action: SettingAction::Change(|c, forward| {
            c.content_customization.code_complexity = c.content_customization.code_complexity.cycle(forward)
        }),
    },
    SettingItem {
        label: "Explanation Verbosity",
        value: |c| c.content_customization.explanation_verbosity.to_string(),
        action: SettingAction::Change(|c, forward| {
            c.content_customization.explanation_verbosity = c.content_customization.explanation_verbosity.cycle(forward)
        }),
    },
    SettingItem {
        label: "Focus Area",
        value: |c| c.content_customization.focus_area.to_string(),
        action: SettingAction::Change(|c, forward| {
            c.content_customization.focus_area = c.content_customization.focus_area.cycle(forward)
        }),
    },
];

static LEARNING_GOALS: [SettingItem; 1] = [SettingItem {
    label: "Learning Goal",
    value: |c| c.content_customization.learning_goal.to_string(),
    action: SettingAction::Change(|c, forward| {
        c.content_customization.learning_goal = c.content_customization.learning_goal.cycle(forward)
    }),
}];

static QUESTION_GENERATOR: [SettingItem; 3] = [
    SettingItem {
        label: "Number of Questions",
        value: |c| c.question_generator_settings.num_questions.to_string(),
        action: SettingAction::Change(|c, forward| {
            let n = &mut c.question_generator_settings.num_questions;
            *n = if forward { *n + 1 } else { n.saturating_sub(1) }.clamp(MIN_QUESTIONS, MAX_QUESTIONS);
        }),
    },
    SettingItem {
        label: "Question Style",
        value: |c| c.question_generator_settings.question_style.to_string(),
        action: SettingAction::Change(|c, forward| {
            c.question_generator_settings.question_style = c.question_generator_settings.question_style.cycle(forward)
        }),
    },
    SettingItem {
        label: "Generate Application",
        value: |c| checkbox(c.question_generator_settings.enable_application_generation),
        action: SettingAction::Change(|c, _| {
            c.question_generator_settings.enable_application_generation =
                !c.question_generator_settings.enable_application_generation
        }),
    },
];

static MODEL: [SettingItem; 1] =
    [SettingItem { label: "Model", value: |c| c.model.clone(), action: SettingAction::PickModel }];

impl SettingsSection {
    pub fn items(self) -> &'static [SettingItem] {
        match self {
            SettingsSection::LearningResources => &LEARNING_RESOURCES,
            SettingsSection::ContentCustomization => &CONTENT_CUSTOMIZATION,
            SettingsSection::LearningGoals => &LEARNING_GOALS,
            SettingsSection::QuestionGenerator => &QUESTION_GENERATOR,
            SettingsSection::Model => &MODEL,
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            SettingsSection::LearningResources => {
                "Controls which additional learning resources are shown alongside the AI-generated content."
            }
            SettingsSection::ContentCustomization => "Controls how the AI generates content for your learning modules.",
            SettingsSection::LearningGoals => "Controls the focus of your learning path in Rust.",
            SettingsSection::QuestionGenerator => {
                "Controls the questions that shape your generated application. \
                 With application generation off, answering the questions leads straight to the learning module."
            }
            SettingsSection::Model => {
                "The OpenRouter model used for all content. Press Enter to pick from the available models."
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn every_section_has_items_that_change_the_config() {
        for section in SettingsSection::iter() {
            assert!(!section.items().is_empty());
            for item in section.items() {
                let mut config = Config::default();
                let before = (item.value)(&config);
                if let SettingAction::Change(change) = item.action {
                    change(&mut config, true);
                    // Question count may be clamped at its limits; everything else must change
                    if item.label != "Number of Questions" {
                        assert_ne!((item.value)(&config), before, "{}", item.label);
                    }
                }
            }
        }
    }

    #[test]
    fn number_of_questions_is_clamped() {
        let change = match QUESTION_GENERATOR[0].action {
            SettingAction::Change(change) => change,
            SettingAction::PickModel => unreachable!(),
        };
        let mut config = Config::default();
        for _ in 0..20 {
            change(&mut config, true);
        }
        assert_eq!(config.question_generator_settings.num_questions, MAX_QUESTIONS);
        for _ in 0..20 {
            change(&mut config, false);
        }
        assert_eq!(config.question_generator_settings.num_questions, MIN_QUESTIONS);
    }
}
