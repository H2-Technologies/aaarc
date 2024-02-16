# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/engine/reference/builder/
FROM rust:1.74.1-slim

WORKDIR /usr/src/app

COPY . .

# Download dependencies as a separate step to take advantage of Docker's caching.
# Leverage a cache mount to /root/.yarn to speed up subsequent builds.
# Leverage a bind mounts to package.json and yarn.lock to avoid having to copy them into
# into this layer.
RUN cargo fetch

RUN apt-get update

RUN apt-get install pkg-config

# Build the application.
RUN cargo build --release

RUN cp ./target/release/ashland-area-amateur-radio-club /usr/local/bin/.

RUN cargo clean

# Run the application.
CMD ["ashland-area-amateur-radio-club"]