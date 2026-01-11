use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    #[serde(default)]
    pub sid: String,
    pub title: Option<String>,
    pub refs: Vec<String>,
}

impl Project {
    pub fn new(id: &str, sid: String) -> Self {
        Self {
            id: id.to_string(),
            sid,
            title: None,
            refs: Vec::new(),
        }
    }
}
