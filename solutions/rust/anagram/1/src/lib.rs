use std::collections::{HashMap, HashSet};

fn is_anagram(word_counter: &HashMap<char, i32>, candidate: &str) -> bool {
    // todo!("Determine if '{candidate}' is an anagram of '{word}'");
    let mut candidate_counter: HashMap<char, i32> = HashMap::new();
    for c in candidate.chars() {
        if candidate_counter.contains_key(&c)
            && word_counter.contains_key(&c)
            && *candidate_counter.get(&c).unwrap() == *word_counter.get(&c).unwrap_or(&0)
        {
            return false;
        } else {
            candidate_counter.insert(c, *candidate_counter.get(&c).unwrap_or(&0) + 1);
        }
    }
    candidate_counter == *word_counter
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut counter: HashMap<char, i32> = HashMap::new();
    let word = word.to_lowercase();
    for c in word.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }
    let mut anagrams = HashSet::new();
    for &possible_anagram in possible_anagrams {
        if word.len() == possible_anagram.len()
            && word.to_lowercase() != possible_anagram.to_lowercase()
            && is_anagram(&counter, &possible_anagram.to_lowercase())
        {
            anagrams.insert(possible_anagram);
        }
    }
    anagrams
}
