# Agentic Workflow Implementation Guide

## 🎯 Your Specific Workflow: OCI → GCP → Fly → Modal → OCI

This guide implements your exact agentic workflow with seamless Tailscale connectivity.

### **Workflow Steps:**
1. **OCI Python Backend** receives user request
2. **GCP Cloud Run** (Vertex AI) for initial LLM processing
3. **Fly.io** serverless for document chunking
4. **Modal GPU** for DeepSeek OCR parsing
5. **Stream processing**: Fly → Modal → GCP → User UI
6. **Storage**: Chat history → Neon, Files → OCI Object Storage
7. **Queueing**: Redis Streams in OCI Cache

---

## 🔧 Platform-Specific Setup

### **4. Modal GPU with Tailscale**
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
- Uses `--authkey` (not `--auth-key`)
- `--hostname=${MODAL_TASK_ID}` for unique hostnames
- SOCKS5 proxy setup with `--socks5-server=localhost:1080`
- Userspace networking with `--tun=userspace-networking`
- Ephemeral nodes (containers auto-cleanup)