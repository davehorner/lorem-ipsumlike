use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

use lorem_ipsumlike::ipsum::classify_word_with_speller;
use lorem_ipsumlike::ipsum_classifier;
// Import the external spellcheck crate.
use spellcheck::Speller;

// Import our corpus (from the dolar module) and classifier.
use lorem_ipsumlike::dolor::corpus;
use lorem_ipsumlike::ipsum::IpsumStatus;
use lorem_ipsumlike::ipsum_classifier::{classify_word};

fn main() -> anyhow::Result<()> {
    // Load training data from training.txt in the manifest directory.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable is not set");
    let training_path = Path::new(&manifest_dir).join("training.txt");
    let training_text = fs::read_to_string(&training_path)
        .expect("Failed to read training.txt from the manifest directory");

    // Create and train the spellchecker.
    let mut speller = Speller {
        letters: "abcdefghijklmnopqrstuvwxyz".to_string(),
        n_words: HashMap::new(),
    };
    speller.train(&training_text);

    // Read all lines from standard input.
    let stdin = io::stdin();
    let input_text: String = stdin.lock().lines().collect::<Result<Vec<_>, _>>()?.join(" ");

    // Split the text into words and filter out those that are considered "ipsum".
    // Here, we assume that both FullyIpsum and Partial are not real words.
    let filtered_words: Vec<&str> = input_text
        .split_whitespace()
        .filter(|word| lorem_ipsumlike::ipsum_classifier::classify_word(&speller, word) == IpsumStatus::NotLorem)
        .collect();

    // Print the filtered text.
    println!("{}", filtered_words.join(" "));
    Ok(())
}

