# Switch to a minimal alpine image for runtime
FROM ubuntu:latest

# Use the official Rust image for building
FROM rustlang/rust:nightly

# Copy your source code into the container
WORKDIR /app

COPY . .

# Install dependencies and build the application
RUN cargo build

# Expose the port your application listens on (if applicable)
EXPOSE $PORT

# Define the command to run your application
CMD ./target/debug/case-service



# This script should have release and /release/<exe>..
# whereas the dev has debug build and /debug/<exe>..


