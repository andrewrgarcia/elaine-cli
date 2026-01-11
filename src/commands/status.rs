use colored::*;
use std::fs;
use crate::state::{elaine_dir, load_index};
use crate::project::Project;
use crate::reference_store::load_ref;
use crate::utils::id::sid_short;

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

    // --- Load all reference IDs -----------------------------------------------

    let refs_dir = elaine_dir().join("refs");
    let mut all_refs: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(&refs_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    all_refs.push(stem.to_string());
                }
            }
        }
    }

    all_refs.sort();


    projects.sort_by(|a, b| a.id.cmp(&b.id));

    use std::collections::HashSet;

    let mut pinned_refs: HashSet<String> = HashSet::new();

    for p in &projects {
        for rid in &p.refs {
            pinned_refs.insert(rid.clone());
        }
    }

    let orphan_refs: Vec<String> = all_refs
    .into_iter()
    .filter(|rid| !pinned_refs.contains(rid))
    .collect();


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
            "{} {}   ({} refs)   {}",
            marker,
            p.id,
            p.refs.len(),
            sid_short(&p.sid).dimmed()
        );

        if is_active {
            println!("{}", line.bright_green().bold());
        } else {
            println!("{}", line);
        }

        if verbose {
            for rid in &p.refs {
                print_ref_verbose(rid, "    ");
            }
        }
    }

    // --- Orphaned references --------------------------------------------------

    if !orphan_refs.is_empty() {
        println!();
        println!(
            "{} ({})",
            "Orphaned references",
            orphan_refs.len()
        );

        if verbose {
            for rid in orphan_refs {
                print_ref_verbose(&rid, "  ");
            }
        }
    }
}


fn print_ref_verbose(rid: &str, indent: &str) {
    if let Some(r) = load_ref(rid) {
        let sid_short = if r.sid.len() >= 8 {
            &r.sid[..8]
        } else {
            &r.sid
        };

        println!(
            "{}{}   {}",
            indent,
            r.id,
            sid_short.dimmed()
        );
    } else {
        // Defensive fallback
        println!("{}{}", indent, rid.dimmed());
    }
}
