#!/bin/bash

# ========================================================================================
# OCI VM TAILSCALE SUBNET ROUTER SETUP
# ========================================================================================
#
# DESCRIPTION:
#   Configures Oracle Cloud Infrastructure (OCI) VM as Tailscale subnet router
#   Enables secure routing: OCI VM → Fly.io/GCP/Modal
#
# PREREQUISITES:
#   - OCI VM with sudo access
#   - Tailscale auth key (ephemeral, pre-authorized)
#   - UDP 41641 open in OCI security list
#
# USAGE:
#   sudo bash setup-oci-vm.sh
#
# STEPS PERFORMED:
#   1. System requirement checks (root, OCI Linux)
#   2. Tailscale installation
#   3. iptables configuration for Tailscale
#   4. OCI subnet auto-detection
#   5. Tailscale subnet router setup
#   6. Configuration verification
#
# ENVIRONMENT VARIABLES (optional):
#   TAILSCALE_AUTH_KEY  - Your Tailscale auth key
#   OCI_SUBNET         - OCI subnet CIDR (auto-detected if not set)
#   TAILSCALE_HOSTNAME - VM hostname in Tailscale
#   ACCEPT_ROUTES      - Accept routes from peers (default: true)
#
# ========================================================================================

set -e  # Exit on any error

# Configuration variables (customize as needed)
TAILSCALE_AUTH_KEY="${TAILSCALE_AUTH_KEY:-}"  # Set via environment variable
OCI_SUBNET="${OCI_SUBNET:-10.0.0.0/24}"      # OCI subnet CIDR
TAILSCALE_HOSTNAME="${TAILSCALE_HOSTNAME:-oci-main-server}"
ACCEPT_ROUTES="${ACCEPT_ROUTES:-true}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

success() {
    echo -e "${GREEN}✓ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

error() {
    echo -e "${RED}✗ $1${NC}"
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root (sudo)"
        exit 1
    fi
}

# Check if we're on OCI Linux
check_oci_linux() {
    if ! grep -q "Oracle Linux" /etc/os-release 2>/dev/null; then
        warning "This script is designed for Oracle Linux. Continue anyway? (y/N)"
        read -r response
        if [[ ! "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            exit 1
        fi
    fi
}

# Install Tailscale
install_tailscale() {
    log "Installing Tailscale..."

    if command -v tailscale &> /dev/null; then
        success "Tailscale already installed"
        return
    fi

    # Install Tailscale using official script
    curl -fsSL https://tailscale.com/install.sh | sh

    success "Tailscale installed successfully"
}

# Configure iptables for Tailscale
configure_iptables() {
    log "Configuring iptables for Tailscale..."

    # Backup current iptables
    iptables-save > /root/iptables.backup.$(date +%Y%m%d_%H%M%S)
    success "iptables backup created"

    # Allow Tailscale traffic
    iptables -I INPUT -p udp --dport 41641 -j ACCEPT 2>/dev/null || true
    iptables -I INPUT -p tcp --dport 22 -j ACCEPT 2>/dev/null || true

    # Save iptables rules
    if command -v netfilter-persistent &> /dev/null; then
        netfilter-persistent save
        success "iptables rules saved with netfilter-persistent"
    else
        warning "netfilter-persistent not found - iptables rules may not persist after reboot"
    fi
}

# Get OCI subnet information
get_oci_subnet() {
    log "Detecting OCI subnet..."

    # Try to detect subnet from network interfaces
    local subnet
    subnet=$(ip route show | grep -E "^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/[0-9]+" | head -1 | awk '{print $1}' 2>/dev/null)

    if [[ -n "$subnet" ]]; then
        OCI_SUBNET="$subnet"
        success "Detected OCI subnet: $OCI_SUBNET"
    else
        warning "Could not auto-detect subnet. Using configured value: $OCI_SUBNET"
    fi
}

# Configure Tailscale subnet router
configure_tailscale() {
    log "Configuring Tailscale subnet router..."

    # Stop any existing tailscale
    systemctl stop tailscaled 2>/dev/null || true
    tailscale down 2>/dev/null || true

    # Start tailscaled
    systemctl start tailscaled
    sleep 2

    # Build tailscale up command
    local cmd="tailscale up"

    if [[ -n "$TAILSCALE_AUTH_KEY" ]]; then
        cmd="$cmd --auth-key=$TAILSCALE_AUTH_KEY"
    else
        warning "No TAILSCALE_AUTH_KEY provided. You'll need to authenticate manually."
    fi

    cmd="$cmd --hostname=$TAILSCALE_HOSTNAME"
    cmd="$cmd --advertise-routes=$OCI_SUBNET,169.254.169.254/32"  # OCI DNS server
    cmd="$cmd --accept-dns=false"  # Don't override OCI DNS

    # Note: --authkey flag removed as it's not needed for OCI VMs
    # OCI VMs use persistent infrastructure, not ephemeral like serverless

    if [[ "$ACCEPT_ROUTES" == "true" ]]; then
        cmd="$cmd --accept-routes"
    fi

    log "Running: $cmd"
    eval "$cmd"

    success "Tailscale configured"
}

# Verify configuration
verify_setup() {
    log "Verifying setup..."

    # Check Tailscale status
    if tailscale status &>/dev/null; then
        success "Tailscale is running"
        echo "Tailscale IP: $(tailscale ip -4)"
        echo "Tailscale status:"
        tailscale status | head -10
    else
        error "Tailscale is not running properly"
        return 1
    fi

    # Check iptables
    if iptables -L | grep -q "41641"; then
        success "iptables configured for Tailscale UDP port"
    else
        warning "UDP 41641 not found in iptables - may need manual configuration"
    fi

    success "Setup verification completed"
}

# Print next steps
print_next_steps() {
    cat << 'EOF'

🎉 Oracle VM Tailscale setup completed!

NEXT STEPS:

1. **Approve routes in Tailscale Admin:**
   - Go to https://login.tailscale.com/admin/machines
   - Find this VM and click "Review route"
   - Approve the advertised routes

2. **Configure Split DNS (optional but recommended):**
   - In Tailscale Admin → DNS
   - Add nameserver for "oraclevcn.com" domain
   - Point to 169.254.169.254 (Oracle DNS)
   - Restrict to this VM's IP

3. **Test connectivity from other nodes:**
   ```bash
   # From Fly.io container
   curl http://oracle-vm-tailscale-ip:8080

   # From other tailnet nodes
   ping oracle-vm-tailscale-ip
   ```

4. **Secure the VM:**
   - Remove public SSH access from Oracle Cloud Console
   - Only allow SSH from Tailscale network

5. **Monitor and troubleshoot:**
   ```bash
   # Check status
   tailscale status
   tailscale ping other-node

   # View logs
   journalctl -u tailscaled -f
   ```

Your Oracle VM is now ready to serve as a subnet router for your entire Tailscale network!

EOF
}

# Main execution
main() {
    echo "🚀 Oracle Cloud VM Tailscale Subnet Router Setup"
    echo "=================================================="
    echo
    echo "🔢 STEP-BY-STEP PROCESS:"
    echo "1. ✅ Check system requirements (root access, Oracle Linux)"
    echo "2. 📦 Install Tailscale"
    echo "3. 🔥 Configure iptables for Tailscale traffic"
    echo "4. 🕵️  Auto-detect Oracle subnet"
    echo "5. ⚙️  Configure Tailscale subnet router"
    echo "6. 🧪 Verify setup and test connectivity"
    echo

    # Step 1: System checks
    echo "📋 STEP 1: Checking system requirements..."
    check_root
    check_oci_linux
    success "System requirements met"
    echo

    # Step 2: Install Tailscale
    echo "📦 STEP 2: Installing Tailscale..."
    install_tailscale
    echo

    # Step 3: Configure iptables
    echo "🔥 STEP 3: Configuring iptables for Tailscale..."
    configure_iptables
    echo

    # Step 4: Get subnet
    echo "🕵️  STEP 4: Detecting OCI subnet..."
    get_oci_subnet
    echo

    # Step 5: Configure Tailscale
    echo "⚙️  STEP 5: Configuring Tailscale subnet router..."
    configure_tailscale
    echo

    # Step 6: Verify setup
    echo "🧪 STEP 6: Verifying setup..."
    verify_setup
    echo

    print_next_steps
}

# Run main function
main "$@"