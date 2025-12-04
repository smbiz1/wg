# Private VPN Desktop Client

A fast, user-friendly desktop VPN client built with Tauri and Solid.js. Provides secure WireGuard VPN connections with automatic server selection and simple management.

## Features

- **Simple UI**: Clean, intuitive interface designed for non-technical users
- **Auto Server Selection**: Automatically connects to the fastest server based on latency
- **Manual Location Choice**: Choose from available servers based on subscription tier
- **Fast & Lightweight**: Built with Tauri (5-10MB binary vs 50-150MB with Electron)
- **Cross-Platform**: Supports Windows, macOS, and Linux
- **Secure**: Uses WireGuard protocol with proper key management
- **System Tray**: Minimizes to system tray for easy access

## Prerequisites

### For Users

- **WireGuard** must be installed:
  - **Windows**: Download from https://www.wireguard.com/install/
  - **macOS**: `brew install wireguard-tools`
  - **Linux**:
    - Ubuntu/Debian: `sudo apt install wireguard`
    - Fedora: `sudo dnf install wireguard-tools`

### For Developers

- **Rust** 1.70+ (https://rustup.rs/)
- **Node.js** 18+ (https://nodejs.org/)
- **WireGuard** (see above)

## Installation (Users)

1. Download the latest release for your platform:
   - Windows: `Private-VPN-Setup.exe`
   - macOS: `Private-VPN.dmg`
   - Linux: `private-vpn.AppImage` or `.deb`

2. Install the application

3. Launch the app and enter your API key from the dashboard

4. Connect to VPN!

## Development Setup

### 1. Install Dependencies

```bash
cd vpn-automation/client

# Install Node dependencies
npm install

# Rust dependencies are managed by Cargo (auto-installed)
```

### 2. Configure API Endpoint

Edit `src-tauri/src/api_client.rs` and update the API URL:

```rust
const API_BASE_URL: &str = "https://api.vpn.example.com"; // Replace with your actual API URL
```

### 3. Run Development Build

```bash
npm run tauri:dev
```

This will:
- Start the Vite dev server for the frontend
- Compile the Rust backend
- Launch the application in development mode with hot reload

### 4. Build for Production

```bash
npm run tauri:build
```

This creates optimized binaries in `src-tauri/target/release/bundle/`:
- Windows: `.exe` installer and `.msi`
- macOS: `.dmg` and `.app`
- Linux: `.deb`, `.AppImage`, `.rpm`

## Project Structure

```
client/
├── src/                    # Frontend (Solid.js)
│   ├── components/         # UI components
│   │   ├── ApiKeyInput.tsx
│   │   ├── ConnectionToggle.tsx
│   │   └── ServerSelector.tsx
│   ├── App.tsx            # Main app component
│   ├── App.css            # Styling
│   └── main.tsx           # Entry point
│
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Tauri app & commands
│   │   ├── wg_manager.rs  # WireGuard management
│   │   ├── network.rs     # Latency testing
│   │   └── api_client.rs  # API communication
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
│
├── package.json           # Node dependencies
├── vite.config.ts         # Vite configuration
└── tsconfig.json          # TypeScript configuration
```

## How It Works

### 1. Authentication Flow

```
User → Login to Web Portal → Generate API Key → Paste into Desktop App
```

The desktop app validates the API key with the provisioning API and fetches available servers based on subscription tier.

### 2. Connection Flow

```
Select Server → Provision Config from API → Save WireGuard Config → Connect
```

1. User selects server (or auto-selects fastest)
2. App calls provisioning API to generate WireGuard config
3. Config is saved to system WireGuard directory
4. App executes platform-specific WireGuard commands to connect

### 3. Platform-Specific Implementation

**Linux/macOS:**
- Uses `wg-quick up <interface>` to connect
- Uses `wg-quick down <interface>` to disconnect
- Uses `wg show <interface>` to check status

**Windows:**
- Uses WireGuard service via `wireguard.exe`
- Or falls back to `wg-quick` if available

## Configuration Files

### WireGuard Configs Location

- **Windows**: `C:\Program Files\WireGuard\`
- **macOS**: `/usr/local/etc/wireguard/`
- **Linux**: `/etc/wireguard/`

Configs are named: `privatevpn_<server_id>.conf`

### App Configuration

The app stores minimal state in memory:
- API key
- Current server connection
- Available servers list

No sensitive data is persisted to disk except WireGuard configs (which are properly secured with 600 permissions on Unix).

## User Guide

### First Time Setup

1. **Get API Key**:
   - Log into your dashboard at https://dashboard.example.com
   - Navigate to VPN section
   - Click "Generate API Key"
   - Copy the generated key

2. **Enter API Key**:
   - Open the Private VPN app
   - Paste your API key
   - Click "Continue"

3. **Connect**:
   - App will automatically select the fastest server
   - Click "Connect" button
   - Wait for connection to establish (usually 2-5 seconds)

### Changing Servers

1. Disconnect from current server
2. Select a different server from the list
3. Click "Connect"

### Managing Connection

- **System Tray**: Click tray icon to show/hide window
- **Auto-Start**: Enable in system settings (not built into app)
- **Status**: Green indicator = Connected, Red = Disconnected

## Troubleshooting

### "WireGuard Not Installed"

Install WireGuard for your platform (see Prerequisites above).

### "Failed to Connect"

1. Check WireGuard is installed and in PATH
2. On Linux/macOS, ensure you have sudo privileges
3. Check firewall isn't blocking UDP port 51820
4. Try selecting a different server

### "Invalid API Key"

1. Verify you copied the entire key
2. Check key hasn't expired in dashboard
3. Ensure your subscription is active

### Connection Drops Frequently

1. Try a different server (may have better routing)
2. Check your internet connection stability
3. Disable battery saver (can interfere with VPN)

### High Latency

1. Use auto-select to pick fastest server
2. Choose server geographically closer to you
3. Check your base internet connection speed

## Development Notes

### Adding New Features

**New Tauri Command:**

1. Add function to `src-tauri/src/main.rs`:
```rust
#[tauri::command]
fn my_command() -> Result<String, String> {
    Ok("Hello".to_string())
}
```

2. Register in `invoke_handler!` macro

3. Call from frontend:
```typescript
import { invoke } from '@tauri-apps/api/tauri';
const result = await invoke<string>('my_command');
```

**New UI Component:**

1. Create in `src/components/MyComponent.tsx`
2. Import and use in `App.tsx`

### Testing

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run frontend in browser (for UI testing)
npm run dev
```

### Code Style

- **Rust**: Follow rustfmt conventions
- **TypeScript**: Use Prettier (auto-format on save)
- **Components**: Functional components with Solid.js primitives

## Security Considerations

### Key Management

- API keys stored in memory only (not persisted)
- WireGuard keys managed by provisioning API (never exposed to client)
- Client configs stored with 600 permissions (Unix)

### Network Security

- All API calls over HTTPS (TLS 1.3)
- WireGuard provides transport encryption
- Pre-shared keys for quantum resistance

### Application Security

- Tauri security features enabled
- No eval() or unsafe JavaScript
- CSP headers configured
- Limited API surface area

## Building for Distribution

### Windows

```bash
npm run tauri:build
```

Produces:
- `Private-VPN.msi` - Windows Installer
- `Private-VPN.exe` - Portable executable

**Code Signing**: Add certificate in `tauri.conf.json`:
```json
"windows": {
  "certificateThumbprint": "YOUR_CERT_THUMBPRINT"
}
```

### macOS

```bash
npm run tauri:build
```

Produces:
- `Private-VPN.dmg` - Disk image
- `Private-VPN.app` - Application bundle

**Code Signing**: Configure in `tauri.conf.json`:
```json
"macOS": {
  "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
}
```

### Linux

```bash
npm run tauri:build
```

Produces:
- `.deb` - Debian package
- `.AppImage` - Universal Linux package
- `.rpm` - Fedora/RedHat package

## Performance

- **Binary Size**: ~5-10 MB (vs Electron's 50-150 MB)
- **Memory Usage**: ~50-100 MB (vs Electron's 200-500 MB)
- **Startup Time**: <1 second
- **CPU Usage**: Minimal when idle

## License

See `../../LICENSE.txt`

## Support

For issues with:
- **Desktop App**: Create issue in repository
- **API/Servers**: Contact support team
- **WireGuard**: See https://www.wireguard.com/
