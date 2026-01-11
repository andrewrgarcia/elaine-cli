use colored::*;
use std::fs;
use std::path::Path;
use std::io::{stdin, stdout, Write};

use crate::reference_store::{load_ref, save_ref};
use crate::utils::id::make_ref_id;
use crate::utils::resolve::{resolve_reference, print_resolve_error};
use crate::project::Project;
use crate::state::elaine_dir;

pub fn run_edit(selector: String) {
    // --- Resolve reference selector (SID / prefix / ID) ------------------
    let ref_id = match resolve_reference(&selector) {
        Ok(id) => id,
        Err(e) => {
            print_resolve_error(e);
            return;
        }
    };

    let mut reference = match load_ref(&ref_id) {
        Some(r) => r,
        None => {
            eprintln!(
                "{}",
                format!("❌ Reference '{}' not found", ref_id).red().bold()
            );
            return;
        }
    };

    println!(
        "{} {} {}",
        "✏️  Editing reference".bold(),
        reference.id.bright_green(),
        format!("({})", &reference.sid[..8]).dimmed() // 👌 opinionated UX restored
    );

    // ---- Core fields ---------------------------------------------------

    let title = prompt_edit("Title", &reference.title);
    let authors = prompt_list("Authors", &reference.authors);
    let editors = prompt_list("Editors", &reference.editors);
    let year = prompt_optional_u16("Year", reference.year);

    // ---- Kind ----------------------------------------------------------

    let kind = prompt_kind(reference.kind);

    // ---- Venue ---------------------------------------------------------

    if let Some(v) = reference.venue.as_mut() {
        println!("{}", "--- Venue ---".dimmed());
        v.journal = prompt_opt("Journal", v.journal.as_deref());
        v.booktitle = prompt_opt("Booktitle", v.booktitle.as_deref());
        v.publisher = prompt_opt("Publisher", v.publisher.as_deref());
        v.series = prompt_opt("Series", v.series.as_deref());
        v.volume = prompt_opt("Volume", v.volume.as_deref());
        v.issue = prompt_opt("Issue", v.issue.as_deref());
        v.pages = prompt_opt("Pages", v.pages.as_deref());
        v.address = prompt_opt("Address", v.address.as_deref());
    }

    // ---- Identifiers ---------------------------------------------------

    println!("{}", "--- Identifiers ---".dimmed());
    reference.identifiers.doi = prompt_opt("DOI", reference.identifiers.doi.as_deref());
    reference.identifiers.url = prompt_opt("URL", reference.identifiers.url.as_deref());
    reference.identifiers.isbn = prompt_opt("ISBN", reference.identifiers.isbn.as_deref());
    reference.identifiers.arxiv = prompt_opt("arXiv", reference.identifiers.arxiv.as_deref());

    reference.title = title;
    reference.authors = authors;
    reference.editors = editors;
    reference.year = year;
    reference.kind = kind;

    // ---- ID reconciliation --------------------------------------------

    let new_id = make_ref_id(&reference.authors, reference.year, &reference.title);

    if new_id != ref_id {
        println!(
            "{}",
            format!(
                "⚠️  ID will change: {} → {}",
                ref_id,
                new_id
            )
            .yellow()
        );

        if !confirm("Proceed with ID change?") {
            println!("{}", "❌ Edit aborted.".red());
            return;
        }

        rename_reference(&ref_id, &new_id);
        reference.id = new_id.clone();
    }

    save_ref(&reference);

    println!(
        "{}",
        format!("✔️  Updated reference '{}'", reference.id)
            .bright_green()
            .bold()
    );
}


fn prompt_edit(label: &str, current: &str) -> String {
    print!("{} [{}]: ", label, current);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let v = input.trim();

    if v.is_empty() {
        current.to_string()
    } else {
        v.to_string()
    }
}

fn prompt_list(label: &str, current: &[String]) -> Vec<String> {
    let joined = current.join(" and ");
    let raw = prompt_edit(label, &joined);

    raw.split(" and ")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn prompt_opt(label: &str, current: Option<&str>) -> Option<String> {
    let display = current.unwrap_or("");
    print!("{} [{}]: ", label, display);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let v = input.trim();

    if v.is_empty() {
        current.map(|s| s.to_string())
    } else {
        Some(v.to_string())
    }
}

fn prompt_optional_u16(label: &str, current: Option<u16>) -> Option<u16> {
    let display = current.map(|y| y.to_string()).unwrap_or_default();
    print!("{} [{}]: ", label, display);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let v = input.trim();

    if v.is_empty() {
        current
    } else {
        v.parse().ok()
    }
}

fn prompt_kind(current: crate::reference::RefKind) -> crate::reference::RefKind {
    use crate::reference::RefKind::*;

    print!("Kind [{:?}]: ", current);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let v = input.trim().to_lowercase();

    match v.as_str() {
        "" => current,
        "article" => Article,
        "inproceedings" => InProceedings,
        "incollection" => InCollection,
        "inbook" => InBook,
        "book" => Book,
        "misc" => Misc,
        _ => current,
    }
}

fn confirm(msg: &str) -> bool {
    print!("{} [Y/n]: ", msg);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let v = input.trim().to_lowercase();

    v.is_empty() || v == "y" || v == "yes"
}



pub fn rename_reference(old_id: &str, new_id: &str) {
    rename_reference_file(old_id, new_id);
    update_all_libraries(old_id, new_id);
}

fn rename_reference_file(old_id: &str, new_id: &str) {
    let old_path = crate::reference_store::ref_path(old_id);
    let new_path = crate::reference_store::ref_path(new_id);

    fs::rename(&old_path, &new_path)
        .expect("❌ Failed to rename reference file");
}

fn update_all_libraries(old_id: &str, new_id: &str) {
    let projects_dir = elaine_dir().join("projects");

    if let Ok(entries) = fs::read_dir(&projects_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if !is_project_file(&path) {
                continue;
            }

            update_library_refs(&path, old_id, new_id);
        }
    }
}

fn update_library_refs(path: &Path, old_id: &str, new_id: &str) {
    let contents = fs::read_to_string(path)
        .expect("❌ Failed to read library file");

    let mut project: Project =
        serde_yaml::from_str(&contents)
            .expect("❌ Failed to parse library YAML");

    let mut changed = false;

    for rid in project.refs.iter_mut() {
        if rid == old_id {
            *rid = new_id.to_string();
            changed = true;
        }
    }

    if changed {
        let new_contents =
            serde_yaml::to_string(&project)
                .expect("❌ Failed to serialize updated library");

        fs::write(path, new_contents)
            .expect("❌ Failed to write updated library file");
    }
}

fn is_project_file(path: &Path) -> bool {
    path.extension().and_then(|s| s.to_str()) == Some("yaml")
}
