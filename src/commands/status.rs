use colored::*;
use std::fs;
use crate::state::{elaine_dir, load_index};
use crate::project::Project;

pub fn run_status(verbose: bool) {
    if !elaine_dir().exists() {
        eprintln!("{}", "❌ Not an Elaine project. Run `eln init` first.".red());
        return;
    }

    let index = load_index();
    let projects_dir = elaine_dir().join("projects");

    let mut projects: Vec<Project> = Vec::new();

    // Load all projects
    if let Ok(entries) = fs::read_dir(&projects_dir) {
        for entry in entries.flatten() {
            if let Ok(contents) = fs::read_to_string(entry.path()) {
                if let Ok(p) = serde_yaml::from_str::<Project>(&contents) {
                    projects.push(p);
                }
            }
        }
    }

    projects.sort_by(|a, b| a.id.cmp(&b.id));

    println!("{}", "Elaine Status".bold());
    println!("{}", "─────────────".dimmed());

    if projects.is_empty() {
        println!("{}", "No projects found.".yellow());
        return;
    }

    for p in projects {
        let is_active = index.active_project.as_deref() == Some(&p.id);
        let marker = if is_active { "*" } else { " " };

        let line = format!(
            "{} {}   ({} refs)",
            marker,
            p.id,
            p.refs.len()
        );

        if is_active {
            println!("{}", line.bright_green().bold());
        } else {
            println!("{}", line);
        }

        if verbose {
            for rid in &p.refs {
                println!("    {}", rid.dimmed());
            }
        }
    }
}
