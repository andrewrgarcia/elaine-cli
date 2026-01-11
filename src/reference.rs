use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub id: String,

    #[serde(default)]
    pub sid: String,

    pub kind: RefKind,
    pub title: String,
    pub authors: Vec<String>,
    pub editors: Vec<String>,
    pub year: Option<u16>,
    pub identifiers: Identifiers,
    pub venue: Option<Venue>,
    pub tags: Vec<String>,
    pub notes: Option<String>,

    #[serde(default)]
    pub attachments: Vec<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum RefKind {
    Article,
    InProceedings,
    InCollection,
    InBook,
    Book,
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
