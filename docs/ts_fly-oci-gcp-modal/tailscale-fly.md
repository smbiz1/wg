Title: Tailscale on Fly.io · Tailscale Docs

URL Source: https://tailscale.com/kb/1132/flydotio

Markdown Content:
Tailscale on Fly.io · Tailscale Docs

Tailscale on Fly.io
===================

[Fly.io](https://fly.io/) is a popular service to deploy full stack apps and databases all over the world, with Fly handling operations and scaling in each region according to demand. Adding Tailscale to a fly.io application is straightforward, allowing the App on Fly to communicate with other nodes and services in your tailnet.

[Step 1: Generate an auth key to authenticate your App on Fly](https://tailscale.com/kb/1132/flydotio#step-1-generate-an-auth-key-to-authenticate-your-app-on-fly)
------------------------------------------------------------------------------------------------------------------------------------------------------------------

First, we'll [generate an auth key](https://tailscale.com/kb/1085/auth-keys) to allow fly.io to authenticate our app to join our network.

Open the [Keys](https://login.tailscale.com/admin/settings/keys) page of the admin console and select **Generate auth key**. We recommend using a reusable and pre-authorized [ephemeral key](https://tailscale.com/kb/1111/ephemeral-nodes) for this purpose, since it will automatically clean up devices after they shut down.

![Image 18: Tailscale's auth key generation page](https://tailscale.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Fephemeral-keys.2d3fc94f.png&w=750&q=75)

The **Pre-approved** option will only display in the dialog if [device approval](https://tailscale.com/kb/1099/device-approval) is enabled in your Tailscale network.

Next, use `flyctl secrets set TAILSCALE_AUTHKEY="tskey-<key>"` to securely store the auth key for the App on Fly to use.

[Step 2: Configure your Dockerfile to install Tailscale](https://tailscale.com/kb/1132/flydotio#step-2-configure-your-dockerfile-to-install-tailscale)
------------------------------------------------------------------------------------------------------------------------------------------------------

Next, we'll use a [multistage Dockerfile](https://docs.docker.com/develop/develop-images/multistage-build/#use-multi-stage-builds), where the first stage builds your application, and the second stage pulls application code and Tailscale into the final image to be uploaded to Fly.

In your `Dockerfile`:

```docker
FROM alpine:latest as builder
WORKDIR /app
COPY . ./
# This is where one could build the application code as well.

# https://docs.docker.com/develop/develop-images/multistage-build/#use-multi-stage-builds
FROM alpine:latest
RUN apk update && apk add ca-certificates iptables ip6tables && rm -rf /var/cache/apk/*

# Copy binary to production image.
COPY --from=builder /app/start.sh /app/start.sh

# Copy Tailscale binaries from the tailscale image on Docker Hub.
COPY --from=docker.io/tailscale/tailscale:stable /usr/local/bin/tailscaled /app/tailscaled
COPY --from=docker.io/tailscale/tailscale:stable /usr/local/bin/tailscale /app/tailscale
RUN mkdir -p /var/run/tailscale /var/cache/tailscale /var/lib/tailscale

# Run on container startup.
CMD ["/app/start.sh"]
```

The Dockerfile specifies `/app/start.sh` as the initial process to run. This script needs to bring Tailscale up and then start the application binary. This is where we can use the `TAILSCALE_AUTHKEY` variable we defined earlier.

Then, create a file named `start.sh` at the root of your app:

```shell
#!/bin/sh

/app/tailscaled --state=/var/lib/tailscale/tailscaled.state --socket=/var/run/tailscale/tailscaled.sock &
/app/tailscale up --auth-key=${TAILSCALE_AUTHKEY} --hostname=fly-app
/app/my-app
```

The next time your App on Fly deploys, it will be able to connect to your private Tailscale network.

If you are using an Alpine base image and an existing Fly machine, you may need to update the machine to ensure that it has kernel support for nftables. For more information, refer to [Fly community post](https://community.fly.io/t/kernel-nftables-support/17669) and [Tailscale issue #10540](https://github.com/tailscale/tailscale/issues/10540).

[Remove ephemeral nodes from a tailnet](https://tailscale.com/kb/1132/flydotio#remove-ephemeral-nodes-from-a-tailnet)
---------------------------------------------------------------------------------------------------------------------

When an ephemeral node goes offline, it is automatically removed from your tailnet. You can also control ephemeral node removal using the [`tailscale logout`](https://tailscale.com/kb/1080/cli#logout) command to either manually force the removal or incorporate the command into the [`tailscaled`](https://tailscale.com/kb/1278/tailscaled) Tailscale daemon. For more information, refer to [Ephemeral nodes](https://tailscale.com/kb/1111/ephemeral-nodes#faq).

Last updated Dec 4, 2025

On this page

*   [Step 1: Generate an auth key to authenticate your App on Fly](https://tailscale.com/kb/1132/flydotio#step-1-generate-an-auth-key-to-authenticate-your-app-on-fly)
*   [Step 2: Configure your Dockerfile to install Tailscale](https://tailscale.com/kb/1132/flydotio#step-2-configure-your-dockerfile-to-install-tailscale)
*   [Remove ephemeral nodes from a tailnet](https://tailscale.com/kb/1132/flydotio#remove-ephemeral-nodes-from-a-tailnet)

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

[Terms of Service](https://tailscale.com/terms)[Privacy Policy](https://tailscale.com/privacy-policy)[California Notice](https://tailscale.com/privacy-policy#california-notice)[Cookie Notice](https://tailscale.com/cookie-notice)![Image 19: Check mark and x on a white and blue pill button](https://cdn.sanity.io/images/w77i7m8x/production/07d853f507039b2489d9818cb6ee7442c1b60e2a-30x14.svg)Your Privacy Choices

WireGuard is a registered trademark of Jason A. Donenfeld.

[](https://twitter.com/tailscale)[](https://www.facebook.com/tailscale/)[](https://www.linkedin.com/company/tailscale)[](https://hachyderm.io/@tailscale)[](https://www.youtube.com/@Tailscale)

© 2025 Tailscale Inc. All rights reserved. Tailscale is a registered trademark of Tailscale Inc.

![Image 20](https://bat.bing.com/action/0?ti=17188063&Ver=2&mid=9b07085e-30f0-4a71-bb81-0b29120ae467&bo=1&sid=d62617b0d1e311f0a6a73923b3cea7e1&vid=d62629b0d1e311f091a0f38764b00bb9&vids=1&msclkid=N&pi=918639831&lg=en-US&sw=800&sh=600&sc=24&tl=Tailscale%20on%20Fly.io%20%C2%B7%20Tailscale%20Docs&p=https%3A%2F%2Ftailscale.com%2Fkb%2F1132%2Fflydotio&r=&lt=698&evt=pageLoad&sv=2&cdb=AQUR&rn=562971)