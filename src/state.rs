use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const ELAINE_DIR: &str = ".elaine";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Index {
    pub active_project: Option<String>,
}

pub fn elaine_dir() -> PathBuf {
    Path::new(ELAINE_DIR).to_path_buf()
}

pub fn index_path() -> PathBuf {
    elaine_dir().join("index.yaml")
}

pub fn load_index() -> Index {
    let path = index_path();
    if !path.exists() {
        return Index::default();
    }

    let contents = fs::read_to_string(path)
        .expect("❌ Failed to read .elaine/index.yaml");

    serde_yaml::from_str(&contents)
        .expect("❌ Failed to parse index.yaml")
}

pub fn save_index(index: &Index) {
    let contents = serde_yaml::to_string(index)
        .expect("❌ Failed to serialize index.yaml");

    fs::write(index_path(), contents)
        .expect("❌ Failed to write index.yaml");
}
