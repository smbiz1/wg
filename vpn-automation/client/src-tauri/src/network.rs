use anyhow::{Context, Result};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub port: u16,
    pub region: String,
    pub country: String,
    pub tier: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<u64>, // milliseconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerList {
    pub servers: Vec<Server>,
    pub tier_info: TierInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierInfo {
    pub name: String,
    pub max_servers: usize,
    pub description: String,
}

/// Measure latency to a server using TCP connection
pub async fn measure_latency(endpoint: &str, port: u16) -> Option<u64> {
    debug!("Measuring latency to {}:{}", endpoint, port);

    let start = std::time::Instant::now();

    // Try to establish TCP connection with timeout
    let connect_result = timeout(
        Duration::from_secs(5),
        tokio::net::TcpStream::connect(format!("{}:{}", endpoint, port)),
    )
    .await;

    match connect_result {
        Ok(Ok(_)) => {
            let latency = start.elapsed().as_millis() as u64;
            debug!("Latency to {}: {} ms", endpoint, latency);
            Some(latency)
        }
        Ok(Err(e)) => {
            warn!("Failed to connect to {}: {}", endpoint, e);
            None
        }
        Err(_) => {
            warn!("Timeout connecting to {}", endpoint);
            None
        }
    }
}

/// Test latency to all servers and sort by fastest
pub async fn test_server_latencies(mut servers: Vec<Server>) -> Vec<Server> {
    debug!("Testing latency to {} servers", servers.len());

    // Test all servers concurrently
    let mut tasks = Vec::new();

    for server in servers.iter_mut() {
        let endpoint = server.endpoint.clone();
        let port = server.port;

        let task = tokio::spawn(async move {
            measure_latency(&endpoint, port).await
        });

        tasks.push((server, task));
    }

    // Collect results
    for (server, task) in tasks {
        match task.await {
            Ok(latency) => {
                server.latency = latency;
            }
            Err(e) => {
                warn!("Failed to measure latency for {}: {}", server.id, e);
                server.latency = None;
            }
        }
    }

    // Sort by latency (None values last)
    servers.sort_by(|a, b| {
        match (a.latency, b.latency) {
            (Some(a_lat), Some(b_lat)) => a_lat.cmp(&b_lat),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });

    debug!("Latency testing complete. Best server: {}",
           servers.first().map(|s| s.id.as_str()).unwrap_or("none"));

    servers
}

/// Get best server based on latency
pub async fn get_best_server(servers: Vec<Server>) -> Option<Server> {
    let tested_servers = test_server_latencies(servers).await;
    tested_servers.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_measure_latency_google_dns() {
        // Test with Google DNS (should be reachable)
        let latency = measure_latency("8.8.8.8", 53).await;
        assert!(latency.is_some());
        assert!(latency.unwrap() < 5000); // Should be less than 5 seconds
    }

    #[tokio::test]
    async fn test_measure_latency_unreachable() {
        // Test with unreachable endpoint
        let latency = measure_latency("192.0.2.1", 12345).await;
        assert!(latency.is_none());
    }

    #[tokio::test]
    async fn test_server_sorting() {
        let servers = vec![
            Server {
                id: "slow".to_string(),
                name: "Slow Server".to_string(),
                endpoint: "example.com".to_string(),
                port: 80,
                region: "Test".to_string(),
                country: "Test".to_string(),
                tier: "basic".to_string(),
                latitude: 0.0,
                longitude: 0.0,
                latency: Some(500),
            },
            Server {
                id: "fast".to_string(),
                name: "Fast Server".to_string(),
                endpoint: "example.com".to_string(),
                port: 80,
                region: "Test".to_string(),
                country: "Test".to_string(),
                tier: "basic".to_string(),
                latitude: 0.0,
                longitude: 0.0,
                latency: Some(50),
            },
        ];

        let sorted = test_server_latencies(servers).await;
        assert_eq!(sorted[0].id, "fast");
        assert_eq!(sorted[1].id, "slow");
    }
}
