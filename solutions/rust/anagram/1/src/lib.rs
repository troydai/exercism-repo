use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result  = HashSet::new();

    let word_base = calculate_char_counter(word);
    for anagram in possible_anagrams {
        if anagram.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let anagram_base = calculate_char_counter(anagram);
        if word_base == anagram_base {
            result.insert(*anagram);
        }
    }

    result
}

fn calculate_char_counter(word: &str) -> HashMap<char, i32> {
    let mut base: HashMap<char, i32> = HashMap::new();
    for (_, c) in word.chars().enumerate() {
        base.entry(c.to_lowercase().next().unwrap()).and_modify(|e| *e += 1).or_insert(1);
    }
    base
}