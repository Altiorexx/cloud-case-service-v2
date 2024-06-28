# Switch to a minimal alpine image for runtime
FROM ubuntu:latest

# Use the official Rust image for building
FROM rustlang/rust:nightly

# Copy your source code into the container
WORKDIR /app

COPY . .

#
ENV PORT="4000"
ENV MONGODB_CONNECTION_STRING="mongodb+srv://ka:admin@cloud-case-db.afemiys.mongodb.net/?retryWrites=true&w=majority&appName=cloud-case-db"
ENV ISSUER="https://case.service.altiore.io"
ENV SECRET="L0ngt4llS4lly"
ENV USER_SERVICE_DOMAIN="https://user.service.altiore.io"
ENV DOMAIN="https://case.service.altiore.io"

# Install dependencies and build the application
RUN cargo build

# Expose the port your application listens on (if applicable)
EXPOSE $PORT

# Define the command to run your application
CMD ./target/debug/case-service





