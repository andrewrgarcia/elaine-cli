use std::collections::HashMap;

use colored::*;

use crate::reference::Reference;
use crate::reference_store::{load_all_refs};

/// Errors produced during selector resolution
#[derive(Debug)]
pub enum ResolveError {
    NotFound(String),
    Ambiguous {
        selector: String,
        matches: Vec<String>,
    },
}

/// Resolve a reference selector to a canonical reference ID.
///
/// Resolution order:
/// 1. Exact SID match
/// 2. Unique SID prefix
///    - 0 matches → fall through
///    - >1 matches → ambiguity error
/// 3. Exact ID match
/// 4. Unique ID prefix
/// 5. Not found
///
/// NOTE:
/// - Ambiguity is terminal
/// - Absence falls through
pub fn resolve_reference(selector: &str) -> Result<String, ResolveError> {
    let refs = load_all_refs();

    // ---- Build lookup tables ---------------------------------------------

    let mut by_sid: HashMap<&str, &Reference> = HashMap::new();
    let mut by_id: HashMap<&str, &Reference> = HashMap::new();

    for r in &refs {
        by_sid.insert(r.sid.as_str(), r);
        by_id.insert(r.id.as_str(), r);
    }

    // ---- 1. Exact SID match ----------------------------------------------

    if let Some(r) = by_sid.get(selector) {
        return Ok(r.id.clone());
    }

    // ---- 2. SID prefix match ---------------------------------------------

    let sid_matches: Vec<&Reference> = by_sid
        .iter()
        .filter(|(sid, _)| sid.starts_with(selector))
        .map(|(_, r)| *r)
        .collect();

    match sid_matches.len() {
        1 => return Ok(sid_matches[0].id.clone()),
        n if n > 1 => {
            return Err(ResolveError::Ambiguous {
                selector: selector.to_string(),
                matches: sid_matches.iter().map(|r| r.id.clone()).collect(),
            });
        }
        _ => {} // fall through
    }

    // ---- 3. Exact ID match -----------------------------------------------

    if let Some(r) = by_id.get(selector) {
        return Ok(r.id.clone());
    }

    // ---- 4. ID prefix match ----------------------------------------------

    let id_matches: Vec<&Reference> = by_id
        .iter()
        .filter(|(id, _)| id.starts_with(selector))
        .map(|(_, r)| *r)
        .collect();

    match id_matches.len() {
        1 => Ok(id_matches[0].id.clone()),
        n if n > 1 => Err(ResolveError::Ambiguous {
            selector: selector.to_string(),
            matches: id_matches.iter().map(|r| r.id.clone()).collect(),
        }),
        _ => Err(ResolveError::NotFound(selector.to_string())),
    }
}

/// Pretty-print a resolve error (CLI-facing)
pub fn print_resolve_error(err: ResolveError) {
    match err {
        ResolveError::NotFound(sel) => {
            eprintln!(
                "{}",
                format!("❌ Reference not found: '{}'", sel)
                    .red()
                    .bold()
            );
        }
        ResolveError::Ambiguous { selector, matches } => {
            eprintln!(
                "{}",
                format!("❌ Ambiguous selector '{}'", selector)
                    .red()
                    .bold()
            );
            for m in matches {
                eprintln!("  {}", m.dimmed());
            }
        }
    }
}

