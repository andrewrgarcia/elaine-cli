use colored::*;
use crate::state::{elaine_dir, load_index};

pub fn run_status() {
    if !elaine_dir().exists() {
        eprintln!("{}", "❌ Not an Elaine project. Run `eln init` first.".red());
        return;
    }

    let index = load_index();

    println!("{}", "Elaine Status".bold());
    match index.active_project {
        Some(p) => println!("  Active project: {}", p.bright_green()),
        None => println!("  Active project: {}", "none".yellow()),
    }
}
