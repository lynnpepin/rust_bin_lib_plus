use lib::{test, create, delete};
use clap::{Parser, Subcommand};
use inquire::{Text, Password, Confirm, Select};

// Parse commands and arguments from the CLI
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// If verbose is set, print extra messages while running.
    #[arg(short, long)]
    verbose: Option<bool>,

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
    Delete { },
}


pub fn main() {
    //test();
    let cli = Cli::parse();

    println!("{cli:?}");

    let verbose = match cli.verbose {
        Some(verbose)  => verbose,
        None => {
            Confirm::new("Verbosity?")
                .with_default(true)
                .with_help_message("If set, extra statements will print while running")
                .prompt()
                .unwrap()
        },
    };
    
    match &cli.command {
        Commands::Create { key, value, password } => {
            let key: String = match key {
                Some(key) => key.to_owned(),
                None      => {
                    Text::new("Provide key for new record:")
                        .with_default("abcdef")
                        .with_help_message("This key will be associated with a value to be looked up later.")
                        .prompt()
                        .unwrap()
                },
            };
            let value: i64 = match value {
                Some(value) => value.to_owned(),
                None      => {
                    Text::new("Provide value for new record:")
                        .with_default("1000")
                        .with_help_message("This value will be associated with key `{key}` during lookup.")
                        .prompt()
                        .unwrap()
                        .parse()
                        .unwrap()
                },
            };
            let password: String = match password {
                Some(password) => password.to_owned(),
                None      => {
                    Password::new("Provide a password (optional)")
                        .with_help_message("Password keys are not shown")
                        .prompt()
                        .unwrap()
                },
            };
            create(key, value, password);
        },
        Commands::Delete { } => {
            delete();
        },
        _ => {
            if verbose {
                println!("hey, how did you even activate the default case?");
            }
        }
    }
}