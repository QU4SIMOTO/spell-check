use crate::bloom::BloomFilter;
use unicode_segmentation::UnicodeWords;

pub struct Checker(BloomFilter);

impl Checker {
    pub fn new(words: UnicodeWords) -> Self {
        let mut bloom_filter = BloomFilter::new(1000_0000, 10);
        for word in words {
            bloom_filter.insert(word);
        }
        Self(bloom_filter)
    }

    pub fn check_word(&mut self, word: &str) -> bool {
        self.0.has(word)
    }
}
