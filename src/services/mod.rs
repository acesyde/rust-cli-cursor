//! Services for external API interactions

use crate::models::{CommandResult, IpResponse};
use reqwest::Client;

/// Service for IP-related operations
pub struct IpService {
    client: Client,
    base_url: String,
}

impl IpService {
    /// Creates a new IP service instance
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.ipify.org".to_string(),
        }
    }

    /// Creates a new IP service instance with a custom base URL
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Fetches the public IP address
    pub async fn get_public_ip(&self) -> CommandResult {
        match self
            .client
            .get(format!("{}/?format=json", self.base_url))
            .send()
            .await
        {
            Ok(response) => match response.json::<IpResponse>().await {
                Ok(ip_response) => CommandResult::success(ip_response.ip),
                Err(e) => CommandResult::error(format!("Failed to parse IP response: {}", e)),
            },
            Err(e) => CommandResult::error(format!("Failed to fetch IP: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_get_public_ip_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/?format=json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"ip": "1.2.3.4"}"#)
            .create();

        let service = IpService::with_base_url(server.url());
        let result = service.get_public_ip().await;

        mock.assert();
        assert!(matches!(result, CommandResult::Success(ip) if ip == "1.2.3.4"));
    }

    #[tokio::test]
    async fn test_get_public_ip_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/?format=json")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let service = IpService::with_base_url(server.url());
        let result = service.get_public_ip().await;

        mock.assert();
        assert!(matches!(result, CommandResult::Error(_)));
    }
}
