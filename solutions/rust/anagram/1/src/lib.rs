use std::collections::HashSet;

pub fn normalize(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
    
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = normalize(word);
    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let candidate_normalized = normalize(candidate);
            candidate_normalized == normalized_word && candidate.to_lowercase() != word.to_lowercase()
        })
        .copied()
        .collect()
}
