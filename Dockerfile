# Build Stage
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copy the Cargo files and source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the project
RUN cargo build --release

# Use a compatible base image with the required GLIBC version (Debian 12)
FROM debian:bookworm-slim

# Install required libraries
RUN apt-get update && apt-get install -y \
    libssl-dev ca-certificates postgresql-client redis-tools && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/time_to_rust .

# Copy the migration script
COPY migrations /usr/src/app/migrations
COPY scripts/init_db.sh /usr/src/app/init_db.sh
RUN chmod +x /usr/src/app/init_db.sh

EXPOSE ${PORT}

ENV RUST_ENV=production
ENV DATABASE_URL=${DATABASE_URL}
ENV REDIS_URL=${REDIS_URL}
ENV BASE_URL=${BASE_URL}
ENV PORT=${PORT}

# Run the migration script and then start the application
CMD ["/usr/src/app/init_db.sh"]
