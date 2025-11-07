# Use the official Rust image as a parent image
FROM rust:1.91 AS builder

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the application with security optimizations
RUN cargo build --release

# Use a minimal, security-hardened runtime image
FROM debian:bookworm-slim

# Security: Install only essential runtime dependencies and security updates
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    wget \
    && apt-get upgrade -y \
    && wget -q https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb \
    && dpkg -i cloudflared-linux-amd64.deb \
    && rm cloudflared-linux-amd64.deb \
    && apt-get remove -y wget \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Security: Create a dedicated non-root user with specific UID/GID
RUN groupadd -r -g 1000 appgroup && \
    useradd -r -u 1000 -g appgroup -s /bin/false -M -d /nonexistent appuser

# Security: Create minimal directory structure
RUN mkdir -p /app && \
    chown appuser:appgroup /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/infinitewebsite /app/infinitewebsite

# Security: Set strict permissions
RUN chown appuser:appgroup /app/infinitewebsite && \
    chmod 755 /app/infinitewebsite

# Security: Remove unnecessary packages and clean up
RUN apt-get autoremove -y && \
    rm -rf /tmp/* /var/tmp/*

# Security: Switch to non-root user
USER appuser:appgroup

# Security: Set working directory
WORKDIR /app

# Expose the port the app runs on
EXPOSE 8000

# Security: Set environment variables with strict settings
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV RUST_LOG=info
ENV RUST_BACKTRACE=0

# Security: Use absolute path and exec form
CMD sh -c 'cloudflared tunnel --no-autoupdate run --token "$CLOUDFLARE_TUNNEL_TOKEN" & /app/infinitewebsite'