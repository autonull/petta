# PeTTa CI/CD Docker Image
# Multi-stage build for minimal runtime image

# Build stage
FROM rust:1.85-slim as builder

# Install SWI-Prolog dependency
RUN apt-get update && apt-get install -y \
    swi-prolog \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy source
COPY . .

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install SWI-Prolog runtime
RUN apt-get update && apt-get install -y \
    swi-prolog \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/petta /app/

# Default command
ENTRYPOINT ["/app/petta"]
CMD []
