use std::collections::HashSet;
use std::ops::Range;
use crate::ipsum::IpsumStatus;
use spellcheck::Speller;
/// Classifies a word as either NotLorem, FullyIpsum, or Partial, using the given corpus.
/// 
/// - If the entire word (in lowercase) is found in the corpus, it's considered FullyIpsum.
/// - Otherwise, we search all substrings of the word. If the longest valid substring (found in the corpus)
///   covers at least 70% of the word, it's classified as Partial; if not, the word is fully filler (FullyIpsum).
/// - If no substring is found, the word is considered NotLorem.
///
/// # Example
/// ```
/// # use std::collections::HashSet;
/// # use ipsum_classifier::{classify_word, IpsumStatus};
/// # let corpus: HashSet<&'static str> = ["lorem", "ipsum", "dolor", "sit", "amet"].iter().cloned().collect();
/// assert_eq!(classify_word("lorem", &corpus), IpsumStatus::FullyIpsum);
/// // For "loremps", if "lorem" (5 letters) is found and "loremps" has 7 letters, 5/7 ≈ 0.71:
/// if let IpsumStatus::Partial { valid, range } = classify_word("loremps", &corpus) {
///     assert_eq!(valid, "lorem");
///     assert_eq!(range, 0..5);
/// }

/// Classifies a word as either NotLorem, FullyIpsum, or Partial.
/// 
/// This function loads the corpus (the "dolar corpus") directly from the `dolar` module.
/// 
/// - If the entire lowercased word is in the corpus, it's marked as FullyIpsum.
/// - Otherwise, if the spellchecker recognizes the entire word (i.e. it exists in `speller.n_words`),
///   it is marked as NotLorem.
/// - Otherwise, we search for valid substrings (from `speller.n_words`). If the longest such substring
///   covers at least 70% of the word’s length:
///     - If that substring is itself in the corpus, we treat the word as FullyIpsum;
///     - Otherwise, it is classified as Partial.
/// - If no valid substring is found, the word is NotLorem.
pub fn classify_word(speller: &Speller, word: &str) -> IpsumStatus {
    // Load the corpus directly.
    let corpus: HashSet<&'static str> = crate::dolor::corpus();
    let lower = word.to_lowercase();

    // If the whole word is in the corpus, it's filler.
    if corpus.contains(lower.as_str()) {
        return IpsumStatus::FullyIpsum;
    }

    // If the entire word is recognized by the spellchecker, it is considered real.
    if speller.n_words.contains_key(&lower) {
        return IpsumStatus::NotLorem;
    }

    // Otherwise, search for valid substrings from the spellcheck dictionary.
    let mut best: Option<(String, Range<usize>)> = None;
    for i in 0..lower.len() {
        for j in (i + 1)..=lower.len() {
            let sub = &lower[i..j];
            if speller.n_words.contains_key(sub) {
                best = match best {
                    Some((ref best_str, ref best_range)) => {
                        if sub.len() > best_str.len() {
                            Some((sub.to_string(), i..j))
                        } else {
                            Some((best_str.clone(), best_range.clone()))
                        }
                    }
                    None => Some((sub.to_string(), i..j)),
                };
            }
        }
    }

    if let Some((valid, range)) = best {
        let ratio = valid.len() as f64 / lower.len() as f64;
        // If the valid substring is itself a known filler word, treat the word as fully filler.
        if corpus.contains(valid.as_str()) {
            return IpsumStatus::FullyIpsum;
        }
        if ratio >= 0.7 {
            IpsumStatus::Partial { valid, range }
        } else {
            IpsumStatus::FullyIpsum
        }
    } else {
        IpsumStatus::NotLorem
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Import the dolar corpus module – here we assume you have a module `dolar` that defines `corpus()`.
    // In your crate, ensure that `pub mod dolar;` is declared in lib.rs.
    use crate::dolor::corpus;

    #[test]
    fn test_classify_fully_ipsum() {
        let corp = corpus();
        // "lorem", "ipsum", "dolor", etc. should be flagged as filler.
        assert_eq!(classify_word("lorem", &corp), IpsumStatus::FullyIpsum);
        assert_eq!(classify_word("ipsum", &corp), IpsumStatus::FullyIpsum);
        assert_eq!(classify_word("dolor", &corp), IpsumStatus::FullyIpsum);
    }

    #[test]
    fn test_classify_not_lorem() {
        let corp = corpus();
        // A word not in the corpus should be considered real.
        assert_eq!(classify_word("tomato", &corp), IpsumStatus::NotLorem);
    }

    #[test]
    fn test_classify_partial() {
        let corp = corpus();
        // "loremps" is 7 characters and contains "lorem" (5 characters). 5/7 ≈ 0.71, so we expect Partial.
        if let IpsumStatus::Partial { valid, range } = classify_word("loremps", &corp) {
            assert_eq!(valid, "lorem");
            assert_eq!(range, 0..5);
        } else {
            panic!("Expected Partial classification for 'loremps'");
        }
    }
}
