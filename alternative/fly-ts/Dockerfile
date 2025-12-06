# Multi-stage Dockerfile for Tailscale-enabled Fly.io deployment
# Based on: https://tailscale.com/kb/1132/flydotio

# Stage 1: Build stage
FROM node:18-alpine as builder
WORKDIR /app
COPY package.json ./
RUN npm install --only=production
COPY *.js ./

# Stage 2: Runtime with Tailscale
FROM alpine:latest

# Install Node.js and required packages for Tailscale
RUN apk update && apk add ca-certificates iptables ip6tables curl nodejs npm && rm -rf /var/cache/apk/*

# Copy application from builder stage
COPY --from=builder /app /app

# Copy Tailscale binaries from official Docker Hub image
COPY --from=docker.io/tailscale/tailscale:stable /usr/local/bin/tailscaled /usr/local/bin/tailscaled
COPY --from=docker.io/tailscale/tailscale:stable /usr/local/bin/tailscale /usr/local/bin/tailscale

# Create Tailscale directories
RUN mkdir -p /var/run/tailscale /var/cache/tailscale /var/lib/tailscale

# Copy start script
COPY start.sh /app/start.sh
RUN chmod +x /app/start.sh

# Expose the application port
EXPOSE 3004

# Run the start script which initializes Tailscale and starts the app
CMD ["/app/start.sh"] 