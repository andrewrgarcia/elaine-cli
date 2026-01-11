use crate::reference::Reference;
use crate::search::engine::SearchResult;

pub fn search(r: &Reference) -> Option<SearchResult> {
    let url = r.identifiers.url.as_ref()?;

    Some(SearchResult {
        label: "Provided URL".into(),
        url: url.clone(),
        confidence: 0.9,
    })
}
