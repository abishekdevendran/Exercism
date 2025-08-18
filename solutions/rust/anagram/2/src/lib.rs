use std::collections::{HashMap, HashSet};

fn get_map(word: &str) -> HashMap<char, i32> {
    word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

fn is_anagram(word_counter: &HashMap<char, i32>, candidate: &str) -> bool {
    // todo!("Determine if '{candidate}' is an anagram of '{word}'");
    let candidate_counter: HashMap<char, i32> = get_map(candidate);
    candidate_counter == *word_counter
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let word=word.to_lowercase();
    let counter = get_map(&word);
    possible_anagrams
        .iter()
        .filter(|&el| {
            let lower_el = el.to_lowercase();
            word.len() == el.len()
                && lower_el != word.to_lowercase()
                && is_anagram(&counter, &lower_el)
        })
        .copied()
        .collect()
}
