# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/engine/reference/builder/

ARG RUST_VERSION=1.74.1

FROM rust:${RUST_VERSION}-alpine

WORKDIR /usr/src/app

COPY . .

# Download dependencies as a separate step to take advantage of Docker's caching.
# Leverage a cache mount to /root/.yarn to speed up subsequent builds.
# Leverage a bind mounts to package.json and yarn.lock to avoid having to copy them into
# into this layer.
RUN cargo fetch

# Build the application.
RUN cargo build --release

# Copy the application to the /usr/local/bin directory
RUN cp target/release/aaarc /usr/local/bin/aaarc

# Clean up the build artifacts.
RUN cargo clean

# Run the application.
CMD ["aaarc"]