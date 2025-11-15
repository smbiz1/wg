# VPN Automation System - Complete Setup Guide

This guide walks you through setting up the entire VPN automation system from scratch.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Server Setup](#server-setup)
3. [Provisioning API Setup](#provisioning-api-setup)
4. [Desktop Client Build](#desktop-client-build)
5. [Integration with Your Portal](#integration-with-your-portal)
6. [Testing](#testing)
7. [Production Deployment](#production-deployment)

## Prerequisites

### Infrastructure

- **15 VPS/Cloud Servers** for VPN endpoints
  - Recommended: 1 CPU, 1GB RAM, 20GB disk, 1 Gbps network
  - Supported OS: Ubuntu 20.04+, Debian 11+, Rocky Linux 8+, Fedora 35+
  - Public IPv4 address for each
  - Optional: IPv6 support

- **1 API Server** for provisioning API
  - Recommended: 2 CPU, 2GB RAM, 20GB disk
  - Same OS options as VPN servers
  - Public IP with HTTPS certificate

- **Domain Names**:
  - API: `api.vpn.example.com`
  - VPN servers: `nyc.vpn.example.com`, `la.vpn.example.com`, etc.

### Developer Tools

- **Linux/macOS**:
  - Git
  - Python 3.8+
  - Node.js 18+
  - Rust 1.70+ (for building client)

- **Windows** (for building Windows client):
  - Git
  - Node.js 18+
  - Rust (via rustup)
  - Visual Studio Build Tools

## Server Setup

### Step 1: Provision VPS Servers

Provision 15 VPS instances in desired locations. For example:

| Provider | Location | Server ID |
|----------|----------|-----------|
| DigitalOcean | NYC | us-east-1 |
| AWS | California | us-west-1 |
| Linode | London | eu-west-1 |
| Hetzner | Frankfurt | eu-central-1 |
| ... | ... | ... |

### Step 2: Configure DNS

Point domain names to server IPs:

```bash
nyc.vpn.example.com     → 1.2.3.4
la.vpn.example.com      → 5.6.7.8
london.vpn.example.com  → 9.10.11.12
# ... etc for all 15 servers
```

### Step 3: Install WireGuard on Each Server

For each of the 15 VPN servers:

```bash
# SSH into server
ssh root@nyc.vpn.example.com

# Clone this repository
git clone https://github.com/yourusername/wg.git
cd wg

# Run WireGuard installation with auto mode
sudo bash wireguard-install.sh --auto \
  --serveraddr nyc.vpn.example.com \
  --port 51820 \
  --clientname initial_client \
  --dns1 8.8.8.8 \
  --dns2 8.8.4.4

# Verify installation
sudo systemctl status wg-quick@wg0
sudo wg show
```

You should see output like:
```
interface: wg0
  public key: <SERVER_PUBLIC_KEY>
  private key: (hidden)
  listening port: 51820
```

**Repeat for all 15 servers**, changing `--serveraddr` to match each server's domain.

### Step 4: Configure Firewall

Ensure UDP port 51820 is open:

```bash
# UFW (Ubuntu/Debian)
sudo ufw allow 51820/udp
sudo ufw status

# firewalld (CentOS/Fedora/Rocky)
sudo firewall-cmd --permanent --add-port=51820/udp
sudo firewall-cmd --reload
```

### Step 5: Test WireGuard

```bash
# Check status
sudo wg show wg0

# You should see 1 peer (the initial_client)
# If no peers, something went wrong
```

## Provisioning API Setup

### Step 1: Prepare API Server

```bash
# SSH into API server
ssh root@api-server.example.com

# Clone repository
git clone https://github.com/yourusername/wg.git
cd wg/vpn-automation/server
```

### Step 2: Update Server Configuration

Edit `servers.json` and replace example endpoints with your actual domains:

```bash
nano servers.json
```

Update each server's `endpoint`:

```json
{
  "id": "us-east-1",
  "name": "New York, USA",
  "endpoint": "nyc.vpn.example.com",  ← Change this
  ...
}
```

Save and exit.

### Step 3: Install Python Dependencies

```bash
# Install Python 3 and pip if not present
sudo apt update
sudo apt install -y python3 python3-pip

# Install dependencies
pip3 install -r requirements.txt
```

### Step 4: Configure Environment Variables

Create `.env` file:

```bash
nano .env
```

Add:

```bash
# IMPORTANT: Change this to a strong random value!
VPN_API_SECRET_KEY=your-very-secure-random-secret-key-here

# Flask configuration
FLASK_ENV=production
FLASK_DEBUG=False
```

Generate a strong secret key:

```bash
python3 -c "import secrets; print(secrets.token_urlsafe(32))"
```

### Step 5: Configure HTTPS (Required for Production)

Install nginx and certbot:

```bash
sudo apt install -y nginx certbot python3-certbot-nginx
```

Create nginx config:

```bash
sudo nano /etc/nginx/sites-available/vpn-api
```

Add:

```nginx
server {
    listen 80;
    server_name api.vpn.example.com;

    location / {
        proxy_pass http://127.0.0.1:5000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

Enable site and get SSL certificate:

```bash
sudo ln -s /etc/nginx/sites-available/vpn-api /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx

# Get SSL certificate (interactive)
sudo certbot --nginx -d api.vpn.example.com
```

### Step 6: Create Systemd Service

```bash
sudo nano /etc/systemd/system/vpn-api.service
```

Add:

```ini
[Unit]
Description=VPN Provisioning API
After=network.target

[Service]
Type=notify
User=root
WorkingDirectory=/root/wg/vpn-automation/server
Environment="VPN_API_SECRET_KEY=your-secret-key-here"
ExecStart=/usr/local/bin/gunicorn -w 4 -b 127.0.0.1:5000 provision_api:app
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Replace `your-secret-key-here` with your actual secret key.

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable vpn-api
sudo systemctl start vpn-api
sudo systemctl status vpn-api
```

### Step 7: Test API

```bash
# Health check
curl https://api.vpn.example.com/api/health

# Expected output:
# {"status":"healthy","timestamp":"2024-..."}

# Generate test API key
curl -X POST https://api.vpn.example.com/api/generate-api-key \
  -H "Content-Type: application/json" \
  -d '{"user_id": "testuser", "tier": "pro"}'

# Save the returned API key for testing

# Test server list
curl https://api.vpn.example.com/api/servers \
  -H "X-API-Key: testuser_pro_<signature>"

# Should return list of 15 servers
```

### Step 8: Disable Test Endpoint (Production)

Once testing is complete, edit `provision_api.py`:

```python
# Comment out or remove the /api/generate-api-key endpoint
# @app.route('/api/generate-api-key', methods=['POST'])
# def generate_api_key():
#     ...
```

Restart service:

```bash
sudo systemctl restart vpn-api
```

## Desktop Client Build

### Step 1: Install Build Tools

**Linux/macOS:**

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Node.js (if not already installed)
# macOS:
brew install node

# Linux:
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
```

**Windows:**

1. Install Rust: https://rustup.rs/
2. Install Node.js: https://nodejs.org/
3. Install Visual Studio Build Tools: https://visualstudio.microsoft.com/downloads/

### Step 2: Update API URL

Edit `vpn-automation/client/src-tauri/src/api_client.rs`:

```rust
const API_BASE_URL: &str = "https://api.vpn.example.com"; // ← Change this
```

### Step 3: Update Dashboard URL

Edit `vpn-automation/client/src/components/ApiKeyInput.tsx`:

```typescript
window.open('https://dashboard.example.com/vpn', '_blank'); // ← Change this
```

### Step 4: Build Client

```bash
cd vpn-automation/client

# Install dependencies
npm install

# Build for your platform
npm run tauri:build
```

**Output locations:**

- **Windows**: `src-tauri/target/release/bundle/msi/Private-VPN.msi`
- **macOS**: `src-tauri/target/release/bundle/dmg/Private-VPN.dmg`
- **Linux**: `src-tauri/target/release/bundle/deb/private-vpn.deb`

### Step 5: Code Signing (Optional but Recommended)

**Windows:**

```bash
# Get code signing certificate
# Then update tauri.conf.json:
{
  "tauri": {
    "bundle": {
      "windows": {
        "certificateThumbprint": "YOUR_CERT_THUMBPRINT"
      }
    }
  }
}
```

**macOS:**

```bash
# Get Apple Developer certificate
# Then update tauri.conf.json:
{
  "tauri": {
    "bundle": {
      "macOS": {
        "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
      }
    }
  }
}
```

**Linux:**

No code signing required, but consider GPG signing .deb packages.

### Step 6: Distribute Client

Upload built binaries to your website for users to download:

```
https://example.com/downloads/vpn/windows/Private-VPN-Setup.exe
https://example.com/downloads/vpn/macos/Private-VPN.dmg
https://example.com/downloads/vpn/linux/private-vpn.deb
```

## Integration with Your Portal

### Step 1: Add API Key Generation to Your Portal

```python
# In your web application (Django/Flask/FastAPI/etc.)

import hmac
import hashlib
import os

# Use the SAME secret key as the provisioning API
VPN_API_SECRET_KEY = os.environ.get('VPN_API_SECRET_KEY')

def generate_vpn_api_key(user_id, tier):
    """Generate VPN API key for a user."""
    signature = hmac.new(
        VPN_API_SECRET_KEY.encode(),
        f"{user_id}:{tier}".encode(),
        hashlib.sha256
    ).hexdigest()[:16]

    return f"{user_id}_{tier}_{signature}"

# In your view/endpoint:
@app.route('/dashboard/vpn')
@login_required
def vpn_dashboard():
    user = get_current_user()

    # Get user's subscription tier
    tier = 'pro' if user.has_pro_subscription else 'basic'

    # Generate or retrieve existing API key
    if not user.vpn_api_key:
        user.vpn_api_key = generate_vpn_api_key(user.id, tier)
        user.save()

    return render_template('vpn_dashboard.html', {
        'api_key': user.vpn_api_key,
        'tier': tier,
        'download_links': {
            'windows': 'https://example.com/downloads/vpn/windows/...',
            'macos': 'https://example.com/downloads/vpn/macos/...',
            'linux': 'https://example.com/downloads/vpn/linux/...',
        }
    })
```

### Step 2: Create VPN Dashboard UI

```html
<!-- vpn_dashboard.html -->
<div class="vpn-section">
  <h2>Private VPN Access</h2>

  <div class="download-section">
    <h3>1. Download the VPN Client</h3>
    <div class="download-buttons">
      <a href="{{ download_links.windows }}" class="btn">
        Download for Windows
      </a>
      <a href="{{ download_links.macos }}" class="btn">
        Download for macOS
      </a>
      <a href="{{ download_links.linux }}" class="btn">
        Download for Linux
      </a>
    </div>
  </div>

  <div class="api-key-section">
    <h3>2. Your VPN API Key</h3>
    <p>Copy this key and paste it into the VPN application:</p>
    <div class="api-key-display">
      <code>{{ api_key }}</code>
      <button onclick="copyApiKey()">Copy</button>
    </div>
    <p class="warning">
      Keep this key private. Anyone with this key can use your VPN quota.
    </p>
  </div>

  <div class="tier-info">
    <h3>Your Plan: {{ tier|capitalize }}</h3>
    <p>
      {% if tier == 'basic' %}
        You have access to 3 VPN servers in major regions.
        <a href="/upgrade">Upgrade to Pro</a> for 15 servers worldwide.
      {% else %}
        You have access to all 15 VPN servers worldwide.
      {% endif %}
    </p>
  </div>
</div>

<script>
function copyApiKey() {
  const apiKey = "{{ api_key }}";
  navigator.clipboard.writeText(apiKey);
  alert('API key copied to clipboard!');
}
</script>
```

### Step 3: Handle Subscription Changes

```python
# When user upgrades/downgrades subscription

@app.route('/subscription/changed', methods=['POST'])
def subscription_changed():
    user = get_current_user()

    # Get new tier
    new_tier = 'pro' if user.has_pro_subscription else 'basic'

    # Regenerate API key with new tier
    user.vpn_api_key = generate_vpn_api_key(user.id, new_tier)
    user.save()

    # Notify user to update desktop app
    send_email(user.email,
        subject="VPN subscription updated",
        body="Your VPN access has changed. Please update your API key in the desktop app."
    )

    return {'success': True}
```

## Testing

### End-to-End Test

1. **Generate API Key**:
   ```bash
   curl -X POST https://api.vpn.example.com/api/generate-api-key \
     -H "Content-Type: application/json" \
     -d '{"user_id": "testuser", "tier": "pro"}'
   ```

2. **Install Desktop Client**:
   - Install built client on test machine
   - Launch application
   - Paste API key

3. **Fetch Servers**:
   - Should see list of 15 servers
   - Each should show latency

4. **Connect to VPN**:
   - Click "Connect" button
   - Should connect to fastest server
   - Check connection status

5. **Verify VPN Connection**:
   ```bash
   # Check your IP (should be VPN server IP)
   curl https://api.ipify.org

   # Check WireGuard status
   sudo wg show
   ```

6. **Test Traffic Routing**:
   ```bash
   # All traffic should route through VPN
   traceroute google.com
   # First hop should be VPN server
   ```

7. **Disconnect**:
   - Click "Disconnect" button
   - Verify IP returns to normal

### Load Testing

```bash
# Test API performance
ab -n 1000 -c 10 https://api.vpn.example.com/api/health

# Test concurrent connections
# (requires multiple test accounts)
```

## Production Deployment

### Security Checklist

- [ ] Change all default passwords
- [ ] Set strong `VPN_API_SECRET_KEY`
- [ ] Enable HTTPS for API (Let's Encrypt)
- [ ] Remove `/api/generate-api-key` endpoint
- [ ] Configure firewall rules on all servers
- [ ] Set up monitoring and alerting
- [ ] Enable fail2ban on all servers
- [ ] Regular security updates
- [ ] Implement rate limiting on API
- [ ] Set up backup strategy

### Monitoring Setup

```bash
# Install monitoring on each server
# Example with Prometheus + Node Exporter

# On each VPN server:
wget https://github.com/prometheus/node_exporter/releases/download/v1.6.1/node_exporter-1.6.1.linux-amd64.tar.gz
tar xvfz node_exporter-*.tar.gz
sudo cp node_exporter-*/node_exporter /usr/local/bin/
sudo chmod +x /usr/local/bin/node_exporter

# Create systemd service
sudo nano /etc/systemd/system/node_exporter.service

# Add:
[Unit]
Description=Node Exporter
After=network.target

[Service]
User=node_exporter
Group=node_exporter
Type=simple
ExecStart=/usr/local/bin/node_exporter

[Install]
WantedBy=multi-user.target

# Start service
sudo systemctl daemon-reload
sudo systemctl enable node_exporter
sudo systemctl start node_exporter
```

### Backup Strategy

```bash
# Backup WireGuard configs (daily)
#!/bin/bash
tar -czf /backup/wg-config-$(date +%Y%m%d).tar.gz /etc/wireguard/
```

### Update Strategy

```bash
# Update WireGuard (monthly)
sudo apt update
sudo apt upgrade wireguard wireguard-tools

# Restart WireGuard
sudo systemctl restart wg-quick@wg0
```

## Troubleshooting

### API Issues

**Problem**: API returns 500 error

**Solution**:
```bash
# Check logs
sudo journalctl -u vpn-api -f

# Common issues:
# - Wrong VPN_API_SECRET_KEY
# - Can't access wireguard-install.sh
# - Permissions issue
```

### VPN Server Issues

**Problem**: Can't connect to VPN server

**Solution**:
```bash
# Check WireGuard status
sudo systemctl status wg-quick@wg0

# Check firewall
sudo ufw status
# Ensure port 51820/udp is open

# Check logs
sudo journalctl -u wg-quick@wg0

# Test connectivity
nc -zvu <server-ip> 51820
```

### Client Issues

**Problem**: Desktop app says "WireGuard not installed"

**Solution**:
- Install WireGuard for your platform
- Ensure `wg` command is in PATH

**Problem**: Connection fails after clicking "Connect"

**Solution**:
- Check logs in app (if logging implemented)
- Verify API key is valid
- Ensure WireGuard has permissions (may need sudo/admin)

## Next Steps

1. **Monitor Usage**: Track connections and bandwidth
2. **Optimize**: Tune server settings based on usage
3. **Scale**: Add more servers as user base grows
4. **Features**: Add usage tracking, kill switch, etc.
5. **Mobile**: Build iOS/Android apps

## Support

For issues:
- API/Server: Check logs, review configuration
- Desktop Client: Check WireGuard installation
- Integration: Verify API key generation logic

Refer to:
- `vpn-automation/server/README.md` - API documentation
- `vpn-automation/client/README.md` - Client documentation
- `docs/ARCHITECTURE.md` - System architecture
