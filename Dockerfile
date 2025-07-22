# Build stage
FROM rust:1-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Build dependencies - this is cached separately
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY . .

# Build application
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates openssl

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/parental-unifi-quick-set /app/
COPY index.html /app/

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./parental-unifi-quick-set"] 