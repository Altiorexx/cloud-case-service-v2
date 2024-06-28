# Switch to a minimal alpine image for runtime
FROM ubuntu:latest

# Use the official Rust image for building
FROM rustlang/rust:nightly

# Copy your source code into the container
WORKDIR /app

COPY . .

# Install dependencies and build the application
RUN cargo build --release

# Expose the port your application listens on (if applicable)
EXPOSE 8000

ENV PORT=8000

# Define the command to run your application
CMD ./target/release/rust-api




