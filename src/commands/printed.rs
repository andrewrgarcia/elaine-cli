use std::fs;
use std::path::Path;

use colored::*;
use serde::Deserialize;

use crate::reference::Reference;

/// Structure of `.elaine/index.yaml`
#[derive(Debug, Deserialize)]
struct ElaineIndex {
    active_project: String,
}

/// Structure of `.elaine/projects/<project>.yaml`
#[derive(Debug, Deserialize)]
struct ProjectFile {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    title: Option<String>,
    refs: Vec<String>,
}



/// Entry point for `eln printed`
pub fn run_printed(all: bool, projects: Vec<String>) {
    let elaine_dir = Path::new(".elaine");

    // --- Sanity checks ------------------------------------------------------

    if !elaine_dir.exists() {
        eprintln!(
            "{}",
            "❌ .elaine/ directory not found. Run `eln init` first."
                .red()
                .bold()
        );
        return;
    }

    // --- Load index ---------------------------------------------------------

    let index_path = elaine_dir.join("index.yaml");
    let index_str = match fs::read_to_string(&index_path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!(
                "{}",
                "❌ Failed to read .elaine/index.yaml"
                    .red()
                    .bold()
            );
            return;
        }
    };

    let index: ElaineIndex = match serde_yaml::from_str(&index_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{}\n{}",
                "❌ Failed to parse .elaine/index.yaml".red().bold(),
                e.to_string().dimmed()
            );
            return;
        }
    };

    let project_ids = match resolve_project_ids(
        elaine_dir,
        all,
        projects,
        &index,
    ) {
        Some(v) => v,
        None => return,
    };


    let ref_ids = match collect_reference_ids(
        elaine_dir,
        &project_ids,
    ) {
        Some(v) => v,
        None => return,
    };


    let mut refs = match load_references(
        elaine_dir,
        &ref_ids,
    ) {
        Some(v) => v,
        None => return,
    };


    // --- Deterministic ordering ---------------------------------------------
    sort_references(&mut refs);

    render_and_write_bibtex(&refs, &project_ids, all);
}


fn resolve_project_ids(
    elaine_dir: &Path,
    all: bool,
    projects: Vec<String>,
    index: &ElaineIndex,
) -> Option<Vec<String>> {
    if all {
        let projects_dir = elaine_dir.join("projects");
        let mut ids = Vec::new();

        let entries = match fs::read_dir(&projects_dir) {
            Ok(e) => e,
            Err(_) => {
                eprintln!(
                    "{}",
                    "❌ Failed to read projects directory"
                        .red()
                        .bold()
                );
                return None;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    ids.push(stem.to_string());
                }
            }
        }

        if ids.is_empty() {
            eprintln!(
                "{}",
                "⚠️  No projects found."
                    .yellow()
            );
            return None;
        }

        ids.sort();
        Some(ids)
    } else if projects.is_empty() {
        Some(vec![index.active_project.clone()])
    } else {
        Some(projects)
    }
}


fn collect_reference_ids(
    elaine_dir: &Path,
    project_ids: &[String],
) -> Option<std::collections::HashSet<String>> {
    use std::collections::HashSet;

    let mut ref_ids: HashSet<String> = HashSet::new();

    for pid in project_ids {
        let project_path = elaine_dir
            .join("projects")
            .join(format!("{}.yaml", pid));

        let project_str = match fs::read_to_string(&project_path) {
            Ok(s) => s,
            Err(_) => {
                eprintln!(
                    "{}",
                    format!("❌ Project '{}' not found", pid)
                        .red()
                        .bold()
                );
                return None;
            }
        };

        let project: ProjectFile = match serde_yaml::from_str(&project_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "{}\n{}",
                    "❌ Failed to parse project file".red().bold(),
                    e.to_string().dimmed()
                );
                return None;
            }
        };

        for rid in project.refs {
            ref_ids.insert(rid);
        }
    }

    if ref_ids.is_empty() {
        eprintln!(
            "{}",
            "⚠️  No references found in selected project(s)."
                .yellow()
        );
        return None;
    }

    Some(ref_ids)
}




fn load_references(
    elaine_dir: &Path,
    ref_ids: &std::collections::HashSet<String>,
) -> Option<Vec<Reference>> {
    let mut refs: Vec<Reference> = Vec::new();

    for rid in ref_ids {
        let ref_path = elaine_dir
            .join("refs")
            .join(format!("{}.yaml", rid));

        let ref_str = match fs::read_to_string(&ref_path) {
            Ok(s) => s,
            Err(_) => {
                eprintln!(
                    "{}",
                    format!("⚠️  Missing reference '{}'", rid)
                        .yellow()
                );
                continue;
            }
        };

        match serde_yaml::from_str::<Reference>(&ref_str) {
            Ok(r) => refs.push(r),
            Err(e) => {
                eprintln!(
                    "{}\n{}",
                    format!("⚠️  Failed to parse reference '{}'", rid)
                        .yellow(),
                    e.to_string().dimmed()
                );
            }
        }
    }

    if refs.is_empty() {
        eprintln!(
            "{}",
            "❌ No valid references loaded."
                .red()
                .bold()
        );
        return None;
    }

    Some(refs)
}


fn render_and_write_bibtex(
    refs: &[Reference],
    project_ids: &[String],
    all: bool,
) {
    // --- Render to stdout --------------------------------------------------

    for r in refs {
        print!("{}", render_bibtex(r));
        println!();
    }

    // --- Determine output filename ----------------------------------------

    let out_name = if all {
        "global_references.bib".to_string()
    } else {
        format!(
            "{}_references.bib",
            project_ids.join("+")
        )
    };

    // --- Render to file ----------------------------------------------------

    let mut out = String::new();
    for r in refs {
        out.push_str(&render_bibtex(r));
        out.push('\n');
    }

    fs::write(&out_name, out)
        .expect("Failed writing BibTeX file");

    println!(
        "{}",
        format!(
            "🖨️  Printed {} references to → {}",
            refs.len(),
            out_name
        )
        .green()
        .bold()
    );
}


fn sort_references(refs: &mut Vec<Reference>) {
    refs.sort_by(|a, b| {
        // Year (None last)
        match (&a.year, &b.year) {
            (Some(ya), Some(yb)) => ya.cmp(yb),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
        // First author
        .then_with(|| {
            let a_author = a.authors.first().map(|s| s.as_str()).unwrap_or("");
            let b_author = b.authors.first().map(|s| s.as_str()).unwrap_or("");
            a_author.cmp(b_author)
        })
        // ID (always present)
        .then_with(|| a.id.cmp(&b.id))
    });
}


fn render_bibtex(r: &Reference) -> String {
    let mut out = String::new();

    // --- Entry header -----------------------------------------------------
    let kind = match r.kind {
        crate::reference::RefKind::Article => "ARTICLE",
        crate::reference::RefKind::InProceedings => "INPROCEEDINGS",
        crate::reference::RefKind::InCollection => "INCOLLECTION",
        crate::reference::RefKind::InBook => "INBOOK",
        crate::reference::RefKind::Book => "BOOK",
        crate::reference::RefKind::Misc => "MISC",
    };

    out.push_str(&format!("@{}{{{},\n", kind, r.id));

    // --- Helper macro -----------------------------------------------------
    macro_rules! field {
        ($name:expr, $val:expr) => {
            if let Some(v) = $val {
                out.push_str(&format!("  {:<9}= {{{}}},\n", $name, v));
            }
        };
    }

    // --- Core fields ------------------------------------------------------
    field!("title", Some(&r.title));

    if !r.authors.is_empty() {
        out.push_str(&format!(
            "  {:<9}= {{{}}},\n",
            "author",
            r.authors.join(" and ")
        ));
    }

    if !r.editors.is_empty() {
        out.push_str(&format!(
            "  {:<9}= {{{}}},\n",
            "editor",
            r.editors.join(" and ")
        ));
    }

    field!("year", r.year.map(|y| y.to_string()).as_deref());

    // --- Venue fields -----------------------------------------------------
    if let Some(v) = &r.venue {
        field!("journal", v.journal.as_deref());
        field!("booktitle", v.booktitle.as_deref());
        field!("publisher", v.publisher.as_deref());
        field!("series", v.series.as_deref());
        field!("volume", v.volume.as_deref());
        field!("number", v.issue.as_deref());
        field!("pages", v.pages.as_deref());
        field!("address", v.address.as_deref());
    }

    // --- Identifiers ------------------------------------------------------
    field!("doi", r.identifiers.doi.as_deref());
    field!("url", r.identifiers.url.as_deref());
    field!("isbn", r.identifiers.isbn.as_deref());

    // --- Close entry ------------------------------------------------------
    out.push_str("}\n");

    out
}
