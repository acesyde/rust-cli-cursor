//! Common types and models used across the application

/// Represents a command result
#[derive(Debug)]
pub enum CommandResult {
    /// Success result with a message
    Success(String),
    /// Error result with an error message
    Error(String),
}

impl CommandResult {
    /// Creates a success result
    pub fn success(message: impl Into<String>) -> Self {
        Self::Success(message.into())
    }

    /// Creates an error result
    pub fn error(message: impl Into<String>) -> Self {
        Self::Error(message.into())
    }
}

/// Represents an IP address response
#[derive(Debug, serde::Deserialize)]
pub struct IpResponse {
    /// The IP address
    pub ip: String,
} 