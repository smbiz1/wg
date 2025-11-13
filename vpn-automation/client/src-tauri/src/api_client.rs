use anyhow::{Context, Result};
use log::{debug, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::network::{Server, TierInfo};

const API_BASE_URL: &str = "https://api.vpn.example.com"; // TODO: Replace with actual API URL

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionResponse {
    pub success: bool,
    pub config: String,
    pub client_name: String,
    pub server: Server,
    pub available_servers: Vec<Server>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerListResponse {
    pub servers: Vec<Server>,
    pub tier: String,
    pub tier_info: TierInfo,
}

pub struct ApiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ApiClient {
    /// Create new API client with API key
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            api_key,
            base_url: API_BASE_URL.to_string(),
        })
    }

    /// Create client with custom base URL (for testing)
    pub fn with_base_url(api_key: String, base_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            api_key,
            base_url,
        })
    }

    /// Get list of available servers for the user
    pub async fn get_servers(&self) -> Result<ServerListResponse> {
        debug!("Fetching server list");

        let url = format!("{}/api/servers", self.base_url);

        let response = self.client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await
            .context("Failed to fetch servers")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("API error ({}): {}", status, error_text);
        }

        let server_list: ServerListResponse = response
            .json()
            .await
            .context("Failed to parse server list response")?;

        info!("Fetched {} servers for tier {}",
              server_list.servers.len(),
              server_list.tier);

        Ok(server_list)
    }

    /// Provision a new WireGuard configuration
    pub async fn provision(&self, server_id: Option<String>) -> Result<ProvisionResponse> {
        debug!("Provisioning VPN config");

        let url = format!("{}/api/provision", self.base_url);

        let body = if let Some(sid) = server_id {
            serde_json::json!({ "server_id": sid })
        } else {
            serde_json::json!({})
        };

        let response = self.client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .json(&body)
            .send()
            .await
            .context("Failed to provision VPN")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Provisioning failed ({}): {}", status, error_text);
        }

        let provision_response: ProvisionResponse = response
            .json()
            .await
            .context("Failed to parse provision response")?;

        info!("Successfully provisioned config for server {}",
              provision_response.server.id);

        Ok(provision_response)
    }

    /// Revoke a client configuration
    pub async fn revoke(&self, client_name: String) -> Result<()> {
        debug!("Revoking client: {}", client_name);

        let url = format!("{}/api/revoke", self.base_url);

        let body = serde_json::json!({ "client_name": client_name });

        let response = self.client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .json(&body)
            .send()
            .await
            .context("Failed to revoke client")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Revocation failed ({}): {}", status, error_text);
        }

        info!("Successfully revoked client: {}", client_name);

        Ok(())
    }

    /// Validate API key by attempting to fetch servers
    pub async fn validate_api_key(&self) -> Result<bool> {
        debug!("Validating API key");

        match self.get_servers().await {
            Ok(_) => {
                info!("API key is valid");
                Ok(true)
            }
            Err(e) => {
                debug!("API key validation failed: {}", e);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_client_creation() {
        let client = ApiClient::new("test_key".to_string());
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_custom_base_url() {
        let client = ApiClient::with_base_url(
            "test_key".to_string(),
            "http://localhost:5000".to_string(),
        );
        assert!(client.is_ok());
    }
}
