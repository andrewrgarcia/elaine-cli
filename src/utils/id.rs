use uuid::Uuid;

/// Generate a globally unique opaque SID (UUID v4).
/// NEVER hash semantic identifiers again.
pub fn make_sid() -> String {
    Uuid::new_v4().to_string()
}

/// Short display helper (CLI only)
pub fn sid_short(sid: &str) -> &str {
    if sid.len() >= 8 { &sid[..8] } else { sid }
}

pub fn make_ref_id(authors: &[String], year: Option<u16>, title: &str) -> String {
    let author_part: String = authors
        .get(0)
        .map(|a| {
            let last = if a.contains(',') {
                // "Last, First"
                a.split(',')
                    .next()
                    .unwrap()
                    .trim()
            } else {
                // "First Last" → take last token
                a.split_whitespace()
                    .last()
                    .unwrap_or(a)
            };

            last.to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .take(5)
                .collect()
        })
        .unwrap_or_else(|| "unkno".to_string());


    let year_part = year.unwrap_or(0);

    let title_part: String = title
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .take(6)
        .collect();

    format!("{}{:04}{}", author_part, year_part, title_part)
}
