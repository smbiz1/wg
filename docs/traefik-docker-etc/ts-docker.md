Title: Using Tailscale with Docker · Tailscale Docs

URL Source: https://tailscale.com/kb/1282/docker

Markdown Content:
Using Tailscale with Docker · Tailscale Docs

===============

![Image 1](https://d.adroll.com/cm/b/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 2](https://d.adroll.com/cm/bombora/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 3](https://d.adroll.com/cm/experian/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 4](https://d.adroll.com/cm/eyeota/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 5](https://d.adroll.com/cm/g/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 6](https://d.adroll.com/cm/index/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 7](https://d.adroll.com/cm/l/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 8](https://d.adroll.com/cm/n/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 9](https://d.adroll.com/cm/o/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 10](https://d.adroll.com/cm/outbrain/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 11](https://d.adroll.com/cm/pubmatic/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 12](https://d.adroll.com/cm/taboola/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 13](https://d.adroll.com/cm/triplelift/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 14](https://d.adroll.com/cm/x/out?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&advertisable=TKO7FOASLRCK5J2S4BRIFC)

![Image 15](https://ipv4.d.adroll.com/seg4/TKO7FOASLRCK5J2S4BRIFC/5J25I7J2IBDGRESKWR4LV3?adroll_fpc=5297ab8cbe865434b9524538eb53558e-1764943583477&pv=84591502041.3526&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&cookie=&adroll_s_ref=&keyw=&p0=3705&adroll_external_data=&adroll_version=2.0)

### Your Privacy Choices

Welcome! We’re glad you’re here and want you to know that we respect your privacy and your right to control how we collect, use, and share your personal data. Please read our [Privacy Policy](https://tailscale.com/privacy-policy "privacyPolicy") to learn about our privacy practices.

Reject All Customize Settings Confirm

[](https://www.ketch.com/?utm_campaign=customer%20banner&utm_source=ketchcookiebanner&utm_medium=banner&org=tailscale)

[](https://tailscale.com/ "Homepage")

Product

Solutions

[Enterprise](https://tailscale.com/enterprise)[Customers](https://tailscale.com/customers)[Docs](https://tailscale.com/kb/1017/install)[Blog](https://tailscale.com/blog)[Pricing](https://tailscale.com/pricing)

[Download](https://tailscale.com/download)[Log in](https://login.tailscale.com/welcome)[Schedule a demo](https://tailscale.com/contact/sales)[Get started - it's free!](https://login.tailscale.com/start)

Product

Meet Tailscale

*   [How it works](https://tailscale.com/blog/how-tailscale-works)
*   [Why Tailscale](https://tailscale.com/why-tailscale)
*   [WireGuard® for Enterprises](https://tailscale.com/wireguard-vpn)
*   [Bring Tailscale to Work](https://tailscale.com/bring-tailscale-to-work)

Explore

*   [Integrations](https://tailscale.com/integrations)
*   [Features](https://tailscale.com/features)
*   [Compare Tailscale](https://tailscale.com/compare)
*   [Community Projects](https://tailscale.com/community/community-projects)
*   [Partnerships](https://tailscale.com/partnerships)

Solutions

By use-case

*   [Business VPN](https://tailscale.com/use-cases/business-vpn)
*   [CI/CD](https://tailscale.com/use-cases/ci-cd)
*   [Infra Access](https://tailscale.com/use-cases/infrastructure-access)
*   [Cloud Connectivity](https://tailscale.com/use-cases/cloud-connectivity)
*   [Zero Trust Networking](https://tailscale.com/use-cases/zero-trust-networking)
*   [Homelab](https://tailscale.com/use-cases/homelab)

By role

*   [DevOps](https://tailscale.com/solutions/devops)
*   [IT](https://tailscale.com/solutions/it)
*   [Security](https://tailscale.com/solutions/security)

[Enterprise](https://tailscale.com/enterprise)

[Customers](https://tailscale.com/customers)

Nav heading here

*   [![Image 16: Alt text ](https://cdn.sanity.io/images/w77i7m8x/production/a06dc612b1e3e4f4df53a72030002600639a8738-300x120.png?w=640&q=75&fit=clip&auto=format)Title here How Cribl Enables Secure Work From Anywhere with Tailscale](https://tailscale.com/customers)

[Docs](https://tailscale.com/kb/1017/install)

[Blog](https://tailscale.com/blog)

[Pricing](https://tailscale.com/pricing)

[Download](https://tailscale.com/download)

[Schedule a demo](https://tailscale.com/contact/sales)

[Get started - it's free!](https://login.tailscale.com/start)[Log in](https://login.tailscale.com/welcome)

© 2025

*   [Start](https://tailscale.com/kb/1346/start)
    *   [Quickstart](https://tailscale.com/kb/1017/install) 
    *    [Install Tailscale](https://tailscale.com/kb/1347/installation) 
    *    [Quick guides](https://tailscale.com/kb/1415/quick-guides) 
    *   [OpenVPN migration guide](https://tailscale.com/kb/1526/openvpn-migration-guide) 
    *   [Legacy VPN migration guide](https://tailscale.com/kb/1549/legacy-vpn-migration-guide) 
    *    [Set up an identity provider](https://tailscale.com/kb/1013/sso-providers) 
    *   [What is Tailscale?](https://tailscale.com/kb/1151/what-is-tailscale) 

*   [How-to Guides](https://tailscale.com/kb/1348/guides)
    *   [Manage Access](https://tailscale.com/kb/1350/manage)
        *    [Manage access control](https://tailscale.com/kb/1393/access-control) 
        *    [Manage just-in-time access](https://tailscale.com/kb/1443/just-in-time-access) 
        *    [Manage devices](https://tailscale.com/kb/1372/manage-devices) 
        *    [Manage users](https://tailscale.com/kb/1373/manage-users) 
        *   [Tailnet Lock](https://tailscale.com/kb/1226/tailnet-lock) 

    *   [Route Traffic](https://tailscale.com/kb/1351/route)
        *    [Set up a subnet router](https://tailscale.com/kb/1019/subnets) 
        *    [Set up an exit node](https://tailscale.com/kb/1103/exit-nodes) 
        *    [Set up an app connector](https://tailscale.com/kb/1342/app-connectors-setup) 
        *    [Use DNS](https://tailscale.com/kb/1054/dns) 
        *   [Set up MagicDNS](https://tailscale.com/kb/1081/magicdns) 
        *   [Set up high availability](https://tailscale.com/kb/1115/high-availability) 

    *   [Set Up Servers](https://tailscale.com/kb/1352/servers)
        *   [Set up a server](https://tailscale.com/kb/1245/set-up-servers) 
        *   [Use tags](https://tailscale.com/kb/1068/tags) 
        *   [Install Tailscale with cloud-init](https://tailscale.com/kb/1293/cloud-init) 
        *   [Use auth keys](https://tailscale.com/kb/1085/auth-keys) 
        *   [Automate key expiry](https://tailscale.com/kb/1028/key-expiry) 
        *    [Use Tailscale SSH](https://tailscale.com/kb/1193/tailscale-ssh) 
        *   [Set up HTTPS certificates](https://tailscale.com/kb/1153/enabling-https) 
        *   [Run an ephemeral node](https://tailscale.com/kb/1111/ephemeral-nodes) 
        *   [Run unattended](https://tailscale.com/kb/1088/run-unattended) 

    *   [Access & Share Services](https://tailscale.com/kb/1354/share)
        *   [Tailscale services](https://tailscale.com/kb/1552/tailscale-services) 
        *   [Endpoint collection](https://tailscale.com/kb/1100/services) 
        *   [Share nodes](https://tailscale.com/kb/1084/sharing) 
        *    [Use Taildrop](https://tailscale.com/kb/1106/taildrop) 

    *   [Share a web server](https://tailscale.com/kb/1353/share-web-server)
        *    [Tailscale Funnel](https://tailscale.com/kb/1223/funnel) 
        *    [Tailscale Serve](https://tailscale.com/kb/1312/serve) 

    *   [Solutions](https://tailscale.com/kb/1355/solutions)
        *   [Secure traffic with Apple TV](https://tailscale.com/kb/1598/apple-tv-route-traffic) 
        *   [Secure GitHub Actions runners](https://tailscale.com/kb/1586/secure-github-runners) 
        *   [Block ads with a Raspberry Pi](https://tailscale.com/kb/1114/pi-hole) 
        *   [Access remote desktops with Windows RDP](https://tailscale.com/kb/1095/secure-windows-rdp) 
        *   [Access remote desktops with RustDesk](https://tailscale.com/kb/1599/rustdesk) 
        *   [Connect to MongoDB Atlas](https://tailscale.com/kb/1601/connect-mongodb-atlas) 
        *   [Code from your iPad](https://tailscale.com/kb/1166/vscode-ipad) 
        *   [Lock down a server](https://tailscale.com/kb/1077/secure-server-ubuntu) 
        *   [Protect production databases](https://tailscale.com/kb/1469/unencrypted-macbooks-postgres) 
        *   [Access a PiKVM](https://tailscale.com/kb/1292/pikvm) 
        *   [Secure external services](https://tailscale.com/kb/1059/ip-blocklist-relays) 
        *    [Just-in-time access](https://tailscale.com/kb/1443/just-in-time-access) 
        *    [Automation](https://tailscale.com/kb/1430/automations) 

*   [Integrations](https://tailscale.com/kb/1356/integrations)
    *    [Cloud servers](https://tailscale.com/kb/1357/cloud-server) 
    *    [Containers and virtualization](https://tailscale.com/kb/1358/containers-and-virtualization) 
        *   [Docker](https://tailscale.com/kb/1282/docker) 
        *    [Kubernetes](https://tailscale.com/kb/1185/kubernetes) 
        *   [LXC containers](https://tailscale.com/kb/1130/lxc-unprivileged) 
        *   [Proxmox](https://tailscale.com/kb/1133/proxmox) 

    *    [Serverless apps](https://tailscale.com/kb/1364/serverless) 
    *    [Databases](https://tailscale.com/kb/1359/database) 
    *    [Remote environments](https://tailscale.com/kb/1363/remote-code) 
    *    [Developer tools](https://tailscale.com/kb/1360/developer-tools) 
    *    [Firewalls](https://tailscale.com/kb/1361/firewall) 
    *    [Web servers](https://tailscale.com/kb/1365/webserver) 
    *    [NAS](https://tailscale.com/kb/1307/nas) 

*   [FAQ](https://tailscale.com/kb/1366/faq)
*   [Logging, Streaming, and Events](https://tailscale.com/kb/1349/log-events)
    *   [Logging overview](https://tailscale.com/kb/1011/log-mesh-traffic) 
    *   [Configuration audit logging](https://tailscale.com/kb/1203/audit-logging) 
    *   [Network flow logs](https://tailscale.com/kb/1219/network-flow-logs) 
    *   [Log streaming](https://tailscale.com/kb/1255/log-streaming) 
    *    [SSH session recording](https://tailscale.com/kb/1246/tailscale-ssh-session-recording) 
    *   [Client metrics](https://tailscale.com/kb/1482/client-metrics) 
    *   [Webhooks](https://tailscale.com/kb/1213/webhooks) 

*   [Manage Your Organization](https://tailscale.com/kb/1431/manage-account)
    *   [Contact preferences](https://tailscale.com/kb/1224/contact-preferences) 
    *    [Pricing and billing](https://tailscale.com/kb/1375/pb-lp) 
    *   [Tailnet name](https://tailscale.com/kb/1217/tailnet-name) 
    *   [Domain ownership](https://tailscale.com/kb/1259/domain-ownership) 

*   [Reference](https://tailscale.com/kb/1367/reference)
    *    [Tailnet policy file syntax](https://tailscale.com/kb/1337/policy-syntax) 
    *   [ACL examples](https://tailscale.com/kb/1192/acl-samples) 
    *   [Grant examples](https://tailscale.com/kb/1458/grant-examples) 
    *    [CLI](https://tailscale.com/kb/1080/cli) 
    *    [API](https://tailscale.com/kb/1101/api) 
    *   [Key prefixes](https://tailscale.com/kb/1277/key-prefixes) 
    *    [Production best practices](https://tailscale.com/kb/1300/production-best-practices) 
    *   [Shared responsibility](https://tailscale.com/kb/1212/shared-responsibility) 
    *    [Technical overviews](https://tailscale.com/kb/1376/tech-overviews) 
    *    [Terminology and concepts](https://tailscale.com/kb/1155/terminology-and-concepts) 
    *   [Tailscale messages](https://tailscale.com/kb/1554/messages) 
    *   [GitHub ↗](https://github.com/tailscale/tailscale) 

*   [Get Support](https://tailscale.com/kb/1432/get-support)
    *    [Troubleshooting](https://tailscale.com/kb/1023/troubleshooting) 
    *   [Support options](https://tailscale.com/kb/1250/support-options) 
    *   [Contact support ↗](https://tailscale.com/contact/support) 
    *   [Generate a bug report](https://tailscale.com/kb/1227/bug-report) 

*   [Resources](https://tailscale.com/kb/1368/resources)
    *   [Changelog ↗](https://tailscale.com/changelog) 
    *   [Comparisons ↗](https://tailscale.com/compare) 
    *   [Release stages](https://tailscale.com/kb/1167/release-stages) 
    *   [Security ↗](https://tailscale.com/security) 
    *   [Tailscale Community Projects](https://tailscale.com/kb/1531/community-projects) 
    *    [Versions](https://tailscale.com/kb/1168/versions) 
    *   [Use cases](https://tailscale.com/kb/1377/use-cases) 
    *   [Invite only features](https://tailscale.com/kb/1222/invite-only-feature) 

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Containers and virtualization](https://tailscale.com/kb/1358/containers-and-virtualization)›Docker

### Documentation

*   [Start](https://tailscale.com/kb/1346/start)
    *   [Quickstart](https://tailscale.com/kb/1017/install) 
    *    [Install Tailscale](https://tailscale.com/kb/1347/installation) 
    *    [Quick guides](https://tailscale.com/kb/1415/quick-guides) 
    *   [OpenVPN migration guide](https://tailscale.com/kb/1526/openvpn-migration-guide) 
    *   [Legacy VPN migration guide](https://tailscale.com/kb/1549/legacy-vpn-migration-guide) 
    *    [Set up an identity provider](https://tailscale.com/kb/1013/sso-providers) 
    *   [What is Tailscale?](https://tailscale.com/kb/1151/what-is-tailscale) 

*   [How-to Guides](https://tailscale.com/kb/1348/guides)
    *   [Manage Access](https://tailscale.com/kb/1350/manage)
        *    [Manage access control](https://tailscale.com/kb/1393/access-control) 
        *    [Manage just-in-time access](https://tailscale.com/kb/1443/just-in-time-access) 
        *    [Manage devices](https://tailscale.com/kb/1372/manage-devices) 
        *    [Manage users](https://tailscale.com/kb/1373/manage-users) 
        *   [Tailnet Lock](https://tailscale.com/kb/1226/tailnet-lock) 

    *   [Route Traffic](https://tailscale.com/kb/1351/route)
        *    [Set up a subnet router](https://tailscale.com/kb/1019/subnets) 
        *    [Set up an exit node](https://tailscale.com/kb/1103/exit-nodes) 
        *    [Set up an app connector](https://tailscale.com/kb/1342/app-connectors-setup) 
        *    [Use DNS](https://tailscale.com/kb/1054/dns) 
        *   [Set up MagicDNS](https://tailscale.com/kb/1081/magicdns) 
        *   [Set up high availability](https://tailscale.com/kb/1115/high-availability) 

    *   [Set Up Servers](https://tailscale.com/kb/1352/servers)
        *   [Set up a server](https://tailscale.com/kb/1245/set-up-servers) 
        *   [Use tags](https://tailscale.com/kb/1068/tags) 
        *   [Install Tailscale with cloud-init](https://tailscale.com/kb/1293/cloud-init) 
        *   [Use auth keys](https://tailscale.com/kb/1085/auth-keys) 
        *   [Automate key expiry](https://tailscale.com/kb/1028/key-expiry) 
        *    [Use Tailscale SSH](https://tailscale.com/kb/1193/tailscale-ssh) 
        *   [Set up HTTPS certificates](https://tailscale.com/kb/1153/enabling-https) 
        *   [Run an ephemeral node](https://tailscale.com/kb/1111/ephemeral-nodes) 
        *   [Run unattended](https://tailscale.com/kb/1088/run-unattended) 

    *   [Access & Share Services](https://tailscale.com/kb/1354/share)
        *   [Tailscale services](https://tailscale.com/kb/1552/tailscale-services) 
        *   [Endpoint collection](https://tailscale.com/kb/1100/services) 
        *   [Share nodes](https://tailscale.com/kb/1084/sharing) 
        *    [Use Taildrop](https://tailscale.com/kb/1106/taildrop) 

    *   [Share a web server](https://tailscale.com/kb/1353/share-web-server)
        *    [Tailscale Funnel](https://tailscale.com/kb/1223/funnel) 
        *    [Tailscale Serve](https://tailscale.com/kb/1312/serve) 

    *   [Solutions](https://tailscale.com/kb/1355/solutions)
        *   [Secure traffic with Apple TV](https://tailscale.com/kb/1598/apple-tv-route-traffic) 
        *   [Secure GitHub Actions runners](https://tailscale.com/kb/1586/secure-github-runners) 
        *   [Block ads with a Raspberry Pi](https://tailscale.com/kb/1114/pi-hole) 
        *   [Access remote desktops with Windows RDP](https://tailscale.com/kb/1095/secure-windows-rdp) 
        *   [Access remote desktops with RustDesk](https://tailscale.com/kb/1599/rustdesk) 
        *   [Connect to MongoDB Atlas](https://tailscale.com/kb/1601/connect-mongodb-atlas) 
        *   [Code from your iPad](https://tailscale.com/kb/1166/vscode-ipad) 
        *   [Lock down a server](https://tailscale.com/kb/1077/secure-server-ubuntu) 
        *   [Protect production databases](https://tailscale.com/kb/1469/unencrypted-macbooks-postgres) 
        *   [Access a PiKVM](https://tailscale.com/kb/1292/pikvm) 
        *   [Secure external services](https://tailscale.com/kb/1059/ip-blocklist-relays) 
        *    [Just-in-time access](https://tailscale.com/kb/1443/just-in-time-access) 
        *    [Automation](https://tailscale.com/kb/1430/automations) 

*   [Integrations](https://tailscale.com/kb/1356/integrations)
    *    [Cloud servers](https://tailscale.com/kb/1357/cloud-server) 
    *    [Containers and virtualization](https://tailscale.com/kb/1358/containers-and-virtualization) 
        *   [Docker](https://tailscale.com/kb/1282/docker) 
        *    [Kubernetes](https://tailscale.com/kb/1185/kubernetes) 
        *   [LXC containers](https://tailscale.com/kb/1130/lxc-unprivileged) 
        *   [Proxmox](https://tailscale.com/kb/1133/proxmox) 

    *    [Serverless apps](https://tailscale.com/kb/1364/serverless) 
    *    [Databases](https://tailscale.com/kb/1359/database) 
    *    [Remote environments](https://tailscale.com/kb/1363/remote-code) 
    *    [Developer tools](https://tailscale.com/kb/1360/developer-tools) 
    *    [Firewalls](https://tailscale.com/kb/1361/firewall) 
    *    [Web servers](https://tailscale.com/kb/1365/webserver) 
    *    [NAS](https://tailscale.com/kb/1307/nas) 

*   [FAQ](https://tailscale.com/kb/1366/faq)
*   [Logging, Streaming, and Events](https://tailscale.com/kb/1349/log-events)
    *   [Logging overview](https://tailscale.com/kb/1011/log-mesh-traffic) 
    *   [Configuration audit logging](https://tailscale.com/kb/1203/audit-logging) 
    *   [Network flow logs](https://tailscale.com/kb/1219/network-flow-logs) 
    *   [Log streaming](https://tailscale.com/kb/1255/log-streaming) 
    *    [SSH session recording](https://tailscale.com/kb/1246/tailscale-ssh-session-recording) 
    *   [Client metrics](https://tailscale.com/kb/1482/client-metrics) 
    *   [Webhooks](https://tailscale.com/kb/1213/webhooks) 

*   [Manage Your Organization](https://tailscale.com/kb/1431/manage-account)
    *   [Contact preferences](https://tailscale.com/kb/1224/contact-preferences) 
    *    [Pricing and billing](https://tailscale.com/kb/1375/pb-lp) 
    *   [Tailnet name](https://tailscale.com/kb/1217/tailnet-name) 
    *   [Domain ownership](https://tailscale.com/kb/1259/domain-ownership) 

*   [Reference](https://tailscale.com/kb/1367/reference)
    *    [Tailnet policy file syntax](https://tailscale.com/kb/1337/policy-syntax) 
    *   [ACL examples](https://tailscale.com/kb/1192/acl-samples) 
    *   [Grant examples](https://tailscale.com/kb/1458/grant-examples) 
    *    [CLI](https://tailscale.com/kb/1080/cli) 
    *    [API](https://tailscale.com/kb/1101/api) 
    *   [Key prefixes](https://tailscale.com/kb/1277/key-prefixes) 
    *    [Production best practices](https://tailscale.com/kb/1300/production-best-practices) 
    *   [Shared responsibility](https://tailscale.com/kb/1212/shared-responsibility) 
    *    [Technical overviews](https://tailscale.com/kb/1376/tech-overviews) 
    *    [Terminology and concepts](https://tailscale.com/kb/1155/terminology-and-concepts) 
    *   [Tailscale messages](https://tailscale.com/kb/1554/messages) 
    *   [GitHub ↗](https://github.com/tailscale/tailscale) 

*   [Get Support](https://tailscale.com/kb/1432/get-support)
    *    [Troubleshooting](https://tailscale.com/kb/1023/troubleshooting) 
    *   [Support options](https://tailscale.com/kb/1250/support-options) 
    *   [Contact support ↗](https://tailscale.com/contact/support) 
    *   [Generate a bug report](https://tailscale.com/kb/1227/bug-report) 

*   [Resources](https://tailscale.com/kb/1368/resources)
    *   [Changelog ↗](https://tailscale.com/changelog) 
    *   [Comparisons ↗](https://tailscale.com/compare) 
    *   [Release stages](https://tailscale.com/kb/1167/release-stages) 
    *   [Security ↗](https://tailscale.com/security) 
    *   [Tailscale Community Projects](https://tailscale.com/kb/1531/community-projects) 
    *    [Versions](https://tailscale.com/kb/1168/versions) 
    *   [Use cases](https://tailscale.com/kb/1377/use-cases) 
    *   [Invite only features](https://tailscale.com/kb/1222/invite-only-feature) 

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Containers and virtualization](https://tailscale.com/kb/1358/containers-and-virtualization)›Docker
Using Tailscale with Docker
===========================

Tailscale has a published Docker image that Tailscale manages and builds from source. It's available in [Docker Hub](https://hub.docker.com/r/tailscale/tailscale) and [GitHub Packages](https://github.com/orgs/tailscale/packages/container/package/tailscale). Watch the video below for a quickstart guide on using Docker with Tailscale.

[Pull image](https://tailscale.com/kb/1282/docker#pull-image)
-------------------------------------------------------------

To pull the image, run:

```shell
docker pull tailscale/tailscale:latest
```

or

```shell
docker pull ghcr.io/tailscale/tailscale:latest
```

### [Supported tags](https://tailscale.com/kb/1282/docker#supported-tags)

Containers are tagged based on the Tailscale [versioning scheme](https://tailscale.com/kb/1168/versions).

*   Use `stable` or `latest` to get the latest stable version.
    *   `v1.58.2`, `v1.58` to get a specific stable version.

*   Use `unstable` to get the latest unstable version.
    *   `unstable-v1.59.37`, `unstable-v1.59.44` to get a specific unstable version.

[Parameters](https://tailscale.com/kb/1282/docker#parameters)
-------------------------------------------------------------

You can set additional parameters for use with the image. All configuration is optional.

### [`TS_ACCEPT_DNS`](https://tailscale.com/kb/1282/docker#ts_accept_dns)

Accept [DNS configuration](https://tailscale.com/kb/1054/dns) from the admin console. Not accepted by default.

### [`TS_AUTH_ONCE`](https://tailscale.com/kb/1282/docker#ts_auth_once)

Attempt to log in only if not already logged in. False by default, to forcibly log in every time the container starts.

### [`TS_AUTHKEY`](https://tailscale.com/kb/1282/docker#ts_authkey)

An [auth key](https://tailscale.com/kb/1085/auth-keys) used to authenticate the container. This is equivalent to what you'd pass to [`tailscale login --auth-key=`](https://tailscale.com/kb/1080/cli#login).

It is also possible to use an [OAuth client](https://tailscale.com/kb/1215/oauth-clients#register-new-nodes-using-oauth-credentials) secret here but the associated tag must be provided using [`TS_EXTRA_ARGS=--advertise-tags=tag:ci`](https://tailscale.com/kb/1282/docker#ts_extra_args).

To mark a containerized node as ephemeral append `?ephemeral=true` to the auth key or OAuth client secret.

### [`TS_DEST_IP`](https://tailscale.com/kb/1282/docker#ts_dest_ip)

Proxy all incoming Tailscale traffic to the specified destination IP.

### [`TS_HEALTHCHECK_ADDR_PORT`](https://tailscale.com/kb/1282/docker#ts_healthcheck_addr_port)

Deprecated. Use `TS_ENABLE_HEALTH_CHECK` (and optionally `TS_LOCAL_ADDR_PORT`) instead from 1.78.

### [`TS_LOCAL_ADDR_PORT`](https://tailscale.com/kb/1282/docker#ts_local_addr_port)

This functionality is available in Tailscale 1.78 and later.

Specifies the `[<addr>]:<port>` on which to serve local metrics and health check HTTP endpoints if enabled through `TS_ENABLE_METRICS` or `TS_ENABLE_HEALTH_CHECK`. Defaults to `[::]:9002` on all available interfaces.

### [`TS_ENABLE_HEALTH_CHECK`](https://tailscale.com/kb/1282/docker#ts_enable_health_check)

This functionality is available in Tailscale 1.78 and later.

Set to `true` to enable an unauthenticated `/healthz` endpoint at the address specified by `TS_LOCAL_ADDR_PORT`.

The health check returns `200 OK` if the node has at least one [tailnet](https://tailscale.com/kb/1136/tailnet) IP address, otherwise it returns `503`.

### [`TS_ENABLE_METRICS`](https://tailscale.com/kb/1282/docker#ts_enable_metrics)

This functionality is available in Tailscale 1.78 and later.

Set to `true` to enable an unauthenticated `/metrics` endpoint at the address specified by `TS_LOCAL_ADDR_PORT`.

Refer to [client metrics](https://tailscale.com/kb/1482/client-metrics) for more information about the metrics.

### [`TS_HOSTNAME`](https://tailscale.com/kb/1282/docker#ts_hostname)

Use the specified hostname for the node. This is equivalent to [`tailscale set --hostname=`](https://tailscale.com/kb/1080/cli#set).

### [`TS_KUBE_SECRET`](https://tailscale.com/kb/1282/docker#ts_kube_secret)

If [running in Kubernetes](https://tailscale.com/kb/1185/kubernetes), the [Kubernetes secret](https://kubernetes.io/docs/concepts/configuration/secret) name where Tailscale state is stored. The default is `tailscale`.

If `TS_AUTHKEY` is not set, and `TS_KUBE_SECRET` contains a secret with an `authkey` field, that key is used as a Tailscale auth key.

### [`TS_OUTBOUND_HTTP_PROXY_LISTEN`](https://tailscale.com/kb/1282/docker#ts_outbound_http_proxy_listen)

Set an address and port for the [HTTP proxy](https://tailscale.com/kb/1112/userspace-networking#socks5-vs-http). This will be passed to `tailscaled --outbound-http-proxy-listen=`. For example, to set the SOCKS5 proxy to port 1055, this is `:1055`, which is equivalent to `tailscaled --outbound-http-proxy-listen=:1055`.

### [`TS_ROUTES`](https://tailscale.com/kb/1282/docker#ts_routes)

Advertise [subnet routes](https://tailscale.com/kb/1019/subnets). This is equivalent to [`tailscale set --advertise-routes=`](https://tailscale.com/kb/1080/cli#set). To accept advertised routes, use [`TS_EXTRA_ARGS`](https://tailscale.com/kb/1282/docker#ts_extra_args) to pass in `--accept-routes`.

### [`TS_SERVE_CONFIG`](https://tailscale.com/kb/1282/docker#ts_serve_config)

Accepts a JSON file to programmatically configure [Serve](https://tailscale.com/kb/1242/tailscale-serve) and [Funnel](https://tailscale.com/kb/1311/tailscale-funnel) functionality. Use [`tailscale serve status --json`](https://tailscale.com/kb/1242/tailscale-serve) to export your current configuration in the correct format.

If this file is bind mounted using a Docker volume, it must be done so as a directory and not an individual file for configuration updates to be correctly detected.

### [`TS_SOCKET`](https://tailscale.com/kb/1282/docker#ts_socket)

Unix socket path used by the Tailscale binary, where the `tailscaled` LocalAPI socket is created. The default is `/var/run/tailscale/tailscaled.sock`. This is equivalent to `tailscaled tailscale --socket=`.

### [`TS_SOCKS5_SERVER`](https://tailscale.com/kb/1282/docker#ts_socks5_server)

Set an address and port for the [SOCKS5 proxy](https://tailscale.com/kb/1112/userspace-networking#socks5-vs-http). This will be passed to `tailscaled --socks5-server=`. For example, to set the SOCKS5 proxy to port 1055, this is `:1055`, which is equivalent to `tailscaled --socks5-server=:1055`.

### [`TS_STATE_DIR`](https://tailscale.com/kb/1282/docker#ts_state_dir)

Directory where the state of `tailscaled` is stored. This needs to persist across container restarts. This will be passed to `tailscaled --statedir=`.

When running on Kubernetes, state is stored by default in the Kubernetes secret with `name:tailscale`. To store state on local disk instead, set `TS_KUBE_SECRET=""` and `TS_STATE_DIR=/path/to/storage/dir`.

### [`TS_USERSPACE`](https://tailscale.com/kb/1282/docker#ts_userspace)

Enable [userspace networking](https://tailscale.com/kb/1112/userspace-networking), instead of kernel networking. Enabled by default. This is equivalent to `tailscaled --tun=userspace-networking`.

### [Extra arguments](https://tailscale.com/kb/1282/docker#extra-arguments)

#### [`TS_EXTRA_ARGS`](https://tailscale.com/kb/1282/docker#ts_extra_args)

Any other flags to pass in to the [Tailscale CLI](https://tailscale.com/kb/1080/cli) in a [`tailscale up`](https://tailscale.com/kb/1080/cli#up) command.

#### [`TS_TAILSCALED_EXTRA_ARGS`](https://tailscale.com/kb/1282/docker#ts_tailscaled_extra_args)

Any other [flags](https://tailscale.com/kb/1278/tailscaled#flags-to-tailscaled) to pass in to `tailscaled`.

### [Code examples](https://tailscale.com/kb/1282/docker#code-examples)

Below is a complete Docker Compose code snippet using an OAuth client secret.

```yaml
---
version: "3.7"
services:
  tailscale-nginx:
    image: tailscale/tailscale:latest
    hostname: tailscale-nginx
    environment:
      - TS_AUTHKEY=tskey-client-notAReal-OAuthClientSecret1Atawk
      - TS_EXTRA_ARGS=--advertise-tags=tag:container
      - TS_STATE_DIR=/var/lib/tailscale
      - TS_USERSPACE=false
    volumes:
      - ${PWD}/tailscale-nginx/state:/var/lib/tailscale
    devices:
      - /dev/net/tun:/dev/net/tun
    cap_add:
      - net_admin
    restart: unless-stopped
  nginx:
    image: nginx
    depends_on:
      - tailscale-nginx
    network_mode: service:tailscale-nginx
```

More examples can be found in [tailscale-dev/docker-guide-code-examples](https://github.com/tailscale-dev/docker-guide-code-examples).

Last updated Oct 30, 2025

On this page

*   [Pull image](https://tailscale.com/kb/1282/docker#pull-image)
    *   [Supported tags](https://tailscale.com/kb/1282/docker#supported-tags)

*   [Parameters](https://tailscale.com/kb/1282/docker#parameters)
    *   [TS_ACCEPT_DNS](https://tailscale.com/kb/1282/docker#ts_accept_dns)
    *   [TS_AUTH_ONCE](https://tailscale.com/kb/1282/docker#ts_auth_once)
    *   [TS_AUTHKEY](https://tailscale.com/kb/1282/docker#ts_authkey)
    *   [TS_DEST_IP](https://tailscale.com/kb/1282/docker#ts_dest_ip)
    *   [TS_HEALTHCHECK_ADDR_PORT](https://tailscale.com/kb/1282/docker#ts_healthcheck_addr_port)
    *   [TS_LOCAL_ADDR_PORT](https://tailscale.com/kb/1282/docker#ts_local_addr_port)
    *   [TS_ENABLE_HEALTH_CHECK](https://tailscale.com/kb/1282/docker#ts_enable_health_check)
    *   [TS_ENABLE_METRICS](https://tailscale.com/kb/1282/docker#ts_enable_metrics)
    *   [TS_HOSTNAME](https://tailscale.com/kb/1282/docker#ts_hostname)
    *   [TS_KUBE_SECRET](https://tailscale.com/kb/1282/docker#ts_kube_secret)
    *   [TS_OUTBOUND_HTTP_PROXY_LISTEN](https://tailscale.com/kb/1282/docker#ts_outbound_http_proxy_listen)
    *   [TS_ROUTES](https://tailscale.com/kb/1282/docker#ts_routes)
    *   [TS_SERVE_CONFIG](https://tailscale.com/kb/1282/docker#ts_serve_config)
    *   [TS_SOCKET](https://tailscale.com/kb/1282/docker#ts_socket)
    *   [TS_SOCKS5_SERVER](https://tailscale.com/kb/1282/docker#ts_socks5_server)
    *   [TS_STATE_DIR](https://tailscale.com/kb/1282/docker#ts_state_dir)
    *   [TS_USERSPACE](https://tailscale.com/kb/1282/docker#ts_userspace)
    *   [Extra arguments](https://tailscale.com/kb/1282/docker#extra-arguments)
        *   [TS_EXTRA_ARGS](https://tailscale.com/kb/1282/docker#ts_extra_args)
        *   [TS_TAILSCALED_EXTRA_ARGS](https://tailscale.com/kb/1282/docker#ts_tailscaled_extra_args)

    *   [Code examples](https://tailscale.com/kb/1282/docker#code-examples)

Related Pages

*   [Userspace networking mode (for containers)](https://tailscale.com/kb/1112/userspace-networking)
*   [Tailscale in LXC containers](https://tailscale.com/kb/1130/lxc-unprivileged)
*   [Tailscale on a Proxmox host](https://tailscale.com/kb/1133/proxmox)
*   [Tailscale on Kubernetes](https://tailscale.com/kb/1185/kubernetes)

Product

[How it works](https://tailscale.com/blog/how-tailscale-works)[Pricing](https://tailscale.com/pricing)[Integrations](https://tailscale.com/integrations)[Features](https://tailscale.com/features)[Compare Tailscale](https://tailscale.com/compare)

Use Cases

[Business VPN](https://tailscale.com/use-cases/business-vpn)[CI/CD](https://tailscale.com/use-cases/ci-cd)[Infra Access](https://tailscale.com/use-cases/infrastructure-access)[Cloud Connectivity](https://tailscale.com/use-cases/cloud-connectivity)[Zero Trust Networking](https://tailscale.com/use-cases/zero-trust-networking)[Homelab](https://tailscale.com/use-cases/homelab)

Resources

[Blog](https://tailscale.com/blog)[Events & Webinars](https://tailscale.com/events-webinars)[Partnerships](https://tailscale.com/partnerships)

Company

[Company](https://tailscale.com/company)[Careers](https://tailscale.com/careers)[Press](https://tailscale.com/press)

Help & Support

[Support](https://tailscale.com/contact/support)[Sales](https://tailscale.com/contact/sales)[Security](https://tailscale.com/security)[Legal](https://tailscale.com/legal)[Open Source](https://tailscale.com/opensource)[Changelog](https://tailscale.com/changelog)[Tailscale Status](https://status.tailscale.com/)

Learn

[SSH keys](https://tailscale.com/learn/generate-ssh-keys)[Docker SSH](https://tailscale.com/learn/ssh-into-docker-container)[NAT Traversal](https://tailscale.com/blog/how-nat-traversal-works)[MagicDNS](https://tailscale.com/blog/2021-09-private-dns-with-magicdns)[PAM](https://tailscale.com/learn/privileged-access-management)[All articles](https://tailscale.com/learn)

[](https://tailscale.com/ "Homepage")

[Terms of Service](https://tailscale.com/terms)[Privacy Policy](https://tailscale.com/privacy-policy)[California Notice](https://tailscale.com/privacy-policy#california-notice)[Cookie Notice](https://tailscale.com/cookie-notice)![Image 17: Check mark and x on a white and blue pill button](https://cdn.sanity.io/images/w77i7m8x/production/07d853f507039b2489d9818cb6ee7442c1b60e2a-30x14.svg)Your Privacy Choices

WireGuard is a registered trademark of Jason A. Donenfeld.

[](https://twitter.com/tailscale)[](https://www.facebook.com/tailscale/)[](https://www.linkedin.com/company/tailscale)[](https://hachyderm.io/@tailscale)[](https://www.youtube.com/@Tailscale)

© 2025 Tailscale Inc. All rights reserved. Tailscale is a registered trademark of Tailscale Inc.

![Image 18](https://bat.bing.com/action/0?ti=17188063&Ver=2&mid=a5a8648e-acd5-4663-8e0b-bc8bd7d6c0f8&bo=1&sid=94e7fd30d1e311f0b2eb05ba7c41a0ea&vid=94e7f310d1e311f0b6a6636cf3b57d87&vids=1&msclkid=N&pi=918639831&lg=en-US&sw=800&sh=600&sc=24&tl=Using%20Tailscale%20with%20Docker%20%C2%B7%20Tailscale%20Docs&p=https%3A%2F%2Ftailscale.com%2Fkb%2F1282%2Fdocker&r=&lt=2204&evt=pageLoad&sv=2&cdb=AQUR&rn=770777)