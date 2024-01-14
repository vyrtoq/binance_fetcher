# Use an official Rust nightly runtime as a parent image
FROM rust:alpine as builder

RUN apk add --no-cache build-base musl-dev openssl-dev openssl git

# Set the OpenSSL env vars for linker
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

# Set the working directory in the Docker image
WORKDIR /app

# Copy over your manifest
COPY Cargo.toml Cargo.lock ./
COPY Settings.toml  Settings.toml ./

# Copy your source tree
COPY ./src ./src

# Build for release.
RUN cargo build --release --bin=binance_fetcher --package=binance_fetcher --features=full --target=x86_64-unknown-linux-musl

# Our second stage, that will be the final image
FROM alpine:latest

# Add Rust binary to the $PATH
# Alpine only
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/local/cargo/bin

# Copy the build artifact from the builder stage.
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/binance_fetcher /usr/local/bin

# Run the binary.
CMD ["binance_fetcher"]