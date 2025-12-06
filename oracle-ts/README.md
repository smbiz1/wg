# OCI VM Tailscale Subnet Router Setup

Automated setup for Oracle Cloud Infrastructure (OCI) VMs as Tailscale subnet routers.

## 📂 Directory Structure

```
oracle-ts/
├── scripts/          # 🏃 Production setup scripts (run on OCI VMs)
│   ├── setup-oci-vm.sh    # Main automated setup
│   ├── ts-init.sh         # Tailscale reconfiguration
│   └── cfg.sh            # Environment configuration
├── tools/            # 🔧 Helper tools (run locally)
│   ├── oci-network-cfg.sh # OCI console networking guide
│   └── test-conn.sh       # Connectivity testing
├── docs/             # 📖 Documentation
│   ├── README.md          # This file
│   ├── setup-steps.md     # Step-by-step setup guide
│   └── faq.md             # Firewall & Cloudflare FAQ
└── ...
```

## 🚀 Quick Start

1. **Configure OCI networking:** `./tools/oci-network-cfg.sh`
2. **Deploy scripts to OCI VM:** `scp scripts/* oci@vm-ip:~/`
3. **Run setup:** `sudo bash setup-oci-vm.sh`
4. **Approve routes:** Tailscale Admin Console
5. **Test:** `./tools/test-conn.sh`

## 📖 Full Documentation

- **[Setup Steps](docs/setup-steps.md)** - Complete step-by-step guide
- **[FAQ](docs/faq.md)** - Firewall, Cloudflare, troubleshooting
- **[Multi-Cloud Agentic Flows](docs/multi-cloud-agentic-flow.md)** - Complex workflow architecture
- **[Agentic Workflow Implementation](docs/agentic-workflow-implementation.md)** - Complete code for OCI→GCP→Fly→Modal flows

## 🏗️ Architecture

```
Internet → Cloudflare → OCI VM (Frontend + API + Subnet Router)
                           │
                           ├── Next.js Frontend (Public)
                           ├── Python Backend (Private)
                           ├── Oracle Object Storage (Internal)
                           └── Oracle Cache/Redis (Internal)
                           │
Serverless Layer ←─────────┼── GCP Cloud Run (Tailscale)
                           │
                           ├── Fly.io Functions (Tailscale)
                           └── Modal GPU (Tailscale)
```

**Key Points:**
- **OCI VM is the ONLY subnet router** (serverless platforms cannot route subnets)
- **Serverless IPs are ephemeral** (use Tailscale for persistent connectivity)
- **Private links available** for Vertex AI (PSC) and OCI Gen AI (Service Gateway)
- **Cloudflare tunnel OR Hyperdrive** for Neon database access