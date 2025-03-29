mod commands;
mod models;
mod services;

use clap::{Parser, Subcommand};
use commands::{HelloCommand, IpCommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello to someone
    Hello {
        /// Name of the person to greet
        name: String,
    },
    /// Get your public IP address
    Ip,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello { name } => {
            let command = HelloCommand::new(name.clone());
            match command.execute() {
                models::CommandResult::Success(msg) => println!("{}", msg),
                models::CommandResult::Error(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Ip => {
            let command = IpCommand::new();
            match command.execute().await {
                models::CommandResult::Success(ip) => println!("Your IP address is: {}", ip),
                models::CommandResult::Error(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
