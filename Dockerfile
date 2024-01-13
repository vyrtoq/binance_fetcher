# Use an official Rust nightly runtime as a parent image
FROM rust:alpine as builder

RUN apk add --no-cache build-base musl-dev openssl-dev openssl git

# Set the working directory in the Docker image
WORKDIR /app

# Copy over your manifest
COPY Cargo.toml Cargo.lock ./

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/app*
RUN cargo build --release

# Our second stage, that will be the final image
FROM alpine:latest

# Add Rust binary to the $PATH
# Alpine only
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/local/cargo/bin

# Copy the build artifact from the builder stage.
COPY --from=builder /usr/src/app/target/release/app /usr/local/bin

# Run the binary.
CMD ["app"]