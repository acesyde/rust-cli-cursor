//! Hello command implementation

use crate::models::CommandResult;

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