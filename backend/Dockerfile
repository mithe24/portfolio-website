# Build stage
FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# Cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/backend*
COPY .sqlx .sqlx
COPY migrations migrations
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/backend .
EXPOSE 8080
CMD ["./backend"]
