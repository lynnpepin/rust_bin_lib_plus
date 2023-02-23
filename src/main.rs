use lib::test;
use clap::{Parser, Subcommand};

// Parse commands and arguments from the CLI
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// If verbose is set, print extra messages while running.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a structure (example command)
    Create {
        #[arg(short, long)]
        key: Option<String>,

        #[arg(short, long="val")]
        value: Option<i64>,

        #[arg(short='X', long)]
        password: Option<String>,
    },
    /// Delete the structure (example command)
    Delete {
        #[arg(short, long)]
        force: Option<String>,
    },
}


pub fn main() {
    //test();
    let cli = Cli::parse();

    println!("{cli:?}");
}