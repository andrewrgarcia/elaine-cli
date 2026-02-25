use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use colored::*;
use regex::Regex;
use walkdir::WalkDir;

pub fn run_purge(path: String, _force: bool) {
    let root = Path::new(&path);

    if !root.exists() {
        eprintln!("{}", "❌ Provided path does not exist".red());
        return;
    }

    println!("{}", "🔎 Scanning LaTeX files…".bold());
    let cited_keys = collect_tex_keys(root);

    println!(
        "{}",
        format!("Found {} cited keys", cited_keys.len())
            .bright_green()
    );

    println!("{}", "📚 Scanning .bib files…".bold());
    let bib_files = collect_bib_files(root);

    if bib_files.is_empty() {
        println!("{}", "⚠️  No .bib files found.".yellow());
        return;
    }

    for bib_path in bib_files {
        purge_single_bib(&bib_path, &cited_keys);
    }

    println!();
    println!("{}", "✔️  Purge complete (non-destructive).".bright_green().bold());
}

// ============================================================
// TEX
// ============================================================

fn collect_tex_keys(root: &Path) -> HashSet<String> {
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

// ============================================================
// BIB
// ============================================================

fn collect_bib_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();

            if path.extension().and_then(|s| s.to_str()) != Some("bib") {
                return false;
            }

            // 🔥 Ignore already purged files
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if name.ends_with("_purged.bib") {
                    return false;
                }
            }

            true
        })
        .map(|e| e.into_path())
        .collect()
}

fn purge_single_bib(path: &Path, cited: &HashSet<String>) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!(
                "{}",
                format!("❌ Failed to read {}", path.display())
                    .red()
            );
            return;
        }
    };

    let entries = parse_bib_entries(&content);

    let mut kept_blocks = Vec::new();
    let mut kept_count = 0;
    let mut removed_count = 0;

    for (key, block) in entries {
        if cited.contains(&key) {
            kept_blocks.push(block);
            kept_count += 1;
        } else {
            removed_count += 1;
        }
    }

    let output_path = purged_path(path);

    let mut new_content = String::new();
    for block in kept_blocks {
        new_content.push_str(&block);
        if !block.ends_with('\n') {
            new_content.push('\n');
        }
        new_content.push('\n');
    }

    if let Err(_) = fs::write(&output_path, new_content) {
        eprintln!(
            "{}",
            format!("❌ Failed writing {}", output_path.display())
                .red()
        );
        return;
    }

    println!(
        "{} → {} (kept {}, removed {})",
        path.file_name().unwrap().to_string_lossy(),
        output_path.file_name().unwrap().to_string_lossy(),
        kept_count.to_string().green(),
        removed_count.to_string().red()
    );
}

fn purged_path(original: &Path) -> PathBuf {
    let stem = original
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let parent = original.parent().unwrap_or(Path::new("."));

    parent.join(format!("{}_purged.bib", stem))
}

// ============================================================
// ENTRY PARSER (brace depth safe)
// ============================================================

fn parse_bib_entries(content: &str) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut current_key: Option<String> = None;

    let entry_start =
        Regex::new(r"@[\w]+\{([^,]+),")
            .unwrap();

    for line in content.lines() {
        if current.is_empty() {
            if let Some(cap) = entry_start.captures(line) {
                current_key = Some(cap[1].trim().to_string());
                current.push_str(line);
                current.push('\n');

                depth = line.matches('{').count() as i32
                      - line.matches('}').count() as i32;
            }
        } else {
            current.push_str(line);
            current.push('\n');

            depth += line.matches('{').count() as i32
                   - line.matches('}').count() as i32;

            if depth == 0 {
                if let Some(key) = current_key.take() {
                    entries.push((key, current.clone()));
                }
                current.clear();
            }
        }
    }

    entries
}