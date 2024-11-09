use spell_check::{Checker, Cli};
use unicode_segmentation::UnicodeSegmentation;

const WORDS: &'static str = include_str!("../words.txt");

fn main() -> Result<(), std::io::Error> {
    let sentence = Cli::parse_arguments()?;
    let output = Checker::new(WORDS.unicode_words()).check_sentence(&sentence);
    println!("{output}");
    Ok(())
}
