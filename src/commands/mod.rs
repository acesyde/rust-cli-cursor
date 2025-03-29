//! Command implementations for the CLI

use crate::models::CommandResult;
use crate::services::IpService;

/// Command for greeting users
pub struct HelloCommand {
    name: String,
}

impl HelloCommand {
    /// Creates a new hello command
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Executes the hello command
    pub fn execute(&self) -> CommandResult {
        CommandResult::success(format!("Hello {}!", self.name))
    }
}

/// Command for getting IP address
pub struct IpCommand {
    service: IpService,
}

impl IpCommand {
    /// Creates a new IP command
    pub fn new() -> Self {
        Self {
            service: IpService::new(),
        }
    }

    /// Executes the IP command
    pub async fn execute(&self) -> CommandResult {
        self.service.get_public_ip().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_command() {
        let command = HelloCommand::new("John".to_string());
        let result = command.execute();
        assert!(matches!(result, CommandResult::Success(msg) if msg == "Hello John!"));
    }
} 