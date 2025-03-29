use clap::{Parser, Subcommand};
use reqwest;

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
            println!("Hello {}!", name);
        }
        Commands::Ip => {
            match get_ip().await {
                Ok(ip) => println!("Your IP address is: {}", ip),
                Err(e) => eprintln!("Error getting IP address: {}", e),
            }
        }
    }
}

async fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.ipify.org?format=json").await?;
    let json: serde_json::Value = response.json().await?;
    let ip = json["ip"].as_str().ok_or("IP not found in response")?;
    Ok(ip.to_string())
}
