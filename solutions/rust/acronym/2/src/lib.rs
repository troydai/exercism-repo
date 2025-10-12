pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split([' ', '-'])
        .filter(|word| !word.is_empty())
        .map(|word| word.trim_matches(['_']))
        .flat_map(extract_acronym)
        .collect::<String>()
}

fn extract_acronym(word: &str) -> Vec<char> {
    let all_caps = word.chars().filter(|c| c.is_uppercase()).collect::<Vec<char>>();

    if all_caps.len() == 0 {
        return vec![word.to_uppercase().chars().next().unwrap()];
    } else if all_caps.len() == word.len() {
        return word.to_uppercase().chars().take(1).collect::<Vec<char>>()
    }

    all_caps
}