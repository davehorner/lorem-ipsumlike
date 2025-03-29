use spellcheck::Speller;
use lorem_ipsumlike::ipsum::{add_ipsum, classify_word_with_speller, remove_ipsum, scan_text};

fn main() -> anyhow::Result<()> {
    // Create and train the speller
    let mut speller = Speller {
        letters: "abcdefghijklmnopqrstuvwxyz".to_string(),
        n_words: std::collections::HashMap::new(),
    };
    speller.train("tomato potato lorem ipsum dolor sit amet consectetur adipiscing elit");

    let test_word = "lorempsm";
    let classification = classify_word_with_speller(&speller, test_word);
    println!("Classification for '{}': {:?}", test_word, classification);

    let sample_text = "lorempsm dolor sssipsum amet";
    let scanned = scan_text(&speller, sample_text);
    for (range, status) in scanned {
        println!("Word at {:?} -> {:?}", range, status);
    }

    let cleaned = remove_ipsum(&speller, sample_text);
    println!("Cleaned text: {}", cleaned);

    let added = add_ipsum(cleaned.as_str());
    println!("After adding ipsum: {}", added);

    Ok(())
}
