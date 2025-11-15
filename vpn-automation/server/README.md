# VPN Provisioning API Server

This API server handles WireGuard client provisioning for your VPN service. It integrates with the existing `wireguard-install.sh` script to generate client configurations while maintaining proper key management.

## Features

- API key-based authentication (integrates with your user portal)
- Tier-based server access (Basic: 3 servers, Pro: 15 servers)
- Automatic client configuration generation
- Client lifecycle management (provision/revoke)
- Secure key handling using existing WireGuard scripts

## Prerequisites

1. **WireGuard Server Setup**: Each VPN server must have WireGuard installed and configured
2. **Python 3.8+**: Required for running the API
3. **Root Access**: API must run with privileges to execute wireguard-install.sh

## Installation

### 1. Install Python Dependencies

```bash
cd vpn-automation/server
pip3 install -r requirements.txt
```

### 2. Configure Environment Variables

Create a `.env` file:

```bash
# Secret key for API key validation (CHANGE THIS!)
VPN_API_SECRET_KEY=your-secret-key-here-use-strong-random-value

# Flask configuration
FLASK_ENV=production
FLASK_DEBUG=False
```

### 3. Set Up WireGuard on Each Server

For each of the 15 servers in `servers.json`, run:

```bash
# Install WireGuard using the existing script
sudo bash ../../wireguard-install.sh --auto \
  --serveraddr nyc.vpn.example.com \
  --port 51820 \
  --clientname initial_client
```

Replace `nyc.vpn.example.com` with the actual endpoint from servers.json.

### 4. Update servers.json

Edit `servers.json` and replace example endpoints with your actual server domains/IPs:

```json
{
  "id": "us-east-1",
  "endpoint": "nyc.vpn.example.com",  // Replace with your actual domain
  ...
}
```

## Running the API

### Development

```bash
python3 provision_api.py
```

The API will start on `http://0.0.0.0:5000`

### Production

Use gunicorn for production deployments:

```bash
gunicorn -w 4 -b 0.0.0.0:5000 provision_api:app
```

Or with systemd service (recommended):

Create `/etc/systemd/system/vpn-api.service`:

```ini
[Unit]
Description=VPN Provisioning API
After=network.target

[Service]
Type=notify
User=root
WorkingDirectory=/path/to/vpn-automation/server
Environment="VPN_API_SECRET_KEY=your-secret-key"
ExecStart=/usr/bin/gunicorn -w 4 -b 0.0.0.0:5000 provision_api:app
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable vpn-api
sudo systemctl start vpn-api
```

## API Endpoints

### 1. Health Check

```bash
GET /api/health
```

Returns server status.

### 2. List Available Servers

```bash
GET /api/servers
Headers:
  X-API-Key: <user-api-key>
```

Returns servers available for the user's subscription tier.

### 3. Provision Client

```bash
POST /api/provision
Headers:
  X-API-Key: <user-api-key>
Body:
  {
    "server_id": "us-east-1"  // Optional
  }
```

Generates a WireGuard client configuration.

### 4. Revoke Client

```bash
POST /api/revoke
Headers:
  X-API-Key: <user-api-key>
Body:
  {
    "client_name": "userid_serverid_abc123"
  }
```

Revokes a client's access.

### 5. Generate API Key (Development Only)

```bash
POST /api/generate-api-key
Body:
  {
    "user_id": "user123",
    "tier": "basic"
  }
```

**⚠️ Remove this endpoint in production!** API keys should be generated through your main application.

## Integration with Your Application

### API Key Generation

The API expects keys in the format: `userid_tier_signature`

In your web portal, generate keys like this:

```python
import hmac
import hashlib

def generate_vpn_api_key(user_id, tier, secret_key):
    """Generate VPN API key for a user."""
    signature = hmac.new(
        secret_key.encode(),
        f"{user_id}:{tier}".encode(),
        hashlib.sha256
    ).hexdigest()[:16]

    return f"{user_id}_{tier}_{signature}"
```

### Authentication Flow

1. User logs into your web portal
2. Portal checks their subscription tier
3. Portal generates API key using `generate_vpn_api_key()`
4. User copies API key and pastes into desktop app
5. Desktop app uses API key for all VPN operations

### Validating API Keys

Replace the `validate_api_key()` function in `provision_api.py` with your actual authentication logic:

```python
def validate_api_key(api_key):
    # Query your database
    user = your_database.get_user_by_api_key(api_key)

    if user and user.subscription_active:
        return {
            'user_id': user.id,
            'tier': user.subscription_tier,
            'email': user.email,
            'expires_at': user.subscription_expires_at
        }

    return None
```

## Server Deployment Architecture

### Option 1: Centralized API (Recommended)

- Deploy API on one server
- API calls `wireguard-install.sh` on remote VPN servers via SSH
- Modify `generate_client_config()` to SSH into specified server

### Option 2: Distributed API

- Run API instance on each VPN server
- Load balancer routes requests to appropriate server
- Each API manages its own server's WireGuard configs

### Option 3: Hybrid

- Central API for user authentication
- Lightweight agents on each VPN server
- API sends commands to agents via message queue

## Security Considerations

### Critical Security Tasks

- [ ] Replace `API_SECRET_KEY` with strong random value
- [ ] Store secret in environment variables or secret manager (not in code)
- [ ] Remove `/api/generate-api-key` endpoint in production
- [ ] Set up HTTPS/TLS (use nginx reverse proxy)
- [ ] Implement rate limiting (use nginx or Flask-Limiter)
- [ ] Add request validation and input sanitization
- [ ] Set up monitoring and alerting
- [ ] Implement API key rotation policy
- [ ] Add audit logging for all operations
- [ ] Restrict API access by IP if possible

### Key Management Security

This API uses the existing `wireguard-install.sh` script for all key operations, which ensures:

- ✅ Keys generated using WireGuard's secure `wg genkey` and `wg genpsk`
- ✅ Proper file permissions (600) on all config files
- ✅ Pre-shared keys for quantum resistance
- ✅ Unique keys per client
- ✅ Secure key distribution (configs never logged)

### HTTPS Setup (Required for Production)

Use nginx as reverse proxy:

```nginx
server {
    listen 443 ssl http2;
    server_name api.vpn.example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location /api/ {
        proxy_pass http://127.0.0.1:5000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Monitoring

### Health Check

Set up monitoring to ping `/api/health` every 60 seconds:

```bash
curl https://api.vpn.example.com/api/health
```

### Logs

Check API logs:

```bash
# If using systemd
journalctl -u vpn-api -f

# If running manually
tail -f /var/log/vpn-api.log
```

## Troubleshooting

### "WireGuard install script not found"

Ensure the API can access `../../wireguard-install.sh` from the server directory.

### "Failed to create client"

Check that:
1. WireGuard is installed and running: `systemctl status wg-quick@wg0`
2. Script has execute permissions: `chmod +x ../../wireguard-install.sh`
3. API is running as root or with sudo

### "Client config file not found"

The script creates configs in various locations. Check:
- `/root/<client_name>.conf`
- `/home/<user>/<client_name>.conf`
- `/etc/wireguard/clients/<client_name>.conf`

## Testing

### Generate Test API Key

```bash
curl -X POST http://localhost:5000/api/generate-api-key \
  -H "Content-Type: application/json" \
  -d '{"user_id": "testuser", "tier": "pro"}'
```

### List Servers

```bash
curl -X GET http://localhost:5000/api/servers \
  -H "X-API-Key: testuser_pro_<signature>"
```

### Provision Client

```bash
curl -X POST http://localhost:5000/api/provision \
  -H "X-API-Key: testuser_pro_<signature>" \
  -H "Content-Type: application/json" \
  -d '{"server_id": "us-east-1"}'
```

## Support

For issues with:
- WireGuard installation: See `../../README.md`
- API integration: Check server logs
- Key management: Refer to `../../docs/clients.md`
