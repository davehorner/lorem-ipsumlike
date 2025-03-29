use clap::{Parser, Subcommand};
use anyhow::Result;
use lorem_ipsumlike::lorem;

#[derive(Parser, Debug)]
#[command(name = "lorem_tool", version = "0.1.0", about = "Generate and detect Lorem Ipsum-like text", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate Lorem Ipsum-like text.
    Gen {
        /// Seed text for generating lorem ipsum (optional).
        #[arg(short, long, default_value = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")]
        seed: String,
        /// Number of words to generate.
        #[arg(short, long, default_value_t = 50)]
        length: usize,
    },
    /// Detect if the given text is Lorem Ipsum-like.
    Det {
        /// The text to detect.
        text: String,
        /// Seed text for generating a sample (optional).
        #[arg(short, long, default_value = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")]
        seed: String,
        /// Number of words for the generated sample.
        #[arg(short, long, default_value_t = 50)]
        length: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gen { seed, length } => {
            let generated = lorem::generate_lorem_sample(seed, *length);
            println!("Generated Lorem Ipsum-like text:\n{}", generated);
        },
        Commands::Det { text, seed, length } => {
            let is_lorem = lorem::detect_lorem_ipsum(text, seed, *length);
            println!("Text is {}Lorem Ipsum-like", if is_lorem { "" } else { "NOT " });
        },
    }
    Ok(())
}

