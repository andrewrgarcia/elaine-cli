use crate::reference::Reference;
use crate::search::engine::SearchResult;

pub fn search(r: &Reference) -> Vec<SearchResult> {
    let q = format!(
        "\"{}\" \"{}\" {}",
        r.title,
        r.authors.get(0).unwrap_or(&"".into()),
        r.year.unwrap_or_default()
    );

    vec![SearchResult {
        label: "Web search".into(),
        url: format!(
            "https://duckduckgo.com/?q={}",
            urlencoding::encode(&q)
        ),
        confidence: 0.2,
    }]
}
