use std::fs;
use std::path::{Path, PathBuf};
use colored::*;
use crate::reference::Reference;

pub fn refs_dir() -> PathBuf {
    Path::new(".elaine").join("refs")
}

pub fn ref_path(ref_id: &str) -> PathBuf {
    refs_dir().join(format!("{}.yaml", ref_id))
}

pub fn ref_exists(ref_id: &str) -> bool {
    ref_path(ref_id).exists()
}

pub fn load_ref(ref_id: &str) -> Reference {
    let path = ref_path(ref_id);

    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("❌ Failed to read reference {}", ref_id));

    serde_yaml::from_str(&contents)
        .unwrap_or_else(|_| panic!("❌ Failed to parse reference {}", ref_id))
}

pub fn save_ref(reference: &Reference) {
    let path = ref_path(&reference.id);

    let contents = serde_yaml::to_string(reference)
        .expect("❌ Failed to serialize reference");

    fs::write(&path, contents)
        .expect("❌ Failed to write reference file");
}

pub fn create_ref_if_missing(reference: Reference) {
    if ref_exists(&reference.id) {
        println!(
            "{}",
            format!("ℹ️  Reference '{}' already exists", reference.id)
                .bright_yellow()
        );
    } else {
        save_ref(&reference);
        println!(
            "{}",
            format!("📚 Added reference '{}'", reference.id)
                .bright_green()
                .bold()
        );
    }
}
