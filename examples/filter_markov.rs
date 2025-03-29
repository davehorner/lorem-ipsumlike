use std::env;
use std::io::{self, BufRead};

/// Inserts a filler word between each pair of words in the input sentence.
/// For example, "Hello world" with filler "lorem" becomes "Hello lorem world".
fn insert_ipsum_between(sentence: &str, filler: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    if words.len() <= 1 {
        return sentence.to_string();
    }
    let mut result = Vec::new();
    for (i, &word) in words.iter().enumerate() {
        result.push(word);
        if i < words.len() - 1 {
            result.push(filler);
        }
    }
    result.join(" ")
}

/// Removes all occurrences of the filler word from the sentence.
/// This simple function splits the text on whitespace and filters out any word equal to `filler`.
fn remove_filler(sentence: &str, filler: &str) -> String {
    sentence
        .split_whitespace()
        .filter(|w| *w != filler)
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    // If arguments are provided, treat them as the sentence.
    // Otherwise, read from standard input.
    let args: Vec<String> = env::args().collect();
    let real_sentence = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        // Read all lines from stdin.
        let stdin = io::stdin();
        stdin.lock().lines().filter_map(Result::ok).collect::<Vec<String>>().join(" ")
    };

    if real_sentence.trim().is_empty() {
        eprintln!("No input sentence provided.");
        std::process::exit(1);
    }

    // Define the filler word. You could later change this or even select one randomly.
    let filler = "lorem";

    // Generate a new sentence with the filler inserted between every word.
    let generated = insert_ipsum_between(&real_sentence, filler);
    println!("Generated text:\n{}\n", generated);

    // Remove the filler words to get back the original sentence.
    let cleaned = remove_filler(&generated, filler);
    println!("Cleaned text:\n{}", cleaned);
}
