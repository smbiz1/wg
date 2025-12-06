#!/bin/bash

# ========================================================================================
# OCI NETWORK CONFIGURATION HELPER
# ========================================================================================
#
# DESCRIPTION:
#   Displays Oracle Cloud Infrastructure (OCI) networking steps
#   Run locally to see exact console steps for Tailscale setup
#
# USAGE:
#   ./tools/oci-network-cfg.sh
#
# WHAT IT DOES:
#   - Shows OCI Console steps to open UDP 41641
#   - Displays security hardening steps
#   - Provides verification commands
#   - Includes troubleshooting tips
#
# ========================================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

log() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_header() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║         Oracle Cloud Network Configuration for Tailscale       ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_oracle_console_steps() {
    echo
    echo -e "${CYAN}📋 ORACLE CLOUD CONSOLE STEPS:${NC}"
    echo "========================================"
    echo
    echo "1. Navigate to Oracle Cloud Console:"
    echo "   https://cloud.oracle.com/networking/vcns"
    echo
    echo "2. Select your Virtual Cloud Network (VCN)"
    echo
    echo "3. Click 'Security Lists' in the left sidebar"
    echo
    echo "4. Select your security list (usually 'Default')"
    echo
    echo "5. Click 'Add Ingress Rules'"
    echo
    echo "6. Add this rule:"
    echo "   ┌─────────────────────────────────────┐"
    echo "   │ Source Type:    CIDR                 │"
    echo "   │ Source CIDR:    0.0.0.0/0            │"
    echo "   │ IP Protocol:    UDP                  │"
    echo "   │ Destination Port Range: 41641       │"
    echo "   │ Is Stateless:   Yes                  │"
    echo "   └─────────────────────────────────────┘"
    echo
    echo "7. Click 'Add Ingress Rules' to save"
    echo
}

print_security_hardening() {
    echo
    echo -e "${CYAN}🔒 SECURITY HARDENING (AFTER TAILSCALE WORKS):${NC}"
    echo "=================================================="
    echo
    echo "1. Remove public SSH access:"
    echo "   - Go to Security Lists → Default"
    echo "   - Delete the 0.0.0.0/0 TCP 22 rule"
    echo
    echo "2. Add Tailscale SSH access (optional):"
    echo "   ┌─────────────────────────────────────┐"
    echo "   │ Source Type:    CIDR                 │"
    echo "   │ Source CIDR:    100.64.0.0/10        │"
    echo "   │ IP Protocol:    TCP                  │"
    echo "   │ Destination Port Range: 22           │"
    echo "   └─────────────────────────────────────┘"
    echo
}

print_verification_commands() {
    echo
    echo -e "${CYAN}🧪 VERIFICATION COMMANDS:${NC}"
    echo "==========================="
    echo
    echo "Run these on your Oracle VM after setup:"
    echo
    echo "# Check Tailscale status"
    echo "tailscale status"
    echo
    echo "# Test UDP connectivity (from another machine)"
    echo "tailscale ping oracle-vm-hostname"
    echo
    echo "# Check network interfaces"
    echo "ip addr show"
    echo "ip route show"
    echo
    echo "# Test Oracle DNS"
    echo "dig @169.254.169.254 instance.subnet01234567.vcn01234567.oraclevcn.com"
    echo
}

print_troubleshooting() {
    echo
    echo -e "${CYAN}🔧 TROUBLESHOOTING:${NC}"
    echo "==================="
    echo
    echo "If connectivity fails:"
    echo
    echo "1. Check security list has UDP 41641 rule"
    echo "2. Verify Tailscale routes are approved in admin console"
    echo "3. Check iptables: iptables -L | grep 41641"
    echo "4. Restart Tailscale: systemctl restart tailscaled"
    echo "5. Check logs: journalctl -u tailscaled -f"
    echo
}

main() {
    print_header

    echo "This script provides Oracle Cloud networking configuration guidance"
    echo "for Tailscale subnet router setup."
    echo

    print_oracle_console_steps
    print_security_hardening
    print_verification_commands
    print_troubleshooting

    echo
    echo -e "${GREEN}🚀 Ready to configure your Oracle Cloud network for Tailscale!${NC}"
    echo
}

# Run main function
main "$@"