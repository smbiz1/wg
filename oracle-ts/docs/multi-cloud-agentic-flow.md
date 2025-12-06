# Multi-Cloud Agentic Workflow with Tailscale

## Overview

This document outlines how to implement seamless, secure multi-cloud agentic workflows using OCI as the central subnet gateway. The example workflow demonstrates complex parallel processing across GCP, Fly.io, Modal, and Oracle Cloud services.

## 🎯 Example Agentic Workflow Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   User Chat UI  │────▶│  OCI VM Backend  │────▶│  GCP Cloud Run  │
│   (Next.js)     │     │  (Python Agent)  │     │  (Vertex AI)    │
└─────────────────┘     └──────────────────┘     └─────────────────┘
         ▲                       │                        │
         │              ┌────────▼────────┐      ┌────────▼────────┐
         │              │   Oracle Cache  │      │   File Upload   │
         │              │   (Redis)       │      │   → OCI Object  │
         │              └─────────────────┘      └─────────────────┘
         │
┌────────▼────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Fly.io Server  │◀────│   Parallel Flow  │────▶│   Modal GPU     │
│  (Chunking)     │     │   Coordinator    │     │   (OCR/Parse)   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
         │                        │                        │
         ▼                        ▼                        ▼
┌────────▼────────┐     ┌────────▼────────┐     ┌────────▼────────┐
│   Stream Chunks │────▶│   Process OCR   │────▶│   LLM Answer     │
│   (Fly→Modal)   │     │   (Modal)       │     │   (GCP→User)     │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                        │
                                               ┌────────▼────────┐
                                               │   Chat History   │
                                               │   → Neon DB      │
                                               └─────────────────┘
```

## 🔐 Tailscale Connectivity Matrix

### ✅ **FULLY TAILSCALE-ENCRYPTED** (Direct Peer-to-Peer)

| Service | Platform | Tailscale Status | Connection Method | IP Persistence |
|---------|----------|------------------|-------------------|----------------|
| **Main Backend** | OCI VM | ✅ **Subnet Gateway** | `tailscale up --advertise-routes=10.0.0.0/24` | Persistent |
| **Cloud Run API** | GCP | ✅ Can run Tailscale | Direct tailnet connection | **Ephemeral**¹ |
| **Serverless VMs** | Fly.io | ✅ Can run Tailscale | Direct tailnet connection | **Ephemeral**² |
| **GPU Instances** | Modal | ✅ Can run Tailscale | Direct tailnet connection | **Ephemeral**³ |
| **File Storage** | OCI Object Storage | ✅ Internal routing | Via OCI subnet gateway | Internal |
| **Redis Cache** | OCI Cache | ✅ Internal routing | Via OCI subnet gateway | Internal |

**¹ GCP**: Static egress IPs possible via Cloud NAT ($cost)  
**² Fly.io**: Static egress IPs available ($0.005/hour)  
**³ Modal**: Static IPs via Proxies feature

### ⚠️ **EXTERNAL SERVICES** (Not on Tailscale)

| Service | Platform | Connection Method | Security Approach |
|---------|----------|-------------------|-------------------|
| **Neon PostgreSQL** | External SaaS | Direct HTTPS | Cloudflare Tunnel or VPN |
| **Vertex AI APIs** | GCP | HTTPS API calls | Service account auth |
| **User UI** | Internet | HTTPS via Cloudflare | Cloudflare protection |

## 📋 **Critical Architecture Decision: Subnet Router Placement**

### **Why OCI VM is the ONLY Subnet Router**
Serverless platforms (GCP Cloud Run, Fly.io, Modal) **cannot** act as subnet routers because:

1. **Ephemeral Nature**: Functions spin up/down constantly - subnet routers need 24/7 persistence
2. **Network Restrictions**: Serverless platforms don't allow IP forwarding or routing table modifications
3. **Stateless Design**: Cannot maintain persistent network state required for routing

**✅ Only OCI VMs can be subnet routers** - they provide persistent infrastructure with full network control.

---

## 🛠️ Implementation Steps

### **STEP 1: OCI Subnet Gateway Setup**
```bash
# On OCI VM - Advertise entire Oracle subnet
sudo tailscale up --advertise-routes=10.0.0.0/24 \
  --hostname=oci-main-gateway \
  --accept-dns=false \
  --accept-routes

# Approve routes in Tailscale Admin Console
# All tailnet devices can now reach 10.0.0.0/24 via OCI VM
```

### **STEP 2: GCP Cloud Run with Tailscale**
```dockerfile
# gcp-cloud-run/Dockerfile
FROM python:3.11-slim

# Install Tailscale
RUN apt-get update && apt-get install -y curl && \
    curl -fsSL https://tailscale.com/install.sh | sh && \
    rm -rf /var/lib/apt/lists/*

# Copy your Vertex AI service
COPY requirements.txt vertex_service.py ./
RUN pip install -r requirements.txt

# Environment variables needed:
# TAILSCALE_AUTH_KEY - From Tailscale
# GOOGLE_APPLICATION_CREDENTIALS - GCP service account

CMD tailscale up --authkey=$TAILSCALE_AUTH_KEY --hostname=gcp-vertex-service && \
    python vertex_service.py
```

**Important Notes:**
- **Ephemeral outbound IPs**: GCP Cloud Run changes IPs frequently
- **Static IPs possible**: Use Cloud NAT + VPC (additional cost ~$0.045/hour)
- **Tailscale provides connectivity**: Works regardless of IP changes
- **Direct encrypted connection**: From OCI backend to Cloud Run service

### **STEP 3: Fly.io Serverless with Tailscale**
```toml
# fly-chunking/fly.toml
app = "chunking-service"
primary_region = "iad"

[build]
  dockerfile = "Dockerfile"

[env]
  TAILSCALE_AUTH_KEY = "your-auth-key"
```

```dockerfile
# fly-chunking/Dockerfile
FROM node:18-alpine

# Install Tailscale
RUN apk add curl && curl -fsSL https://tailscale.com/install.sh | sh

# Copy chunking service
COPY package.json chunk-service.js ./
RUN npm install

CMD tailscale up --authkey=$TAILSCALE_AUTH_KEY --hostname=fly-chunk-service && \
    npm start
```

**Important Notes:**
- **Ephemeral outbound IPs**: Fly.io changes IPs frequently by default
- **Static egress IPs available**: $0.005/hour per machine (tie to specific VM)
- **Tailscale provides stable connectivity**: Regardless of IP changes
- **Direct encrypted streaming**: To Modal and back to OCI

### **STEP 4: Modal GPU with Tailscale**
```python
# modal-ocr/modal_ocr.py
import modal
import httpx

# Use Modal's recommended Tailscale setup with custom entrypoint
image = (
    modal.Image.debian_slim(python_version="3.11")
    .apt_install("curl")
    .run_commands("curl -fsSL https://tailscale.com/install.sh | sh")
    .pip_install("transformers", "torch", "pillow", "fastapi", "uvicorn")
    .add_local_file("modal-entrypoint.sh", "/root/entrypoint.sh", copy=True)
    .run_commands("chmod a+x /root/entrypoint.sh")
    .entrypoint(["/root/entrypoint.sh"])
)

app = modal.App("deepseek-ocr", image=image)

# Configure SOCKS5 proxy for all outbound connections
with image.imports():
    import socket
    import socks

    if not modal.is_local():
        socks.set_default_proxy(socks.SOCKS5, "localhost", 1080)
        socket.socket = socks.socksocket

@app.function(
    secrets=[
        modal.Secret.from_name("tailscale-auth", required_keys=["TAILSCALE_AUTHKEY"]),
        modal.Secret.from_dict({
            "ALL_PROXY": "socks5://localhost:1080/",
            "HTTP_PROXY": "http://localhost:1080/",
            "HTTPS_PROXY": "http://localhost:1080/",
        })
    ],
    gpu="A100",
    timeout=3600
)
@modal.web_endpoint(method="POST")
def process_ocr(request: dict):
    """DeepSeek OCR processing on GPU with Modal's Tailscale setup"""

    # Tailscale is already connected via entrypoint script
    # MODAL_TASK_ID provides unique hostname per execution
    # SOCKS5 proxy handles all outbound connections

    # Your OCR logic here
    image_data = request["image"]
    extracted_text = run_deepseek_ocr(image_data)

    # Stream results back to GCP via SOCKS5 proxy
    async with httpx.AsyncClient() as client:
        await client.post(
            f"http://{request['gcp_ip']}:8080/process-extracted",
            json={"extracted_text": extracted_text}
        )

    return {"status": "processed", "text": extracted_text}

def run_deepseek_ocr(image_data):
    # Implement DeepSeek OCR logic
    # This would use transformers/torch for actual OCR
    return "Extracted text from image"
```

**Key Modal Implementation Details:**
- Uses `--authkey` (not `--auth-key`) in entrypoint script
- `--hostname=${MODAL_TASK_ID}` for unique hostnames per execution
- SOCKS5 proxy setup with `--socks5-server=localhost:1080`
- Userspace networking with `--tun=userspace-networking`
- Ephemeral nodes (containers auto-cleanup when function ends)

### **STEP 5: External Services Integration**

#### **Neon PostgreSQL (External)**
**Option A: Cloudflare Tunnel (Recommended)**
```bash
# On OCI VM - Create tunnel to Neon
cloudflared tunnel create neon-db-tunnel
cloudflared tunnel route dns neon-db-tunnel yourdomain.com

# Connect via tunnel
psycopg2.connect("postgresql://user:pass@yourdomain.com:5432/db")
```

**Option B: VPN Gateway**
```bash
# OCI VM as OpenVPN/WireGuard server
# Neon connections route through OCI VPN
```

#### **Vertex AI APIs**
```python
# Direct HTTPS API calls (already secure)
import vertexai

vertexai.init(project="your-project")
model = GenerativeModel("gemini-pro")
response = model.generate_content("Your prompt")
```

## 🔄 Data Flow Security

### **Encrypted Streams (Tailscale)**
```python
# OCI Backend → Fly.io Chunking (Encrypted)
import httpx

async def stream_chunks_to_fly(chunks, fly_tailscale_ip):
    async with httpx.AsyncClient() as client:
        for chunk in chunks:
            await client.post(
                f"http://{fly_tailscale_ip}:8080/process",
                json=chunk,
                # Fully encrypted via Tailscale
            )
```

### **Parallel Processing Coordination**
```python
# OCI Backend coordinates all services
async def run_agentic_workflow(file_data):
    # Step 1: Store file in OCI Object Storage (internal)
    file_id = await store_in_oci_object_storage(file_data)

    # Step 2: Queue in OCI Redis (internal)
    await redis.lpush("processing_queue", file_id)

    # Step 3: Trigger GCP Cloud Run (Tailscale encrypted)
    vertex_response = await call_gcp_cloud_run(file_id)

    # Step 4: Parallel processing - Fly.io + Modal
    chunk_task = process_with_fly(file_id)      # Tailscale
    ocr_task = process_with_modal(file_id)      # Tailscale

    results = await asyncio.gather(chunk_task, ocr_task)

    # Step 5: Combine results via GCP (API calls)
    final_answer = await combine_in_vertex_ai(results)

    # Step 6: Store chat history (external via tunnel)
    await store_in_neon(final_answer)

    return final_answer
```

## 🌐 Cloudflare Integration Points

### **Public User Interface**
```
User Browser → Cloudflare → OCI VM (Next.js App)
```

**Setup:**
```bash
# On OCI VM
cloudflared tunnel --url=http://localhost:3000 --no-autoupdate

# Cloudflare DNS: yourapp.com → tunnel
```

### **API Protection**
```
External API Calls → Cloudflare → OCI VM Backend
```

**Benefits:**
- DDoS protection
- SSL termination
- Rate limiting
- Before Tailscale authentication

## 🔥 Firewall Configuration

### **OCI Security Lists**
```bash
# Allow Tailscale UDP (required)
# Source: 0.0.0.0/0, Protocol: UDP, Port: 41641

# Allow Cloudflare (if using tunnel)
# Source: cloudflare_ips, Protocol: TCP, Port: 443

# Deny all other inbound (default)
```

### **Tailscale ACLs (Recommended)**
```json
{
  "acls": [
    {
      "action": "accept",
      "users": ["oci-backend"],
      "ports": ["gcp-cloud-run:8080", "fly-service:3000", "modal-gpu:5000"]
    }
  ]
}
```

## 🚀 Performance Optimization

### **Connection Prioritization**
1. **Tailscale Direct**: OCI ↔ GCP/Fly/Modal (lowest latency)
2. **Internal OCI**: Object Storage, Redis Cache (no network hops)
3. **Cloudflare Tunnel**: Neon DB (optimized routing)
4. **HTTPS APIs**: Vertex AI (global CDN)

### **Load Balancing**
```bash
# Multiple OCI VMs as gateways
tailscale up --advertise-routes=10.0.0.0/24 --hostname=oci-gateway-01
tailscale up --advertise-routes=10.0.0.0/24 --hostname=oci-gateway-02

# Tailscale automatically load balances
```

### **Caching Strategy**
- **OCI Cache (Redis)**: Session state, queue management
- **Cloudflare CDN**: Static assets, API responses
- **Browser Cache**: Chat UI assets

## 🛡️ Security Architecture

### **Defense in Depth**
1. **Cloudflare**: External DDoS/rate limiting
2. **OCI Security Lists**: Network-level filtering
3. **Tailscale ACLs**: Application-level access control
4. **Service Authentication**: API keys, service accounts
5. **End-to-End Encryption**: Tailscale WireGuard + HTTPS

### **Zero Trust Principles**
- **Never trust, always verify**: Every service authenticates
- **Least privilege**: Minimal required access
- **Micro-segmentation**: Isolated service communication
- **Continuous monitoring**: Log all cross-service calls

## 📊 Monitoring & Observability

### **Tailscale Network Insights**
```bash
# Monitor all connections
tailscale status
tailscale ping gcp-cloud-run
tailscale ping fly-service
tailscale ping modal-gpu
```

### **Distributed Tracing**
```python
# Add tracing headers across services
import opentelemetry as otel

tracer = otel.trace.get_tracer(__name__)

with tracer.start_as_current_span("agentic-workflow"):
    with tracer.start_as_current_span("call-gcp"):
        result = await call_gcp_service()
    with tracer.start_as_current_span("call-fly"):
        result = await call_fly_service()
```

### **Health Checks**
```bash
# Cross-service health verification
curl http://gcp-tailscale-ip:8080/health
curl http://fly-tailscale-ip:3000/health
curl http://modal-tailscale-ip:5000/health
```

## 🔧 Troubleshooting Complex Flows

### **Connection Issues**
```bash
# Test Tailscale mesh
tailscale ping gcp-service
tailscale ping fly-service

# Check routes
tailscale ip route

# Debug OCI subnet access
ping 10.0.0.100  # OCI VM
ping 10.0.0.5    # Other OCI service
```

### **Performance Bottlenecks**
```bash
# Monitor latency
tailscale ping --peer gcp-service --count=10

# Check network paths
traceroute gcp-tailscale-ip

# OCI internal latency
ping oci-redis.internal
```

### **Debugging Data Flows**
```bash
# Enable Tailscale debug logging
tailscale debug --enable

# Monitor Redis streams
redis-cli XREAD STREAMS processing_queue 0

# Check Cloudflare tunnel status
cloudflared tunnel list
```

## 🎯 Key Success Factors

1. **Single Source of Truth**: OCI subnet gateway as central orchestrator
2. **Consistent Encryption**: Tailscale for 90%+ of connections
3. **Hybrid External Access**: Cloudflare tunnels for SaaS services
4. **Monitoring Coverage**: End-to-end observability
5. **Failure Resilience**: Graceful degradation and retries

This architecture enables complex agentic workflows while maintaining security, performance, and observability across your entire multi-cloud stack.