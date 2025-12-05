const express = require('express');
const { execSync } = require('child_process');
const os = require('os');

const app = express();
const PORT = process.env.PORT || 3004;

// Middleware
app.use(express.json());

// Helper function to execute shell commands safely
function execCommand(command) {
  try {
    return execSync(command, { encoding: 'utf8' }).trim();
  } catch (error) {
    return `Error: ${error.message}`;
  }
}

// Routes
app.get('/', (req, res) => {
  const tailscaleIP = execCommand('/usr/local/bin/tailscale ip -4');
  const tailscaleStatus = execCommand('/usr/local/bin/tailscale status');

  res.json({
    message: "Hello from Fly.io via Tailscale!",
    timestamp: new Date().toISOString(),
    hostname: os.hostname(),
    tailscaleIP: tailscaleIP,
    flyRegion: process.env.FLY_REGION || 'unknown',
    tailscaleStatus: tailscaleStatus.split('\n').slice(0, 5), // First 5 lines of status
    instructions: "Access this service from your Tailscale network using the IP above"
  });
});

app.get('/health', (req, res) => {
  res.json({ status: 'healthy', timestamp: new Date().toISOString() });
});

app.get('/tailscale', (req, res) => {
  const ip4 = execCommand('/usr/local/bin/tailscale ip -4');
  const ip6 = execCommand('/usr/local/bin/tailscale ip -6');
  const status = execCommand('/usr/local/bin/tailscale status');
  const ping = execCommand('/usr/local/bin/tailscale ping --self');

  res.json({
    ipv4: ip4,
    ipv6: ip6,
    status: status,
    ping: ping,
    networkInterfaces: os.networkInterfaces()
  });
});

// API endpoint to demonstrate connectivity from Oracle VMs
app.get('/oracle-test', (req, res) => {
  const clientIP = req.ip || req.connection.remoteAddress;
  const tailscaleIP = execCommand('/usr/local/bin/tailscale ip -4');

  res.json({
    message: "Successfully connected from Oracle VM to Fly.io via Tailscale!",
    clientIP: clientIP,
    serverTailscaleIP: tailscaleIP,
    timestamp: new Date().toISOString(),
    success: true
  });
});

// Network test endpoint
app.get('/network-test', (req, res) => {
  const routes = execCommand('/usr/local/bin/tailscale ip route');
  const peers = execCommand('/usr/local/bin/tailscale ping --peers');

  res.json({
    message: "Network connectivity test",
    routes: routes.split('\n'),
    peers: peers.split('\n'),
    timestamp: new Date().toISOString()
  });
});

app.listen(PORT, '0.0.0.0', () => {
  const tailscaleIP = execCommand('/usr/local/bin/tailscale ip -4');
  console.log(`🚀 Server running on:`);
  console.log(`   Local: http://localhost:${PORT}`);
  console.log(`   Tailscale: http://${tailscaleIP}:${PORT}`);
  console.log(`   Fly.io region: ${process.env.FLY_REGION || 'unknown'}`);
  console.log(`📡 Ready to accept connections from your Tailscale network!`);
});