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

#[cfg(test)]
mod tests {
    use super::*;
    use models::CommandResult;

    #[test]
    fn test_hello_command_success() {
        // Given
        let name = "Alice";
        let command = commands::HelloCommand::new(name.to_string());

        // When
        let result = command.execute();

        // Then
        assert!(matches!(result, CommandResult::Success(_)));
        if let CommandResult::Success(msg) = result {
            assert_eq!(msg, "Hello, Alice!");
        }
    }

    #[tokio::test]
    async fn test_ip_command_success() {
        // Given
        let command = commands::IpCommand::new();

        // When
        let result = command.execute().await;

        // Then
        assert!(matches!(result, CommandResult::Success(_)));
        if let CommandResult::Success(ip) = result {
            assert!(ip.parse::<std::net::IpAddr>().is_ok());
        }
    }
}
