use std::io::{self, Read};
use colored::*;
use crate::bibtex::parse_bibtex;
use crate::reference_store::create_or_update_ref;
use crate::state::load_index;
use crate::project_store::{load_project, save_project};
use crate::reference::{Reference, RefKind, Identifiers, Venue};
use crate::utils::id::{make_ref_id, make_sid};

pub fn run_add(interactive: bool, args: Vec<String>) {
    if interactive {
        run_add_interactive();
        return;
    }

    if !args.is_empty() && args.iter().all(|a| is_bib_file(a)) {
        run_add_bib_files(args);
        return;
    }

    if args.len() >= 2 {
        run_add_manual(args);
        return;
    }

    run_add_bibtex();
}


fn run_add_interactive() {
    println!("{}", "🧩 Interactive reference entry".bold());

    // --- Core fields ----------------------------------------------------

    let title = prompt_required("Title");

    let authors_raw = prompt_required("Authors (use 'and' between names)");
    let authors: Vec<String> = authors_raw
        .split(" and ")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let year = prompt_optional("Year")
        .and_then(|y| y.parse::<u16>().ok());

    // --- Kind ---
    let kind = match prompt_optional("Kind [Article]")
        .as_deref()
        .unwrap_or("Article")
        .to_lowercase()
        .as_str()
    {
        "inproceedings" => RefKind::InProceedings,
        "incollection" => RefKind::InCollection,
        "inbook" => RefKind::InBook,
        "book" => RefKind::Book,
        "misc" => RefKind::Misc,
        _ => RefKind::Article,
    };

    // --- Editors ---
    let editors = prompt_optional("Editors (use 'and')")
        .map(|s| {
            s.split(" and ")
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .collect()
        })
        .unwrap_or_else(Vec::new);

    println!("{}", "--- Venue ---".dimmed());

    let venue = Venue {
        journal: prompt_optional("Journal"),
        booktitle: prompt_optional("Booktitle"),
        publisher: prompt_optional("Publisher"),
        series: prompt_optional("Series"),
        volume: prompt_optional("Volume"),
        issue: prompt_optional("Issue"),
        pages: prompt_optional("Pages"),
        address: prompt_optional("Address"),
    };

    println!("{}", "--- Identifiers ---".dimmed());

    let identifiers = Identifiers {
        doi: prompt_optional("DOI"),
        url: prompt_optional("URL"),
        isbn: prompt_optional("ISBN"),
        arxiv: prompt_optional("arXiv"),
    };

    let tags = prompt_optional("Tags (comma-separated)")
        .map(|s| {
            s.split(',')
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .collect()
        })
        .unwrap_or_else(Vec::new);

    let notes = prompt_optional("Notes");

    // --- Build reference ---
    let id = make_ref_id(&authors, year, &title);

    let reference = Reference {
        id: id.clone(),
        sid: make_sid(),
        kind,
        title,
        authors,
        editors,
        year,
        identifiers,
        venue: Some(venue),
        tags,
        notes,
        attachments: Vec::new(),
    };

    create_or_update_ref(reference);
    attach_to_active_project(&id);

    println!(
        "{}",
        format!("📚 Added reference '{}'", id)
            .bright_green()
            .bold()
    );
}


fn run_add_bib_files(paths: Vec<String>) {
    let mut combined = String::new();

    for p in &paths {
        match std::fs::read_to_string(p) {
            Ok(s) => {
                let normalized = normalize_bibtex_for_regex(&s);
                combined.push_str(&normalized);
                combined.push('\n');
            }
            Err(e) => {
                eprintln!(
                    "{} {} ({})",
                    "❌ Failed to read".red().bold(),
                    p,
                    e
                );
                return;
            }
        }
    }

    let refs = parse_bibtex(&combined);

    if refs.is_empty() {
        eprintln!("{}", "❌ No BibTeX entries detected".red().bold());
        return;
    }

    let index = load_index();
    let mut project = index
        .active_project
        .as_ref()
        .map(|pid| load_project(pid));

    for r in refs {
        let rid = r.id.clone();
        create_or_update_ref(r);

        if let Some(ref mut p) = project {
            if !p.refs.contains(&rid) {
                p.refs.push(rid);
            }
        }
    }

    if let Some(p) = project {
        save_project(&p);
        println!(
            "{}",
            format!("🔗 Attached references to project '{}'", p.id)
                .bright_green()
                .bold()
        );
    }
}


fn run_add_manual(args: Vec<String>) {
    let title = args[0].clone();
    let authors: Vec<String> = args[1]
        .split(" and ")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if authors.is_empty() {
        eprintln!("{}", "❌ Author field cannot be empty".red().bold());
        return;
    }

    let year = args.get(2).and_then(|y| y.parse::<u16>().ok());

    let id = make_ref_id(&authors, year, &title);

    let reference = Reference {
        id: id.clone(),
        sid: make_sid(), // ✅ REQUIRED
        kind: RefKind::Article,
        title,
        authors,
        editors: vec![],
        year,
        identifiers: Identifiers::default(),
        venue: Some(Venue {
            journal: None,
            booktitle: None,
            publisher: None,
            series: None,
            volume: None,
            issue: None,
            pages: None,
            address: None,
        }),
        tags: vec![],
        notes: None,
        attachments: Vec::new()
    };

    create_or_update_ref(reference);
    attach_to_active_project(&id);

    println!(
        "{}",
        format!("📚 Added reference '{}'", id)
            .bright_green()
            .bold()
    );
}


fn run_add_bibtex() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("❌ Failed to read stdin");

    if input.trim().is_empty() {
        eprintln!("{}", "❌ No input provided".red().bold());
        return;
    }

    let normalized = normalize_bibtex_for_regex(&input);
    let refs = parse_bibtex(&normalized);

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
                .bold()
        );
    }
}


// --------------------------------------------------
// Helpers
// --------------------------------------------------
fn is_bib_file(arg: &str) -> bool {
    arg.ends_with(".bib") && std::path::Path::new(arg).exists()
}

fn normalize_bibtex_for_regex(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();
            if trimmed == "}" {
                "}".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}


fn attach_to_active_project(ref_id: &str) {
    let index = load_index();
    if let Some(pid) = index.active_project {
        let mut project = load_project(&pid);
        if !project.refs.contains(&ref_id.to_string()) {
            project.refs.push(ref_id.to_string());
            save_project(&project);
        }
    }
}

fn prompt_required(label: &str) -> String {
    use std::io::{stdin, stdout, Write};

    loop {
        print!("{}: ", label);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let v = input.trim();

        if !v.is_empty() {
            return v.to_string();
        }

        println!(
            "{}",
            format!("⚠️  {} is required.", label).yellow()
        );
    }
}

fn prompt_optional(label: &str) -> Option<String> {
    use std::io::{stdin, stdout, Write};

    print!("{} [optional]: ", label);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let v = input.trim();

    if v.is_empty() {
        None
    } else {
        Some(v.to_string())
    }
}
