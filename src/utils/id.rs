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
        .take(10) // 🔒 HARD LIMIT
        .collect();

    format!("{}{:04}{}", author_part, year_part, title_part)
}
