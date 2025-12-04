FROM oven/bun:1.2.13-slim

WORKDIR /app

# Copy source code
COPY router.ts ./

# Expose the port the app runs on
EXPOSE 3004

# Start the application
CMD ["bun", "run", "router.ts"] 