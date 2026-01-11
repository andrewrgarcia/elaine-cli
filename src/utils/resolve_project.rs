use std::collections::HashMap;
use colored::*;

use crate::project::Project;
use crate::project_store::load_all_projects;

/// Errors during project resolution
#[derive(Debug)]
pub enum ResolveProjectError {
    NotFound(String),
    Ambiguous {
        selector: String,
        matches: Vec<String>,
    },
}

/// Resolve project selector to canonical project ID
///
/// Resolution order:
/// 1. Exact SID
/// 2. Unique SID prefix
/// 3. Exact ID
/// 4. Unique ID prefix
pub fn resolve_project(selector: &str) -> Result<String, ResolveProjectError> {
    let projects = load_all_projects();

    let mut by_sid: HashMap<&str, &Project> = HashMap::new();
    let mut by_id: HashMap<&str, &Project> = HashMap::new();

    for p in &projects {
        by_id.insert(&p.id, p);
        by_sid.insert(&p.sid, p);
    }

    // 1. Exact SID
    if let Some(p) = by_sid.get(selector) {
        return Ok(p.id.clone());
    }

    // 2. SID prefix
    let sid_matches: Vec<&Project> = by_sid
        .iter()
        .filter(|(sid, _)| sid.starts_with(selector))
        .map(|(_, p)| *p)
        .collect();

    match sid_matches.len() {
        1 => return Ok(sid_matches[0].id.clone()),
        n if n > 1 => {
            return Err(ResolveProjectError::Ambiguous {
                selector: selector.to_string(),
                matches: sid_matches.iter().map(|p| p.id.clone()).collect(),
            });
        }
        _ => {}
    }

    // 3. Exact ID
    if let Some(p) = by_id.get(selector) {
        return Ok(p.id.clone());
    }

    // 4. ID prefix
    let id_matches: Vec<&Project> = by_id
        .iter()
        .filter(|(id, _)| id.starts_with(selector))
        .map(|(_, p)| *p)
        .collect();

    match id_matches.len() {
        1 => Ok(id_matches[0].id.clone()),
        n if n > 1 => Err(ResolveProjectError::Ambiguous {
            selector: selector.to_string(),
            matches: id_matches.iter().map(|p| p.id.clone()).collect(),
        }),
        _ => Err(ResolveProjectError::NotFound(selector.to_string())),
    }
}

pub fn print_project_resolve_error(err: ResolveProjectError) {
    match err {
        ResolveProjectError::NotFound(sel) => {
            eprintln!(
                "{}",
                format!("❌ Project not found: '{}'", sel).red().bold()
            );
        }
        ResolveProjectError::Ambiguous { selector, matches } => {
            eprintln!(
                "{}",
                format!("❌ Ambiguous project selector '{}'", selector)
                    .red()
                    .bold()
            );
            for m in matches {
                eprintln!("  {}", m.dimmed());
            }
        }
    }
}
