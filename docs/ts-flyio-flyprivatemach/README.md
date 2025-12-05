# Tailscale-Enabled Fly.io Application

This directory contains a complete setup for deploying Docker applications on Fly.io with Tailscale networking, allowing secure access from your Tailscale network (including your Oracle VMs).

## 🚀 Quick Start

### 1. Prerequisites

- Fly.io account and `flyctl` CLI installed
- Tailscale account with auth key
- Docker (for local testing)

### 2. Generate Tailscale Auth Key

```bash
# Go to https://login.tailscale.com/admin/settings/authkeys
# Generate an ephemeral, pre-authorized auth key
# Copy the key (starts with "tskey-")
```

### 3. Deploy to Fly.io

```bash
# Clone or navigate to this directory
cd /path/to/ts-flyio-flyprivatemach

# Update fly.toml with your desired app name
# Edit fly.toml: change 'app = "tailscale-app"' to your preferred name

# Deploy the app
flyctl launch

# Set the Tailscale auth key as a secret
flyctl secrets set TAILSCALE_AUTHKEY="tskey-your-key-here"

# Deploy with the secret
flyctl deploy
```

### 4. Access Your App

Once deployed, your app will be available on your Tailscale network:

```bash
# Get your app's Tailscale IP
flyctl logs

# Or check the app directly
curl http://your-tailscale-ip:3004/
```

## 📋 Files Overview

- **`Dockerfile`** - Multi-stage build with Tailscale integration
- **`fly.toml`** - Fly.io configuration
- **`start.sh`** - Initialization script for Tailscale and app startup
- **`app.js`** - Sample Node.js application demonstrating Tailscale connectivity
- **`package.json`** - Node.js dependencies

## 🔧 Configuration

### Environment Variables

- **`TAILSCALE_AUTHKEY`** - Your Tailscale auth key (set via `flyctl secrets set`)
- **`FLY_APP_NAME`** - Automatically set by Fly.io, used for Tailscale hostname

### Customizing the App

1. **Replace the sample app**: Modify `app.js` or replace with your own application
2. **Update dependencies**: Edit `package.json` for your app's requirements
3. **Change ports**: Update `fly.toml` and `app.js` if using different ports

## 🌐 Usage Examples

### From Your Oracle VMs

```bash
# Get the Tailscale IP of your Fly.io app
# (check flyctl logs or app output)

# Test connectivity
curl http://fly-app-tailscale-ip:3004/

# Access the Oracle VM test endpoint
curl http://fly-app-tailscale-ip:3004/oracle-test

# Check Tailscale status
curl http://fly-app-tailscale-ip:3004/tailscale
```

### API Endpoints

- **`GET /`** - App info with Tailscale IP and status
- **`GET /health`** - Health check endpoint
- **`GET /tailscale`** - Detailed Tailscale network information
- **`GET /oracle-test`** - Specific test for Oracle VM connectivity
- **`GET /network-test`** - Network connectivity information

## 🔒 Security Notes

- The app is only accessible via your Tailscale network
- Uses ephemeral Tailscale nodes for automatic cleanup
- No public HTTP endpoints (unlike typical web apps)
- All communication is encrypted via Tailscale/WireGuard

## 🐛 Troubleshooting

### Check App Status
```bash
flyctl status
flyctl logs
```

### Tailscale Issues
```bash
# SSH into the running container
flyctl ssh console

# Check Tailscale status
tailscale status
tailscale ip -4
```

### Common Issues

1. **Auth key expired**: Generate a new ephemeral key and update the secret
2. **Network issues**: Check that your Oracle VMs are on the same Tailscale network
3. **Port conflicts**: Ensure port 3004 isn't used by other services

## 📚 Additional Resources

- [Tailscale on Fly.io Official Guide](https://tailscale.com/kb/1132/flydotio)
- [Fly.io Documentation](https://fly.io/docs/)
- [Tailscale Auth Keys](https://tailscale.com/kb/1085/auth-keys)

## 🔄 Updating Your App

```bash
# Make changes to your code
# Then deploy
flyctl deploy
```

The Tailscale connection will persist across deployments.