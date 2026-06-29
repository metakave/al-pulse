# ---- Build stage ----
FROM rust:1.82-slim AS builder

# Install system deps required by sqlx + reqwest (openssl, pkg-config)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the whole repo (workspace)
COPY . .

# Build only the backend binary from the workspace
RUN cargo build --release --manifest-path backend/Cargo.toml --bin backend

# ---- Runtime stage ----
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary
COPY --from=builder /app/target/release/backend ./backend

# Copy pre-built frontend static files
COPY --from=builder /app/frontend/dist ./frontend/dist

EXPOSE 8080
CMD ["./backend"]
