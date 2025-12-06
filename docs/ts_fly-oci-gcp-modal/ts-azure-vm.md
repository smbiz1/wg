Title: Access Azure Linux VMs privately using Tailscale · Tailscale Docs

URL Source: https://tailscale.com/kb/1142/cloud-azure-linux

Markdown Content:
Access Azure Linux VMs privately using Tailscale · Tailscale Docs

===============

![Image 4](https://d.adroll.com/cm/b/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 5](https://d.adroll.com/cm/bombora/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 6](https://d.adroll.com/cm/experian/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 7](https://d.adroll.com/cm/eyeota/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 8](https://d.adroll.com/cm/g/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 9](https://d.adroll.com/cm/index/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 10](https://d.adroll.com/cm/l/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 11](https://d.adroll.com/cm/n/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 12](https://d.adroll.com/cm/o/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 13](https://d.adroll.com/cm/outbrain/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 14](https://d.adroll.com/cm/pubmatic/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 15](https://d.adroll.com/cm/taboola/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 16](https://d.adroll.com/cm/triplelift/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 17](https://d.adroll.com/cm/x/out?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&advertisable=TKO7FOASLRCK5J2S4BRIFC)

![Image 18](https://ipv4.d.adroll.com/seg4/TKO7FOASLRCK5J2S4BRIFC/5J25I7J2IBDGRESKWR4LV3?adroll_fpc=5bf87832d36057a46a5f2595d000fe98-1764943551780&pv=39706399534.99522&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&cookie=&adroll_s_ref=&keyw=&p0=3579&adroll_external_data=&adroll_version=2.0)

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

*   [![Image 19: Alt text ](https://cdn.sanity.io/images/w77i7m8x/production/a06dc612b1e3e4f4df53a72030002600639a8738-300x120.png?w=640&q=75&fit=clip&auto=format)Title here How Cribl Enables Secure Work From Anywhere with Tailscale](https://tailscale.com/customers)

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

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Cloud servers](https://tailscale.com/kb/1357/cloud-server)›Azure Linux VMs

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

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Cloud servers](https://tailscale.com/kb/1357/cloud-server)›Azure Linux VMs
Access Azure Linux VMs privately using Tailscale
================================================

Microsoft Azure is a cloud service provider offering Linux and Windows virtual machines, to which Tailscale can be used to provide secure connectivity. This topic covers Linux VMs running within Azure. For Windows VMs, see [Access Azure Windows VMs privately using Tailscale](https://tailscale.com/kb/1143/azure-windows).

[Prerequisites](https://tailscale.com/kb/1142/cloud-azure-linux#prerequisites)
------------------------------------------------------------------------------

Before you begin this guide, you'll need a Tailscale network set up and configured with at least one existing device. Read our [getting started guide](https://tailscale.com/kb/1017/install) if you need help with this.

[Step 1: Set up the Tailscale client for Linux VMs](https://tailscale.com/kb/1142/cloud-azure-linux#step-1-set-up-the-tailscale-client-for-linux-vms)
-----------------------------------------------------------------------------------------------------------------------------------------------------

First, [create a Virtual Machine in the Azure Portal](https://portal.azure.com/#blade/HubsExtension/BrowseResource/resourceType/Microsoft.Compute%2FVirtualMachines) running Linux. Tailscale supports [many of the Linux distributions](https://tailscale.com/kb/1031/install-linux) offered by Azure Marketplace images.

If at least one side of a tunnel has "easy NAT," where Tailscale can determine the UDP port number on the far side of the NAT device, then it will make [direct connections to minimize latency.](https://tailscale.com/blog/how-tailscale-works) We ensure that the Azure nodes can make direct connections by allowing UDP port 41641 to ingress through the firewall.

In the Networking step of setting up the VM, choose Advanced for the NIC network security group and create a network security policy to allow UDP port 41641 to ingress.

![Image 20: Network Security Group allow port 41641](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fnetwork-security-group.079689c7.jpg&w=3840&q=75)
Then ssh to the system and follow the steps to [install Tailscale on Linux](https://tailscale.com/kb/1031/install-linux).

[Step 2: Advertise routes from the VM](https://tailscale.com/kb/1142/cloud-azure-linux#step-2-advertise-routes-from-the-vm)
---------------------------------------------------------------------------------------------------------------------------

For the benefit of the _other_ nodes in the tailnet we'll set up [split DNS](https://tailscale.com/kb/1054/dns#tailscale-dns-settings) to allow use of the same DNS names as are used inside of Azure. The [Azure DNS server address is 168.63.129.16](https://docs.microsoft.com/en-us/azure/virtual-network/virtual-networks-name-resolution-for-vms-and-role-instances#azure-provided-name-resolution), which is an Anycasted address that will go to the nearest DNS server within Azure.

We'll have our VM advertise routes for both the subnet it sits on as well as the Azure DNS server. For example, if the subnet address range is 10.1.0.0/24, the command would be:

```shell
tailscale set --advertise-routes=10.1.0.0/24,168.63.129.16/32 --accept-dns=false
```

For Azure VMs it is generally best to let Azure handle the DNS configuration, not have Tailscale override it, so we added `--accept-dns=false`.

[Step 3: Add Azure DNS for your tailnet](https://tailscale.com/kb/1142/cloud-azure-linux#step-3-add-azure-dns-for-your-tailnet)
-------------------------------------------------------------------------------------------------------------------------------

In the [DNS](https://login.tailscale.com/admin/dns) page of the admin console we add a nameserver restricted to the `internal.cloudapp.net` domain, pointing to the Azure DNS server which we made available through our VM.

![Image 21: Adding a Split DNS resolver for internal.cloudapp.net](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fazure-add-DNS.35617d9c.jpg&w=3840&q=75)
Now the same hostnames which work between nodes running within Azure will also be available to all nodes on our tailnet.

[Step 4: Remove public SSH access](https://tailscale.com/kb/1142/cloud-azure-linux#step-4-remove-public-ssh-access)
-------------------------------------------------------------------------------------------------------------------

As we can now ssh to the system over the private Tailscale network, there is no reason to leave the SSH port open on a public IP address. In the Settings > Network tab select the ingress rule for "SSH" and delete it.

![Image 22: Disable public SSH port.](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fazure-disable-ssh.a5877f02.jpg&w=3840&q=75)

Last updated Dec 4, 2025

On this page

*   [Prerequisites](https://tailscale.com/kb/1142/cloud-azure-linux#prerequisites)
*   [Step 1: Set up the Tailscale client for Linux VMs](https://tailscale.com/kb/1142/cloud-azure-linux#step-1-set-up-the-tailscale-client-for-linux-vms)
*   [Step 2: Advertise routes from the VM](https://tailscale.com/kb/1142/cloud-azure-linux#step-2-advertise-routes-from-the-vm)
*   [Step 3: Add Azure DNS for your tailnet](https://tailscale.com/kb/1142/cloud-azure-linux#step-3-add-azure-dns-for-your-tailnet)
*   [Step 4: Remove public SSH access](https://tailscale.com/kb/1142/cloud-azure-linux#step-4-remove-public-ssh-access)

Related Pages

*   [Subnet routers](https://tailscale.com/kb/1019/subnets)
*   [Connect to an AWS VPC using subnet routes](https://tailscale.com/kb/1021/install-aws)
*   [Connect to external services with IP block lists](https://tailscale.com/kb/1059/ip-blocklist-relays)
*   [Connecting without installing Tailscale](https://tailscale.com/kb/1109/devices-without-tailscale)
*   [Set up high availability](https://tailscale.com/kb/1115/high-availability)

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

[Terms of Service](https://tailscale.com/terms)[Privacy Policy](https://tailscale.com/privacy-policy)[California Notice](https://tailscale.com/privacy-policy#california-notice)[Cookie Notice](https://tailscale.com/cookie-notice)![Image 23: Check mark and x on a white and blue pill button](https://cdn.sanity.io/images/w77i7m8x/production/07d853f507039b2489d9818cb6ee7442c1b60e2a-30x14.svg)Your Privacy Choices

WireGuard is a registered trademark of Jason A. Donenfeld.

[](https://twitter.com/tailscale)[](https://www.facebook.com/tailscale/)[](https://www.linkedin.com/company/tailscale)[](https://hachyderm.io/@tailscale)[](https://www.youtube.com/@Tailscale)

© 2025 Tailscale Inc. All rights reserved. Tailscale is a registered trademark of Tailscale Inc.

![Image 24](https://bat.bing.com/action/0?ti=17188063&Ver=2&mid=63f7f232-e553-4705-8b7d-a52c934b7c18&bo=1&sid=81b43770d1e311f0bc6a937ace7beb27&vid=81b45260d1e311f0bd44b34d2b519830&vids=1&msclkid=N&pi=918639831&lg=en-US&sw=800&sh=600&sc=24&tl=Access%20Azure%20Linux%20VMs%20privately%20using%20Tailscale%20%C2%B7%20Tailscale%20Docs&p=https%3A%2F%2Ftailscale.com%2Fkb%2F1142%2Fcloud-azure-linux&r=&lt=697&evt=pageLoad&sv=2&cdb=AQUR&rn=278119)