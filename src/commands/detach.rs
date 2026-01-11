use colored::*;

use crate::utils::resolve::{resolve_reference, print_resolve_error};
use crate::reference_store::{load_ref, save_ref};

pub fn run_detach(selector: String, index: Option<usize>, all: bool) {
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

    if r.attachments.is_empty() {
        println!("{}", "ℹ️  No attachments to remove".yellow());
        return;
    }

    // --- Remove all ------------------------------------------------------
    if all {
        let n = r.attachments.len();
        r.attachments.clear();
        save_ref(&r);

        println!(
            "{}",
            format!("🧹 Removed {} attachment(s)", n)
                .bright_green()
                .bold()
        );
        return;
    }

    // --- Determine index -------------------------------------------------
    // Default: first attachment
    let idx = match index {
        Some(i) if i >= 1 && i <= r.attachments.len() => i - 1,
        Some(_) => {
            eprintln!(
                "{}",
                format!(
                    "❌ Invalid attachment index (1–{})",
                    r.attachments.len()
                )
                .red()
                .bold()
            );
            return;
        }
        None => 0, // 👈 THIS IS THE KEY FIX
    };

    let removed = r.attachments.remove(idx);
    save_ref(&r);

    println!(
        "{}",
        format!("🗑️  Detached {}", removed)
            .bright_green()
            .bold()
    );
}
