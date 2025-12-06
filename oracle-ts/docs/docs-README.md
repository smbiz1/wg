# Oracle Cloud VM Tailscale Subnet Router Setup

This directory contains automated setup scripts for configuring Oracle Cloud VMs as Tailscale subnet routers in our secure VPN architecture. Oracle VMs serve as the central routing hub for all traffic from other platforms (Fly.io, GCP, Modal).

## 🏗️ Architecture Overview

```
┌──────────────────┐    ┌─────────────────┐    ┌──────────────────┐
│  Oracle VM Main  │────│   Fly.io Apps   │────│   Other Tailnet   │
│ (Frontend + API) │    │  (Containers)   │    │  Nodes (GCP, Modal│
│  Subnet Router   │    │                 │    │      etc.)       │
└──────────────────┘    └─────────────────┘    └──────────────────┘
         │
         │ (Cloudflare Edge)
         ▼
    ┌─────────────┐
    │   Internet  │
    │   Users     │
    └─────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Oracle Cloud account with VM instances
- Tailscale network admin access
- SSH access to Oracle VMs
- `sudo` privileges on Oracle VMs

### 1. Generate Tailscale Auth Key
```bash
# Go to https://login.tailscale.com/admin/settings/authkeys
# Create an ephemeral, pre-authorized auth key
# Copy the key (starts with "tskey-")
```

### 2. Configure Oracle Cloud Networking
```bash
# Run this locally to see Oracle Cloud Console steps
./oracle-network-config.sh
```
This will display the steps to add UDP port 41641 to your security list.

### 3. Set Up Oracle VM
```bash
# Copy scripts to your Oracle VM
scp setup-oracle-vm.sh oracle@your-vm-ip:~/
scp tailscale-init.sh oracle@your-vm-ip:~/

# SSH into VM and run setup
ssh oracle@your-vm-ip
sudo bash setup-oracle-vm.sh
```

### 4. Set Environment Variables (Optional)
```bash
# For automated setup
export TAILSCALE_AUTH_KEY="tskey-your-key-here"
export TAILSCALE_HOSTNAME="oracle-main-server"
export ORACLE_SUBNET="10.0.0.0/24"  # Your Oracle subnet
```

### 5. Approve Routes in Tailscale Admin
1. Go to [Tailscale Admin Console](https://login.tailscale.com/admin/machines)
2. Find your Oracle VM
3. Click **Review route** → **Approve**

## 📁 Files Overview

### 🚀 **PRODUCTION SCRIPTS** (Run on Oracle VMs)
- **`setup-oracle-vm.sh`** - **MAIN SETUP SCRIPT** - Complete automated Oracle VM Tailscale configuration
- **`tailscale-init.sh`** - Tailscale reconfiguration script (use after initial setup)
- **`config.sh`** - Environment variables template (copy to .env and customize)

### 🔧 **CONFIGURATION HELPERS** (Run locally)
- **`oracle-network-config.sh`** - Oracle Cloud Console networking guide & verification
- **`test-connectivity.sh`** - Post-setup connectivity testing & diagnostics

### 📖 **DOCUMENTATION**
- **`README.md`** - Complete setup guide and troubleshooting (this file)

---

## 🔢 **STEP-BY-STEP SETUP PROCESS**

### **STEP 1: Oracle Cloud Console Configuration**
```bash
# Run locally to see Oracle Cloud networking steps
./oracle-network-config.sh
```
**What happens:** Displays exact steps to add UDP 41641 to Oracle security list

### **STEP 2: Generate Tailscale Auth Key**
1. Go to https://login.tailscale.com/admin/settings/authkeys
2. Create **ephemeral, pre-authorized** auth key
3. Copy the `tskey-...` key

### **STEP 3: Deploy Scripts to Oracle VM**
```bash
# Copy setup scripts to your Oracle VM
scp setup-oracle-vm.sh oracle@your-vm-ip:~/
scp tailscale-init.sh oracle@your-vm-ip:~/
scp config.sh oracle@your-vm-ip:~/
```

### **STEP 4: Configure Environment (Optional)**
```bash
# On Oracle VM - edit config.sh with your values
nano config.sh
source config.sh  # Load variables
```

### **STEP 5: Run Main Setup**
```bash
# SSH into Oracle VM
ssh oracle@your-vm-ip

# Run automated setup (requires sudo)
sudo bash setup-oracle-vm.sh
```
**What the script does:**
1. Checks root access and Oracle Linux
2. Installs Tailscale
3. Configures iptables for Tailscale
4. Auto-detects Oracle subnet
5. Initializes Tailscale with subnet routing
6. Tests configuration

### **STEP 6: Approve Routes in Tailscale Admin**
1. Go to https://login.tailscale.com/admin/machines
2. Find your Oracle VM
3. Click **"Review route"** → **Approve**

### **STEP 7: Test Connectivity**
```bash
# Run connectivity tests
./test-connectivity.sh
```
**Tests performed:**
- Tailscale daemon status
- Route advertisement
- iptables configuration
- Oracle DNS accessibility
- Peer connectivity

### **STEP 8: Deploy Your Applications**
Your Oracle VM is now a Tailscale subnet router. Deploy:
- Next.js frontend
- Python backend API
- Any other services

### **STEP 9: Configure Cloudflare (Optional)**
For public access with edge security:
1. Set up Cloudflare Tunnel OR reverse proxy
2. Point DNS to Cloudflare
3. Route traffic through Tailscale network

---

## 🔥 **FIREWALL & CLOUDFLARE FAQ**

### **Q: Does Oracle require disabling firewalls for Tailscale?**
**A: NO** - Keep local firewalls (iptables/ufw) enabled. Tailscale works through them.

**What you need:**
- ✅ **Oracle Security List**: UDP 41641 open (0.0.0.0/0)
- ✅ **Local iptables**: Allow UDP 41641 (setup script does this)
- ✅ **ufw/firewalld**: Can remain active

### **Q: How does Cloudflare integrate with Tailscale-protected services?**
**Two approaches:**

**Option A - Cloudflare Tunnel (Recommended):**
```
Internet → Cloudflare Edge → Cloudflare Tunnel → Oracle VM (Tailscale-protected)
```
- Install `cloudflared` on Oracle VM
- Creates outbound-only secure tunnel
- No inbound ports needed

**Option B - Reverse Proxy:**
```
Internet → Cloudflare Edge → Public Reverse Proxy → Tailscale → Internal Services
```
- Nginx/Caddy on separate public VM
- Routes to Tailscale-protected Oracle VMs
- Requires public-facing proxy server

## ⚙️ Configuration Options

### Environment Variables
- `TAILSCALE_AUTH_KEY` - Your Tailscale auth key
- `TAILSCALE_HOSTNAME` - VM hostname in Tailscale (default: oracle-main-server)
- `ORACLE_SUBNET` - Your Oracle subnet CIDR (auto-detected if possible)
- `ACCEPT_ROUTES` - Whether to accept routes from other nodes (default: true)

### Manual Configuration
If you prefer manual setup:

```bash
# Install Tailscale
curl -fsSL https://tailscale.com/install.sh | sh

# Configure iptables
sudo iptables -I INPUT -p udp --dport 41641 -j ACCEPT
sudo netfilter-persistent save

# Start Tailscale
sudo tailscale up --auth-key=tskey-... --hostname=oracle-router \
  --advertise-routes=10.0.0.0/24,169.254.169.254/32 \
  --accept-dns=false --accept-routes
```

## 🔧 Advanced Configuration

### Split DNS Setup
For Oracle internal hostnames to work across your tailnet:

1. Go to **DNS** tab in Tailscale Admin
2. Add nameserver for `oraclevcn.com` domain
3. Point to `169.254.169.254` (Oracle DNS)
4. Restrict to your Oracle VM's IP

### Multiple Oracle VMs
For high availability, set up multiple Oracle VMs as routers:

```bash
# On second VM
export TAILSCALE_HOSTNAME="oracle-backup-router"
sudo bash setup-oracle-vm.sh
```

### Custom Subnet Ranges
If you have multiple subnets:

```bash
export ORACLE_SUBNET="10.0.0.0/24,10.0.1.0/24,192.168.1.0/24"
sudo bash setup-oracle-vm.sh
```

## 🧪 Testing & Verification

### From Oracle VM
```bash
# Check status
tailscale status
tailscale ip -4

# Test connectivity
tailscale ping fly-app-hostname
tailscale ping gcp-instance-hostname
```

### From Other Platforms
```bash
# From Fly.io container
curl http://oracle-router-ip:8080

# From GCP/Modal
ping oracle-router-ip
```

### Oracle-Specific Tests
```bash
# Test Oracle DNS
dig @169.254.169.254 instance.subnet01234567.vcn01234567.oraclevcn.com

# Check network
ip route show
ip addr show
```

## 🔒 Security Considerations

1. **Firewall Rules**: UDP 41641 enables direct WireGuard connections
2. **SSH Access**: Remove public SSH after Tailscale works
3. **Auth Keys**: Use ephemeral keys for VMs
4. **Route Approval**: Manually approve advertised routes
5. **ACLs**: Configure Tailscale ACLs for access control

## 🐛 Troubleshooting

### Common Issues

**"Permission denied" on setup:**
```bash
sudo bash setup-oracle-vm.sh
```

**Tailscale not connecting:**
```bash
# Check auth key
tailscale status

# Restart daemon
sudo systemctl restart tailscaled

# Reinitialize
sudo bash tailscale-init.sh
```

**Routes not approved:**
- Check Tailscale Admin Console
- Look for pending route approvals

**DNS issues:**
- Ensure `--accept-dns=false` is used
- Configure split DNS for Oracle domains

**Firewall blocks:**
- Verify UDP 41641 in Oracle security list
- Check iptables: `iptables -L | grep 41641`

### Logs & Debugging
```bash
# Tailscale logs
journalctl -u tailscaled -f

# Network debugging
tcpdump -i any udp port 41641

# iptables debugging
iptables -L -v -n
```

## 📚 Integration with Other Platforms

### Fly.io Apps
After Oracle VM setup, Fly.io apps can route through Oracle:

```bash
# In fly-ts/start.sh, add route to Oracle
tailscale up --accept-routes
```

### GCP Instances
Configure GCP instances to use Oracle VM as gateway:

```bash
# On GCP instance
tailscale up --accept-routes
# Routes will be available via Oracle VM
```

### Modal Endpoints
Modal functions can access Oracle-routed networks:

```bash
# Modal environment will inherit Tailscale routes
```

## 🔄 Maintenance

### Updating Tailscale
```bash
# Update package
sudo apt update && sudo apt install tailscale

# Restart service
sudo systemctl restart tailscaled
```

### Reconfiguring Routes
```bash
# Bring down
tailscale down

# Reconfigure
sudo bash tailscale-init.sh
```

### Backup & Recovery
```bash
# iptables backup is automatic in setup script
# Tailscale state persists across restarts
# Auth keys are ephemeral - regenerate if needed
```

## 📋 Checklist

- [ ] Oracle VM created and accessible via SSH
- [ ] UDP 41641 added to Oracle security list
- [ ] Tailscale auth key generated
- [ ] Setup script run successfully
- [ ] Routes approved in Tailscale Admin
- [ ] Connectivity tested from other platforms
- [ ] Public SSH access removed
- [ ] Split DNS configured (optional)

## 🎯 Next Steps

1. **Test cross-platform connectivity**
2. **Set up monitoring/alerting**
3. **Configure backup Oracle VMs**
4. **Implement access controls**
5. **Document your network topology**

---

## 📖 Related Documentation

- [Oracle VM Setup Guide](../docs/ts_fly-oci-gcp-modal/tailscale-oracle-vm-setup-guide.md)
- [Fly.io Tailscale Setup](../fly-ts/README.md)
- [Tailscale Subnet Routers](https://tailscale.com/kb/1019/subnets)
- [Oracle Cloud Networking](https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/overview.htm)