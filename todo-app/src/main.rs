mod model;
mod tools;


use crate::tools::commands::{list, done, add, remove, help};
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List {},
    Add { item: String },
    Done { id: i64 },
    Remove { id: i64 },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::List {}) => {
            list();
        }
        Some(Commands::Add { item }) => {
            add(item);
        }
        Some(Commands::Done { id }) => done(id),
        Some(Commands::Remove { id }) => remove(id),
        _ => help(),
    }

}




