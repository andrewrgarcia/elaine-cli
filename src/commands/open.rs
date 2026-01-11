use colored::*;
use std::process::Command;

use crate::utils::resolve::{resolve_reference, print_resolve_error};
use crate::reference_store::load_ref;

pub fn run_open(selector: String) {
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

    if r.attachments.is_empty() {
        eprintln!(
            "{}",
            "❌ No attachments linked to this reference".red().bold()
        );
        return;
    }

    let path = if r.attachments.len() == 1 {
        &r.attachments[0]
    } else {
        println!("{}", "Multiple attachments:".bold());
        for (i, a) in r.attachments.iter().enumerate() {
            println!("  [{}] {}", i + 1, a.dimmed());
        }

        let idx = prompt_index(r.attachments.len());
        &r.attachments[idx]
    };

    open_path(path);
}

fn prompt_index(max: usize) -> usize {
    use std::io::{stdin, stdout, Write};

    loop {
        print!("Select attachment [1-{}]: ", max);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse::<usize>() {
            if n >= 1 && n <= max {
                return n - 1;
            }
        }

        println!("{}", "Invalid selection".yellow());
    }
}

fn open_path(path: &str) {
    let result = if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(path).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", "", path])
            .spawn()
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported OS",
        ))
    };

    if result.is_err() {
        eprintln!(
            "{}",
            format!("❌ Failed to open {}", path).red().bold()
        );
    }
}
