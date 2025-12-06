Title: Use UFW to lock down an Ubuntu server · Tailscale Docs

URL Source: https://tailscale.com/kb/1077/secure-server-ubuntu

Markdown Content:
Use UFW to lock down an Ubuntu server · Tailscale Docs

===============

![Image 1](https://d.adroll.com/cm/b/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 2](https://d.adroll.com/cm/bombora/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 3](https://d.adroll.com/cm/experian/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 4](https://d.adroll.com/cm/eyeota/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 5](https://d.adroll.com/cm/g/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 6](https://d.adroll.com/cm/index/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 7](https://d.adroll.com/cm/l/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 8](https://d.adroll.com/cm/n/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 9](https://d.adroll.com/cm/o/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 10](https://d.adroll.com/cm/outbrain/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 11](https://d.adroll.com/cm/pubmatic/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 12](https://d.adroll.com/cm/taboola/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 13](https://d.adroll.com/cm/triplelift/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 14](https://d.adroll.com/cm/x/out?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&advertisable=TKO7FOASLRCK5J2S4BRIFC)

![Image 15](https://ipv4.d.adroll.com/seg4/TKO7FOASLRCK5J2S4BRIFC/5J25I7J2IBDGRESKWR4LV3?adroll_fpc=2150be0d487b39bab8fcf63da0d87445-1764944036493&pv=28118643089.383522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&cookie=&adroll_s_ref=&keyw=&p0=4104&adroll_external_data=&adroll_version=2.0)

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

[Docs](https://tailscale.com/kb)›[How-to Guides](https://tailscale.com/kb/1348/guides)›[Solutions](https://tailscale.com/kb/1355/solutions)›Lock down a server

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

[Docs](https://tailscale.com/kb)›[How-to Guides](https://tailscale.com/kb/1348/guides)›[Solutions](https://tailscale.com/kb/1355/solutions)›Lock down a server
Use UFW to lock down an Ubuntu server
=====================================

Any server on the public internet is bound to be attacked by bots looking for weak or leaked passwords and unsafely configured services. Even security experts can misconfigure a database, or an unwitting member of the team can accidentally open up a vulnerability, leaving your devices or network open to attack.

If you have an existing server, you can view this bot traffic by running `sudo less /var/log/auth.log`. If your server is like many on the web, you'll see lots of "invalid user admin" or "invalid user test".

Tailscale simplifies network security by letting you keep your servers away from the public web, while keeping it easy to connect.

The best way to secure a server with Tailscale is to accept connections from Tailscale, and ignore any public internet traffic. Since your Tailscale network is invisible, except to those in your network, attackers won't even be able to find it.

[Prerequisites](https://tailscale.com/kb/1077/secure-server-ubuntu#prerequisites)
---------------------------------------------------------------------------------

Before you begin this guide, you'll need an Ubuntu server to secure. This guide assumes you're setting up a [DigitalOcean Ubuntu server](https://www.digitalocean.com/products/linux-distribution/ubuntu), but the steps should be similar for most hosting providers and versions of Ubuntu.

You'll also need a Tailscale network, known as a tailnet. For information about creating a tailnet, see the [Tailscale quickstart](https://tailscale.com/kb/1017/install).

Next, you'll need to install the Tailscale client on your local machine and log in.

[Download Tailscale](https://tailscale.com/download)
We'll follow the same steps on the Ubuntu server next.

[Step 1: ssh into your new Ubuntu server](https://tailscale.com/kb/1077/secure-server-ubuntu#step-1-ssh-into-your-new-ubuntu-server)
------------------------------------------------------------------------------------------------------------------------------------

After spinning up a new server, ssh into it with your account details.

```shell
ssh <username>@<server host ip>
```

[Step 2: Install Tailscale on your Ubuntu server](https://tailscale.com/kb/1077/secure-server-ubuntu#step-2-install-tailscale-on-your-ubuntu-server)
----------------------------------------------------------------------------------------------------------------------------------------------------

1.   Install Tailscale using the one-line script below, or read our [detailed install instructions for Ubuntu](https://tailscale.com/download/linux)

 
```shell
curl -fsSL https://tailscale.com/install.sh | sh
``` 
2.   Authenticate and connect your machine to your Tailscale network

 
```shell
sudo tailscale up
``` 
3.   (Optional) If you signed in with a custom domain (not a `@gmail.com` address) visit the [admin console](https://login.tailscale.com/admin) and authorize your new endpoint.

4.   (Optional) Disable key expiry for this server

As a security feature, Tailscale requires periodic reauthentication. To prevent getting locked out, you may want to disable expiry on certain endpoints, such as this trusted server. Disable key expiry by following [these instructions](https://tailscale.com/kb/1028/key-expiry).

If you leave key expiry on, be familiar with how to regain server access. For example, DigitalOcean provides access via a [droplet console](https://www.digitalocean.com/docs/droplets/resources/console).

[Step 3: ssh over Tailscale](https://tailscale.com/kb/1077/secure-server-ubuntu#step-3-ssh-over-tailscale)
----------------------------------------------------------------------------------------------------------

An important step —since we're about to restrict ssh access to be only over Tailscale, we'll exit the machine and re-ssh with our Tailscale IP.

First, [find and copy your machine's Tailscale IP](https://tailscale.com/kb/1033/ip-and-dns-addresses). The easiest way to do this is to run

```shell
tailscale ip -4
```

And copy the 100.x.y.z shown.

Once you've found it, `exit` your ssh session, and start a new one with your newly copied Tailscale IP.

```shell
ssh <username>@<copied 100.x.y.z address>
```

[Step 4: Enable UFW](https://tailscale.com/kb/1077/secure-server-ubuntu#step-4-enable-ufw)
------------------------------------------------------------------------------------------

For this guide, we'll use [UFW](https://help.ubuntu.com/community/UFW) (Uncomplicated Firewall) to restrict non-Tailscale traffic to our server. It comes pre-installed on Ubuntu 18.04, so no installation is needed.

Before we continue editing rules, you'll need to enable UFW if it isn't already enabled.

```shell
sudo ufw enable
```

[Step 5: Restrict all other traffic](https://tailscale.com/kb/1077/secure-server-ubuntu#step-5-restrict-all-other-traffic)
--------------------------------------------------------------------------------------------------------------------------

Next, we'll set up rules to reject all incoming non-Tailscale traffic, and allow all outgoing traffic by default.

```shell
sudo ufw default deny incoming
sudo ufw default allow outgoing
```

Now that we've set these defaults check your existing firewall rules you might need to keep.

```shell
sudo ufw status verbose
```

You might see a list of firewall rules, like this:

```shell
To                          Action      From
--                          ------      ----
22/tcp                      ALLOW IN    Anywhere
80/tcp                      ALLOW IN    Anywhere
443/tcp                     ALLOW IN    Anywhere
Anywhere on tailscale0      ALLOW IN    Anywhere
22/tcp (v6)                 ALLOW IN    Anywhere (v6)
80/tcp (v6)                 ALLOW IN    Anywhere (v6)
443/tcp (v6)                ALLOW IN    Anywhere (v6)
Anywhere (v6) on tailscale0 ALLOW IN    Anywhere (v6)
```

If you do not see an `Anywhere on tailscale0` rule, you can create one manually:

```shell
sudo ufw allow in on tailscale0
```

All other connections are denied by default and so not listed above. We want to limit this list to the minimum set needed.

To completely lock down your server while retaining ssh access, you could delete every rule except for the "Anywhere on tailscale0" rule.

For the example above, we'll delete all "22/tcp" rules, which will remove the ability to ssh over regular connections:

```shell
sudo ufw delete 22/tcp
```

Now, only "Anywhere on tailscale0" remains, meaning ssh can only occur over Tailscale.

```shell
To                          Action      From
--                          ------      ----
80/tcp                      ALLOW       Anywhere
443/tcp                     ALLOW       Anywhere
Anywhere on tailscale0      ALLOW IN    Anywhere
80/tcp (v6)                 ALLOW       Anywhere (v6)
443/tcp (v6)                ALLOW       Anywhere (v6)
Anywhere (v6) on tailscale0 ALLOW IN    Anywhere (v6)
```

If you expose a public web service (80/tcp, 443/tcp), you'll want to keep those rules around. For less public services like FTP (21/tcp) or a database, consider connecting devices that rely on those services over Tailscale too.

This guide assumes ssh is running on the default port, port 22. If you've changed your ssh port, you may need to change these instructions as well.

[Step 6: Restart ufw and ssh](https://tailscale.com/kb/1077/secure-server-ubuntu#step-6-restart-ufw-and-ssh)
------------------------------------------------------------------------------------------------------------

Once you've set up firewall rules to restrict all non-Tailscale connections, restart ufw and ssh

```shell
sudo ufw reload
sudo service ssh restart
```

Now your server will ignore any ssh requests that don't come from users authenticated to your private Tailscale network.

[Step 7: Test and verify](https://tailscale.com/kb/1077/secure-server-ubuntu#step-7-test-and-verify)
----------------------------------------------------------------------------------------------------

Let's make sure that everything is working as expected.

First, let's `exit` the existing ssh session.

Then, let's try to connect with the public IP address from earlier.

You should see that we're not able to connect, and the operation times out.

```shell
ssh <username>@<server host ip>
ssh: connect to host <server host ip> port 22: Operation timed out
```

Now, let's try to ssh in using the Tailscale IP address (starting with 100.x.y.z) from earlier.

```shell
ssh <username>@<copied 100.x.y.z address>
```

Everything now works as expected. Type `exit` to close the ssh connection again.

This time, quit the Tailscale client on your local machine.

If you try to `ssh` to the Ubuntu server again, you'll see that the operation now times out and we are no longer able to connect.

```shell
ssh <username>@<copied 100.x.y.z address>
ssh: connect to host <copied 100.x.y.z address> port 22: Operation timed out
```

We've now verified that we can only connect when we're successfully authenticated to the Tailscale client running on our local machine.

[(Optional) enable multifactor authentication (MFA) for all ssh connections](https://tailscale.com/kb/1077/secure-server-ubuntu#optional-enable-multifactor-authentication-mfa-for-all-ssh-connections)
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Now that your server can only be accessed via Tailscale, you can enforce login rules in using your Tailscale network's [identity provider](https://tailscale.com/kb/1013/sso-providers), knowing they will apply to all your ssh connections too.

For example, you may want to configure your identity provider to [require multifactor authentication (MFA)](https://tailscale.com/kb/1075/multifactor-auth) for every sign-in.

Thanks to _/u/mgozmovies_ whose experimentation and [write-up on /r/tailscale](https://old.reddit.com/r/Tailscale/comments/hwnc0l/restricting_ssh_access_to_tailscale_interface_on) inspired this topic.

Last updated Sep 30, 2025

On this page

*   [Prerequisites](https://tailscale.com/kb/1077/secure-server-ubuntu#prerequisites)
*   [Step 1: ssh into your new Ubuntu server](https://tailscale.com/kb/1077/secure-server-ubuntu#step-1-ssh-into-your-new-ubuntu-server)
*   [Step 2: Install Tailscale on your Ubuntu server](https://tailscale.com/kb/1077/secure-server-ubuntu#step-2-install-tailscale-on-your-ubuntu-server)
*   [Step 3: ssh over Tailscale](https://tailscale.com/kb/1077/secure-server-ubuntu#step-3-ssh-over-tailscale)
*   [Step 4: Enable UFW](https://tailscale.com/kb/1077/secure-server-ubuntu#step-4-enable-ufw)
*   [Step 5: Restrict all other traffic](https://tailscale.com/kb/1077/secure-server-ubuntu#step-5-restrict-all-other-traffic)
*   [Step 6: Restart ufw and ssh](https://tailscale.com/kb/1077/secure-server-ubuntu#step-6-restart-ufw-and-ssh)
*   [Step 7: Test and verify](https://tailscale.com/kb/1077/secure-server-ubuntu#step-7-test-and-verify)
*   [(Optional) enable multifactor authentication (MFA) for all ssh connections](https://tailscale.com/kb/1077/secure-server-ubuntu#optional-enable-multifactor-authentication-mfa-for-all-ssh-connections)

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

![Image 18](https://bat.bing.com/action/0?ti=17188063&Ver=2&mid=09e58fff-38f7-455b-b96a-c1d905d1a6ae&bo=1&sid=a266ec40d1e411f084245b0bd70fa304&vid=a2676690d1e411f0a85b133f29f3efca&vids=1&msclkid=N&pi=918639831&lg=en-US&sw=800&sh=600&sc=24&tl=Use%20UFW%20to%20lock%20down%20an%20Ubuntu%20server%20%C2%B7%20Tailscale%20Docs&p=https%3A%2F%2Ftailscale.com%2Fkb%2F1077%2Fsecure-server-ubuntu&r=&lt=621&evt=pageLoad&sv=2&cdb=AQUR&rn=534590)