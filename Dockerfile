# Build stage
FROM rust:latest AS build

# Create a new empty shell project
RUN USER=root cargo new --bin moon_phase_demo
WORKDIR /moon_phase_demo

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build this project, this will pull in all the dependencies and compile them
RUN cargo build --release

# Remove the auto-generated "Hello, world!" source code
RUN rm src/*.rs

# Copy the source code files
COPY ./src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:buster-slim

# Install SSL certificates
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the build stage
COPY --from=build /moon_phase_demo/target/release/moon_phase_demo /usr/local/bin

COPY --from=build /moon_phase_demo/target/release/build-css /usr/local/bin

# Set the startup command
CMD ["/usr/local/bin/build-css"]
CMD ["/usr/local/bin/moon_phase_demo"]
