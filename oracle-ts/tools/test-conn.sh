#!/bin/bash

# ========================================================================================
# OCI VM TAILSCALE CONNECTIVITY TEST
# ========================================================================================
#
# DESCRIPTION:
#   Comprehensive connectivity testing for OCI VM Tailscale setup
#   Verifies subnet router functionality and network connectivity
#
# USAGE:
#   ./tools/test-conn.sh
#
# TESTS PERFORMED:
#   - Tailscale daemon status and authentication
#   - Route advertisement verification
#   - iptables configuration check
#   - OCI DNS accessibility
#   - Peer-to-peer connectivity
#   - Network diagnostics
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

# Check if Tailscale is running
check_tailscale() {
    log "Checking Tailscale status..."

    if ! command -v tailscale &> /dev/null; then
        error "Tailscale not installed"
        return 1
    fi

    if ! tailscale status &>/dev/null; then
        error "Tailscale not running or not authenticated"
        return 1
    fi

    success "Tailscale is running"
    echo "IP: $(tailscale ip -4)"
    echo "Hostname: $(tailscale status | grep -E "^[a-zA-Z]" | head -1 | awk '{print $1}')"
}

# Check advertised routes
check_routes() {
    log "Checking advertised routes..."

    local routes
    routes=$(tailscale status | grep "routes" | head -1)

    if [[ -z "$routes" ]]; then
        warning "No routes advertised (this is normal if approval is pending)"
    else
        success "Routes advertised: $routes"
    fi
}

# Check iptables configuration
check_iptables() {
    log "Checking iptables configuration..."

    if iptables -L | grep -q "41641"; then
        success "UDP 41641 rule found in iptables"
    else
        warning "UDP 41641 rule not found in iptables"
        echo "Run: sudo iptables -I INPUT -p udp --dport 41641 -j ACCEPT"
    fi
}

# Test Oracle DNS
test_oracle_dns() {
    log "Testing Oracle DNS..."

    # Try to resolve an Oracle metadata hostname
    if dig @169.254.169.254 instance.subnet01234567.vcn01234567.oraclevcn.com +short &>/dev/null; then
        success "Oracle DNS is accessible"
    else
        warning "Oracle DNS test failed (this may be normal)"
    fi
}

# Test connectivity to other nodes (if any are specified)
test_peers() {
    log "Testing connectivity to other tailnet nodes..."

    local peers
    peers=$(tailscale status | grep -E "^[a-zA-Z0-9].*\.[a-zA-Z]" | awk '{print $1}' | head -5)

    if [[ -z "$peers" ]]; then
        warning "No other tailnet nodes found to test"
        return
    fi

    echo "Found peers: $peers"

    for peer in $peers; do
        log "Testing connection to $peer..."
        if timeout 10 tailscale ping "$peer" &>/dev/null; then
            success "Can reach $peer"
        else
            warning "Cannot reach $peer (may be normal if offline)"
        fi
    done
}

# Network diagnostics
network_diagnostics() {
    log "Network diagnostics..."

    echo "Network interfaces:"
    ip addr show | grep -E "inet |link/" | head -10

    echo -e "\nRoutes:"
    ip route show | head -10

    echo -e "\nTailscale routes:"
    tailscale ip route 2>/dev/null || echo "No routes available"
}

# Main execution
main() {
    echo "🔍 Oracle VM Tailscale Connectivity Test"
    echo "========================================="

    check_tailscale
    echo

    check_routes
    echo

    check_iptables
    echo

    test_oracle_dns
    echo

    test_peers
    echo

    network_diagnostics
    echo

    success "Connectivity test completed!"
    echo
    echo "💡 Tips:"
    echo "   - If routes aren't working, check Tailscale Admin Console for approvals"
    echo "   - If UDP 41641 is blocked, add it to Oracle Cloud security list"
    echo "   - For DNS issues, ensure --accept-dns=false was used"
    echo "   - Check journalctl -u tailscaled -f for detailed logs"
}

# Run main function
main "$@"