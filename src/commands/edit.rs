use colored::*;
use std::io::{stdin, stdout, Write};

use crate::reference::Reference;
use crate::reference_store::{load_ref, save_ref};
use crate::utils::id::make_ref_id;
use crate::state::load_index;
use crate::project_store::{load_project, save_project};

pub fn run_edit(ref_id: String) {
    let mut reference = match try_load(&ref_id) {
        Some(r) => r,
        None => return,
    };

    println!(
        "{} {}",
        "✏️  Editing reference".bold(),
        ref_id.bright_green()
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

fn try_load(ref_id: &str) -> Option<Reference> {
    match load_ref(ref_id) {
        Some(r) => Some(r),
        None => {
            eprintln!(
                "{}",
                format!("❌ Reference '{}' not found", ref_id).red()
            );
            None
        }
    }
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

fn rename_reference(old_id: &str, new_id: &str) {
    let old_path = crate::reference_store::ref_path(old_id);
    let new_path = crate::reference_store::ref_path(new_id);

    std::fs::rename(&old_path, &new_path)
        .expect("Failed to rename reference file");

    let index = load_index();
    if let Some(pid) = index.active_project {
        let mut project = load_project(&pid);
        for r in project.refs.iter_mut() {
            if r == old_id {
                *r = new_id.to_string();
            }
        }
        save_project(&project);
    }
}
