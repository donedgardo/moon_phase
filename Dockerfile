# Build stage
FROM rust:latest AS build

# Create a new empty shell project to cache depenency builds
RUN USER=root cargo new moon-phases
RUN ls -al

COPY ./Cargo.lock ./moon-phases/Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

WORKDIR /moon-phases

RUN ls -al
# Copy over your manifests


# Build this project, this will pull in all the dependencies and compile them
RUN cargo build --release

# Remove the auto-generated "Hello, world!" source code
RUN rm src/*.rs

# Copy the source code files
COPY ./src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM ubuntu:latest

# Install SSL certificates
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY ./static ./static
COPY --from=build /moon-phases/target/release/moon-phases /usr/local/bin
COPY --from=build /moon-phases/target/release/build-css /usr/local/bin

# Set the startup command
CMD ["/usr/local/bin/build-css"]
CMD ["/usr/local/bin/moon-phases"]
