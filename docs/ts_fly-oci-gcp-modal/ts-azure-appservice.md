Title: Using Tailscale on Azure App Service · Tailscale Docs

URL Source: https://tailscale.com/kb/1126/azure-app-service

Markdown Content:
Using Tailscale on Azure App Service · Tailscale Docs

===============

![Image 3](https://d.adroll.com/cm/b/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 4](https://d.adroll.com/cm/bombora/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 5](https://d.adroll.com/cm/experian/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 6](https://d.adroll.com/cm/eyeota/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 7](https://d.adroll.com/cm/g/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 8](https://d.adroll.com/cm/index/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 9](https://d.adroll.com/cm/l/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 10](https://d.adroll.com/cm/n/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 11](https://d.adroll.com/cm/o/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 12](https://d.adroll.com/cm/outbrain/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 13](https://d.adroll.com/cm/pubmatic/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 14](https://d.adroll.com/cm/taboola/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 15](https://d.adroll.com/cm/triplelift/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 16](https://d.adroll.com/cm/x/out?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&advertisable=TKO7FOASLRCK5J2S4BRIFC)

![Image 17](https://ipv4.d.adroll.com/seg4/TKO7FOASLRCK5J2S4BRIFC/5J25I7J2IBDGRESKWR4LV3?adroll_fpc=b19397fdf1f1cc7d3d36b2d62c9d7291-1764943551225&pv=26346445642.794437&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&cookie=&adroll_s_ref=&keyw=&p0=5874&adroll_external_data=&adroll_version=2.0)

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

*   [![Image 18: Alt text ](https://cdn.sanity.io/images/w77i7m8x/production/a06dc612b1e3e4f4df53a72030002600639a8738-300x120.png?w=640&q=75&fit=clip&auto=format)Title here How Cribl Enables Secure Work From Anywhere with Tailscale](https://tailscale.com/customers)

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
        *   [AWS Lightsail](https://tailscale.com/kb/1128/aws-lightsail) 
        *   [AWS VPC](https://tailscale.com/kb/1021/install-aws) 
        *   [Azure App Service](https://tailscale.com/kb/1126/azure-app-service) 
        *   [Azure Linux VMs](https://tailscale.com/kb/1142/cloud-azure-linux) 
        *   [Azure Windows VMs](https://tailscale.com/kb/1143/cloud-azure-windows) 
        *   [Google Compute Engine VMs](https://tailscale.com/kb/1147/cloud-gce) 
        *   [Hetzner VMs](https://tailscale.com/kb/1150/cloud-hetzner) 
        *   [Oracle Cloud VMs](https://tailscale.com/kb/1149/cloud-oracle) 

    *    [Containers and virtualization](https://tailscale.com/kb/1358/containers-and-virtualization) 
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

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Cloud servers](https://tailscale.com/kb/1357/cloud-server)›Azure App Service

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
        *   [AWS Lightsail](https://tailscale.com/kb/1128/aws-lightsail) 
        *   [AWS VPC](https://tailscale.com/kb/1021/install-aws) 
        *   [Azure App Service](https://tailscale.com/kb/1126/azure-app-service) 
        *   [Azure Linux VMs](https://tailscale.com/kb/1142/cloud-azure-linux) 
        *   [Azure Windows VMs](https://tailscale.com/kb/1143/cloud-azure-windows) 
        *   [Google Compute Engine VMs](https://tailscale.com/kb/1147/cloud-gce) 
        *   [Hetzner VMs](https://tailscale.com/kb/1150/cloud-hetzner) 
        *   [Oracle Cloud VMs](https://tailscale.com/kb/1149/cloud-oracle) 

    *    [Containers and virtualization](https://tailscale.com/kb/1358/containers-and-virtualization) 
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

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Cloud servers](https://tailscale.com/kb/1357/cloud-server)›Azure App Service
Using Tailscale on Azure App Service
====================================

[Azure App Service](https://azure.microsoft.com/en-us/products/app-service) is a popular cloud-hosting platform for running applications without managing servers yourself. However, it can be difficult to use Tailscale on Azure App Service, since it doesn't provide a `/dev/net/tun` device that Tailscale needs.

You can use Tailscale's [userspace networking mode](https://tailscale.com/kb/1112/userspace-networking) to connect your apps to your Tailscale network.

[Step 1: Generate an auth key to authenticate your Azure App Service apps](https://tailscale.com/kb/1126/azure-app-service#step-1-generate-an-auth-key-to-authenticate-your-azure-app-service-apps)
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

First, we'll generate an [auth key](https://tailscale.com/kb/1085/auth-keys) to allow Azure to authenticate our app to join our network.

Open the [Keys](https://login.tailscale.com/admin/settings/keys) page of the admin console and select **Generate auth key**. We recommend using an [ephemeral key](https://tailscale.com/kb/1111/ephemeral-nodes) for this purpose, since it will automatically clean up devices after they shut down.

![Image 19: Tailscale's auth key generation page](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fephemeral-keys.de85b7fd.png&w=750&q=75)

The **Pre-approved** option will only display in the dialog if [device approval](https://tailscale.com/kb/1099/device-approval) is enabled in your Tailscale network.

Next, navigate to the [Azure Portal](https://portal.azure.com/) and then the **Configuration** page for your app. For **Config Var**, create a variable named `TAILSCALE_AUTHKEY`, with the `tskey-<key>` value you just created.

![Image 20: Azure App Service config var interface](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fazure-config-vars.17fcfa0c.png&w=3840&q=75)
[Step 2: Configure your Dockerfile to install Tailscale](https://tailscale.com/kb/1126/azure-app-service#step-2-configure-your-dockerfile-to-install-tailscale)
---------------------------------------------------------------------------------------------------------------------------------------------------------------

We recommend using a [multistage Dockerfile](https://docs.docker.com/develop/develop-images/multistage-build/#use-multi-stage-builds) where the first stage builds your application, and the second stage pulls application code and Tailscale into the final image to be uploaded to Azure.

1.   Create an `sshd_config` file and add it to the Docker build directory. If this file does not exist before building the Dockerfile, the build will fail.

2.   Create a `start.sh` file at the root of your app. The Dockerfile specifies `/app/start.sh` as the initial process to run. This script needs to bring Tailscale up and then start the application binary. Use the `TAILSCALE_AUTHKEY` variable defined earlier when you bring Tailscale up.

Here's a `start.sh` example file. Make sure to replace values needed for your build.

 
```yaml
#!/bin/sh

/usr/bin/ssh-keygen -A
mkdir -p /var/run/sshd
/usr/sbin/sshd

/app/tailscaled --tun=userspace-networking --socks5-server=localhost:1055 &
/app/tailscale up --auth-key=${TAILSCALE_AUTHKEY} --hostname=azure-app
echo Tailscale started
ALL_PROXY=socks5://localhost:1055/ /app/my-app
``` 
3.   Create a `Dockerfile` at the root of your app and include the following details. Make sure to replace values needed for your build.

 
```dockerfile
FROM golang:1-alpine3.21 AS builder
WORKDIR /app
COPY . ./
# This is where one could build the application code as well.
FROM dotnetcore-docs-hello-world-linux.

FROM alpine:latest
RUN apk update && apk add ca-certificates bash sudo && rm -rf /var/cache/apk/*

# Azure allows SSH access to the container. This isn't needed for Tailscale to
# operate, but is really useful for debugging the application.
RUN apk add openssh openssh-keygen && echo "root:Docker!" | chpasswd
RUN apk add netcat-openbsd
RUN mkdir -p /etc/ssh
COPY sshd_config /etc/ssh/
EXPOSE 80 2222

# Copy binary to production image.
COPY --from=builder /app/start.sh /app/start.sh
# Change start.sh to be executable
RUN chmod +x /app/start.sh

# Copy Tailscale binaries from the tailscale image on Docker Hub.
COPY --from=docker.io/tailscale/tailscale:stable /usr/local/bin/tailscaled /app/tailscaled
COPY --from=docker.io/tailscale/tailscale:stable /usr/local/bin/tailscale /app/tailscale
RUN mkdir -p /var/run/tailscale /var/cache/tailscale /var/lib/tailscale

# Run on container startup.
CMD ["./app/start.sh"]
``` 

The next time your Azure app deploys, it will be able to connect to your private Tailscale network.

[Remove ephemeral nodes from a tailnet](https://tailscale.com/kb/1126/azure-app-service#remove-ephemeral-nodes-from-a-tailnet)
------------------------------------------------------------------------------------------------------------------------------

When an ephemeral node goes offline, it is automatically removed from your tailnet. You can also control ephemeral node removal using the [`tailscale logout`](https://tailscale.com/kb/1080/cli#logout) command to either manually force the removal or incorporate the command into the [`tailscaled`](https://tailscale.com/kb/1278/tailscaled) Tailscale daemon. For more information, see [Ephemeral nodes](https://tailscale.com/kb/1111/ephemeral-nodes#faq).

Last updated Dec 4, 2025

On this page

*   [Step 1: Generate an auth key to authenticate your Azure App Service apps](https://tailscale.com/kb/1126/azure-app-service#step-1-generate-an-auth-key-to-authenticate-your-azure-app-service-apps)
*   [Step 2: Configure your Dockerfile to install Tailscale](https://tailscale.com/kb/1126/azure-app-service#step-2-configure-your-dockerfile-to-install-tailscale)
*   [Remove ephemeral nodes from a tailnet](https://tailscale.com/kb/1126/azure-app-service#remove-ephemeral-nodes-from-a-tailnet)

Related Pages

*   [Connect to external services with IP block lists](https://tailscale.com/kb/1059/ip-blocklist-relays)
*   [Auth keys](https://tailscale.com/kb/1085/auth-keys)
*   [Tailscale on Heroku](https://tailscale.com/kb/1107/heroku)
*   [Tailscale on Google Cloud Run](https://tailscale.com/kb/1108/cloudrun)
*   [Ephemeral nodes](https://tailscale.com/kb/1111/ephemeral-nodes)

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

[Terms of Service](https://tailscale.com/terms)[Privacy Policy](https://tailscale.com/privacy-policy)[California Notice](https://tailscale.com/privacy-policy#california-notice)[Cookie Notice](https://tailscale.com/cookie-notice)![Image 21: Check mark and x on a white and blue pill button](https://cdn.sanity.io/images/w77i7m8x/production/07d853f507039b2489d9818cb6ee7442c1b60e2a-30x14.svg)Your Privacy Choices

WireGuard is a registered trademark of Jason A. Donenfeld.

[](https://twitter.com/tailscale)[](https://www.facebook.com/tailscale/)[](https://www.linkedin.com/company/tailscale)[](https://hachyderm.io/@tailscale)[](https://www.youtube.com/@Tailscale)

© 2025 Tailscale Inc. All rights reserved. Tailscale is a registered trademark of Tailscale Inc.

![Image 22](https://bat.bing.com/action/0?ti=17188063&Ver=2&mid=579fb98e-d61c-4391-90e2-5bb17bea079a&bo=1&sid=81901900d1e311f0b7bb693f2e3ed78c&vid=81908d00d1e311f096ee354f330133c3&vids=1&msclkid=N&pi=918639831&lg=en-US&sw=800&sh=600&sc=24&tl=Using%20Tailscale%20on%20Azure%20App%20Service%20%C2%B7%20Tailscale%20Docs&p=https%3A%2F%2Ftailscale.com%2Fkb%2F1126%2Fazure-app-service&r=&lt=1258&evt=pageLoad&sv=2&cdb=AQUB&rn=954813)