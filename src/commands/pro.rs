use colored::*;
use std::fs;

use crate::state::{elaine_dir, load_index, save_index};
use crate::project_store::{create_project_if_missing, load_project};

pub fn run_pro(project_id: Option<String>) {
    if !elaine_dir().exists() {
        eprintln!("{}", "❌ Not an Elaine project. Run `eln init` first.".red());
        return;
    }

    match project_id {
        Some(pid) => switch_project(&pid),
        None => list_projects(),
    }
}

fn switch_project(project_id: &str) {
    create_project_if_missing(project_id);

    let mut index = load_index();
    index.active_project = Some(project_id.to_string());
    save_index(&index);

    println!(
        "{}",
        format!("✔️  Active project set to '{}'", project_id)
            .bright_green()
            .bold()
    );
}

fn list_projects() {
    let projects_dir = elaine_dir().join("projects");

    let index = load_index();
    let active = index.active_project;

    let mut projects: Vec<(String, usize)> = Vec::new();

    let entries = match fs::read_dir(&projects_dir) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("{}", "❌ Failed to read projects directory".red());
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }

        let id = match path.file_stem().and_then(|s| s.to_str()) {
            Some(v) => v.to_string(),
            None => continue,
        };

        let project = load_project(&id);
        projects.push((id, project.refs.len()));
    }

    if projects.is_empty() {
        println!("{}", "No projects found.".yellow());
        return;
    }

    projects.sort_by(|a, b| a.0.cmp(&b.0));

    println!("{}", "Projects:".bold());

    for (id, count) in projects {
        let marker = match &active {
            Some(a) if a == &id => "*",
            _ => " ",
        };

        println!(
            "{} {} ({:>2} refs)",
            marker,
            id,
            count
        );
    }
}
