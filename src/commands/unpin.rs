use colored::*;

use crate::state::load_index;
use crate::project_store::{load_project, save_project};
use crate::utils::resolve::{resolve_reference, print_resolve_error};
use crate::utils::resolve_project::{resolve_project, print_project_resolve_error};

pub fn run_unpin(ref_selector: String, project_selector: Option<String>) {
    // --- Resolve reference (SID or ID) -----------------------------------

    let ref_id = match resolve_reference(&ref_selector) {
        Ok(id) => id,
        Err(e) => {
            print_resolve_error(e);
            return;
        }
    };

    // --- Resolve project (SID or ID) -------------------------------------

    let index = load_index();

    let pid = match project_selector {
        Some(sel) => match resolve_project(&sel) {
            Ok(p) => p,
            Err(e) => {
                print_project_resolve_error(e);
                return;
            }
        },
        None => match index.active_project {
            Some(p) => p,
            None => {
                eprintln!("{}", "❌ No active project set.".red());
                return;
            }
        },
    };

    let mut proj = load_project(&pid);

    // --- Unpin -----------------------------------------------------------

    if !proj.refs.contains(&ref_id) {
        eprintln!(
            "{}",
            format!("❌ Reference '{}' not pinned to '{}'", ref_id, pid).red()
        );
        return;
    }

    proj.refs.retain(|r| r != &ref_id);
    save_project(&proj);

    println!(
        "{}",
        format!("📍 Unpinned '{}' from '{}'", ref_id, pid)
            .bright_green()
    );

    // --- Orphan detection (PRESERVED) -----------------------------------

    if is_orphaned(&ref_id) {
        println!(
            "{}",
            format!(
                "⚠️  Reference '{}' is now orphaned (not pinned to any project)",
                ref_id
            )
            .yellow()
        );
    }
}

fn is_orphaned(ref_id: &str) -> bool {
    let projects_dir = std::path::Path::new(".elaine/projects");

    if let Ok(entries) = std::fs::read_dir(projects_dir) {
        for e in entries.flatten() {
            let contents = std::fs::read_to_string(e.path()).unwrap_or_default();
            if contents.contains(ref_id) {
                return false;
            }
        }
    }

    true
}
