use colored::*;
use std::path::Path;

use crate::utils::resolve::{resolve_reference, print_resolve_error};
use crate::reference_store::{load_ref, save_ref};

pub fn run_attach(selector: String, path: String) {
    let ref_id = match resolve_reference(&selector) {
        Ok(id) => id,
        Err(e) => {
            print_resolve_error(e);
            return;
        }
    };

    let mut r = match load_ref(&ref_id) {
        Some(r) => r,
        None => {
            eprintln!("{}", "❌ Reference not found".red().bold());
            return;
        }
    };

    let p = Path::new(&path);
    if !p.exists() {
        eprintln!(
            "{}",
            format!("❌ File does not exist: {}", path).red().bold()
        );
        return;
    }

    let abs = p
        .canonicalize()
        .unwrap_or_else(|_| p.to_path_buf())
        .to_string_lossy()
        .to_string();

    if r.attachments.contains(&abs) {
        println!("{}", "ℹ️  Attachment already linked".yellow());
        return;
    }

    r.attachments.push(abs.clone());
    save_ref(&r);

    println!(
        "{}",
        format!("📎 Linked attachment → {}", abs)
            .bright_green()
            .bold()
    );
}
