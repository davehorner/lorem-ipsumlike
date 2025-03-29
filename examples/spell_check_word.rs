
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use spellcheck::Speller;

fn main() {
    // Expect exactly one argument: the word to check.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: {} <word>", args[0]).unwrap();
        writeln!(std::io::stderr(), "Example: {} tometo", args[0]).unwrap();
        std::process::exit(1);
    }

    // Get the manifest directory from the environment variable set by Cargo.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable is not set");
    // Build the path to training.txt in the manifest directory.
    let training_path = Path::new(&manifest_dir).join("training.txt");
    let contents = fs::read_to_string(&training_path)
        .expect("Failed to read training.txt from the manifest directory");

    // Create and train the speller.
    let mut speller = Speller {
        letters: "abcdefghijklmnopqrstuvwxyz".to_string(),
        n_words: HashMap::new(),
    };
    speller.train(&contents);


    // Instead of correcting, we simply check if the word exists in the dictionary.
    if speller.n_words.contains_key(&args[1]) {
        println!("{} is spelled correctly.", args[1]);
    } else {
        println!("{} is spelled wrong.", args[1]);
    }

    // Correct the supplied word.
    println!("{} -> {}", &args[1], speller.correct(&args[1]));
}

