use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fragment {
    pub name: String,
    pub pattern: String,
    #[serde(default)]
    pub description: String,
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
