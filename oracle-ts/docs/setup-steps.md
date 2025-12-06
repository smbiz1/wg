# OCI VM Setup Steps

## 🔢 Complete Step-by-Step Setup Process

### STEP 1: OCI Console Network Configuration
**Run locally:** `./tools/oci-network-cfg.sh`
- Opens UDP port 41641 in OCI security list
- Required for Tailscale direct connections

### STEP 2: Generate Tailscale Auth Key
1. Visit: https://login.tailscale.com/admin/settings/authkeys
2. Create: **Ephemeral, pre-authorized** key
3. Copy: `tskey-...` value

### STEP 3: Deploy Scripts to OCI VM
```bash
# Copy all production scripts
scp scripts/* oci@your-oci-vm-ip:~/
```

### STEP 4: Configure Environment (Optional)
```bash
# On OCI VM - customize if needed
nano cfg.sh
source cfg.sh
```

### STEP 5: Execute Main Setup
```bash
# SSH to OCI VM
ssh oci@your-oci-vm-ip

# Run automated setup (requires sudo)
sudo bash setup-oci-vm.sh
```
**Script performs:**
1. System requirement checks
2. Tailscale installation
3. iptables configuration
4. Subnet auto-detection
5. Tailscale router setup
6. Configuration verification

### STEP 6: Approve Routes in Tailscale Admin
1. Navigate: https://login.tailscale.com/admin/machines
2. Locate: Your OCI VM
3. Action: **Review route** → **Approve**

### STEP 7: Execute Connectivity Tests
```bash
# Run comprehensive tests
./tools/test-conn.sh
```
**Validates:**
- Tailscale daemon status
- Route advertisement
- iptables rules
- OCI DNS accessibility
- Peer connectivity

### STEP 8: Deploy Applications
Your OCI VM is now a functional Tailscale subnet router.

**Deploy:**
- Next.js frontend application
- Python backend API
- Additional services as needed

### STEP 9: Configure Cloudflare (Optional)
**For public internet access:**

**Recommended:** Cloudflare Tunnel
- Install `cloudflared` on OCI VM
- Create outbound secure tunnel
- No inbound ports required

**Alternative:** Reverse Proxy
- Deploy Nginx/Caddy on separate public VM
- Route traffic through Tailscale to OCI VMs

---

## 🎯 Post-Setup Checklist

- [ ] OCI security list: UDP 41641 open
- [ ] Tailscale auth key: Generated and configured
- [ ] Setup scripts: Deployed to OCI VM
- [ ] Main setup: Executed successfully
- [ ] Routes: Approved in Tailscale admin
- [ ] Connectivity: Tested and verified
- [ ] Applications: Deployed and running
- [ ] Cloudflare: Configured (if needed)

---

## 🆘 Troubleshooting

### Setup Fails
```bash
# Check system requirements
whoami  # Should be root
cat /etc/os-release  # Should show Oracle Linux

# Verify network
curl -fsSL https://tailscale.com/install.sh | head -5  # Network test
```

### Tailscale Issues
```bash
# Check daemon
systemctl status tailscaled

# View logs
journalctl -u tailscaled -f

# Reinitialize
sudo bash scripts/ts-init.sh
```

### Route Approval Pending
- Return to Tailscale Admin Console
- Find VM under "Machines" → "Review route"
- Click **Approve** for advertised routes

### Connectivity Problems
```bash
# Run diagnostics
./tools/test-conn.sh

# Manual tests
tailscale status
tailscale ping other-tailnet-device
```