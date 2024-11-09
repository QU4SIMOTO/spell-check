use crate::bloom::BloomFilter;
use colored::Colorize;
use unicode_segmentation::{UnicodeSegmentation, UnicodeWords};

pub struct Checker(BloomFilter);

impl Checker {
    pub fn new(words: UnicodeWords) -> Self {
        let mut bloom_filter = BloomFilter::new(10_000_000, 10);
        for word in words {
            bloom_filter.insert(word);
        }
        Self(bloom_filter)
    }

    pub fn check_word(&mut self, word: &str) -> bool {
        self.0.has(word)
    }

    pub fn check_sentence(&mut self, sentence: &str) -> String {
        let mut output = String::with_capacity(sentence.len());
        for word in sentence.unicode_words() {
            if self.check_word(word) {
                output.push_str(word);
            } else {
                output.push_str(&word.red());
            }
            output.push(' ');
        }
        output
    }
}
