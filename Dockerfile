FROM rustlang/rust:nightly as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY migrations ./migrations

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    default-libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/freeradical /app/freeradical

# Copy required directories
COPY templates ./templates
COPY migrations ./migrations

# Create uploads directory
RUN mkdir -p /app/uploads

EXPOSE 8000

CMD ["./freeradical"]
