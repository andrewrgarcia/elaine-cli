use std::collections::HashSet;
use std::fs;
use std::path::Path;

use colored::*;
use regex::Regex;
use walkdir::WalkDir;

use crate::reference_store::{load_all_refs, save_ref};

pub fn run_purge(path: String, force: bool) {
    let root = Path::new(&path);

    if !root.exists() {
        eprintln!("{}", "❌ Provided path does not exist".red());
        return;
    }

    println!("{}", "🔎 Scanning LaTeX files…".bold());

    let cited = collect_citations(root);

    if cited.is_empty() {
        println!("{}", "⚠️  No citations detected.".yellow());
    } else {
        println!(
            "{}",
            format!("Found {} cited keys", cited.len())
                .bright_green()
        );
    }

    let mut refs = load_all_refs();

    let mut newly_uncited = Vec::new();
    let mut newly_cited = Vec::new();

    for r in refs.iter_mut() {
        let is_cited = cited.contains(&r.id);

        match (r.uncited, is_cited) {
            (false, false) => {
                r.uncited = true;
                newly_uncited.push(r.id.clone());
            }
            (true, true) => {
                r.uncited = false;
                newly_cited.push(r.id.clone());
            }
            _ => {}
        }

        if force {
            save_ref(r);
        }
    }

    println!();
    println!("{}", "Purge results".bold());
    println!("{}", "─────────────".dimmed());

    println!("Uncited: {}", newly_uncited.len());
    for id in &newly_uncited {
        println!("  {}", id.red());
    }

    println!("Re-cited: {}", newly_cited.len());
    for id in &newly_cited {
        println!("  {}", id.green());
    }

    if !force {
        println!();
        println!("{}", "Dry run. Use --force to apply changes.".yellow());
    } else {
        println!();
        println!("{}", "✔️  YAML updated.".bright_green().bold());
    }
}


fn collect_citations(root: &Path) -> HashSet<String> {
    let mut keys = HashSet::new();

    let cite_re =
        Regex::new(r"\\cite[a-zA-Z*]*\{([^}]+)\}")
            .unwrap();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("tex") {
            continue;
        }

        let content = fs::read_to_string(path)
            .unwrap_or_default();

        for caps in cite_re.captures_iter(&content) {
            for key in caps[1].split(',') {
                keys.insert(key.trim().to_string());
            }
        }
    }

    keys
}