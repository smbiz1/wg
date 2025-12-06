Title: Access Google Compute Engine VMs privately using Tailscale · Tailscale Docs

URL Source: https://tailscale.com/kb/1147/cloud-gce

Markdown Content:
Access Google Compute Engine VMs privately using Tailscale · Tailscale Docs

===============

![Image 5](https://d.adroll.com/cm/b/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 6](https://d.adroll.com/cm/bombora/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 7](https://d.adroll.com/cm/experian/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 8](https://d.adroll.com/cm/eyeota/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 9](https://d.adroll.com/cm/g/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 10](https://d.adroll.com/cm/index/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 11](https://d.adroll.com/cm/l/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 12](https://d.adroll.com/cm/n/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 13](https://d.adroll.com/cm/o/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 14](https://d.adroll.com/cm/outbrain/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 15](https://d.adroll.com/cm/pubmatic/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 16](https://d.adroll.com/cm/taboola/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 17](https://d.adroll.com/cm/triplelift/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)![Image 18](https://d.adroll.com/cm/x/out?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&advertisable=TKO7FOASLRCK5J2S4BRIFC)

![Image 19](https://ipv4.d.adroll.com/seg4/TKO7FOASLRCK5J2S4BRIFC/5J25I7J2IBDGRESKWR4LV3?adroll_fpc=2252cf2daf5156f005436563a9cf0fb0-1764943513538&pv=70245490018.40942&arrfrr=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&cookie=&adroll_s_ref=&keyw=&p0=5624&adroll_external_data=&adroll_version=2.0)

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

*   [![Image 20: Alt text ](https://cdn.sanity.io/images/w77i7m8x/production/a06dc612b1e3e4f4df53a72030002600639a8738-300x120.png?w=640&q=75&fit=clip&auto=format)Title here How Cribl Enables Secure Work From Anywhere with Tailscale](https://tailscale.com/customers)

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

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Cloud servers](https://tailscale.com/kb/1357/cloud-server)›Google Compute Engine VMs

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

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Cloud servers](https://tailscale.com/kb/1357/cloud-server)›Google Compute Engine VMs
Access Google Compute Engine VMs privately using Tailscale
==========================================================

Google Cloud provides Linux virtual machines and you can use Tailscale to provide secure connectivity.

[Prerequisites](https://tailscale.com/kb/1147/cloud-gce#prerequisites)
----------------------------------------------------------------------

Before you begin this guide, you'll need a Tailscale network (known as a tailnet) set up and configured with at least one existing device. Read our [getting started guide](https://tailscale.com/kb/1017/install) if you need help with this.

[Step 1: Set up the Tailscale client for the VM](https://tailscale.com/kb/1147/cloud-gce#step-1-set-up-the-tailscale-client-for-the-vm)
---------------------------------------------------------------------------------------------------------------------------------------

First, [create a Virtual Machine in the GCE Console](https://cloud.google.com/compute/docs/instances/create-start-instance).

When creating the instance select **Management, security, disks, networking, sole tenancy**, select **Networking**, and select the **Network Interface**. Because we're later going to enable subnet routing on this VM, we want to set **IP forwarding** to **On**.

![Image 21: The Network Interface configurations featuring the IP forwarding setting.](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fgce-ip-forwarding.1b910e77.png&w=1920&q=75)
Once you create the VM, SSH to the system and follow the steps to [install Tailscale on Linux](https://tailscale.com/kb/1031/install-linux).

[Step 2: Allow UDP port 41641](https://tailscale.com/kb/1147/cloud-gce#step-2-allow-udp-port-41641)
---------------------------------------------------------------------------------------------------

If at least one side of a tunnel has "easy NAT," where Tailscale can determine the UDP port number on the far side of the NAT device, then it will make [direct connections to minimize latency.](https://tailscale.com/blog/how-tailscale-works). We ensure that GCE nodes can make direct connections by allowing UDP port `41641` to ingress through the firewall.

In **VPC Network**>**Firewall** we add two rules:

1.   An ingress rule to allow `0.0.0.0/0` for UDP port `41641` to all instances.
2.   An ingress rule to allow `::/0` for UDP port `41641` to all instances.

![Image 22: The firewall rule for an example project featuring Direction of traffic set to Ingress and Protocols and ports set to 41641.](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fgce-firewall-rule.d7563267.png&w=750&q=75)
[Step 3: Advertise routes from the VM](https://tailscale.com/kb/1147/cloud-gce#step-3-advertise-routes-from-the-vm)
-------------------------------------------------------------------------------------------------------------------

To enable connections from your tailnet to the GCP subnet, configure the VM to act as a [subnet router](https://tailscale.com/kb/1019/subnets).

First, enable IP forwarding on the VM.

If your Linux system has a `/etc/sysctl.d` directory, use:

```shell
echo 'net.ipv4.ip_forward = 1' | sudo tee -a /etc/sysctl.d/99-tailscale.conf
echo 'net.ipv6.conf.all.forwarding = 1' | sudo tee -a /etc/sysctl.d/99-tailscale.conf
sudo sysctl -p /etc/sysctl.d/99-tailscale.conf
```

Otherwise, use:

```shell
echo 'net.ipv4.ip_forward = 1' | sudo tee -a /etc/sysctl.conf
echo 'net.ipv6.conf.all.forwarding = 1' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p /etc/sysctl.conf
```

After enabling IP forwarding, configure the VM to advertise routes for the subnet it sits on. For example, if the subnet address range is `10.182.0.0/24`, the command would be:

```shell
tailscale set --advertise-routes=10.182.0.0/24 --accept-dns=false
```

For GCE VMs it is generally best to let Google handle the DNS configuration, not have Tailscale override it, so we added `--accept-dns=false`.

[Step 4: Add GCE DNS for your tailnet](https://tailscale.com/kb/1147/cloud-gce#step-4-add-gce-dns-for-your-tailnet)
-------------------------------------------------------------------------------------------------------------------

For the benefit of the _other_ nodes in the tailnet we'll set up [split DNS](https://tailscale.com/kb/1054/dns#tailscale-dns-settings) to allow use of the same DNS names as the ones that are inside of GCE.

The hostnames inside of GCE are of the form:

```shell
<vm-name>.<gce-project-name>.internal
```

Use the Google Cloud CLI command [`gcloud dns policies create`](https://cloud.google.com/sdk/gcloud/reference/dns/policies/create) to create a new [Cloud DNS](https://cloud.google.com/dns) policy that lets inbound forwarding for your tailnet:

```shell
gcloud dns policies create inbound-dns \
  --project="YOUR_VPC_PROJECT" \
  --description="Expose DNS endpoints per subnet" \
  --networks="YOUR_VPC" \
  --enable-inbound-forwarding
```

where:

*   `YOUR_VPC_PROJECT` is your Google Cloud [project ID](https://cloud.google.com/sdk/gcloud/reference#--project).
*   `YOUR_VPC` is the comma separated list of network names to associate with the policy.

Use the [`gcloud compute addresses list`](https://cloud.google.com/sdk/gcloud/reference/compute/addresses/list) to verify that your tailnet recognizes the DNS resolver for your tailnet subnet:

```shell
gcloud compute addresses list \
  --project="YOUR_VPC_PROJECT" \
  --filter='purpose="DNS_RESOLVER"' \
  --format='csv(address, region, subnetwork)' \
  | grep YOUR_TAILNET_SUBNET
```

where:

*   `YOUR_VPC_PROJECT` is your Google Cloud [project ID](https://cloud.google.com/sdk/gcloud/reference#--project).
*   `YOUR_TAILNET_SUBNET` is your subnet machine name.

Use the IP address returned from this command as a DNS resolver for your tailnet:

1.   Open the [DNS](https://login.tailscale.com/admin/dns) page in the admin console.

2.   Select **Add name server**.

3.   Select **Custom**.

4.   For **Nameserver**, enter the IP address from the `gcloud compute addresses list` command that you ran above. In this example, we use `10.243.117.59`.

5.   Ensure **Restrict to search domain** is checked.

6.   For **Search Domain**, enter **internal**.

7.   Select **Save**.

![Image 23: The Add nameserver configuration with an IP address and Search Domain set to internal.](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fgce-add-nameserver.4bd56bd3.png&w=3840&q=75)

Now the same hostnames which work between nodes running within GCE will also be available to all nodes in your tailnet.

[Step 5: Remove public SSH access](https://tailscale.com/kb/1147/cloud-gce#step-5-remove-public-ssh-access)
-----------------------------------------------------------------------------------------------------------

As we can now SSH to the system over the private Tailscale network, there is no reason to leave the SSH port open on a public IP address. You can delete the `default-allow-ssh` rule from **VPC network**>**Firewall**.

![Image 24: The firewall rule details for an example project highlighting a the Delete command in the top right.](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fgce-remove-ssh.fc7a024a.png&w=3840&q=75)

Last updated Dec 4, 2025

On this page

*   [Prerequisites](https://tailscale.com/kb/1147/cloud-gce#prerequisites)
*   [Step 1: Set up the Tailscale client for the VM](https://tailscale.com/kb/1147/cloud-gce#step-1-set-up-the-tailscale-client-for-the-vm)
*   [Step 2: Allow UDP port 41641](https://tailscale.com/kb/1147/cloud-gce#step-2-allow-udp-port-41641)
*   [Step 3: Advertise routes from the VM](https://tailscale.com/kb/1147/cloud-gce#step-3-advertise-routes-from-the-vm)
*   [Step 4: Add GCE DNS for your tailnet](https://tailscale.com/kb/1147/cloud-gce#step-4-add-gce-dns-for-your-tailnet)
*   [Step 5: Remove public SSH access](https://tailscale.com/kb/1147/cloud-gce#step-5-remove-public-ssh-access)

Related Pages

*   [Subnet routers](https://tailscale.com/kb/1019/subnets)
*   [Connect to an AWS VPC using subnet routes](https://tailscale.com/kb/1021/install-aws)
*   [Connect to external services with IP block lists](https://tailscale.com/kb/1059/ip-blocklist-relays)
*   [Tailscale on Google Cloud Run](https://tailscale.com/kb/1108/cloudrun)
*   [Connecting without installing Tailscale](https://tailscale.com/kb/1109/devices-without-tailscale)

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

[Terms of Service](https://tailscale.com/terms)[Privacy Policy](https://tailscale.com/privacy-policy)[California Notice](https://tailscale.com/privacy-policy#california-notice)[Cookie Notice](https://tailscale.com/cookie-notice)![Image 25: Check mark and x on a white and blue pill button](https://cdn.sanity.io/images/w77i7m8x/production/07d853f507039b2489d9818cb6ee7442c1b60e2a-30x14.svg)Your Privacy Choices

WireGuard is a registered trademark of Jason A. Donenfeld.

[](https://twitter.com/tailscale)[](https://www.facebook.com/tailscale/)[](https://www.linkedin.com/company/tailscale)[](https://hachyderm.io/@tailscale)[](https://www.youtube.com/@Tailscale)

© 2025 Tailscale Inc. All rights reserved. Tailscale is a registered trademark of Tailscale Inc.

![Image 26](https://bat.bing.com/action/0?ti=17188063&Ver=2&mid=01bf545c-5e62-4e4d-bea3-79ce3b44b101&bo=1&sid=69ed7330d1e311f092f48ff1d355a5be&vid=69ed89e0d1e311f0b3849bc1defbcec1&vids=1&msclkid=N&pi=918639831&lg=en-US&sw=800&sh=600&sc=24&tl=Access%20Google%20Compute%20Engine%20VMs%20privately%20using%20Tailscale%20%C2%B7%20Tailscale%20Docs&p=https%3A%2F%2Ftailscale.com%2Fkb%2F1147%2Fcloud-gce&r=&lt=1949&evt=pageLoad&sv=2&cdb=AQUR&rn=355274)