FROM rust:1.95-slim AS builder

WORKDIR /usr/src/app

# Install dependencies required for building Dioxus and typical Rust apps
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Install the dioxus CLI (version 0.7.7 to match Cargo.toml)
RUN cargo install dioxus-cli --version 0.7.7

# Copy the source code
COPY . .

# Build the fullstack project for release using Dioxus CLI
RUN dx build --release

# Final runtime image
FROM debian:trixie-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built server binary and the generated web assets
COPY --from=builder /usr/src/app/target/dx/dioxus_rig_app/release/web /app

# The default Dioxus fullstack port
EXPOSE 8080

# Set IP to 0.0.0.0 so Railway can route traffic to the container
ENV IP=0.0.0.0

# The server executable is placed at the root of the output directory by the Dioxus CLI 
# Wait, based on the `list_dir` output, there is a `server` executable directly in the `web` folder.
CMD ["./server"]
