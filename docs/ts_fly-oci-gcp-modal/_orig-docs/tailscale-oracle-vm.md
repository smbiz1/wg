Title: Access Oracle Cloud VMs privately using Tailscale · Tailscale Docs

URL Source: https://tailscale.com/kb/1149/cloud-oracle

Markdown Content:
Access Oracle Cloud VMs privately using Tailscale · Tailscale Docs
===============

Oracle Cloud provides Linux virtual machines, notably featuring the ARM CPU architecture. We can use Tailscale to securely access Oracle virtual machines.

Prerequisites
Before you begin this guide, you'll need a Tailscale network set up and configured with at least one existing device. Read our getting started guide if you need help with this.

Step 1: Set up the Tailscale client for the VM
First, create a Virtual Machine in the OCN Console.

ssh to the system and follow the steps to install Tailscale on Oracle Linux.

Step 2: Allow UDP port 41641
If at least one side of a tunnel has "easy NAT," where Tailscale can determine the UDP port number on the far side of the NAT device, then it will make direct connections to minimize latency. We ensure that OCN nodes can make direct connections by allowing UDP port 41641 to ingress through the firewall.

In the Networking tab select Virtual Cloud Networks select the specific VCN the VM has been created on.

Virtual Cloud Networks list
Select Security Lists in the left hand column, and the security list in use (probably a Default).

Security lists
Add a Stateless ingress rule for 0.0.0.0/0 UDP port 41641.

Adding an Ingress Rule
Step 3: Advertise routes from the VM
For the benefit of the other nodes in the tailnet we'll set up split DNS to allow use of the same DNS names as are used inside of Oracle Cloud. The DNS server provided by Oracle is 169.254.169.254, and supports hostnames of the form instance.subnet01234567.vcn01234567.oraclevcn.com.

We'll have our VM advertise routes for both the subnet it sits on as well as the Oracle DNS server. For example, if the subnet address range is 10.0.0.0/24, the command would be:


tailscale set --advertise-routes=10.0.0.0/24,169.254.169.254/32 --accept-dns=false
For Oracle Cloud VMs it is generally best to let Oracle handle the DNS configuration, not have Tailscale override it, so we added --accept-dns=false.

Step 4: Add Oracle DNS for your tailnet
In the DNS page of the admin console we add a nameserver restricted to the oraclevcn.com domain, pointing to the Oracle DNS server which we made available through our VM.

Adding a Split DNS resolver for oraclevcn.com
Now the same hostnames which work between nodes running within Oracle Cloud will also be available to all nodes on our tailnet.

Step 5: Remove public SSH access
As we can now ssh to the system over the private Tailscale network, there is no reason to leave the SSH port open on a public IP address. In the Network > Security List the SSH rule can be removed.

Disable public SSH port.
Troubleshooting
If you find that your tailnet nodes cannot access your Oracle Cloud Linux VM, you may need to update the VM's iptables configuration.

Before you modify iptables, make a backup of the current configuration:


sudo iptables-save > ~/iptables.old
Check your current iptables configuration by running:


sudo iptables --list --line-numbers
If you want to provide HTTP access from your tailnet to the VM, run:


sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 80 -j ACCEPT
sudo netfilter-persistent save
Alternatively, if you see a specific rule that is rejecting your ingress access, you can delete it by line number:


sudo iptables -D INPUT <line-number>
Check the iptables list again to make sure your change is in effect.


sudo iptables --list --line-numbers
Then, try accessing the Oracle Cloud Linux VM from your tailnet node again.

For more information about the iptables command on Oracle Linux VMs, see Oracle's support content at Linux OS Service 'iptables' (Doc ID 564940.1).

Last updated Dec 4, 2025

On this page

*   [Prerequisites](https://tailscale.com/kb/1149/cloud-oracle#prerequisites)
*   [Step 1: Set up the Tailscale client for the VM](https://tailscale.com/kb/1149/cloud-oracle#step-1-set-up-the-tailscale-client-for-the-vm)
*   [Step 2: Allow UDP port 41641](https://tailscale.com/kb/1149/cloud-oracle#step-2-allow-udp-port-41641)
*   [Step 3: Advertise routes from the VM](https://tailscale.com/kb/1149/cloud-oracle#step-3-advertise-routes-from-the-vm)
*   [Step 4: Add Oracle DNS for your tailnet](https://tailscale.com/kb/1149/cloud-oracle#step-4-add-oracle-dns-for-your-tailnet)
*   [Step 5: Remove public SSH access](https://tailscale.com/kb/1149/cloud-oracle#step-5-remove-public-ssh-access)
*   [Troubleshooting](https://tailscale.com/kb/1149/cloud-oracle#troubleshooting)
Related Pages
Subnet routers
Connect to an AWS VPC using subnet routes
Connect to external services with IP block lists
Connecting without installing Tailscale
Set up high availability
Product

How it works
Pricing
Integrations
Features
Compare Tailscale
Use Cases

Business VPN
CI/CD
Infra Access
Cloud Connectivity
Zero Trust Networking
Homelab