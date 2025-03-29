
pub mod lorem;
pub mod ipsum;
pub mod dolor;
pub mod ipsum_classifier;
pub mod markov_amet;

#[cfg(test)]
mod spellcheck_tests {
    use super::*; // Import functions from the current module
    use spellcheck::Speller;
    use std::collections::HashMap;

    /// Helper function: creates a new Speller and trains it on a simple sentence.
    fn train_speller() -> Speller {
        let mut n_words = HashMap::new();
        let mut speller = Speller {
            letters: "abcdefghijklmnopqrstuvwxyz".to_string(),
            n_words,
        };
        // Use a classic sentence that contains all letters.
        let training_text = "the quick brown fox jumps over the lazy dog";
        speller.train(training_text);
        speller
    }

    #[test]
    fn test_spellcheck_correct() {
        let mut speller = train_speller();
        // Test individual corrections.
        let corrected_quik = speller.correct("quik");
        // Expect "quick" (or the best approximation from the training set)
        assert_eq!(corrected_quik, "quick", "Expected 'quik' to be corrected to 'quick'");

        let corrected_lzy = speller.correct("lzy");
        assert_eq!(corrected_lzy, "lazy", "Expected 'lzy' to be corrected to 'lazy'");
    }

    #[test]
    fn test_spellcheck_sentence() {
        let mut speller = train_speller();
        // Given a sentence with some misspellings.
        let sentence = "the quik borwn fox jumpd over teh lzy dog";
        let corrected: Vec<String> = sentence
            .split_whitespace()
            .map(|w| speller.correct(w))
            .collect();

        let expected = vec![
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"
        ];
        assert_eq!(
            corrected, expected,
            "Expected the corrected sentence to match the expected output"
        );
    }
}

