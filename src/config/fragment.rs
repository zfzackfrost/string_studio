use serde::{Deserialize, Serialize};

use crate::pattern::CompositePattern;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fragment {
    pub name: String,
    pub pattern: CompositePattern,
    #[serde(default)]
    pub description: String,
}

impl Fragment {
    pub fn new(name: &str, pattern: CompositePattern, description: &str) -> Self {
        Self {
            name: String::from(name),
            pattern,
            description: String::from(description),
        }
    }
}

impl Default for Fragment {
    fn default() -> Self {
        Self {
            name: String::new(),
            pattern: Default::default(),
            description: String::new(),
        }
    }
}
