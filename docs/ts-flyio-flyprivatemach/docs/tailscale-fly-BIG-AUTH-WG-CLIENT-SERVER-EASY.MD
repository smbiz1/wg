# FLY.io MCP Client/Server Auth – Quick Reference

## 1. Restrict Access to Private Network

When setting up your machine, always pass the private IP flags to prevent access outside your private WireGuard (WG) network.  
See: <https://fly.io/docs/mcp/access-control/flycast/>

---

## 2. HTTP Authorization

Reference: <https://fly.io/docs/mcp/access-control/http-authorization/>

### Transports & Authentication

- **HTTP Streaming:**  
  Uses OAuth 2.1 for authentication.  
  *Note: As of Spring 2025, not widely implemented. Both MCP client and server must support it.*

- **SSE Transport:**  
  Deprecated. Previously allowed bearer token authentication via MCP inspector, but not recommended for new deployments.

- **Stdio Transports:**  
  No built-in authentication.  
  Authentication is handled by `fly mcp proxy` and `fly mcp wrap`.  
  These are designed for single-user-at-a-time servers, so OAuth is overkill.  
  Instead, use either **Basic** or **Bearer** HTTP Authorization.

---

## 3. Setting Secrets

### Basic Authentication

Set two secrets in your application:
