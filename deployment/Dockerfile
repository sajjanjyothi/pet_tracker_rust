# Use a Rust base image
FROM rust:1.67.0-slim-buster

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml file to check dependencies
COPY Cargo.toml Cargo.lock ./

# Copy the rest of the project
COPY . .

# Build the project
RUN cargo build --release

# Copy the binary to the output directory
COPY target/release/pet_tracker /usr/local/bin/

# Command to run when container starts
CMD ["pet_tracker"]
