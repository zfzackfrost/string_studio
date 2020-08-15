use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fragment {
    pub name: String,
    pub pattern: String,
    #[serde(default)]
    pub description: String,
}

impl Fragment {
    pub fn new(name: &str, pattern: &str, description: &str) -> Self {
        Self {
            name: String::from(name),
            pattern: String::from(pattern),
            description: String::from(description),
        }
    }
}

impl Default for Fragment {
    fn default() -> Self {
        Self {
            name: String::new(),
            pattern: String::new(),
            description: String::new(),
        }
    }
}
