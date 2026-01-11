use crate::reference::Reference;
use crate::search::engine::SearchResult;

pub fn search(r: &Reference) -> Vec<SearchResult> {
    let mut parts = Vec::new();

    parts.push(format!("\"{}\"", r.title));

    if let Some(a) = r.authors.get(0) {
        parts.push(a.clone());
    }

    if let Some(y) = r.year {
        parts.push(y.to_string());
    }

    let q = parts.join(" ");

    vec![SearchResult {
        label: "Google Scholar".into(),
        url: format!(
            "https://scholar.google.com/scholar?q={}",
            urlencoding::encode(&q)
        ),
        confidence: 0.85,
    }]
}
