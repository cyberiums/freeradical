FROM rustlang/rust:nightly as builder

WORKDIR /app

# Install Python development dependencies for pyo3
RUN apt-get update && apt-get install -y \
    python3-dev \
    python3-pip \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY migrations ./migrations
COPY migrations_postgres ./migrations_postgres
COPY static ./static

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:sid-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    default-libmysqlclient-dev \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/freeradical /app/freeradical

# Copy required directories
COPY templates ./templates
COPY migrations ./migrations
COPY migrations_postgres ./migrations_postgres
COPY static ./static

# Create uploads directory
RUN mkdir -p /app/uploads

EXPOSE 8000

CMD ["./freeradical"]
