use crate::reference::Reference;
use crate::search::strategies::*;

pub struct SearchResult {
    pub label: String,
    pub url: String,
    pub confidence: f32,
}

pub fn search_reference(r: &Reference) -> Vec<SearchResult> {
    let mut out = Vec::new();

    // 1. DOI (hard stop)
    if let Some(res) = doi::search(r) {
        out.push(res);
        return out;
    }

    // 2. Explicit URL
    if let Some(res) = url::search(r) {
        out.push(res);
    }

    // 3. Google Scholar
    out.extend(google_scholar::search(r));

    // 4. Fallback web search
    if out.is_empty() {
        out.extend(web::search(r));
    }

    out
}
