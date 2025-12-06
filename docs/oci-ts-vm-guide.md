# Oracle Cloud VM Tailscale Setup Guide

## Overview
This guide covers setting up Oracle Cloud VMs as the main servers in our Tailscale network architecture. Oracle VMs will serve as subnet routers, allowing all outgoing traffic from other Tailscale machines (Fly.io, GCP, Modal) to route through Oracle Cloud infrastructure.

## Prerequisites
- Oracle Cloud account with VM instances created
- Tailscale network with admin access
- SSH access to Oracle VMs
- Existing Tailscale setup on at least one device

## Key Oracle-Specific Requirements

### 1. UDP Port 41641 Firewall Rule
Oracle Cloud blocks direct WireGuard connections by default. You MUST allow UDP port 41641 for optimal performance:

**In Oracle Cloud Console:**
1. Navigate to **Networking** → **Virtual Cloud Networks**
2. Select your VCN
3. Click **Security Lists** (left sidebar)
4. Select your security list (usually "Default")
5. Click **Add Ingress Rules**
6. Add: `0.0.0.0/0, UDP, 41641, Stateless`

This enables "easy NAT" traversal for direct peer-to-peer connections.

### 2. Oracle DNS Configuration
Oracle uses a special DNS server at `169.254.169.254` for internal hostnames like:
- `instance.subnet01234567.vcn01234567.oraclevcn.com`

**Critical:** Use `--accept-dns=false` to prevent Tailscale from overriding Oracle's DNS.

## Step-by-Step Setup

### Step 1: Install Tailscale on Oracle VM
```bash
# SSH into your Oracle VM
ssh -i your-key.pem ubuntu@your-vm-ip

# Install Tailscale (Oracle Linux)
curl -fsSL https://tailscale.com/install.sh | sh

# Start Tailscale (interactive auth)
sudo tailscale up

# Or use auth key (recommended for automation)
sudo tailscale up --auth-key=tskey-your-key-here
```

### Step 2: Configure Subnet Router
Oracle VMs will advertise routes to allow other Tailscale nodes to reach Oracle subnets:

```bash
# Get your subnet CIDR (e.g., 10.0.0.0/24)
ip route show

# Advertise routes including Oracle DNS server
sudo tailscale up --advertise-routes=10.0.0.0/24,169.254.169.254/32 --accept-dns=false --hostname=oracle-main-server
```

**Important Flags:**
- `--advertise-routes`: Makes subnet available to tailnet
- `--accept-dns=false`: Prevents Tailscale DNS override
- `--hostname`: Descriptive name for the VM

### Step 3: Approve Routes in Tailscale Admin
1. Go to [Tailscale Admin Console](https://login.tailscale.com/admin/machines)
2. Find your Oracle VM
3. Click **Review route** → **Approve**

### Step 4: Configure Split DNS (Optional but Recommended)
For Oracle internal hostnames to work across tailnet:

1. Go to **DNS** tab in Tailscale Admin
2. Add nameserver for `oraclevcn.com` domain
3. Point to `169.254.169.254` (Oracle DNS)
4. Restrict to your Oracle VM's IP

### Step 5: iptables Configuration (If Needed)
Oracle Linux may have restrictive iptables rules. If connectivity fails:

```bash
# Backup current iptables
sudo iptables-save > ~/iptables.backup

# Check current rules
sudo iptables --list --line-numbers

# Allow Tailscale traffic (if blocked)
sudo iptables -I INPUT 6 -m state --state NEW -p udp --dport 41641 -j ACCEPT
sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 22 -j ACCEPT  # SSH over Tailscale

# For web services (if running)
sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 80 -j ACCEPT
sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 443 -j ACCEPT

# Save changes
sudo netfilter-persistent save
```

### Step 6: Remove Public SSH Access
Once Tailscale is working:

1. In Oracle Cloud Console → Security Lists
2. Remove public SSH ingress rule (0.0.0.0/0 TCP 22)
3. Add Tailscale subnet rule if needed: `100.64.0.0/10, TCP, 22`

## Verification Steps

### Test Connectivity from Oracle VM
```bash
# Check Tailscale status
tailscale status

# Ping other tailnet nodes
tailscale ping node-name

# Test subnet routing
ping 10.0.0.x  # Other Oracle VMs in same subnet
```

### Test from Other Platforms (Fly.io, GCP, Modal)
```bash
# From Fly.io container
curl http://oracle-vm-tailscale-ip:8080

# Test DNS resolution
nslookup instance.subnet01234567.vcn01234567.oraclevcn.com
```

## Architecture Benefits

1. **Central Routing**: All traffic from Fly.io/GCP/Modal routes through Oracle
2. **DNS Integration**: Access Oracle internal hostnames from anywhere
3. **Security**: No public IPs needed after setup
4. **Performance**: Direct WireGuard connections via UDP 41641
5. **Scalability**: Multiple Oracle VMs can share routes

## Troubleshooting

### Common Issues

**Can't reach Oracle VM from other nodes:**
- Check UDP 41641 is open in security list
- Verify routes are approved in Tailscale admin
- Check iptables rules

**DNS resolution fails:**
- Ensure `--accept-dns=false` is used
- Configure split DNS for `oraclevcn.com`
- Test Oracle DNS: `dig @169.254.169.254 hostname.oraclevcn.com`

**Route advertisement fails:**
- Check VM has proper subnet access
- Verify Tailscale version supports subnet routing
- Check Tailscale admin for route approval notifications

### Oracle-Specific Commands
```bash
# Check Oracle DNS
dig @169.254.169.254 instance.subnet01234567.vcn01234567.oraclevcn.com

# View network interfaces
ip addr show
ip route show

# Oracle instance metadata
curl http://169.254.169.254/opc/v2/instance/
```

## Security Considerations

1. **Ephemeral Keys**: Use for short-lived VMs
2. **ACLs**: Restrict access to necessary subnets only
3. **Monitoring**: Enable Tailscale audit logs
4. **Updates**: Keep Tailscale updated for security patches

## Next Steps

With Oracle VMs configured as subnet routers:
1. Deploy Fly.io apps that route through Oracle
2. Set up GCP instances with Oracle routing
3. Configure Modal endpoints via Oracle network
4. Implement monitoring and alerting

## References
- [Tailscale Oracle Cloud Guide](https://tailscale.com/kb/1149/cloud-oracle)
- [Subnet Routers](https://tailscale.com/kb/1019/subnets)
- [Oracle Cloud Networking](https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/overview.htm)