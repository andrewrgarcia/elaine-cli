use colored::*;
use crate::state::load_index;
use crate::project_store::{load_project, save_project};
use crate::utils::resolve::{resolve_reference, print_resolve_error};
use crate::utils::resolve_project::{resolve_project, print_project_resolve_error};

pub fn run_pin(ref_selector: String, project_selector: Option<String>) {
    let ref_id = match resolve_reference(&ref_selector) {
        Ok(id) => id,
        Err(e) => {
            print_resolve_error(e);
            return;
        }
    };

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

    if proj.refs.contains(&ref_id) {
        println!(
            "{}",
            format!("ℹ️  Reference '{}' already pinned to '{}'", ref_id, pid).yellow()
        );
        return;
    }

    proj.refs.push(ref_id.clone());
    save_project(&proj);

    println!(
        "{}",
        format!("📌 Pinned '{}' → '{}'", ref_id, pid)
            .bright_green()
            .bold()
    );
}
