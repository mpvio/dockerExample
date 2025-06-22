# Build stage
FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# Runtime stage
FROM debian:latest
RUN apt-get update && apt-get install -y libssl3 openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/side-rust /usr/local/bin/side-rust
CMD ["side-rust"]