use std::fs;
use std::path::{Path, PathBuf};
use colored::*;
use std::io::{self, Write};

use crate::reference::Reference;
use crate::utils::id::make_sid;

pub fn refs_dir() -> PathBuf {
    Path::new(".elaine").join("refs")
}

pub fn ref_path(ref_id: &str) -> PathBuf {
    refs_dir().join(format!("{}.yaml", ref_id))
}

pub fn load_ref(ref_id: &str) -> Option<Reference> {
    let path = ref_path(ref_id);
    let contents = fs::read_to_string(&path).ok()?;
    let mut r: Reference = serde_yaml::from_str(&contents).ok()?;

    // --- Lazy SID migration ---------------------------------------------
    if r.sid.len() < 16 {
        r.sid = make_sid();
        save_ref(&r);
    }

    Some(r)
}

pub fn save_ref(reference: &Reference) {
    let path = ref_path(&reference.id);
    let contents = serde_yaml::to_string(reference)
        .expect("❌ Failed to serialize reference");

    fs::write(&path, contents)
        .expect("❌ Failed to write reference file");
}

pub fn create_or_update_ref(reference: Reference) {
    let path = ref_path(&reference.id);

    if path.exists() {
        println!(
            "{}",
            format!("ℹ️  Reference '{}' already exists.", reference.id)
                .bright_yellow()
        );

        print!("Overwrite? [Y/n]: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase();

        if choice == "n" || choice == "no" {
            println!(
                "{}",
                format!("❌ Skipped reference '{}'", reference.id).red()
            );
            return;
        }

        save_ref(&reference);
        println!(
            "{}",
            format!("♻️  Updated reference '{}'", reference.id)
                .bright_green()
                .bold()
        );
    } else {
        save_ref(&reference);
        println!(
            "{}",
            format!("📚 Added reference '{}'", reference.id)
                .bright_green()
                .bold()
        );
    }
}

pub fn load_all_refs() -> Vec<Reference> {
    let mut refs = Vec::new();

    if let Ok(entries) = fs::read_dir(refs_dir()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }

            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(mut r) = serde_yaml::from_str::<Reference>(&contents) {
                    // migrate here too
                    if r.sid.len() < 16 {
                        r.sid = make_sid();
                        let _ = fs::write(&path, serde_yaml::to_string(&r).unwrap());
                    }
                    refs.push(r);
                }
            }
        }
    }

    refs
}
