use colored::*;
use crate::state::load_index;
use crate::project_store::{load_project, save_project};

pub fn run_unpin(ref_id: String, project: Option<String>) {
    let index = load_index();

    let pid = match project {
        Some(p) => p,
        None => match index.active_project {
            Some(p) => p,
            None => {
                eprintln!("{}", "❌ No active project set.".red());
                return;
            }
        },
    };

    let mut proj = load_project(&pid);

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

    // orphan detection (UX sugar, not required but very good)
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
