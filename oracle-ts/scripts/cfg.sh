# ========================================================================================
# OCI VM TAILSCALE CONFIGURATION
# ========================================================================================
#
# DESCRIPTION:
#   Environment variables for OCI VM Tailscale setup
#   Source this file: source cfg.sh
#   Or copy values to your environment
#
# REQUIRED:
#   TAILSCALE_AUTH_KEY - Generate at: https://login.tailscale.com/admin/settings/authkeys
#
# OPTIONAL:
#   TAILSCALE_HOSTNAME - VM name in Tailscale network
#   OCI_SUBNET        - Your OCI subnet CIDR (auto-detected if not set)
#   ACCEPT_ROUTES     - Accept routes from peers (default: true)
#
# ========================================================================================

# REQUIRED: Tailscale auth key (ephemeral, pre-authorized)
export TAILSCALE_AUTH_KEY="tskey-your-key-here"

# OPTIONAL: Custom hostname for this VM in Tailscale
export TAILSCALE_HOSTNAME="oci-main-server"

# OPTIONAL: Your OCI subnet CIDR (auto-detected if not set)
export OCI_SUBNET="10.0.0.0/24"

# OPTIONAL: Accept routes from other Tailscale nodes (default: true)
export ACCEPT_ROUTES="true"

# OPTIONAL: Additional subnets to advertise (comma-separated)
# export ADDITIONAL_SUBNETS="192.168.1.0/24,172.16.0.0/16"