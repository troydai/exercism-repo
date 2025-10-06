pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split([' ', '-'])
        .filter(|word| !word.is_empty())
        .map(|word| word.trim_matches(['_']))
        .map(|word| word.to_uppercase().chars().next().unwrap())
        .collect::<String>()
}
