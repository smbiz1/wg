# OCI VM Setup FAQ

## 🔥 Firewall & Networking

### Q: Does OCI require disabling firewalls for Tailscale?
**A: NO** - Local firewalls remain active. Tailscale operates through them.

**Required configurations:**
- **OCI Security List**: UDP 41641 open (0.0.0.0/0, stateless)
- **Local iptables**: UDP 41641 allowed (setup script handles this)
- **ufw/firewalld**: Can remain enabled

**Setup script automatically:**
- Adds iptables rule for UDP 41641
- Backs up existing iptables configuration
- Preserves firewall functionality

### Q: What if I use ufw or firewalld?
**A: Compatible** - Tailscale works with all common Linux firewalls.

The setup script uses iptables directly (universal on Oracle Linux), but ufw/firewalld can remain active alongside.

### Q: Can I restrict UDP 41641 to specific IPs?
**A: Not recommended** - Tailscale requires broad UDP access for NAT traversal.

Keep the OCI security list rule as `0.0.0.0/0` for UDP 41641. All traffic is encrypted via WireGuard.

## ☁️ Cloudflare Integration

### Q: How to expose Tailscale-protected apps via Cloudflare?

**Two approaches:**

### Option A: Cloudflare Tunnel (Recommended)
```
Internet → Cloudflare Edge → cloudflared → OCI VM Apps
```

**Setup:**
1. Install `cloudflared` on OCI VM
2. Authenticate with Cloudflare
3. Create tunnel to OCI VM
4. Configure DNS routing

**Benefits:**
- Outbound-only connection
- No inbound ports needed
- Apps remain Tailscale-protected
- Secure by default

### Option B: Reverse Proxy
```
Internet → Cloudflare Edge → Public Proxy VM → Tailscale → OCI VM Apps
```

**Setup:**
1. Deploy Nginx/Caddy on separate public VM
2. Configure reverse proxy to Tailscale IPs
3. Point Cloudflare DNS to public proxy

**Benefits:**
- Full proxy control
- Advanced routing rules
- SSL termination

**Drawbacks:**
- Additional infrastructure
- Public-facing proxy management

## 🏗️ Architecture Questions

### Q: Why OCI VMs as subnet gateways?
**A: Only OCI VMs can be subnet routers** - Serverless platforms cannot due to ephemeral nature and network restrictions.

**Multi-Cloud Agentic Workflows:**
```
OCI VM (Main Backend) → GCP Cloud Run → Fly.io → Modal GPU → Back to OCI
     ↓                        ↓            ↓          ↓
   Oracle Cache            Vertex AI     Chunking    OCR/Parse
   Object Storage          Models        Service     Service
```

**Serverless IP Behavior:**
- **OCI VM**: Persistent IP (can be subnet router)
- **GCP Cloud Run**: Ephemeral IPs (static possible via Cloud NAT + VPC)
- **Fly.io**: Ephemeral IPs (static egress IPs available for $0.005/hour)
- **Modal**: Ephemeral IPs (static available via Proxies feature)

**Benefits:**
- Global infrastructure with high bandwidth
- Cost-effective compute for main applications
- Direct access to Oracle services (Object Storage, Cache/Redis)
- Single subnet gateway for entire multi-cloud stack

### Q: How does the complex agentic workflow (OCI → GCP → Fly → Modal → back to OCI) work with Tailscale?
**A: Seamless encrypted routing** - See [Multi-Cloud Agentic Flows](multi-cloud-agentic-flow.md) for complete implementation.

**Connectivity Overview:**
- ✅ **OCI Backend ↔ GCP Cloud Run**: Direct Tailscale encryption
- ✅ **OCI Backend ↔ Fly.io**: Direct Tailscale encryption
- ✅ **OCI Backend ↔ Modal GPU**: Direct Tailscale encryption
- ✅ **Internal OCI services**: Oracle Object Storage, Cache/Redis (subnet routing)
- ⚠️ **Neon PostgreSQL**: External - requires Cloudflare Tunnel or VPN

**Implementation:**
1. **OCI VM as subnet gateway** advertises `10.0.0.0/24`
2. **All platforms run Tailscale** for peer-to-peer encryption
3. **External services** (Neon) use secure tunnels
4. **Parallel processing** maintains encryption throughout

**Security:** 90%+ of connections fully encrypted via Tailscale WireGuard.

### Q: Can I use multiple OCI VMs?
**A: Yes** - For high availability:
1. Set up multiple OCI VMs as routers
2. Advertise same routes from each
3. Tailscale load-balances automatically

## 🔧 Configuration

### Q: How to customize subnet advertisement?
**A: Environment variables:**
```bash
# In cfg.sh
export ORACLE_SUBNET="10.0.0.0/24,192.168.1.0/24"  # Multiple subnets
export TAILSCALE_HOSTNAME="oci-router-01"           # Custom hostname
export ACCEPT_ROUTES="true"                         # Accept peer routes
```

### Q: What if auto-detection fails?
**A: Manual subnet specification:**
```bash
# Check your subnet
ip route show | grep -E "^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/[0-9]+"

# Set manually in cfg.sh
export ORACLE_SUBNET="10.0.0.0/24"
```

## 🐛 Common Issues

### Q: "Permission denied" during setup
**A: Run with sudo:**
```bash
sudo bash scripts/setup-oci-vm.sh
```

### Q: Tailscale fails to connect
**A: Check prerequisites:**
```bash
# Verify OCI networking
./tools/oci-network-cfg.sh  # Shows required steps

# Check auth key
echo $TAILSCALE_AUTH_KEY   # Should start with "tskey-"

# Verify internet
curl -fsSL https://tailscale.com/install.sh | head -1
```

### Q: Routes stuck in "pending"
**A: Manual approval required:**
1. Visit Tailscale Admin Console
2. Find VM under "Machines"
3. Click **Review route** → **Approve**

### Q: DNS resolution fails
**A: Check --accept-dns flag:**
- Setup uses `--accept-dns=false` (correct for OCI)
- For custom DNS, configure Tailscale split DNS
- Test: `dig @169.254.169.254 instance.subnet.vcn.oraclevcn.com`

## 📊 Performance

### Q: What's the performance impact?
**A: Minimal** - Tailscale uses:
- WireGuard encryption (fast)
- Direct UDP connections when possible
- Automatic failover to relayed connections

### Q: How much bandwidth overhead?
**A: ~5-10%** - WireGuard encryption adds minimal overhead.

## 🔒 Security

### Q: Is this secure?
**A: Enterprise-grade:**
- WireGuard encryption
- Automatic key rotation
- Network segmentation
- ACL-based access control

### Q: What about the UDP 41641 exposure?
**A: Safe** - All traffic is:
- Encrypted end-to-end
- Authenticated via Tailscale
- NAT-traversal only (no data exposure)

### Q: Can I restrict access further?
**A: Yes:**
- Tailscale ACLs for granular control
- Subnet isolation
- User-based authentication
- Device approval workflows

### Q: How do ephemeral Tailscale keys work with serverless?
**A: Perfect for serverless** - Ephemeral nodes auto-cleanup when functions end.

**Setup:**
```bash
# Generate ephemeral auth key in Tailscale admin
# Use in serverless functions:

# GCP Cloud Run
tailscale up --auth-key=$KEY --ephemeral --hostname=gcp-function-$RANDOM

# Fly.io Functions
tailscale up --auth-key=$KEY --ephemeral --hostname=fly-func-$RANDOM

# Modal GPUs
tailscale up --auth-key=$KEY --ephemeral --hostname=modal-gpu-$RANDOM
```

**Benefits:**
- No manual cleanup needed
- Automatic node removal after function ends
- Security by default (nodes don't persist)
- Cost optimization (no lingering nodes)

**In-memory option for containers:**
```bash
# For containerized serverless
tailscaled --state=mem:  # No disk persistence
tailscale up --auth-key=$KEY  # Auto-ephemeral
```