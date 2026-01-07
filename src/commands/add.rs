use std::io::{self, Read};
use colored::*;
use crate::bibtex::parse_bibtex;
use crate::reference_store::create_or_update_ref;
use crate::state::{load_index};
use crate::project_store::{load_project, save_project};

pub fn run_add() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("❌ Failed to read stdin");

    if input.trim().is_empty() {
        eprintln!("{}", "❌ No input provided".red());
        return;
    }

    let refs = parse_bibtex(&input);

    if refs.is_empty() {
        eprintln!(
            "{}\n{}\n{}",
            "❌ No BibTeX entries detected".red().bold(),
            "Elaine expects each BibTeX entry to end with a closing brace.".yellow(),
            "↳ Make sure the last line is just:    }".yellow()
        );
        return;
    }


    let index = load_index();
    let active_project = index.active_project.clone();

    let mut project = active_project
        .as_ref()
        .map(|pid| load_project(pid));

    for r in refs {
        let rid = r.id.clone();
        create_or_update_ref(r);

        if let Some(ref mut proj) = project {
            if !proj.refs.contains(&rid) {
                proj.refs.push(rid);
            }
        }
    }

    if let Some(p) = project {
        save_project(&p);
        println!(
            "{}",
            format!("🔗 Attached references to project '{}'", p.id)
                .bright_green()
        );
    }
}
