use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: Option<String>,
    pub refs: Vec<String>,
}

impl Project {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            title: None,
            refs: Vec::new(),
        }
    }
}
