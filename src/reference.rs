use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub id: String,
    pub kind: RefKind,

    pub title: String,
    pub authors: Vec<String>,
    pub editors: Vec<String>,
    pub year: Option<u16>,

    pub identifiers: Identifiers,
    pub venue: Option<Venue>,

    pub tags: Vec<String>,
    pub notes: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum RefKind {
    Article,
    InProceedings,
    InCollection,
    Book,
    InBook,
    Misc,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Identifiers {
    pub doi: Option<String>,
    pub arxiv: Option<String>,
    pub isbn: Option<String>,
    pub url: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Venue {
    pub journal: Option<String>,
    pub booktitle: Option<String>,
    pub publisher: Option<String>,
    pub series: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub address: Option<String>,
}


impl Reference {
    pub fn new_minimal(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            authors: Vec::new(),
            year: None,
            identifiers: Identifiers::default(),
            venue: None,
            tags: Vec::new(),
            notes: None,
        }
    }
}
