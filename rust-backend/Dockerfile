# rust-backend/Dockerfile
# Optimized multi-stage build with caching for faster rebuilds

# Builder stage
FROM rustlang/rust:nightly AS builder

# Install system dependencies needed for compilation
RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates build-essential \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency manifests first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependency compilation
RUN mkdir src && echo "fn main() {}" > src/main.rs \
 && cargo build --release \
 && rm -rf src

# Copy the actual source code
COPY . .

# Build the release binary (will use cached dependencies if unchanged)
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/rust-backend ./rust-backend

# Copy runtime configuration files
COPY .env ./
COPY log4rs.yaml ./
RUN touch ./log.log && chmod 666 ./log.log

# Ensure /app is writable
RUN chmod -R 777 /app

EXPOSE 8080

CMD ["./rust-backend"]