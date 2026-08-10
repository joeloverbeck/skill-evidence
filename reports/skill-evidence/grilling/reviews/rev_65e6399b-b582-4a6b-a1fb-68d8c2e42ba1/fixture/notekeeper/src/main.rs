mod store;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "notekeeper", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { body: String, #[arg(long)] tag: Vec<String> },
    List { #[arg(long)] tag: Option<String>, #[arg(long)] limit: Option<usize> },
    Search { query: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Add { body, tag } => {
            let note = store::Note::new(body, tag);
            store::append(&note).expect("write note");
            println!("added note at {}", note.written);
        }
        Command::List { tag, limit } => {
            for note in store::load_recent(tag.as_deref(), limit.unwrap_or(20)) {
                println!("{}  {}  [{}]", note.written, note.body, note.tags.join(","));
            }
        }
        Command::Search { query } => {
            for note in store::search(&query) {
                println!("{}  {}", note.written, note.body);
            }
        }
    }
}
