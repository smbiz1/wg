#!/bin/sh

# Modal Tailscale entrypoint script
# Based on Modal's official Tailscale integration
# https://modal.com/docs/guide/custom-container#entrypoint

set -e

# Start tailscaled with userspace networking and SOCKS5 proxy
tailscaled --tun=userspace-networking --socks5-server=localhost:1080 --outbound-http-proxy-listen=localhost:1080 &

# Connect to Tailscale using ephemeral key
# MODAL_TASK_ID provides unique hostname per function execution
tailscale up --authkey=${TAILSCALE_AUTHKEY} --hostname=${MODAL_TASK_ID:-modal-gpu}

# Wait for connection and proxy setup
retry_count=0
max_retries=5

while [ $retry_count -lt $max_retries ]; do
    # Test SOCKS5 proxy connectivity
    if curl -x socks5://localhost:1080 -o /dev/null -L -s -w '%{http_code}' https://www.google.com | grep -q "200"; then
        echo "✅ Tailscale connected, SOCKS5 proxy ready"
        exec "$@"  # Execute the original command
        exit 0
    else
        echo "⏳ Attempt $((retry_count+1))/$max_retries: Waiting for Tailscale..."
        retry_count=$((retry_count+1))
        sleep 1
    fi
done

echo "❌ Failed to connect to Tailscale after $max_retries attempts"
exit 1