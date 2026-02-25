use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use colored::*;
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

        extract_citations_from_tex(&content, &mut keys);
    }

    keys
}

fn extract_citations_from_tex(content: &str, keys: &mut HashSet<String>) {
    let bytes = content.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Look for \cite
        if bytes[i] == b'\\' {
            if content[i..].starts_with("\\cite") {
                i += 5;

                // Skip optional letters (citep, citet, citealp, etc.)
                while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
                    i += 1;
                }

                // Skip optional *
                if i < bytes.len() && bytes[i] == b'*' {
                    i += 1;
                }

                // Skip whitespace
                while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }

                // Must now be {
                if i < bytes.len() && bytes[i] == b'{' {
                    i += 1;
                    let start = i;
                    let mut depth = 1;

                    while i < bytes.len() && depth > 0 {
                        match bytes[i] {
                            b'{' => depth += 1,
                            b'}' => depth -= 1,
                            _ => {}
                        }
                        i += 1;
                    }

                    let end = i - 1; // exclude closing brace

                    if end > start {
                        let inside = &content[start..end];

                        for raw_key in inside.split(',') {
                            let cleaned = raw_key
                                .trim()
                                .replace("\\_", "_");

                            if !cleaned.is_empty() {
                                keys.insert(cleaned);
                            }
                        }
                    }
                }
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
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
    let bytes = content.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'@' {
            let start = i;
            i += 1;

            // find first '{'
            while i < bytes.len() && bytes[i] != b'{' {
                i += 1;
            }

            if i >= bytes.len() {
                break;
            }

            i += 1; // skip '{'
            let key_start = i;

            while i < bytes.len() && bytes[i] != b',' {
                i += 1;
            }

            if i >= bytes.len() {
                break;
            }

            let key = content[key_start..i]
                .trim()
                .replace("\\_", "_");

            let mut depth = 1;
            i += 1;

            while i < bytes.len() {
                match bytes[i] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    b'@' if depth > 0 => {
                        // 🔥 RECOVERY POINT
                        // malformed entry — resync
                        break;
                    }
                    _ => {}
                }
                i += 1;
            }

            let end = i.min(bytes.len());
            let block = content[start..end].to_string();
            entries.push((key, block));
        } else {
            i += 1;
        }
    }

    entries
}