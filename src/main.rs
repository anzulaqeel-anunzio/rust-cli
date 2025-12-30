// Developed for Anunzio International by Anzul Aqeel. Contact +971545822608 or +971585515742. Linkedin Profile: linkedin.com/in/anzulaqeel

use clap::{Parser, Subcommand};
use colored::*;
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Greets a user
    Greet {
        /// Name of the person to greet
        #[arg(short, long)]
        name: String,
    },
    /// Prints system info (mock)
    Info,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Greet { name }) => {
            println!("{} {}, welcome to the Rust CLI Template!", "Hello".green().bold(), name);
        }
        Some(Commands::Info) => {
            println!("{}", "Rust CLI Template v0.1.0".blue().bold());
            println!("Developed for Anunzio International");
        }
        None => {
            println!("No command provided. Run with --help for more info.");
        }
    }

    Ok(())
}

// Developed for Anunzio International by Anzul Aqeel. Contact +971545822608 or +971585515742. Linkedin Profile: linkedin.com/in/anzulaqeel
