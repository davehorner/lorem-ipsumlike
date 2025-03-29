//! markov_amet.rs
//!
//! A simple Lorem Ipsum–like generator and detector using a Markov chain model.
//!
//! This module builds a Markov chain from a seed text (typically Lorem Ipsum–like text) and uses it
//! to generate sample text. It also includes a simple heuristic detector using Jaccard similarity.
//!
//! Note: This is a toy example and not a replacement for a full NLP model.

use rand::prelude::{IteratorRandom, SliceRandom, IndexedRandom};
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Builds a simple Markov chain model from the given seed text.
/// The model maps each word (in lowercase) to a vector of possible subsequent words.
pub fn build_markov_chain(seed: &str) -> HashMap<String, Vec<String>> {
    let mut chain: HashMap<String, Vec<String>> = HashMap::new();
    let words: Vec<&str> = seed.split_whitespace().collect();
    for window in words.windows(2) {
        if let [first, second] = window {
            let key = first.to_lowercase();
            chain.entry(key).or_default().push(second.to_string());
        }
    }
    chain
}

/// Generates text using the provided Markov chain model.
/// It starts with a random word from the model's keys and continues for `num_words` words.
pub fn generate_markov_text(chain: &HashMap<String, Vec<String>>, num_words: usize) -> String {
    let mut rng = thread_rng();
    // Use the SliceRandom trait so that choose() is available.
    let start = chain.keys().choose(&mut rng).unwrap().to_string();
    let mut result = vec![start.clone()];
    let mut current = start;
    for _ in 1..num_words {
        if let Some(choices) = chain.get(&current) {
            current = choices.choose(&mut rng).unwrap().clone();
            result.push(current.clone());
        } else {
            break;
        }
    }
    result.join(" ")
}

/// Generates a Lorem Ipsum–like sample text using the given seed and desired length (in words).
pub fn generate_lorem_sample(seed: &str, sample_length: usize) -> String {
    let chain = build_markov_chain(seed);
    generate_markov_text(&chain, sample_length)
}

/// Computes the Jaccard similarity between two sets of words.
/// The similarity is defined as (intersection size) / (union size).
pub fn jaccard_similarity(set_a: &HashSet<String>, set_b: &HashSet<String>) -> f64 {
    let intersection: HashSet<_> = set_a.intersection(set_b).cloned().collect();
    let union: HashSet<_> = set_a.union(set_b).cloned().collect();
    if union.is_empty() {
        0.0
    } else {
        intersection.len() as f64 / union.len() as f64
    }
}

/// Detects whether the input text appears to be Lorem Ipsum–like.
/// The function generates a sample from the given seed, converts both the generated sample
/// and the input text into word sets (case-insensitive), and then calculates their Jaccard similarity.
/// If the similarity is greater than 0.3, the text is flagged as Lorem Ipsum–like.
pub fn detect_lorem_ipsum(text: &str, seed: &str, sample_length: usize) -> bool {
    let sample = generate_lorem_sample(seed, sample_length);
    let sample_set: HashSet<String> = sample
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();
    let text_set: HashSet<String> = text
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();
    jaccard_similarity(&sample_set, &text_set) > 0.3
}

/// A generator function that builds the Markov chain model and generates a sample text.
/// Returns both the chain model and the generated sample.
pub fn generate_lorem_model(seed: &str, sample_length: usize) -> (HashMap<String, Vec<String>>, String) {
    let chain = build_markov_chain(seed);
    let sample = generate_markov_text(&chain, sample_length);
    (chain, sample)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

    #[test]
    fn test_generate_lorem_sample() {
        let sample = generate_lorem_sample(SEED, 20);
        println!("Generated sample: {}", sample);
        assert!(!sample.is_empty());
    }

    #[test]
    fn test_detect_lorem_ipsum() {
        let test_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
        let result = detect_lorem_ipsum(test_text, SEED, 30);
        // Since test_text is similar to the seed, we expect a positive detection.
        assert!(result);
    }
}
