use std::ops::Range;

use crate::ipsum_classifier::classify_word;
// Assuming Speller is defined in a module named `speller` within the crate
use spellcheck::Speller;


/// Represents the classification result of a word.
#[derive(Debug, PartialEq)]
pub enum IpsumStatus {
    /// Not Ipsum or Lorem
    NotLorem,
    /// No valid substring was found in the word the word appears to be pure ipsum.
    FullyIpsum,
    /// A valid substring was found inside the word.
    /// Contains the valid substring and its position (start..end) within the word.
    Partial { valid: String, range: Range<usize> },
}

// /// A simple classifier function that uses the dolar corpus to decide if a word is "Ipsum" (i.e. filler) or not.
// /// If the entire word is found in the corpus, it is classified as FullyIpsum.  
// /// Otherwise, if a substring from the corpus covers at least 70% of the word, it's Partial;
// /// if no substring is found, the word is NotLorem.
// pub fn classify_word(word: &str, corpus: &HashSet<&'static str>) -> IpsumStatus {
//     let lower = word.to_lowercase();

//     // If the word is entirely in the corpus, it's fake.
//     if corpus.contains(lower.as_str()) {
//         return IpsumStatus::FullyIpsum;
//     }

//     // Otherwise, search for valid substrings from the corpus.
//     let mut best: Option<(String, Range<usize>)> = None;
//     for i in 0..lower.len() {
//         for j in (i + 1)..=lower.len() {
//             let sub = &lower[i..j];
//             if corpus.contains(sub) {
//                 best = match best {
//                     Some((ref best_str, ref best_range)) => {
//                         if sub.len() > best_str.len() {
//                             Some((sub.to_string(), i..j))
//                         } else {
//                             Some((best_str.clone(), best_range.clone()))
//                         }
//                     }
//                     None => Some((sub.to_string(), i..j)),
//                 };
//             }
//         }
//     }

//     if let Some((valid, range)) = best {
//         let ratio = valid.len() as f64 / lower.len() as f64;
//         if ratio >= 0.7 {
//             IpsumStatus::Partial { valid, range }
//         } else {
//             IpsumStatus::FullyIpsum
//         }
//     } else {
//         IpsumStatus::NotLorem
//     }
// }

// /// Given a reference to a trained speller and a word, classify it as either Correct,
// /// FullyIpsum, or Partial if it contains a valid substring.
// /// 
// /// The spellers dictionary is assumed to be stored in lowercase.
// /// (This function performs case-insensitive matching.)
pub fn classify_word_with_speller(speller: &Speller, word: &str) -> IpsumStatus {
    let lower_word = word.to_lowercase();

    // If the whole word is found, it's correct.
    if speller.n_words.contains_key(&lower_word) {
        return IpsumStatus::NotLorem;
    }

    // Otherwise, search for valid substrings.
    // We'll try every possible substring and pick the longest valid one.
    let mut best: Option<(String, Range<usize>)> = None;
    for i in 0..lower_word.len() {
        for j in (i + 1)..=lower_word.len() {
            let sub = &lower_word[i..j];
            if speller.n_words.contains_key(sub) {
                let candidate = (sub.to_string(), i..j);
                best = match best {
                    Some((ref best_str, ref best_range)) => {
                        if candidate.0.len() > best_str.len() {
                            Some(candidate)
                        } else {
                            Some((best_str.clone(), best_range.clone()))
                        }
                    }
                    None => Some(candidate),
                };
            }
        }
    }

    match best {
        Some((valid, range)) => IpsumStatus::Partial { valid, range },
        None => IpsumStatus::FullyIpsum,
    }
}

/// Scans the given text, splitting it into words and returning a vector with the starting
/// and ending character positions (in the original text) along with the IpsumStatus for each word.
pub fn scan_text(speller: &Speller, text: &str) -> Vec<(Range<usize>, IpsumStatus)> {
    let mut results = Vec::new();
    let mut index = 0;
    for word in text.split_whitespace() {
        let start = text[index..].find(word).map(|pos| index + pos).unwrap_or(index);
        let end = start + word.len();
        let status = classify_word_with_speller(speller, word);
        results.push((start..end, status));
        index = end; // advance index; note: for simplicity, we assume single spaces.
    }
    results
}

/// Removes words that are detected as ipsum (either fully or partially) from the input text.
/// Returns a new string with only correct words.
pub fn remove_ipsum(speller: &Speller, text: &str) -> String {
    text.split_whitespace()
        .filter(|word| classify_word_with_speller(speller, word) == IpsumStatus::NotLorem)
        .collect::<Vec<_>>()
        .join(" ")
}

/// For demonstration, a function that ipsum back in.
/// This is a placeholder that, for every removed word, inserts a generic Lorem Ipsum phrase.
pub fn add_ipsum(text: &str) -> String {
    // For each word, if it is empty (i.e. removed), add a placeholder.
    // Here we simply append a generic lorem ipsum phrase.
    let ipsum_phrase = "lorem ipsum";
    if text.is_empty() {
        ipsum_phrase.to_string()
    } else {
        format!("{} {}", text, ipsum_phrase)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const SEED: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

    #[test]
    fn test_generate_lorem_sample_produces_ipsum() {
        // Generate a sample of 50 words using the seed.
        let sample = crate::lorem::generate_lorem_sample(SEED, 50);
        println!("Generated sample: {}", sample);
        // The generated sample should look like Lorem Ipsum,
        // so the detector should say it is ipsum-like.
        assert!(
            crate::lorem::detect_lorem_ipsum(&sample, SEED, 50),
            "The generated sample was not detected as ipsum-like"
        );
    }

    #[test]
    fn test_detect_non_ipsum_text() {
        let non_ipsum = "This is a regular English sentence that should not be classified as Lorem Ipsum.";
        assert!(
            !crate::lorem::detect_lorem_ipsum(non_ipsum, SEED, 50),
            "Non-ipsum text was incorrectly detected as ipsum-like"
        );
    }

    #[test]
    fn test_classify_word_for_ipsum_words() {
        let corpus: HashSet<&str> = crate::dolor::corpus();
        // Words from the corpus (filler) should be classified as FullyIpsum.
        assert_eq!(classify_word("lorem", &corpus), IpsumStatus::FullyIpsum);
        assert_eq!(classify_word("ipsum", &corpus), IpsumStatus::FullyIpsum);
        // A non-ipsum word (e.g. "tomato") should be classified as NotLorem.
        assert_eq!(classify_word("tomato", &corpus), IpsumStatus::NotLorem);
    }
}
