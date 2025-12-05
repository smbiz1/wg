use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use log::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardConfig {
    pub config_content: String,
    pub client_name: String,
    pub server_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error(String),
}

pub struct WireGuardManager {
    config_dir: PathBuf,
}

impl WireGuardManager {
    pub fn new() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;
        fs::create_dir_all(&config_dir)?;

        Ok(Self { config_dir })
    }

    /// Get platform-specific config directory
    fn get_config_dir() -> Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            // Windows: C:\Program Files\WireGuard\
            Ok(PathBuf::from("C:\\Program Files\\WireGuard"))
        }

        #[cfg(target_os = "macos")]
        {
            // macOS: /usr/local/etc/wireguard/
            Ok(PathBuf::from("/usr/local/etc/wireguard"))
        }

        #[cfg(target_os = "linux")]
        {
            // Linux: /etc/wireguard/
            Ok(PathBuf::from("/etc/wireguard"))
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            anyhow::bail!("Unsupported operating system")
        }
    }

    /// Save WireGuard configuration to disk
    pub fn save_config(&self, config: &WireGuardConfig) -> Result<PathBuf> {
        let filename = format!("privatevpn_{}.conf", config.server_id);
        let config_path = self.config_dir.join(&filename);

        info!("Saving WireGuard config to {:?}", config_path);

        // Ensure config directory exists
        fs::create_dir_all(&self.config_dir)
            .context("Failed to create config directory")?;

        // Write config file
        fs::write(&config_path, &config.config_content)
            .context("Failed to write config file")?;

        // Set appropriate permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&config_path)?.permissions();
            perms.set_mode(0o600); // Owner read/write only
            fs::set_permissions(&config_path, perms)?;
        }

        info!("Config saved successfully");
        Ok(config_path)
    }

    /// Connect to VPN using the saved configuration
    pub fn connect(&self, server_id: &str) -> Result<()> {
        let filename = format!("privatevpn_{}", server_id);
        info!("Connecting to VPN: {}", filename);

        #[cfg(target_os = "windows")]
        {
            self.connect_windows(&filename)
        }

        #[cfg(target_os = "macos")]
        {
            self.connect_macos(&filename)
        }

        #[cfg(target_os = "linux")]
        {
            self.connect_linux(&filename)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            anyhow::bail!("Unsupported operating system")
        }
    }

    /// Disconnect from VPN
    pub fn disconnect(&self, server_id: &str) -> Result<()> {
        let filename = format!("privatevpn_{}", server_id);
        info!("Disconnecting from VPN: {}", filename);

        #[cfg(target_os = "windows")]
        {
            self.disconnect_windows(&filename)
        }

        #[cfg(target_os = "macos")]
        {
            self.disconnect_macos(&filename)
        }

        #[cfg(target_os = "linux")]
        {
            self.disconnect_linux(&filename)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            anyhow::bail!("Unsupported operating system")
        }
    }

    /// Get current connection status
    pub fn get_status(&self, server_id: &str) -> ConnectionStatus {
        let filename = format!("privatevpn_{}", server_id);

        #[cfg(target_os = "windows")]
        {
            self.get_status_windows(&filename)
        }

        #[cfg(target_os = "macos")]
        {
            self.get_status_macos(&filename)
        }

        #[cfg(target_os = "linux")]
        {
            self.get_status_linux(&filename)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            ConnectionStatus::Error("Unsupported OS".to_string())
        }
    }

    /// Delete configuration file
    pub fn delete_config(&self, server_id: &str) -> Result<()> {
        let filename = format!("privatevpn_{}.conf", server_id);
        let config_path = self.config_dir.join(&filename);

        if config_path.exists() {
            fs::remove_file(&config_path)
                .context("Failed to delete config file")?;
            info!("Deleted config: {:?}", config_path);
        }

        Ok(())
    }

    // Platform-specific implementations

    #[cfg(target_os = "linux")]
    fn connect_linux(&self, interface: &str) -> Result<()> {
        debug!("Connecting on Linux: {}", interface);

        // Use wg-quick to bring up the interface
        let output = Command::new("wg-quick")
            .arg("up")
            .arg(interface)
            .output()
            .context("Failed to execute wg-quick")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("wg-quick failed: {}", stderr);
            anyhow::bail!("Failed to connect: {}", stderr);
        }

        info!("Connected successfully");
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn disconnect_linux(&self, interface: &str) -> Result<()> {
        debug!("Disconnecting on Linux: {}", interface);

        let output = Command::new("wg-quick")
            .arg("down")
            .arg(interface)
            .output()
            .context("Failed to execute wg-quick")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("wg-quick down failed: {}", stderr);
            // Don't fail if already disconnected
        }

        info!("Disconnected successfully");
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn get_status_linux(&self, interface: &str) -> ConnectionStatus {
        // Check if interface exists using `wg show`
        let output = match Command::new("wg")
            .arg("show")
            .arg(interface)
            .output()
        {
            Ok(output) => output,
            Err(e) => {
                error!("Failed to check WireGuard status: {}", e);
                return ConnectionStatus::Error(e.to_string());
            }
        };

        if output.status.success() {
            ConnectionStatus::Connected
        } else {
            ConnectionStatus::Disconnected
        }
    }

    #[cfg(target_os = "macos")]
    fn connect_macos(&self, interface: &str) -> Result<()> {
        debug!("Connecting on macOS: {}", interface);

        // macOS uses wg-quick via brew or direct install
        let output = Command::new("wg-quick")
            .arg("up")
            .arg(interface)
            .output()
            .context("Failed to execute wg-quick. Is WireGuard installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("wg-quick failed: {}", stderr);
            anyhow::bail!("Failed to connect: {}", stderr);
        }

        info!("Connected successfully");
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn disconnect_macos(&self, interface: &str) -> Result<()> {
        debug!("Disconnecting on macOS: {}", interface);

        let output = Command::new("wg-quick")
            .arg("down")
            .arg(interface)
            .output()
            .context("Failed to execute wg-quick")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("wg-quick down failed: {}", stderr);
        }

        info!("Disconnected successfully");
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn get_status_macos(&self, interface: &str) -> ConnectionStatus {
        let output = match Command::new("wg")
            .arg("show")
            .arg(interface)
            .output()
        {
            Ok(output) => output,
            Err(e) => {
                error!("Failed to check WireGuard status: {}", e);
                return ConnectionStatus::Error(e.to_string());
            }
        };

        if output.status.success() {
            ConnectionStatus::Connected
        } else {
            ConnectionStatus::Disconnected
        }
    }

    #[cfg(target_os = "windows")]
    fn connect_windows(&self, interface: &str) -> Result<()> {
        debug!("Connecting on Windows: {}", interface);

        // Windows uses wireguard.exe or wg.exe
        // First try the WireGuard GUI approach
        let output = Command::new("wireguard.exe")
            .arg("/installtunnelservice")
            .arg(format!("C:\\Program Files\\WireGuard\\{}.conf", interface))
            .output();

        match output {
            Ok(output) if output.status.success() => {
                info!("Connected successfully");
                Ok(())
            }
            _ => {
                // Fallback: try wg-quick if available
                let output = Command::new("wg-quick")
                    .arg("up")
                    .arg(interface)
                    .output()
                    .context("Failed to connect. Is WireGuard installed?")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    anyhow::bail!("Failed to connect: {}", stderr);
                }

                info!("Connected successfully");
                Ok(())
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn disconnect_windows(&self, interface: &str) -> Result<()> {
        debug!("Disconnecting on Windows: {}", interface);

        let output = Command::new("wireguard.exe")
            .arg("/uninstalltunnelservice")
            .arg(interface)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                info!("Disconnected successfully");
                Ok(())
            }
            _ => {
                // Fallback
                let output = Command::new("wg-quick")
                    .arg("down")
                    .arg(interface)
                    .output()
                    .context("Failed to disconnect")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    warn!("Failed to disconnect: {}", stderr);
                }

                info!("Disconnected successfully");
                Ok(())
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn get_status_windows(&self, interface: &str) -> ConnectionStatus {
        // Check if service is running
        let output = match Command::new("sc")
            .arg("query")
            .arg(format!("WireGuardTunnel${}", interface))
            .output()
        {
            Ok(output) => output,
            Err(e) => {
                error!("Failed to query service: {}", e);
                return ConnectionStatus::Error(e.to_string());
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.contains("RUNNING") {
            ConnectionStatus::Connected
        } else {
            ConnectionStatus::Disconnected
        }
    }

    /// Check if WireGuard is installed on the system
    pub fn is_wireguard_installed() -> bool {
        #[cfg(target_os = "linux")]
        {
            Command::new("which")
                .arg("wg")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("which")
                .arg("wg")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }

        #[cfg(target_os = "windows")]
        {
            Path::new("C:\\Program Files\\WireGuard\\wireguard.exe").exists()
                || Command::new("where")
                    .arg("wg.exe")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            false
        }
    }
}
