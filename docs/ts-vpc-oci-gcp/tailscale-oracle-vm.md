Title: Access Oracle Cloud VMs privately using Tailscale · Tailscale Docs

URL Source: https://tailscale.com/kb/1149/cloud-oracle

Markdown Content:
Access Oracle Cloud VMs privately using Tailscale · Tailscale Docs
===============

[](https://tailscale.com/ "Homepage")

Product

Solutions

[Enterprise](https://tailscale.com/enterprise)[Customers](https://tailscale.com/customers)[Docs](https://tailscale.com/kb/1017/install)[Blog](https://tailscale.com/blog)[Pricing](https://tailscale.com/pricing)

[Download](https://tailscale.com/download)[Log in](https://login.tailscale.com/welcome)[Get started](https://login.tailscale.com/start)

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
*   [Partnerships](https://tailscale.com/partnerships)

Solutions

By use-case

*   [Remote Access](https://tailscale.com/use-cases/remote-access)
*   [Site-to-site Networking](https://tailscale.com/use-cases/site-to-site-networking)
*   [Multi-Cloud Networking](https://tailscale.com/use-cases/multi-cloud-networking)
*   [Kubernetes Networking](https://tailscale.com/use-cases/kubernetes)
*   [Edge & IoT Deployments](https://tailscale.com/use-cases/iot)
*   [Zero Trust Networking](https://tailscale.com/use-cases/zero-trust-networking)
*   [AI Workloads](https://tailscale.com/use-cases/ai)
*   [Secure SaaS](https://tailscale.com/use-cases/secure-saas)
*   [Business VPN](https://tailscale.com/use-cases/business-vpn)
*   [Homelab](https://tailscale.com/use-cases/homelab)

By role

*   [DevOps](https://tailscale.com/solutions/devops)
*   [IT](https://tailscale.com/solutions/it)
*   [Security](https://tailscale.com/solutions/security)

[Enterprise](https://tailscale.com/enterprise)

[Customers](https://tailscale.com/customers)

[Docs](https://tailscale.com/kb/1017/install)

[Blog](https://tailscale.com/blog)

[Pricing](https://tailscale.com/pricing)

[Download](https://tailscale.com/download)

[Get started](https://login.tailscale.com/start)[Login](https://login.tailscale.com/welcome)

© 2025

*   [Start](https://tailscale.com/kb/1346/start)
    *   [Quickstart](https://tailscale.com/kb/1017/install)
    *   [Install Tailscale](https://tailscale.com/kb/1347/installation)
    *   [Quick guides](https://tailscale.com/kb/1415/quick-guides)
    *   [Set up an identity provider](https://tailscale.com/kb/1013/sso-providers)
    *   [What is Tailscale?](https://tailscale.com/kb/1151/what-is-tailscale)
*   [How-to Guides](https://tailscale.com/kb/1348/guides)
    *   [Manage Access](https://tailscale.com/kb/1350/manage)
        *   [Manage access control](https://tailscale.com/kb/1393/access-control)
        *   [Manage devices](https://tailscale.com/kb/1372/manage-devices)
        *   [Manage users](https://tailscale.com/kb/1373/manage-users)
        *   [Tailnet lock](https://tailscale.com/kb/1226/tailnet-lock)
    *   [Route Traffic](https://tailscale.com/kb/1351/route)
        *   [Set up a subnet router](https://tailscale.com/kb/1019/subnets)
        *   [Set up an exit node](https://tailscale.com/kb/1103/exit-nodes)
        *   [Set up an app connector](https://tailscale.com/kb/1281/app-connectors)
        *   [Use DNS](https://tailscale.com/kb/1054/dns)
        *   [Set up MagicDNS](https://tailscale.com/kb/1081/magicdns)
        *   [Set up high availability](https://tailscale.com/kb/1115/high-availability)
    *   [Set Up Servers](https://tailscale.com/kb/1352/servers)
        *   [Set up a server](https://tailscale.com/kb/1245/set-up-servers)
        *   [Use tags](https://tailscale.com/kb/1068/tags)
        *   [Install Tailscale with cloud-init](https://tailscale.com/kb/1293/cloud-init)
        *   [Use auth keys](https://tailscale.com/kb/1085/auth-keys)
        *   [Use Tailscale SSH](https://tailscale.com/kb/1193/tailscale-ssh)
        *   [Set up HTTPS certificates](https://tailscale.com/kb/1153/enabling-https)
        *   [Run an ephemeral node](https://tailscale.com/kb/1111/ephemeral-nodes)
        *   [Run unattended](https://tailscale.com/kb/1088/run-unattended)
    *   [Access & Share Services](https://tailscale.com/kb/1354/share)
        *   [View services](https://tailscale.com/kb/1100/services)
        *   [Share nodes](https://tailscale.com/kb/1084/sharing)
        *   [Use Taildrop](https://tailscale.com/kb/1106/taildrop)
    *   [Share a web server](https://tailscale.com/kb/1353/share-web-server)
        *   [Tailscale Funnel](https://tailscale.com/kb/1223/funnel)
        *   [Tailscale Serve](https://tailscale.com/kb/1312/serve)
    *   [Solutions](https://tailscale.com/kb/1355/solutions)
        *   [Code from your iPad](https://tailscale.com/kb/1166/vscode-ipad)
        *   [Lock down a server](https://tailscale.com/kb/1077/secure-server-ubuntu)
        *   [Access a PiKVM](https://tailscale.com/kb/1292/pikvm)
        *   [Run a Pi-hole](https://tailscale.com/kb/1114/pi-hole)
        *   [Secure external services](https://tailscale.com/kb/1059/ip-blocklist-relays)
        *   [Just-in-time access](https://tailscale.com/kb/1443/just-in-time-access)
        *   [On-demand access](https://tailscale.com/kb/1374/ondemand-access)
        *   [Automation](https://tailscale.com/kb/1430/automations)
*   [Integrations](https://tailscale.com/kb/1356/integrations)
    *   [Cloud servers](https://tailscale.com/kb/integrations/cloud-server)
        *   [AWS Lightsail](https://tailscale.com/kb/1128/aws-lightsail)
        *   [AWS VPC](https://tailscale.com/kb/1021/install-aws)
        *   [Azure App Service](https://tailscale.com/kb/1126/azure-app-service)
        *   [Azure Linux VMs](https://tailscale.com/kb/1142/cloud-azure-linux)
        *   [Azure Windows VMs](https://tailscale.com/kb/1143/cloud-azure-windows)
        *   [Google Compute Engine VMs](https://tailscale.com/kb/1147/cloud-gce)
        *   [Hetzner VMs](https://tailscale.com/kb/1150/cloud-hetzner)
        *   [Oracle Cloud VMs](https://tailscale.com/kb/1149/cloud-oracle)
    *   [Containers and virtualization](https://tailscale.com/kb/1358/containers-and-virtualization)
    *   [Serverless apps](https://tailscale.com/kb/1364/serverless)
    *   [Databases](https://tailscale.com/kb/1359/database)
    *   [Remote environments](https://tailscale.com/kb/1363/remote-code)
    *   [Developer tools](https://tailscale.com/kb/1360/developer-tools)
    *   [Firewalls](https://tailscale.com/kb/1361/firewall)
    *   [Web servers](https://tailscale.com/kb/1365/webserver)
    *   [NAS](https://tailscale.com/kb/1307/nas)
*   [FAQ](https://tailscale.com/kb/1366/faq)
*   [Logging, Streaming, and Events](https://tailscale.com/kb/1349/log-events)
    *   [Logging overview](https://tailscale.com/kb/1011/log-mesh-traffic)
    *   [Configuration audit logging](https://tailscale.com/kb/1203/audit-logging)
    *   [Network flow logs](https://tailscale.com/kb/1219/network-flow-logs)
    *   [Log streaming](https://tailscale.com/kb/1255/log-streaming)
    *   [SSH session recording](https://tailscale.com/kb/1246/tailscale-ssh-session-recording)
    *   [Client metrics](https://tailscale.com/kb/1482/client-metrics)
    *   [Webhooks](https://tailscale.com/kb/1213/webhooks)
*   [Manage Your Organization](https://tailscale.com/kb/1431/manage-account)
    *   [Contact preferences](https://tailscale.com/kb/1224/contract-preferences)
    *   [Pricing and billing](https://tailscale.com/kb/1375/pb-lp)
    *   [Tailnet name](https://tailscale.com/kb/1217/tailnet-name)
    *   [Domain ownership](https://tailscale.com/kb/1259/domain-ownership)
*   [Reference](https://tailscale.com/kb/1367/reference)
    *   [ACL syntax](https://tailscale.com/kb/1337/acl-syntax)
    *   [ACL samples](https://tailscale.com/kb/1192/acl-samples)
    *   [CLI](https://tailscale.com/kb/1080/cli)
    *   [API](https://tailscale.com/kb/1101/api)
    *   [Key prefixes](https://tailscale.com/kb/1277/key-prefixes)
    *   [Production best practices](https://tailscale.com/kb/1300/production-best-practices)
    *   [Shared responsibility](https://tailscale.com/kb/1212/shared-responsibility)
    *   [Technical overviews](https://tailscale.com/kb/1376/tech-overviews)
    *   [Terminology and concepts](https://tailscale.com/kb/1155/terminology-and-concepts)
    *   [GitHub ↗](https://github.com/tailscale/tailscale)
*   [Get Support](https://tailscale.com/kb/1432/get-support)
    *   [Troubleshooting](https://tailscale.com/kb/1023/troubleshooting)
    *   [Support options](https://tailscale.com/kb/1250/support-options)
    *   [Contact support ↗](https://tailscale.com/contact/support)
    *   [Generate a bug report](https://tailscale.com/kb/1227/bug-report)
*   [Resources](https://tailscale.com/kb/1368/resources)
    *   [Changelog ↗](https://tailscale.com/changelog)
    *   [Comparisons ↗](https://tailscale.com/compare)
    *   [Release stages](https://tailscale.com/kb/1167/release-stages)
    *   [Security ↗](https://tailscale.com/security)
    *   [Versions](https://tailscale.com/kb/1168/versions)
    *   [Use cases](https://tailscale.com/kb/1377/use-cases)
    *   [Invite only features](https://tailscale.com/kb/1222/invite-only-feature)

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Cloud servers](https://tailscale.com/kb/integrations/cloud-server)›Oracle Cloud VMs

### Documentation

*   [Start](https://tailscale.com/kb/1346/start)
    *   [Quickstart](https://tailscale.com/kb/1017/install)
    *   [Install Tailscale](https://tailscale.com/kb/1347/installation)
    *   [Quick guides](https://tailscale.com/kb/1415/quick-guides)
    *   [Set up an identity provider](https://tailscale.com/kb/1013/sso-providers)
    *   [What is Tailscale?](https://tailscale.com/kb/1151/what-is-tailscale)
*   [How-to Guides](https://tailscale.com/kb/1348/guides)
    *   [Manage Access](https://tailscale.com/kb/1350/manage)
        *   [Manage access control](https://tailscale.com/kb/1393/access-control)
        *   [Manage devices](https://tailscale.com/kb/1372/manage-devices)
        *   [Manage users](https://tailscale.com/kb/1373/manage-users)
        *   [Tailnet lock](https://tailscale.com/kb/1226/tailnet-lock)
    *   [Route Traffic](https://tailscale.com/kb/1351/route)
        *   [Set up a subnet router](https://tailscale.com/kb/1019/subnets)
        *   [Set up an exit node](https://tailscale.com/kb/1103/exit-nodes)
        *   [Set up an app connector](https://tailscale.com/kb/1281/app-connectors)
        *   [Use DNS](https://tailscale.com/kb/1054/dns)
        *   [Set up MagicDNS](https://tailscale.com/kb/1081/magicdns)
        *   [Set up high availability](https://tailscale.com/kb/1115/high-availability)
    *   [Set Up Servers](https://tailscale.com/kb/1352/servers)
        *   [Set up a server](https://tailscale.com/kb/1245/set-up-servers)
        *   [Use tags](https://tailscale.com/kb/1068/tags)
        *   [Install Tailscale with cloud-init](https://tailscale.com/kb/1293/cloud-init)
        *   [Use auth keys](https://tailscale.com/kb/1085/auth-keys)
        *   [Use Tailscale SSH](https://tailscale.com/kb/1193/tailscale-ssh)
        *   [Set up HTTPS certificates](https://tailscale.com/kb/1153/enabling-https)
        *   [Run an ephemeral node](https://tailscale.com/kb/1111/ephemeral-nodes)
        *   [Run unattended](https://tailscale.com/kb/1088/run-unattended)
    *   [Access & Share Services](https://tailscale.com/kb/1354/share)
        *   [View services](https://tailscale.com/kb/1100/services)
        *   [Share nodes](https://tailscale.com/kb/1084/sharing)
        *   [Use Taildrop](https://tailscale.com/kb/1106/taildrop)
    *   [Share a web server](https://tailscale.com/kb/1353/share-web-server)
        *   [Tailscale Funnel](https://tailscale.com/kb/1223/funnel)
        *   [Tailscale Serve](https://tailscale.com/kb/1312/serve)
    *   [Solutions](https://tailscale.com/kb/1355/solutions)
        *   [Code from your iPad](https://tailscale.com/kb/1166/vscode-ipad)
        *   [Lock down a server](https://tailscale.com/kb/1077/secure-server-ubuntu)
        *   [Access a PiKVM](https://tailscale.com/kb/1292/pikvm)
        *   [Run a Pi-hole](https://tailscale.com/kb/1114/pi-hole)
        *   [Secure external services](https://tailscale.com/kb/1059/ip-blocklist-relays)
        *   [Just-in-time access](https://tailscale.com/kb/1443/just-in-time-access)
        *   [On-demand access](https://tailscale.com/kb/1374/ondemand-access)
        *   [Automation](https://tailscale.com/kb/1430/automations)
*   [Integrations](https://tailscale.com/kb/1356/integrations)
    *   [Cloud servers](https://tailscale.com/kb/integrations/cloud-server)
        *   [AWS Lightsail](https://tailscale.com/kb/1128/aws-lightsail)
        *   [AWS VPC](https://tailscale.com/kb/1021/install-aws)
        *   [Azure App Service](https://tailscale.com/kb/1126/azure-app-service)
        *   [Azure Linux VMs](https://tailscale.com/kb/1142/cloud-azure-linux)
        *   [Azure Windows VMs](https://tailscale.com/kb/1143/cloud-azure-windows)
        *   [Google Compute Engine VMs](https://tailscale.com/kb/1147/cloud-gce)
        *   [Hetzner VMs](https://tailscale.com/kb/1150/cloud-hetzner)
        *   [Oracle Cloud VMs](https://tailscale.com/kb/1149/cloud-oracle)
    *   [Containers and virtualization](https://tailscale.com/kb/1358/containers-and-virtualization)
    *   [Serverless apps](https://tailscale.com/kb/1364/serverless)
    *   [Databases](https://tailscale.com/kb/1359/database)
    *   [Remote environments](https://tailscale.com/kb/1363/remote-code)
    *   [Developer tools](https://tailscale.com/kb/1360/developer-tools)
    *   [Firewalls](https://tailscale.com/kb/1361/firewall)
    *   [Web servers](https://tailscale.com/kb/1365/webserver)
    *   [NAS](https://tailscale.com/kb/1307/nas)
*   [FAQ](https://tailscale.com/kb/1366/faq)
*   [Logging, Streaming, and Events](https://tailscale.com/kb/1349/log-events)
    *   [Logging overview](https://tailscale.com/kb/1011/log-mesh-traffic)
    *   [Configuration audit logging](https://tailscale.com/kb/1203/audit-logging)
    *   [Network flow logs](https://tailscale.com/kb/1219/network-flow-logs)
    *   [Log streaming](https://tailscale.com/kb/1255/log-streaming)
    *   [SSH session recording](https://tailscale.com/kb/1246/tailscale-ssh-session-recording)
    *   [Client metrics](https://tailscale.com/kb/1482/client-metrics)
    *   [Webhooks](https://tailscale.com/kb/1213/webhooks)
*   [Manage Your Organization](https://tailscale.com/kb/1431/manage-account)
    *   [Contact preferences](https://tailscale.com/kb/1224/contract-preferences)
    *   [Pricing and billing](https://tailscale.com/kb/1375/pb-lp)
    *   [Tailnet name](https://tailscale.com/kb/1217/tailnet-name)
    *   [Domain ownership](https://tailscale.com/kb/1259/domain-ownership)
*   [Reference](https://tailscale.com/kb/1367/reference)
    *   [ACL syntax](https://tailscale.com/kb/1337/acl-syntax)
    *   [ACL samples](https://tailscale.com/kb/1192/acl-samples)
    *   [CLI](https://tailscale.com/kb/1080/cli)
    *   [API](https://tailscale.com/kb/1101/api)
    *   [Key prefixes](https://tailscale.com/kb/1277/key-prefixes)
    *   [Production best practices](https://tailscale.com/kb/1300/production-best-practices)
    *   [Shared responsibility](https://tailscale.com/kb/1212/shared-responsibility)
    *   [Technical overviews](https://tailscale.com/kb/1376/tech-overviews)
    *   [Terminology and concepts](https://tailscale.com/kb/1155/terminology-and-concepts)
    *   [GitHub ↗](https://github.com/tailscale/tailscale)
*   [Get Support](https://tailscale.com/kb/1432/get-support)
    *   [Troubleshooting](https://tailscale.com/kb/1023/troubleshooting)
    *   [Support options](https://tailscale.com/kb/1250/support-options)
    *   [Contact support ↗](https://tailscale.com/contact/support)
    *   [Generate a bug report](https://tailscale.com/kb/1227/bug-report)
*   [Resources](https://tailscale.com/kb/1368/resources)
    *   [Changelog ↗](https://tailscale.com/changelog)
    *   [Comparisons ↗](https://tailscale.com/compare)
    *   [Release stages](https://tailscale.com/kb/1167/release-stages)
    *   [Security ↗](https://tailscale.com/security)
    *   [Versions](https://tailscale.com/kb/1168/versions)
    *   [Use cases](https://tailscale.com/kb/1377/use-cases)
    *   [Invite only features](https://tailscale.com/kb/1222/invite-only-feature)

[Docs](https://tailscale.com/kb)›[Integrations](https://tailscale.com/kb/1356/integrations)›[Cloud servers](https://tailscale.com/kb/integrations/cloud-server)›Oracle Cloud VMs

Access Oracle Cloud VMs privately using Tailscale
=================================================

Oracle Cloud provides Linux virtual machines, notably featuring the ARM CPU architecture. We can use Tailscale to securely access Oracle virtual machines.

[Prerequisites](https://tailscale.com/kb/1149/cloud-oracle#prerequisites)
-------------------------------------------------------------------------

Before you begin this guide, you'll need a Tailscale network set up and configured with at least one existing device. Read our [getting started guide](https://tailscale.com/kb/1017/install) if you need help with this.

[Step 1: Set up the Tailscale client for the VM](https://tailscale.com/kb/1149/cloud-oracle#step-1-set-up-the-tailscale-client-for-the-vm)
------------------------------------------------------------------------------------------------------------------------------------------

First, [create a Virtual Machine in the OCN Console](https://cloud.oracle.com/compute/instances/create).

ssh to the system and follow the steps to [install Tailscale on Oracle Linux](https://tailscale.com/kb/1031/install-linux).

[Step 2: Allow UDP port 41641](https://tailscale.com/kb/1149/cloud-oracle#step-2-allow-udp-port-41641)
------------------------------------------------------------------------------------------------------

If at least one side of a tunnel has "easy NAT," where Tailscale can determine the UDP port number on the far side of the NAT device, then it will make [direct connections to minimize latency.](https://tailscale.com/blog/how-tailscale-works) We ensure that OCN nodes can make direct connections by allowing UDP port 41641 to ingress through the firewall.

In the [Networking tab](https://cloud.oracle.com/networking/vcns) select **Virtual Cloud Networks** select the specific VCN the VM has been created on.

![Image 11: Virtual Cloud Networks list](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Focn-virtual-networks.3e00ae29.jpg&w=3840&q=75)

Select **Security Lists** in the left hand column, and the security list in use (probably a Default).

![Image 12: Security lists](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Focn-security-lists.cbdef29c.jpg&w=1920&q=75)

Add a Stateless ingress rule for 0.0.0.0/0 UDP port 41641.

![Image 13: Adding an Ingress Rule](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Focn-ingress-rule.c4c62704.jpg&w=3840&q=75)

[Step 3: Advertise routes from the VM](https://tailscale.com/kb/1149/cloud-oracle#step-3-advertise-routes-from-the-vm)
----------------------------------------------------------------------------------------------------------------------

For the benefit of the _other_ nodes in the tailnet we'll set up [split DNS](https://tailscale.com/kb/1054/dns#tailscale-dns-settings) to allow use of the same DNS names as are used inside of Oracle Cloud. The DNS server provided by Oracle is 169.254.169.254, and supports hostnames of the form instance.subnet01234567.vcn01234567.oraclevcn.com.

We'll have our VM advertise routes for both the subnet it sits on as well as the Oracle DNS server. For example, if the subnet address range is 10.0.0.0/24, the command would be:

```markup
tailscale up --advertise-routes=10.0.0.0/24,169.254.169.254/32 --accept-dns=false
```

For Oracle Cloud VMs it is generally best to let Oracle handle the DNS configuration, not have Tailscale override it, so we added `--accept-dns=false`.

[Step 4: Add Oracle DNS for your tailnet](https://tailscale.com/kb/1149/cloud-oracle#step-4-add-oracle-dns-for-your-tailnet)
----------------------------------------------------------------------------------------------------------------------------

In the [admin console DNS section](https://login.tailscale.com/admin/dns) we add a nameserver restricted to the `oraclevcn.com` domain, pointing to the Oracle DNS server which we made available through our VM.

![Image 14: Adding a Split DNS resolver for oraclevcn.com](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Focn-nameserver.d9120af0.jpg&w=3840&q=75)

Now the same hostnames which work between nodes running within Oracle Cloud will also be available to all nodes on our tailnet.

[Step 5: Remove public SSH access](https://tailscale.com/kb/1149/cloud-oracle#step-5-remove-public-ssh-access)
--------------------------------------------------------------------------------------------------------------

As we can now ssh to the system over the private Tailscale network, there is no reason to leave the SSH port open on a public IP address. In the Network \> Security List the SSH rule can be removed.

![Image 15: Disable public SSH port.](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Focn-remove-ssh.8e01a96c.png&w=3840&q=75)

[Troubleshooting](https://tailscale.com/kb/1149/cloud-oracle#troubleshooting)
-----------------------------------------------------------------------------

If you find that your tailnet nodes cannot access your Oracle Cloud Linux VM, you may need to update the VM's `iptables` configuration.

Before you modify `iptables`, make a backup of the current configuration:

```markup
sudo iptables-save > ~/iptables.old
```

Check your current `iptables` configuration by running:

```markup
sudo iptables --list --line-numbers
```

If you want to provide HTTP access from your tailnet to the VM, run:

```markup
sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 80 -j ACCEPT
sudo netfilter-persistent save
```

Alternatively, if you see a specific rule that is rejecting your ingress access, you can delete it by line number:

```markup
sudo iptables -D INPUT <line-number>
```

Check the `iptables` list again to make sure your change is in effect.

```markup
sudo iptables --list --line-numbers
```

Then, try accessing the Oracle Cloud Linux VM from your tailnet node again.

For more information about the `iptables` command on Oracle Linux VMs, see Oracle's support content at [Linux OS Service 'iptables' (Doc ID 564940.1)](https://support.oracle.com/knowledge/Oracle%20Linux%20and%20Virtualization/564940_1.html).

Last updated Dec 20, 2024

On this page

*   [Prerequisites](https://tailscale.com/kb/1149/cloud-oracle#prerequisites)
*   [Step 1: Set up the Tailscale client for the VM](https://tailscale.com/kb/1149/cloud-oracle#step-1-set-up-the-tailscale-client-for-the-vm)
*   [Step 2: Allow UDP port 41641](https://tailscale.com/kb/1149/cloud-oracle#step-2-allow-udp-port-41641)
*   [Step 3: Advertise routes from the VM](https://tailscale.com/kb/1149/cloud-oracle#step-3-advertise-routes-from-the-vm)
*   [Step 4: Add Oracle DNS for your tailnet](https://tailscale.com/kb/1149/cloud-oracle#step-4-add-oracle-dns-for-your-tailnet)
*   [Step 5: Remove public SSH access](https://tailscale.com/kb/1149/cloud-oracle#step-5-remove-public-ssh-access)
*   [Troubleshooting](https://tailscale.com/kb/1149/cloud-oracle#troubleshooting)

Related Pages

*   [Subnet routers](https://tailscale.com/kb/1019/subnets)
*   [Connect to an AWS VPC using subnet routes](https://tailscale.com/kb/1021/install-aws)
*   [Connect to external services with IP block lists](https://tailscale.com/kb/1059/ip-blocklist-relays)
*   [Connecting when you can't install Tailscale](https://tailscale.com/kb/1109/devices-without-tailscale)
*   [Set up high availability](https://tailscale.com/kb/1115/high-availability)

Product

[How it works](https://tailscale.com/blog/how-tailscale-works)[Pricing](https://tailscale.com/pricing)[Integrations](https://tailscale.com/integrations)[Features](https://tailscale.com/features)[Compare Tailscale](https://tailscale.com/compare)

Use Cases

[Business VPN](https://tailscale.com/use-cases/business-vpn)[Remote Access](https://tailscale.com/use-cases/remote-access)[Site-to-Site Networking](https://tailscale.com/use-cases/site-to-site-networking)[Homelab](https://tailscale.com/use-cases/homelab)[Enterprise](https://tailscale.com/enterprise)

Resources

[Blog](https://tailscale.com/blog)[Events & Webinars](https://tailscale.com/events-webinars)[Partnerships](https://tailscale.com/partnerships)

Company

[Company](https://tailscale.com/company)[Careers](https://tailscale.com/careers)[Press](https://tailscale.com/press)

Help & Support

[Support](https://tailscale.com/contact/support)[Sales](https://tailscale.com/contact/sales)[Security](https://tailscale.com/security)[Legal](https://tailscale.com/legal)[Open Source](https://tailscale.com/opensource)[Changelog](https://tailscale.com/changelog)

Learn

[SSH keys](https://tailscale.com/learn/generate-ssh-keys)[Docker SSH](https://tailscale.com/learn/ssh-into-docker-container)[DevSecOps](https://tailscale.com/learn/devsecops)[Multicloud](https://tailscale.com/learn/multicloud)[NAT Traversal](https://tailscale.com/blog/how-nat-traversal-works)[MagicDNS](https://tailscale.com/blog/2021-09-private-dns-with-magicdns)[PAM](https://tailscale.com/learn/privileged-access-management)[PoLP](https://tailscale.com/learn/principle-of-least-privilege)[All articles](https://tailscale.com/learn)

[](https://tailscale.com/ "Homepage")

[Terms of Service](https://tailscale.com/terms)[Privacy Policy](https://tailscale.com/privacy-policy)

WireGuard is a registered trademark of Jason A. Donenfeld.

[](https://twitter.com/tailscale)[](https://www.facebook.com/tailscale/)[](https://www.linkedin.com/company/tailscale)[](https://hachyderm.io/@tailscale)[](https://www.youtube.com/@Tailscale)

© 2025 Tailscale Inc. All rights reserved. Tailscale is a registered trademark of Tailscale Inc.