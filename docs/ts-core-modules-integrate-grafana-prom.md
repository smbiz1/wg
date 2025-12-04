- Core: The main product is the private, identity‑aware mesh VPN (“tailnet”) built on WireGuard with direct peer‑to‑peer paths.

- Connectivity: Attach clouds and LANs with subnet routers, site‑to‑site, exit nodes, app connectors, and fallbacks (DERP/Peer Relays).

- Enhancements: Services, Serve, Funnel, Taildrop, Tailscale SSH, tsnet, and App Capabilities extend what you can expose and how you authorize it.

- Policy & Admin: Visual Policy Editor, ACLs/Grants, auto‑approvers, device posture, Tailnet Lock, HTTPS certs, logging/streaming.

- Integrations & Ops: Grafana Cloud PDC, Kubernetes operator, containers, tsrecorder; guidance for MCP/DCR and securing AI servers.

Core Platform: The tailnet you build everything on

Tailscale’s core is an identity‑aware mesh VPN that connects devices directly over WireGuard. Each device gets a stable Tailscale IP (and optionally IPv6), and connections are enforced locally by your policy. The control plane coordinates keys and discovery; data is end‑to‑end encrypted. MagicDNS assigns human‑readable names in your tailnet, so you can reach peers by name instead of IP. HTTPS certificates can be auto‑provisioned for tailnet URLs to serve applications securely.

What this gives you:

- Direct paths across NATs with minimal config; automatic fallback when direct isn’t possible.

- Device/user identity woven into the network: access is enforced at the edge, not centralized choke points.

- Simple addressing via MagicDNS; TLS everywhere via tailnet certificates.

Resources: What is Tailscale? (https://tailscale.com/kb/1151/what-is-tailscale) • Features overview (https://tailscale.com/features) • MagicDNS docs (https://tailscale.com/kb/1081/magicdns)

Connect it to clouds and networks

These primitives plug the tailnet into the rest of your infrastructure and shape traffic flows.

- Subnet routers: Advertise on‑prem/VPC subnets inside the tailnet, including HA router options and overlap resolution (4via6).
Resources: Features (https://tailscale.com/features)

- Site‑to‑site networking: Stitch networks (datacenter, VPC, branch) without renumbering, leaning on the mesh for reachability and policy.
Resources: Features (https://tailscale.com/features)

- Exit nodes: Route all outbound traffic via a designated egress point when you need privacy‑VPN behavior or location‑specific egress.
Resources: Features (https://tailscale.com/features)

- App connectors: Domain‑specific egress (think “split DNS” for routing). Useful when SaaS apps require traffic from a known/whitelisted IP or for geo‑specific tests.
Resources: Tailscale Explained: App connectors (https://tailscale.com/kb/1281/app-connectors)

- Kubernetes, containers, and proxies: The container image and operator add cluster primitives (e.g., Kubernetes proxy, ProxyGroup HA), plus tsrecorder for session capture.
Resources: Features (https://tailscale.com/features)

- Fallback relay choices:

 ▫ DERP: Managed relay fleet prioritizing reliability/NAT traversal when direct paths fail.

 ▫ Peer Relays: Customer‑managed, single‑UDP relay nodes that deliver near‑direct throughput behind strict NAT/firewalls; prefer direct → peer relay → DERP.
Resources: Introducing Peer Relays (https://tailscale.com/blog/peer-relays-beta#how-it-works)

Add‑on modules and enhancements

These features sit atop the core to make services addressable, exposable, and identity‑aware.

- Services (TailVIPs + MagicDNS for logical resources): Define first‑class services with stable IPv4/IPv6 TailVIPs and names, map endpoints on one or many hosts, get built‑in HA and regional steering, and gate access via Grants. Great when you can’t install Tailscale directly on the workload.
Resources: Introducing Tailscale Services (https://tailscale.com/blog/services-beta#your-services-on-tailscale) • Architecture (https://tailscale.com/blog/services-beta#architecture)

- Serve: Private reverse proxy for your tailnet with identity headers (user login/name/profile pic) and optional App Capabilities forwarded as HTTP headers; enforces ACLs; ideal for internal UIs/APIs.
Resources: Tailscale Serve · Docs (https://tailscale.com/kb/1312/serve)

- Funnel: Public sharing of a single local resource over TLS via Funnel relay servers and a unique URL; hides device IPs; relays cannot decrypt (end‑to‑end). Guardrails via required MagicDNS, certs, node attribute, and limited ports.
Resources: Tailscale Funnel docs (https://tailscale.com/kb/1223/funnel)

- Taildrop: Encrypted, peer‑to‑peer file transfers between your own devices; resumes large transfers; cross‑platform; alpha.
Resources: Taildrop · Docs (https://tailscale.com/kb/1106/taildrop)

- Tailscale SSH: Brokered SSH that uses your tailnet identity and ACLs; no user key management; fine‑grained SSH access rules and “check mode” for recent SSO re‑auth on sensitive actions.
Resources: Tailscale SSH blog (https://tailscale.com/blog/tailscale-ssh) • Tailscale SSH docs (https://tailscale.com/kb/1193/tailscale-ssh)

- tsnet (embedded Tailscale): Embed the client in Go apps to run services with native tailnet identity/routing without external sidecars.
Resources: Features (https://tailscale.com/features)

- App Capabilities via headers: Serve can forward Grants‑based capability JSON in the ‎⁠Tailscale-App-Capabilities⁠ header, across any language, for least‑privilege app behavior without bespoke SDKs.
Resources: App capabilities announcement (https://tailscale.com/blog/app-capabilities#how-it-works)

Policy setup and management

Identity and authorization live in the policy engine; these tools make it usable at scale.

- ACLs + Grants: Express “who can reach which resources” (devices/tags/services/TailVIPs), on which IP/ports, and (optionally) “what actions” via App Capabilities. SSH access rules add per‑user/role constraints for SSH.
Resources: Tailnet policy file syntax (https://tailscale.com/kb/1337/policy-syntax) • Grant examples (https://tailscale.com/kb/1458/grant-samples)

- Visual Policy Editor (GA): GUI over your HuJSON policy file, with live previews and auto‑complete; you can toggle back to code at any time; Terraform and REST API stay in play.
Resources: Visual policy editor GA (https://tailscale.com/kb/1550/visual-editor?utm_source=blog&utm_medium=content&utm_campaign=fall-update-2025)

- Auto‑approvers and node attributes: Automate host approvals into Services by tag/IP; set ‎⁠nodeAttrs⁠ like ‎⁠funnel⁠ to opt‑in specific features.
Resources: Funnel docs (nodeAttrs) (https://tailscale.com/kb/1223/funnel)

- Device posture & Tailnet Lock: Enforce device attributes and trusted‑key verification; pair with SSO and SCIM to keep identities current.
Resources: Features (https://tailscale.com/features)

Admin, identity, logging, and integrations

Operational glue and ecosystem hooks that round out the platform.

- HTTPS certificates: Auto‑provision TLS for tailnet URLs; used by Serve/Funnel; mind Let’s Encrypt rate limits when rotating frequently.
Resources: HTTPS certificates docs (https://tailscale.com/kb/1153/enabling-https) • Funnel docs (https://tailscale.com/kb/1223/funnel)

- Logging/streaming/events: Configuration Audit Logs, Network Flow Logs, SSH session recording, Log streaming to SIEMs, and Webhooks for near‑real‑time signals.
Resources: Features (https://tailscale.com/features)

- Client hardening: Encrypted state at rest in 1.86.x; recommended exit node auto‑selection and cross‑platform fixes.
Resources: Monthly update (Aug 2025) (https://tailscale.com/blog/august-25-product-update#visual-policy-editor)

- Grafana Cloud Private Data Source Connect (PDC): Grafana joins as an ephemeral node (tag‑scoped) to query private data sources via MagicDNS; no exposed ports, proxies, or SSH tunnels.
Resources: Grafana Cloud integration (https://grafana.com/products/cloud/?pg=blog&plcmt=body-txt)

- MCP/DCR and tsidp: If your IdP lacks Dynamic Client Registration, host a DCR‑capable auth server behind Tailscale and limit who can reach it via Grants; for agent/gateway patterns, plan for STS token exchange (support landing in tsidp).
Resources: MCP + DCR + tsidp (https://tailscale.com/blog/dynamic-client-registration-dcr-for-mcp-ai#how-does-tailscale-make-a-more-secure-dcr-easy)

- Security insight: Avoid exposing local AI servers; keep LLM/MCP endpoints private‑by‑default behind the tailnet; enforce identity‑based authorization to prevent misuse and cost blowups.
Resources: Exposed AI servers (https://tailscale.com/blog/AI-endpoints-on-public-web#there-has-to-be-a-better-way-and-there-is)

Resources

- Services: Introducing Tailscale Services (https://tailscale.com/blog/services-beta#your-services-on-tailscale) • Architecture (https://tailscale.com/blog/services-beta#architecture)

- Peer Relays: Introducing Tailscale Peer Relays (https://tailscale.com/blog/peer-relays-beta#how-it-works)

- Serve: Tailscale Serve · Docs (https://tailscale.com/kb/1312/serve)

- Funnel: Tailscale Funnel · Docs (https://tailscale.com/kb/1223/funnel)

- MagicDNS: MagicDNS · Docs (https://tailscale.com/kb/1081/magicdns)

- Taildrop: Taildrop · Docs (https://tailscale.com/kb/1106/taildrop)

- Tailscale SSH: Blog (https://tailscale.com/blog/tailscale-ssh) • Docs (https://tailscale.com/kb/1193/tailscale-ssh)

- Features overview: Tailscale Features (https://tailscale.com/features)

- What is Tailscale?: Overview (https://tailscale.com/kb/1151/what-is-tailscale)

- App Connectors explainer: App connectors (https://tailscale.com/kb/1281/app-connectors)

- App Capabilities via headers: Announcement (https://tailscale.com/blog/app-capabilities#how-it-works)

- Visual Policy Editor GA: Announcement (https://tailscale.com/kb/1550/visual-editor?utm_source=blog&utm_medium=content&utm_campaign=fall-update-2025)

- HTTPS certificates: Docs (https://tailscale.com/kb/1153/enabling-https)

- Grafana Cloud PDC: Integration (https://grafana.com/products/cloud/?pg=blog&plcmt=body-txt)

- MCP + DCR + tsidp: Insights (https://tailscale.com/blog/dynamic-client-registration-dcr-for-mcp-ai#how-does-tailscale-make-a-more-secure-dcr-easy)

- Exposed AI servers: Blog (https://tailscale.com/blog/AI-endpoints-on-public-web#there-has-to-be-a-better-way-and-there-is)

- Monthly update (Aug 2025): Rollup (https://tailscale.com/blog/august-25-product-update#visual-policy-editor)