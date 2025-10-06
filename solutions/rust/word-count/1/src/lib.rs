use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let elements: Vec<String> = words
        .split([' ', ',', '.', '\n', ':', '!', '&', '@', '$', '%', '^', '&'])
        .map(|s| normalize(s))
        .filter(|s| s.len() > 0)
        .collect();

    let mut freqency: HashMap<String, u32> = HashMap::new();

    elements.iter().for_each(|word| {
        *(freqency.entry(word.clone()).or_insert(0)) += 1;
    });

    freqency
}

fn normalize(input: &str) -> String {
    input
        .trim_start_matches(['\''])
        .trim_end_matches(['\''])
        .to_string()
        .to_lowercase()
}