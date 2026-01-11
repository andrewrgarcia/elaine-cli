use colored::*;
use std::fs;
use std::collections::HashSet;

use crate::state::{elaine_dir, load_index};
use crate::project::Project;
use crate::reference_store::load_ref;
use crate::utils::id::sid_short;

pub fn run_status(verbose: u8, sort: Option<String>) {
    guard_initialized();

    let index = load_index();

    let projects = load_projects();
    let all_refs = load_all_ref_ids();

    let orphan_refs = compute_orphans(&projects, &all_refs);

    print_header();

    if projects.is_empty() {
        println!("{}", "No libraries found.".yellow());
        return;
    }

    print_projects(&projects, &index, verbose, sort.as_deref());

    print_orphans(&orphan_refs, verbose, sort.as_deref());
}

// ======================================================================
// Helpers
// ======================================================================

fn guard_initialized() {
    if !elaine_dir().exists() {
        eprintln!("{}", "❌ Not an Elaine project. Run `eln init` first.".red());
        std::process::exit(1);
    }
}

fn load_projects() -> Vec<Project> {
    let projects_dir = elaine_dir().join("projects");
    let mut projects = Vec::new();

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
    projects
}

fn load_all_ref_ids() -> Vec<String> {
    let refs_dir = elaine_dir().join("refs");
    let mut refs = Vec::new();

    if let Ok(entries) = fs::read_dir(&refs_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    refs.push(stem.to_string());
                }
            }
        }
    }

    refs.sort();
    refs
}

fn compute_orphans(projects: &[Project], all_refs: &[String]) -> Vec<String> {
    let mut pinned = HashSet::new();

    for p in projects {
        for rid in &p.refs {
            pinned.insert(rid.clone());
        }
    }

    all_refs
        .iter()
        .filter(|rid| !pinned.contains(*rid))
        .cloned()
        .collect()
}

fn print_header() {
    println!("{}", "Elaine Status".bold());
    println!("{}", "─────────────".dimmed());
}

fn print_projects(
    projects: &[Project],
    index: &crate::state::Index,
    verbose: u8,
    sort: Option<&str>,
) {
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

        if verbose >= 1 {
            let mut refs = p.refs.clone();
            sort_refs(&mut refs, sort);

            for rid in &refs {
                print_ref(rid, "    ", verbose);
            }
        }
    }
}

fn print_orphans(
    orphan_refs: &[String],
    verbose: u8,
    sort: Option<&str>,
) {
    if orphan_refs.is_empty() {
        return;
    }

    println!();
    println!(
        "{} ({})",
        "Orphaned references".bold(),
        orphan_refs.len()
    );

    if verbose >= 1 {
        let mut refs = orphan_refs.to_vec();
        sort_refs(&mut refs, sort);

        for rid in &refs {
            print_ref(rid, "  ", verbose);
        }
    }
}


fn sort_refs(refs: &mut Vec<String>, sort: Option<&str>) {
    match sort {
        None | Some("id") => {
            refs.sort();
        }
        Some("title") => {
            refs.sort_by(|a, b| {
                let ra = load_ref(a);
                let rb = load_ref(b);
                ra.and_then(|r| Some(r.title))
                    .cmp(&rb.and_then(|r| Some(r.title)))
            });
        }
        Some("author") => {
            refs.sort_by(|a, b| {
                let ra = load_ref(a);
                let rb = load_ref(b);
                let aa = ra.and_then(|r| r.authors.get(0).cloned()).unwrap_or_default();
                let ab = rb.and_then(|r| r.authors.get(0).cloned()).unwrap_or_default();
                aa.cmp(&ab)
            });
        }
        Some("year") => {
            refs.sort_by(|a, b| {
                let ra = load_ref(a);
                let rb = load_ref(b);
                let ya = ra.and_then(|r| r.year).unwrap_or(0);
                let yb = rb.and_then(|r| r.year).unwrap_or(0);
                ya.cmp(&yb)
            });
        }
        _ => {}
    }
}


// ======================================================================
// Reference renderers
// ======================================================================

fn print_ref(rid: &str, indent: &str, verbose: u8) {
    if let Some(r) = load_ref(rid) {
        let sid = sid_short(&r.sid);
        let doc = if !r.attachments.is_empty() { " 📄" } else { "" };

        match verbose {
            1 => {
                // identity-only
                println!(
                    "{}{}   {}{}",
                    indent,
                    r.id,
                    sid.dimmed(),
                    doc
                );
            }
            _ => {
                // identity + semantic fused
                let author = r.authors.get(0).map(String::as_str).unwrap_or("Unknown");
                let year = r.year
                    .map(|y| y.to_string())
                    .unwrap_or_else(|| "n.d.".into());

                println!(
                    "{}{}   {}  {} ({}, {}){}",
                    indent,
                    r.id,
                    sid.dimmed(),
                    r.title,
                    author,
                    year,
                    doc
                );
            }
        }

        // --- Attachments (always shown in -v / -vv) ---------------------
        for a in &r.attachments {
            println!(
                "{}  ↳ {}",
                indent,
                std::path::Path::new(a)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .dimmed()
            );
        }
    } else {
        println!("{}{}", indent, rid.dimmed());
    }
}
