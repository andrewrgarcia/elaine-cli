use regex::Regex;
use crate::reference::{Reference, RefKind, Identifiers, Venue};
use crate::utils::id::make_sid;

pub fn parse_bibtex(input: &str) -> Vec<Reference> {
    let entry_re =
        Regex::new(r"(?is)@(\w+)\s*\{\s*([^,]+),([\s\S]*?)\n\}").unwrap();

    let field_re =
        Regex::new(r#"(?is)(\w+)\s*=\s*(\{([^}]*)\}|"([^"]*)")"#).unwrap();

    let mut refs = Vec::new();

    for cap in entry_re.captures_iter(input) {
        let kind_raw = cap[1].to_lowercase();
        // let id = cap[2].trim().to_string();
        let id = cap[2]
            .trim()
            .replace('/', "_")
            .replace(':', "_")
            .to_string();

        let body = &cap[3];

        let mut title: Option<String> = None;
        let mut authors = Vec::new();
        let mut editors = Vec::new();
        let mut year = None;
        let mut identifiers = Identifiers::default();

        let mut venue = Venue {
            journal: None,
            booktitle: None,
            publisher: None,
            series: None,
            volume: None,
            issue: None,
            pages: None,
            address: None,
        };

        for f in field_re.captures_iter(body) {
            let key = f[1].to_lowercase();
            let raw_val = f.get(3).or_else(|| f.get(4)).unwrap().as_str();

            let val = raw_val
                .replace('\n', " ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            match key.as_str() {
                "title" => title = Some(val),
                "author" => authors = split_names(&val),
                "editor" => editors = split_names(&val),
                "year" => year = val.parse().ok(),
                "doi" => identifiers.doi = Some(val),
                "isbn" => identifiers.isbn = Some(val),
                "url" => identifiers.url = Some(val),
                "journal" => venue.journal = Some(val),
                "booktitle" => venue.booktitle = Some(val),
                "publisher" => venue.publisher = Some(val),
                "series" => venue.series = Some(val),
                "volume" => venue.volume = Some(val),
                "number" => venue.issue = Some(val),
                "pages" => venue.pages = Some(val),
                "address" | "location" => venue.address = Some(val),
                _ => {}
            }
        }

        let title = match title {
            Some(t) => t,
            None => {
                eprintln!("⚠️  Skipping entry '{}' (missing title)", id);
                continue;
            }
        };

        let kind = match kind_raw.as_str() {
            "article" => RefKind::Article,
            "inproceedings" => RefKind::InProceedings,
            "incollection" => RefKind::InCollection,
            "inbook" => RefKind::InBook,
            "book" => RefKind::Book,
            _ => RefKind::Misc,
        };

        let reference = Reference {
            id,
            sid: make_sid(),
            kind,
            title,
            authors,
            editors,
            year,
            identifiers,
            venue: Some(venue),
            tags: Vec::new(),
            notes: None,
            attachments: Vec::new(),
        };

        refs.push(reference);
    }

    refs
}

fn split_names(s: &str) -> Vec<String> {
    s.split(" and ")
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
}
