use crate::reference::Reference;
use crate::search::engine::SearchResult;

pub fn search(r: &Reference) -> Option<SearchResult> {
    let doi = r.identifiers.doi.as_ref()?;

    Some(SearchResult {
        label: "DOI".into(),
        url: format!("https://doi.org/{}", doi),
        confidence: 1.0,
    })
}
