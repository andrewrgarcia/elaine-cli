use colored::*;
use crate::utils::resolve::{resolve_reference, print_resolve_error};
use crate::reference_store::load_ref;
use crate::search::engine::search_reference;

pub fn run_search(selector: String) {
    let ref_id = match resolve_reference(&selector) {
        Ok(id) => id,
        Err(e) => {
            print_resolve_error(e);
            return;
        }
    };

    let r = match load_ref(&ref_id) {
        Some(r) => r,
        None => {
            eprintln!("{}", "❌ Reference not found".red().bold());
            return;
        }
    };

    println!(
        "{}",
        format!(
            "🔍 Search results for: {} ({}, {})",
            r.title,
            r.authors.get(0).unwrap_or(&"".into()),
            r.year.unwrap_or_default()
        )
        .bold()
    );

    let results = search_reference(&r);

    if results.is_empty() {
        println!("{}", "⚠️  No results found".yellow());
        return;
    }

    for res in results {
        println!(
            "[{:.2}] {}\n      {}",
            res.confidence,
            res.label.bold(),
            res.url.dimmed()
        );
    }
}
