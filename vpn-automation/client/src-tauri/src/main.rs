#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api_client;
mod network;
mod wg_manager;

use std::sync::Mutex;
use anyhow::Result;
use log::{error, info};
use tauri::{Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use serde::{Deserialize, Serialize};

use api_client::ApiClient;
use network::{Server, test_server_latencies, get_best_server};
use wg_manager::{WireGuardConfig, WireGuardManager, ConnectionStatus};

// Application state
struct AppState {
    api_key: Mutex<Option<String>>,
    current_server: Mutex<Option<Server>>,
    available_servers: Mutex<Vec<Server>>,
    current_client_name: Mutex<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectionInfo {
    status: ConnectionStatus,
    server: Option<Server>,
}

// Tauri commands (called from frontend)

#[tauri::command]
async fn save_api_key(
    api_key: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    info!("Saving API key");

    // Validate API key
    let client = ApiClient::new(api_key.clone())
        .map_err(|e| format!("Failed to create API client: {}", e))?;

    let is_valid = client.validate_api_key()
        .await
        .map_err(|e| format!("Failed to validate API key: {}", e))?;

    if is_valid {
        *state.api_key.lock().unwrap() = Some(api_key);
        info!("API key saved successfully");
        Ok(true)
    } else {
        error!("Invalid API key");
        Ok(false)
    }
}

#[tauri::command]
async fn fetch_servers(
    state: State<'_, AppState>,
) -> Result<Vec<Server>, String> {
    info!("Fetching available servers");

    let api_key = state.api_key.lock().unwrap().clone()
        .ok_or("API key not set")?;

    let client = ApiClient::new(api_key)
        .map_err(|e| format!("Failed to create API client: {}", e))?;

    let server_list = client.get_servers()
        .await
        .map_err(|e| format!("Failed to fetch servers: {}", e))?;

    // Test latency for all servers
    let servers_with_latency = test_server_latencies(server_list.servers).await;

    // Store in state
    *state.available_servers.lock().unwrap() = servers_with_latency.clone();

    info!("Fetched {} servers", servers_with_latency.len());

    Ok(servers_with_latency)
}

#[tauri::command]
async fn connect_vpn(
    server_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ConnectionInfo, String> {
    info!("Connecting to VPN");

    let api_key = state.api_key.lock().unwrap().clone()
        .ok_or("API key not set")?;

    // Get server to connect to
    let server = if let Some(sid) = server_id.clone() {
        // User selected specific server
        let servers = state.available_servers.lock().unwrap();
        servers.iter()
            .find(|s| s.id == sid)
            .cloned()
            .ok_or("Server not found")?
    } else {
        // Auto-select best server
        let servers = state.available_servers.lock().unwrap().clone();
        if servers.is_empty() {
            return Err("No servers available. Please fetch servers first.".to_string());
        }
        get_best_server(servers)
            .await
            .ok_or("Failed to select best server")?
    };

    // Provision configuration from API
    let client = ApiClient::new(api_key)
        .map_err(|e| format!("Failed to create API client: {}", e))?;

    let provision_response = client.provision(Some(server.id.clone()))
        .await
        .map_err(|e| format!("Failed to provision VPN: {}", e))?;

    // Save WireGuard configuration
    let wg_manager = WireGuardManager::new()
        .map_err(|e| format!("Failed to create WireGuard manager: {}", e))?;

    let wg_config = WireGuardConfig {
        config_content: provision_response.config,
        client_name: provision_response.client_name.clone(),
        server_id: server.id.clone(),
    };

    wg_manager.save_config(&wg_config)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    // Connect to VPN
    wg_manager.connect(&server.id)
        .map_err(|e| format!("Failed to connect: {}", e))?;

    // Update state
    *state.current_server.lock().unwrap() = Some(server.clone());
    *state.current_client_name.lock().unwrap() = Some(provision_response.client_name);

    info!("Connected to VPN: {}", server.name);

    Ok(ConnectionInfo {
        status: ConnectionStatus::Connected,
        server: Some(server),
    })
}

#[tauri::command]
async fn disconnect_vpn(
    state: State<'_, AppState>,
) -> Result<ConnectionInfo, String> {
    info!("Disconnecting from VPN");

    let server = state.current_server.lock().unwrap().clone()
        .ok_or("Not connected to any server")?;

    let wg_manager = WireGuardManager::new()
        .map_err(|e| format!("Failed to create WireGuard manager: {}", e))?;

    wg_manager.disconnect(&server.id)
        .map_err(|e| format!("Failed to disconnect: {}", e))?;

    // Clear state
    *state.current_server.lock().unwrap() = None;

    info!("Disconnected from VPN");

    Ok(ConnectionInfo {
        status: ConnectionStatus::Disconnected,
        server: None,
    })
}

#[tauri::command]
async fn get_connection_status(
    state: State<'_, AppState>,
) -> Result<ConnectionInfo, String> {
    let server = state.current_server.lock().unwrap().clone();

    if let Some(ref srv) = server {
        let wg_manager = WireGuardManager::new()
            .map_err(|e| format!("Failed to create WireGuard manager: {}", e))?;

        let status = wg_manager.get_status(&srv.id);

        Ok(ConnectionInfo {
            status,
            server: Some(srv.clone()),
        })
    } else {
        Ok(ConnectionInfo {
            status: ConnectionStatus::Disconnected,
            server: None,
        })
    }
}

#[tauri::command]
fn check_wireguard_installed() -> Result<bool, String> {
    Ok(WireGuardManager::is_wireguard_installed())
}

#[tauri::command]
async fn test_server_latency(
    server_id: String,
    state: State<'_, AppState>,
) -> Result<Option<u64>, String> {
    let servers = state.available_servers.lock().unwrap();
    let server = servers.iter()
        .find(|s| s.id == server_id)
        .ok_or("Server not found")?;

    let latency = network::measure_latency(&server.endpoint, server.port).await;

    Ok(latency)
}

fn main() {
    env_logger::init();

    info!("Starting Private VPN Client");

    // Check if WireGuard is installed
    if !WireGuardManager::is_wireguard_installed() {
        eprintln!("\n!!! WARNING !!!");
        eprintln!("WireGuard is not installed on this system.");
        eprintln!("Please install WireGuard before using this application.");
        eprintln!("\nInstallation instructions:");
        eprintln!("  - Windows: https://www.wireguard.com/install/");
        eprintln!("  - macOS: brew install wireguard-tools");
        eprintln!("  - Linux: sudo apt install wireguard (Debian/Ubuntu)");
        eprintln!("           sudo dnf install wireguard-tools (Fedora)");
        eprintln!("");
    }

    // Create system tray
    let tray_menu = SystemTrayMenu::new()
        .add_item(SystemTrayMenuItem::new("show", "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(SystemTrayMenuItem::new("quit", "Quit"));

    let system_tray = SystemTray::new().with_menu(tray_menu);

    // Build Tauri app
    tauri::Builder::default()
        .manage(AppState {
            api_key: Mutex::new(None),
            current_server: Mutex::new(None),
            available_servers: Mutex::new(Vec::new()),
            current_client_name: Mutex::new(None),
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            save_api_key,
            fetch_servers,
            connect_vpn,
            disconnect_vpn,
            get_connection_status,
            check_wireguard_installed,
            test_server_latency,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
