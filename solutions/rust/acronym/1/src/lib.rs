pub fn abbreviate(phrase: &str) -> String {
    // todo!("Given the phrase '{phrase}', return its acronym");
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().take(1).map(|c|c.to_ascii_uppercase()).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect()
}
