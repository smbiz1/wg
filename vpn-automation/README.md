# VPN Automation System

A complete private VPN solution with automatic server selection, user-friendly desktop client, and tier-based access control. Built on top of WireGuard with minimal additions to preserve all existing WireGuard logic.

## Overview

This system provides:

- **Desktop VPN Client**: Fast, lightweight Tauri app (5-10MB) with simple GUI
- **Provisioning API**: Backend for client configuration management
- **15 Global Servers**: Worldwide coverage with tier-based access
- **Auto Server Selection**: Connects to fastest server based on latency
- **Secure Key Management**: Proper WireGuard key handling using existing scripts

## Quick Start

### For Users

1. Download the VPN client for your platform from your dashboard
2. Install the application
3. Get your API key from the web portal
4. Paste the API key into the desktop app
5. Click "Connect"!

See detailed instructions: [Client README](client/README.md)

### For Developers

```bash
# Clone repository
git clone https://github.com/yourusername/wg.git
cd wg/vpn-automation

# Set up server API
cd server
pip3 install -r requirements.txt
python3 provision_api.py

# Build desktop client
cd ../client
npm install
npm run tauri:dev
```

See complete setup guide: [docs/SETUP.md](docs/SETUP.md)

## Architecture

```
User's Device (Desktop Client)
        ↓ HTTPS
Provisioning API
        ↓ SSH/Local
WireGuard Servers (15 locations)
        ↓ VPN Tunnel
Your Application (AI Models, GPU Instances, etc.)
```

See detailed architecture: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

## Components

### 1. Desktop Client ([client/](client/))

**Technology**: Tauri + Solid.js + Rust

**Features**:
- Simple, non-technical user interface
- Automatic server selection by latency
- Manual location selection
- System tray integration
- Cross-platform (Windows, macOS, Linux)

**Size**: ~5-10MB (vs Electron's 50-150MB)

### 2. Provisioning API ([server/](server/))

**Technology**: Python + Flask

**Features**:
- API key-based authentication
- Tier-based server access (Basic: 3 servers, Pro: 15 servers)
- WireGuard configuration generation
- Client lifecycle management
- Integrates with existing wireguard-install.sh

### 3. VPN Servers

**Technology**: WireGuard (via wireguard-install.sh)

**Locations**: 15 global servers
- North America: New York, Los Angeles, Toronto
- Europe: London, Frankfurt, Stockholm, Milan
- Asia Pacific: Singapore, Tokyo, Mumbai, Sydney, Hong Kong
- South America: São Paulo
- Middle East: Dubai
- Africa: Cape Town

## Subscription Tiers

### Basic Tier
- Access to 3 servers (US East, US West, London)
- Suitable for basic VPN needs
- Full WireGuard encryption

### Pro Tier
- Access to all 15 servers worldwide
- Global coverage
- Best for international users

## Key Features

### Security
- ✅ WireGuard protocol (modern, fast, secure)
- ✅ Unique keys per client
- ✅ Pre-shared keys for quantum resistance
- ✅ Proper file permissions (600)
- ✅ HMAC-SHA256 API key signing
- ✅ No key exposure in logs

### Performance
- ✅ Automatic latency testing
- ✅ Connects to fastest server
- ✅ Optimized network settings (BBR congestion control)
- ✅ Lightweight client (low CPU/memory usage)

### User Experience
- ✅ One-click connect
- ✅ No terminal commands required
- ✅ Simple GUI for non-technical users
- ✅ Status indicators
- ✅ Easy server switching

### Integration
- ✅ API key from your existing portal
- ✅ Subscription tier enforcement
- ✅ End-to-end encryption to your infrastructure
- ✅ Minimal changes to existing WireGuard setup

## Repository Structure

```
wg/
├── wireguard-install.sh        # Original WireGuard install script (unchanged)
├── README.md                   # Original README (unchanged)
│
└── vpn-automation/             # ← New additions (this directory)
    │
    ├── server/                 # Provisioning API
    │   ├── provision_api.py    # Flask API
    │   ├── servers.json        # Server configuration
    │   ├── requirements.txt    # Python dependencies
    │   └── README.md           # API documentation
    │
    ├── client/                 # Desktop application
    │   ├── src/                # Frontend (Solid.js)
    │   ├── src-tauri/          # Backend (Rust)
    │   ├── package.json
    │   └── README.md           # Client documentation
    │
    ├── docs/                   # Documentation
    │   ├── SETUP.md            # Complete setup guide
    │   └── ARCHITECTURE.md     # System architecture
    │
    └── README.md               # This file
```

## Design Philosophy

### Minimal Changes to WireGuard

All existing WireGuard logic is **preserved**:
- ✅ No modifications to wireguard-install.sh
- ✅ Uses existing key generation (wg genkey, wg genpsk)
- ✅ Uses existing configuration format
- ✅ Uses existing client management (--addclient, --removeclient)
- ✅ Same security model and best practices

### Additive Architecture

New components are **added alongside** existing code:
- Provisioning API calls wireguard-install.sh
- Desktop client uses standard WireGuard commands
- Server configs follow existing patterns
- No breaking changes to original functionality

## Getting Started

### Prerequisites

- **WireGuard**: Must be installed on servers and client machines
- **Python 3.8+**: For provisioning API
- **Node.js 18+**: For building desktop client
- **Rust 1.70+**: For building desktop client

### Installation

1. **Set up VPN servers** (15 servers):
   ```bash
   # On each server
   sudo bash wireguard-install.sh --auto \
     --serveraddr server.vpn.example.com \
     --port 51820
   ```

2. **Deploy Provisioning API**:
   ```bash
   cd vpn-automation/server
   pip3 install -r requirements.txt
   python3 provision_api.py
   ```

3. **Build Desktop Client**:
   ```bash
   cd vpn-automation/client
   npm install
   npm run tauri:build
   ```

4. **Integrate with your portal**:
   - Generate API keys in your web application
   - Display download links for desktop client
   - Users paste API key into desktop app

See [docs/SETUP.md](docs/SETUP.md) for complete step-by-step instructions.

## Usage

### For End Users

1. Log into your dashboard
2. Navigate to VPN section
3. Download desktop client for your platform
4. Copy your API key
5. Install and launch the desktop app
6. Paste API key
7. Click "Connect" (app auto-selects fastest server)
8. Browse securely through VPN!

### For Administrators

**Add VPN server**:
1. Provision new VPS
2. Install WireGuard using wireguard-install.sh
3. Add entry to servers.json
4. Deploy/restart API

**Manage users**:
- Generate API keys via your portal
- API enforces tier-based access
- Revoke access by regenerating API key

**Monitor**:
- Check API logs: `journalctl -u vpn-api`
- Check server status: `sudo wg show`
- Monitor connections and bandwidth

## Documentation

- **[Complete Setup Guide](docs/SETUP.md)**: Step-by-step installation
- **[Architecture](docs/ARCHITECTURE.md)**: System design and data flow
- **[Server API](server/README.md)**: Provisioning API documentation
- **[Desktop Client](client/README.md)**: Client app documentation

## Integration Example

### Generate API Key (in your web portal)

```python
import hmac
import hashlib

def generate_vpn_api_key(user_id, tier, secret_key):
    signature = hmac.new(
        secret_key.encode(),
        f"{user_id}:{tier}".encode(),
        hashlib.sha256
    ).hexdigest()[:16]

    return f"{user_id}_{tier}_{signature}"

# Usage
api_key = generate_vpn_api_key("user123", "pro", "your-secret-key")
# Returns: user123_pro_a1b2c3d4e5f6g7h8
```

### Display to User (in your dashboard)

```html
<div class="vpn-section">
  <h2>Private VPN Access</h2>

  <div class="api-key">
    <label>Your API Key:</label>
    <code>{{ api_key }}</code>
    <button onclick="copyToClipboard()">Copy</button>
  </div>

  <div class="downloads">
    <a href="/downloads/vpn-windows.exe">Download for Windows</a>
    <a href="/downloads/vpn-macos.dmg">Download for macOS</a>
    <a href="/downloads/vpn-linux.deb">Download for Linux</a>
  </div>
</div>
```

## Use Cases

### Primary Use Case: Secure Access to Your Infrastructure

Users connect to VPN → Traffic routed through your servers → End-to-end encrypted connection to your AI models/GPU instances

**Benefits**:
- User's ISP can't see traffic
- Bypass geographic restrictions
- Additional security layer
- Consistent IP for whitelisting

### Additional Use Cases

- General privacy/security for users
- Access geo-restricted content
- Secure connections on public WiFi
- Corporate VPN replacement
- Testing from different locations

## Performance Benchmarks

| Metric | Value |
|--------|-------|
| Client binary size | 5-10 MB |
| Memory usage (idle) | 50-100 MB |
| Connection time | 2-5 seconds |
| Startup time | <1 second |
| CPU usage (idle) | <1% |
| Throughput | Limited by server bandwidth |

## Security Considerations

### Implemented Security Features

- ✅ Modern cryptography (WireGuard)
- ✅ Unique keys per client
- ✅ Pre-shared keys (post-quantum)
- ✅ Proper file permissions
- ✅ HTTPS for API communication
- ✅ API key signing (HMAC-SHA256)
- ✅ No plaintext key storage outside configs

### Recommended Production Hardening

- [ ] Enable HTTPS with valid certificate
- [ ] Set up rate limiting on API
- [ ] Implement API key rotation
- [ ] Add audit logging
- [ ] Set up monitoring and alerting
- [ ] Regular security updates
- [ ] Fail2ban on all servers
- [ ] DDoS protection

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for full security analysis.

## Troubleshooting

### Common Issues

**"WireGuard not installed"**
- Solution: Install WireGuard for your platform

**"Invalid API key"**
- Solution: Generate new key from dashboard

**"Failed to connect"**
- Solution: Check WireGuard is running, firewall allows UDP 51820

**High latency**
- Solution: Use auto-select or choose closer server

See [docs/SETUP.md](docs/SETUP.md#troubleshooting) for detailed troubleshooting.

## Contributing

This is a private fork for your organization. To contribute:

1. Create feature branch
2. Make changes
3. Test thoroughly
4. Submit pull request

## Roadmap

### Phase 1: MVP (Current)
- ✅ Desktop client (Windows, macOS, Linux)
- ✅ Provisioning API
- ✅ 15 global servers
- ✅ Auto server selection
- ✅ Tier-based access

### Phase 2: Enhancements
- [ ] Usage tracking and billing integration
- [ ] Kill switch (block traffic if VPN disconnects)
- [ ] Split tunneling
- [ ] Connection logs and statistics
- [ ] Automatic reconnection

### Phase 3: Mobile
- [ ] iOS app
- [ ] Android app
- [ ] Shared backend with desktop

### Phase 4: Advanced
- [ ] Multi-hop connections
- [ ] WireGuard config rotation
- [ ] Dynamic server addition/removal
- [ ] Load balancing across servers

## License

Same license as the base WireGuard installation script (see `../LICENSE.txt`).

## Support

For issues and questions:

- **Setup/Deployment**: See [docs/SETUP.md](docs/SETUP.md)
- **Architecture**: See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- **API**: See [server/README.md](server/README.md)
- **Client**: See [client/README.md](client/README.md)
- **WireGuard**: See main repository README at `../README.md`

## Acknowledgments

- **WireGuard**: https://www.wireguard.com/
- **wireguard-install**: Original installation script
- **Tauri**: https://tauri.app/
- **Solid.js**: https://www.solidjs.com/

---

**Note**: This system is designed to work alongside the existing WireGuard installation script without modifying it. All additions are in the `vpn-automation/` directory, preserving the original functionality and security model of WireGuard.
