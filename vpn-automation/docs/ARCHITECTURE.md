# VPN Automation System Architecture

This document describes the complete architecture of the private VPN system, including server provisioning, client management, and desktop application.

## Overview

The system consists of three main components:

1. **VPN Servers**: WireGuard servers running on 15 global locations
2. **Provisioning API**: Backend API for client configuration management
3. **Desktop Client**: User-facing application for VPN connection

## System Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      User's Device                          │
│  ┌────────────────────────────────────────────────────┐    │
│  │            Desktop VPN Client (Tauri)              │    │
│  │  ┌──────────────┐  ┌──────────────────────────┐  │    │
│  │  │   Solid.js   │  │    Rust Backend          │  │    │
│  │  │   Frontend   │◄─┤  - WireGuard Manager     │  │    │
│  │  │     (UI)     │  │  - Network Latency Test  │  │    │
│  │  └──────────────┘  │  - API Client            │  │    │
│  │                     └──────────────────────────┘  │    │
│  └────────────────────────────────────────────────────┘    │
│                          │ HTTPS                            │
│                          ▼                                  │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                  Provisioning API Server                    │
│  ┌────────────────────────────────────────────────────┐    │
│  │               Flask API (Python)                    │    │
│  │  - API Key Validation                              │    │
│  │  - Server List Management                          │    │
│  │  - Client Configuration Generation                 │    │
│  │  - Integration with wireguard-install.sh          │    │
│  └────────────────────────────────────────────────────┘    │
│                          │                                  │
│                          ▼                                  │
│  ┌────────────────────────────────────────────────────┐    │
│  │          Your Authentication System                 │    │
│  │  - User Management                                  │    │
│  │  - Subscription Tiers                              │    │
│  │  - API Key Generation                              │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                    VPN Server Network                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │ US-East  │  │ US-West  │  │ London   │  │Frankfurt │  │
│  │(Basic)   │  │(Basic)   │  │(Basic)   │  │  (Pro)   │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
│                                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │Singapore │  │  Tokyo   │  │  Mumbai  │  │São Paulo │  │
│  │  (Pro)   │  │  (Pro)   │  │  (Pro)   │  │  (Pro)   │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
│                                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │ Toronto  │  │Stockholm │  │  Sydney  │  │  Dubai   │  │
│  │  (Pro)   │  │  (Pro)   │  │  (Pro)   │  │  (Pro)   │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
│                                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                │
│  │Cape Town │  │  Milan   │  │Hong Kong │                │
│  │  (Pro)   │  │  (Pro)   │  │  (Pro)   │                │
│  └──────────┘  └──────────┘  └──────────┘                │
│                                                             │
│  Each server runs:                                         │
│  - WireGuard (via wireguard-install.sh)                   │
│  - Ubuntu/Debian/Rocky Linux/Fedora                       │
│  - Optimized network settings                             │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   User's Application                        │
│  - AI Models                                                │
│  - GPU Instances                                            │
│  - End-to-End Encrypted Connection via VPN                 │
└─────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. Desktop Client (Tauri + Solid.js)

**Technology Stack:**
- **Frontend**: Solid.js (reactive UI)
- **Backend**: Rust (via Tauri)
- **Build**: Vite (fast bundling)
- **Size**: ~5-10MB binary

**Key Features:**
- API key authentication
- Server latency testing
- Automatic server selection
- Manual server override
- System tray integration
- Cross-platform (Windows, macOS, Linux)

**Architecture:**
```
Frontend (Solid.js)
  ├── App.tsx                    # Main component
  ├── ApiKeyInput                # Initial authentication
  ├── ConnectionToggle           # Connect/disconnect UI
  └── ServerSelector             # Location picker

Backend (Rust)
  ├── main.rs                    # Tauri commands & app logic
  ├── wg_manager.rs              # WireGuard operations
  │   ├── save_config()          # Save .conf to disk
  │   ├── connect()              # Start VPN connection
  │   ├── disconnect()           # Stop VPN connection
  │   └── get_status()           # Check connection state
  ├── network.rs                 # Latency testing
  │   ├── measure_latency()      # TCP connection timing
  │   ├── test_server_latencies()# Test all servers
  │   └── get_best_server()      # Find fastest
  └── api_client.rs              # API communication
      ├── get_servers()          # Fetch server list
      ├── provision()            # Get WireGuard config
      └── revoke()               # Revoke access
```

### 2. Provisioning API (Flask)

**Technology Stack:**
- **Framework**: Flask (Python 3.8+)
- **Server**: Gunicorn (production)
- **Protocol**: HTTPS/REST

**Endpoints:**

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/health` | Health check |
| GET | `/api/servers` | List available servers for user |
| POST | `/api/provision` | Generate WireGuard config |
| POST | `/api/revoke` | Revoke client access |
| POST | `/api/generate-api-key` | Generate key (dev only) |

**Key Features:**
- API key validation (HMAC-SHA256)
- Tier-based server filtering
- Integration with wireguard-install.sh
- Secure key handling (600 permissions)
- Client lifecycle management

**Architecture:**
```
Flask API
  ├── validate_api_key()         # Authenticate user
  │   └── → Your auth system integration
  ├── get_available_servers()    # Filter by tier
  │   ├── Basic: 3 servers
  │   └── Pro: 15 servers
  ├── generate_client_config()   # Create WireGuard config
  │   ├── Call wireguard-install.sh --addclient
  │   ├── Read generated .conf file
  │   └── Return config + metadata
  └── revoke_client()            # Remove client
      └── Call wireguard-install.sh --removeclient
```

### 3. VPN Servers (WireGuard)

**Server Specifications:**
- **OS**: Ubuntu 20.04/22.04, Debian 11+, Rocky Linux 8+, Fedora 35+
- **WireGuard**: Latest stable version
- **Network**: Optimized with BBR congestion control
- **Firewall**: UFW or firewalld
- **Monitoring**: Health checks every 60 seconds

**Configuration:**
- **Interface**: wg0
- **Network**: 10.7.0.0/24 (IPv4), fddd:2c4:2c4:2c4::/64 (IPv6)
- **Port**: 51820 (UDP)
- **Max Clients**: 253 per server

**Server Locations & Tiers:**

| Location | Server ID | Tier | Region |
|----------|-----------|------|--------|
| New York, USA | us-east-1 | Basic | North America |
| Los Angeles, USA | us-west-1 | Basic | North America |
| London, UK | eu-west-1 | Basic | Europe |
| Frankfurt, Germany | eu-central-1 | Pro | Europe |
| Singapore | ap-southeast-1 | Pro | Asia Pacific |
| Tokyo, Japan | ap-northeast-1 | Pro | Asia Pacific |
| Mumbai, India | ap-south-1 | Pro | Asia Pacific |
| São Paulo, Brazil | sa-east-1 | Pro | South America |
| Toronto, Canada | ca-central-1 | Pro | North America |
| Stockholm, Sweden | eu-north-1 | Pro | Europe |
| Sydney, Australia | ap-southeast-2 | Pro | Asia Pacific |
| Dubai, UAE | me-south-1 | Pro | Middle East |
| Cape Town, South Africa | af-south-1 | Pro | Africa |
| Milan, Italy | eu-south-1 | Pro | Europe |
| Hong Kong | ap-east-1 | Pro | Asia Pacific |

## Data Flow

### User Onboarding Flow

```
1. User signs up in your web portal
2. User subscribes (Basic or Pro tier)
3. User navigates to VPN section
4. Portal generates API key (user_id + tier + signature)
5. User copies API key
6. User pastes into desktop app
7. App validates key with Provisioning API
8. App fetches available servers (filtered by tier)
9. App ready to connect
```

### Connection Flow

```
1. User clicks "Connect" (or app auto-selects best server)

2. Desktop App → Provisioning API
   POST /api/provision
   Headers: { X-API-Key: <key> }
   Body: { server_id: "us-east-1" }

3. Provisioning API
   ├── Validates API key
   ├── Checks server availability for tier
   ├── Generates unique client name
   ├── Calls: wireguard-install.sh --addclient <name>
   ├── Reads generated config file
   └── Returns WireGuard config

4. Desktop App
   ├── Saves config to /etc/wireguard/privatevpn_<server>.conf
   ├── Sets permissions to 600
   └── Executes: wg-quick up privatevpn_<server>

5. WireGuard establishes connection

6. User traffic routes through VPN server

7. VPN server forwards to destination
   (e.g., your AI model/GPU instances)
```

### Disconnection Flow

```
1. User clicks "Disconnect"

2. Desktop App
   └── Executes: wg-quick down privatevpn_<server>

3. WireGuard tears down connection

4. User traffic returns to normal routing
```

### Server Selection (Auto)

```
1. App fetches server list from API

2. For each server:
   ├── Measure TCP connection time to port 51820
   └── Store latency (milliseconds)

3. Sort servers by latency (ascending)

4. Select first server (lowest latency)

5. Connect to selected server
```

## Security Architecture

### Key Management

**WireGuard Keys:**
```
Server:
  ├── Private Key: Generated once, stored in /etc/wireguard/wg0.conf (600)
  └── Public Key: Derived from private, shared with clients

Client:
  ├── Private Key: Generated per client, stored in client .conf (600)
  ├── Public Key: Derived from private, stored in server config
  └── Pre-Shared Key: Generated per client, additional encryption layer
```

**Key Generation Flow:**
```
1. User requests VPN connection
2. Provisioning API calls: wg genkey (client private key)
3. Provisioning API calls: wg genpsk (pre-shared key)
4. API derives public key: echo $private_key | wg pubkey
5. API adds peer to server config with public key + PSK
6. API creates client config with private key + server public + PSK
7. Client config returned to desktop app
8. Desktop app saves config (never logs it)
```

**Key Security:**
- All configs have 600 permissions (owner-only)
- Keys never transmitted in logs
- Keys never stored in plaintext outside configs
- PSK provides post-quantum security layer
- Unique keys per client (no key sharing)

### Authentication & Authorization

**API Key Format:**
```
userid_tier_signature
```

**API Key Generation (in your portal):**
```python
import hmac
import hashlib

signature = hmac.new(
    SECRET_KEY.encode(),
    f"{user_id}:{tier}".encode(),
    hashlib.sha256
).hexdigest()[:16]

api_key = f"{user_id}_{tier}_{signature}"
```

**API Key Validation (in provisioning API):**
```python
def validate_api_key(api_key):
    parts = api_key.split('_')
    user_id, tier, provided_sig = parts

    expected_sig = hmac.new(
        SECRET_KEY.encode(),
        f"{user_id}:{tier}".encode(),
        hashlib.sha256
    ).hexdigest()[:16]

    if provided_sig == expected_sig:
        return get_user_from_db(user_id)
    return None
```

**Security Features:**
- HMAC-SHA256 signature prevents tampering
- Tier embedded in key (can't upgrade without new key)
- Secret key never exposed to clients
- Keys can be rotated/revoked server-side

### Network Security

**Transport Encryption:**
- Desktop App ↔ API: HTTPS (TLS 1.3)
- Client ↔ VPN Server: WireGuard (ChaCha20-Poly1305)
- VPN Server ↔ Destination: Normal routing (optionally HTTPS)

**Firewall Rules (per VPN server):**
```
Allow UDP 51820 (WireGuard)
Allow 10.7.0.0/24 (VPN clients)
NAT/MASQUERADE for VPN traffic
Drop all other unsolicited inbound
```

## Scalability

### Horizontal Scaling

**Current Limits:**
- 253 clients per server (IP address limit: 10.7.0.2 - 10.7.0.254)
- 15 servers total
- Theoretical max: 3,795 concurrent connections

**Scaling Options:**

1. **Add More Servers:**
   - Add entries to `servers.json`
   - Deploy WireGuard on new servers
   - Update tier configurations

2. **Multiple Subnets per Server:**
   - Use multiple WireGuard interfaces (wg0, wg1, wg2...)
   - Each interface gets different subnet (10.7.0.0/24, 10.8.0.0/24...)
   - Increases capacity to 253 × interfaces per server

3. **Load Balancing:**
   - Run API on multiple servers
   - Use nginx/HAProxy for load balancing
   - Distribute provisioning across server pool

### Vertical Scaling

**Server Resources:**
- CPU: Minimal (WireGuard is very efficient)
- RAM: ~512MB base + ~1MB per client
- Network: Primary bottleneck
  - 1 Gbps supports ~800 clients at 1 Mbps each
  - 10 Gbps supports ~8,000 clients at 1 Mbps each

### Monitoring & Metrics

**Key Metrics to Track:**
1. **Server Metrics:**
   - CPU usage
   - Network throughput
   - Active connections
   - Packet loss
   - Latency

2. **API Metrics:**
   - Requests per second
   - Error rates
   - Response times
   - Failed authentications

3. **Client Metrics:**
   - Connection success rate
   - Average connection time
   - Disconnection reasons
   - Server distribution

**Recommended Tools:**
- Prometheus + Grafana (metrics)
- ELK Stack (logs)
- Uptime Kuma (health checks)
- Sentry (error tracking)

## Deployment Architecture

### Option 1: Centralized API (Recommended)

```
                    ┌─────────────────┐
                    │  Load Balancer  │
                    │   (nginx/AWS)   │
                    └────────┬────────┘
                             │
              ┌──────────────┴───────────────┐
              │                              │
       ┌──────▼──────┐              ┌───────▼──────┐
       │ Provisioning│              │ Provisioning │
       │  API (1)    │              │  API (2)     │
       └──────┬──────┘              └───────┬──────┘
              │                              │
              └──────────────┬───────────────┘
                             │ SSH
              ┌──────────────┴───────────────┐
              │                              │
       ┌──────▼──────┐              ┌───────▼──────┐
       │   VPN       │              │    VPN       │
       │ Server 1    │     ...      │  Server 15   │
       └─────────────┘              └──────────────┘
```

**Pros:**
- Centralized management
- Easy to scale API
- Consistent configuration

**Cons:**
- API is single point of failure (mitigated with load balancer)
- Requires SSH access to all VPN servers

### Option 2: Distributed API

```
       ┌─────────────┐              ┌─────────────┐
       │   VPN       │              │    VPN      │
       │ Server 1    │              │  Server 15  │
       │   + API     │     ...      │   + API     │
       └──────┬──────┘              └──────┬──────┘
              │                              │
              └──────────────┬───────────────┘
                             │
                    ┌────────▼────────┐
                    │  DNS Round      │
                    │  Robin / LB     │
                    └─────────────────┘
```

**Pros:**
- No single point of failure
- Lower latency (API on same server)
- Independent scaling

**Cons:**
- Complex deployment
- Harder to maintain consistency
- More servers to manage

## Integration with Your Application

### End-to-End Encryption Flow

```
User Device (VPN Client)
      │
      │ [WireGuard Encrypted]
      ▼
VPN Server (your control)
      │
      │ [Normal routing or additional encryption]
      ▼
Your AI Model / GPU Instance
```

**Benefits:**
- User's ISP can't see traffic content
- Traffic appears to originate from VPN server IP
- Can bypass geographic restrictions
- Additional security layer for sensitive data

### API Integration Points

**Your Portal → Provisioning API:**
```python
# Generate API key when user subscribes
api_key = generate_vpn_api_key(user.id, user.tier, SECRET_KEY)

# Store in database
user.vpn_api_key = api_key
user.save()

# Display to user
return {
    'api_key': api_key,
    'instructions': 'Paste this into the VPN desktop app'
}
```

**Your Backend → VPN Servers:**
```python
# Whitelist VPN server IPs for your AI model endpoints
ALLOWED_VPN_IPS = [
    '1.2.3.4',      # us-east-1
    '5.6.7.8',      # us-west-1
    # ... all 15 servers
]

@app.route('/ai/inference')
def inference():
    if request.remote_addr not in ALLOWED_VPN_IPS:
        # Also allow direct connections from authenticated users
        if not is_authenticated(request):
            abort(403)

    # Process request
    return run_ai_model(request.data)
```

## Maintenance & Operations

### Regular Tasks

**Daily:**
- Check API health endpoint
- Monitor server CPU/memory
- Review error logs

**Weekly:**
- Update servers.json if locations change
- Review user connection metrics
- Check for failed connections

**Monthly:**
- Rotate API secret keys
- Update WireGuard if new version
- Review capacity planning

**Quarterly:**
- Audit all client configs
- Test disaster recovery
- Review security posture

### Disaster Recovery

**VPN Server Failure:**
1. Remove from servers.json
2. Clients will fail to connect
3. Users select different server
4. Or app auto-selects next best

**API Server Failure:**
1. Load balancer routes to healthy instance
2. Or bring up new API server
3. Update DNS if needed

**Complete Outage:**
1. Deploy API from backup
2. Redeploy VPN servers using wireguard-install.sh
3. Users may need to reconnect (new keys)

## Future Enhancements

### Potential Features

1. **Dynamic Server Addition:**
   - API endpoint to add/remove servers
   - Auto-update client without restart

2. **Usage Tracking:**
   - Data transfer per user
   - Connection time tracking
   - Billing integration

3. **Kill Switch:**
   - Block all traffic if VPN disconnects
   - Prevent IP leaks

4. **Split Tunneling:**
   - Route only specific traffic through VPN
   - Local traffic bypasses VPN

5. **Multi-Hop:**
   - Route through multiple VPN servers
   - Enhanced privacy

6. **WireGuard Config Rotation:**
   - Periodic key regeneration
   - Automatic config updates

7. **Mobile Apps:**
   - iOS app (Swift)
   - Android app (Kotlin)
   - Share backend with desktop

## Conclusion

This architecture provides a secure, scalable, and user-friendly VPN solution that integrates seamlessly with your existing application infrastructure. The use of WireGuard ensures modern cryptography and excellent performance, while the Tauri-based desktop client offers a lightweight and fast user experience.

Key strengths:
- ✅ Preserves all existing WireGuard logic
- ✅ Minimal additions to repository
- ✅ User-friendly GUI for non-technical users
- ✅ Automatic server selection by latency
- ✅ Tier-based access control
- ✅ Secure key management
- ✅ Cross-platform support
- ✅ Easy to maintain and scale
