FROM rust:1.58-alpine as builder

# Create a new binary application
RUN cargo new --bin rust-and-docker
WORKDIR /rust-and-docker

# Copy the source code
COPY Cargo.toml Cargo.toml
COPY src src

# Build the application in release mode
RUN cargo build --release

# Use a smaller base image
FROM alpine:latest

# Copy the compiled binary from the builder stage
COPY --from=builder /rust-and-docker/target/release/rust-and-docker /usr/local/bin/rust-and-docker

# Set the entrypoint
CMD [ "/usr/local/bin/rust-and-docker" ]
