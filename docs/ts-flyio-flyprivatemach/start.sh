#!/bin/sh

# Start Tailscale daemon in background
/usr/local/bin/tailscaled --state=/var/lib/tailscale/tailscaled.state --socket=/var/run/tailscale/tailscaled.sock &

# Wait for tailscaled to start
sleep 2

# Authenticate with Tailscale using the auth key from environment variable
# Use ephemeral node for auto-cleanup when container stops
/usr/local/bin/tailscale up --auth-key=${TAILSCALE_AUTHKEY} --hostname=fly-${FLY_APP_NAME:-app} --accept-routes

# Wait for Tailscale to be ready
sleep 5

# Display Tailscale IP for debugging
echo "Tailscale IP: $(/usr/local/bin/tailscale ip -4)"
echo "Tailscale status:"
/usr/local/bin/tailscale status

# Start the Node.js application
echo "Starting Node.js Tailscale demo application..."
cd /app && node app.js