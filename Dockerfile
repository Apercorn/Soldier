# Use the official Rust image as the build environment
FROM rustlang/rust:nightly AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Use a minimal base image for running
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/soldier-core /app/soldier-core
COPY .env .env
CMD ["/app/soldier-core"]
