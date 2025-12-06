#!/bin/bash

# ========================================================================================
# TAILSCALE INITIALIZATION FOR OCI VMs
# ========================================================================================
#
# DESCRIPTION:
#   Initializes/reinitializes Tailscale on OCI VMs
#   Use after initial setup or for configuration changes
#
# USAGE:
#   sudo bash ts-init.sh
#
# ENVIRONMENT VARIABLES:
#   TAILSCALE_AUTH_KEY  - Your Tailscale auth key
#   TAILSCALE_HOSTNAME - VM hostname in Tailscale
#   OCI_SUBNET         - OCI subnet CIDR to advertise
#
# ========================================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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
if [[ $EUID -ne 0 ]]; then
    error "This script must be run as root"
    exit 1
fi

# Get subnet information
get_subnet() {
    ip route show | grep -E "^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/[0-9]+" | head -1 | awk '{print $1}' || echo "10.0.0.0/24"
}

# Main configuration
SUBNET="${OCI_SUBNET:-$(get_subnet)}"
HOSTNAME="${TAILSCALE_HOSTNAME:-oci-subnet-router}"
AUTH_KEY="$TAILSCALE_AUTH_KEY"

log "Initializing Tailscale on Oracle VM"
log "Subnet: $SUBNET"
log "Hostname: $HOSTNAME"

# Stop existing tailscale
systemctl stop tailscaled 2>/dev/null || true
tailscale down 2>/dev/null || true

# Start daemon
systemctl start tailscaled
sleep 3

# Build command
CMD="tailscale up --hostname=$HOSTNAME --advertise-routes=$SUBNET,169.254.169.254/32 --accept-dns=false --accept-routes"

if [[ -n "$AUTH_KEY" ]]; then
    CMD="$CMD --auth-key=$AUTH_KEY"
fi

log "Running: $CMD"
eval "$CMD"

# Verify
sleep 2
if tailscale status &>/dev/null; then
    success "Tailscale initialized successfully"
    echo "IP: $(tailscale ip -4)"
    echo "Status: $(tailscale status | head -1)"
else
    error "Tailscale initialization failed"
    exit 1
fi

log "Don't forget to approve routes in Tailscale Admin Console!"