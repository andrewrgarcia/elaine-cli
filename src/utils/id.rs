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
    let author_part = authors
        .get(0)
        .map(|a| {
            a.split(',')
                .next()
                .unwrap_or(a)
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
        })
        .unwrap_or_else(|| "unknown".to_string());

    let year_part = year.unwrap_or(0);

    let title_part: String = title
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .take(10)
        .collect();

    format!("{}{:04}{}", author_part, year_part, title_part)
}
