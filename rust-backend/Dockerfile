# Stage 1 - Builder
FROM rustlang/rust:nightly-slim as builder

WORKDIR /app


# Install system deps needed for OpenSSL and pkg-config
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy manifests first for caching
COPY Cargo.toml Cargo.lock ./

# Create dummy src to pass build check (avoids rebuilds)
RUN mkdir src && echo "fn main() { println!(\"dummy\"); }" > src/main.rs && \
    cargo build --release && rm -rf src

# Now copy real source
COPY ./src ./src

# Build the release binary
RUN cargo build --release

# Stage 2 - Final runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install SSL if needed for reqwest or HTTPS
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-backend .

# Optional: include .env for runtime config
COPY .env .

EXPOSE 8080

CMD ["./rust-backend"]