use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(default_value = "-")]
    sentence: String,
}

impl Cli {
    pub fn parse_arguments() -> Result<String, std::io::Error> {
        let sentence = Cli::parse().sentence;
        match Cli::parse().sentence.as_str() {
            "-" => std::io::read_to_string(std::io::stdin()),
            _ => Ok(sentence),
        }
    }
}
