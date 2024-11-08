use spell_check::Checker;
use std::io::BufRead;
use unicode_segmentation::UnicodeSegmentation;

const WORDS: &'static str = include_str!("../words.txt");

fn main() {
    let mut spell_check = Checker::new(WORDS.unicode_words());

    let mut line = String::new();
    let stdin = std::io::stdin();
    loop {
        stdin.lock().read_line(&mut line).unwrap();
        for word in line.unicode_words() {
            let word_exists = spell_check.check_word(word);
            println!("{word} - {word_exists}");
        }
    }
}
