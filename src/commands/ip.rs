//! IP command implementation

use crate::models::CommandResult;
use crate::services::IpService;

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