use std::fs;
use colored::*;
use crate::state::{elaine_dir, save_index, Index};

pub fn run_init() {
    let dir = elaine_dir();

    if dir.exists() {
        eprintln!("{}", "⚠️  .elaine already exists".yellow());
        return;
    }

    fs::create_dir(&dir).expect("❌ Failed to create .elaine/");
    fs::create_dir(dir.join("refs")).expect("❌ Failed to create refs/");
    fs::create_dir(dir.join("projects")).expect("❌ Failed to create projects/");

    save_index(&Index::default());

    println!("{}", "✔️  Initialized Elaine registry (.elaine/)".bright_green().bold());
}
