mod parser;

use chrono::Datelike;
use clap::{Parser, Subcommand};
use std::env;
use std::process::exit;

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(about = "Tool for building a dataset of books from archive.org")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Dumps all books in text format")]
    Parse {
        #[arg(short, long)]
        language: String,

        #[arg(short, long)]
        output_dir: String,

        #[arg(long, short, default_value_t = 4)]
        workers: i32,

        #[arg(long, default_value_t = 1920)]
        year_from: i32,

        #[arg(long, default_value_t = chrono::Utc::now().year())]
        year_to: i32,

        #[arg(short, long, default_value_t = true)]
        clean_text: bool,

        #[arg(short, long, default_value_t = false)]
        save_pdf: bool,
    },
    #[command(about = "Upload books as a dataset to HuggingFace")]
    Upload {
        #[arg(short, long)]
        input_dir: String,

        #[arg(short, long)]
        token: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match &args.command {
        Commands::Upload { input_dir, token } => {
            let token = token.clone().unwrap_or_else(|| {
                env::var("HF_TOKEN").unwrap_or_else(|_| {
                    eprintln!(
                        "Error: You need to explicitly specify the token from the HuggingFace \
                        account via the option (--token <TOKEN>) or via an environment variable \
                        (HF_TOKEN=<TOKEN>)"
                    );
                    exit(1)
                })
            });
            println!("Token: {}", token)
        }
        Commands::Parse {
            language,
            output_dir,
            workers,
            year_to,
            year_from,
            clean_text,
            save_pdf,
        } => {
            let language = language.to_string();
            let year_from = year_from.to_string();
            let year_to = year_to.to_string();
            let workers = workers.clone();
            let output_dir = output_dir.clone();
            let parser = parser::Parser::create(language, year_from, year_to, workers, output_dir);
            parser.start().await;
        }
    }
}
