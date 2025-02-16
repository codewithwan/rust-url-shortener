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
    libssl-dev ca-certificates postgresql-client && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/time_to_rust .

# Copy the migration script
COPY migrations /usr/src/app/migrations

EXPOSE ${PORT}

ENV RUST_ENV=production
ENV DATABASE_URL=postgres://postgres:Admin1234@db:5432/shortlink
ENV BASE_URL=http://localhost:3030
ENV PORT=3030

# Run the migration script and then start the application
CMD ["sh", "-c", "PGPASSWORD=Admin1234 psql -h db -U postgres -d shortlink -f /usr/src/app/migrations/2025-02-14-create-shortlink-table.sql && ./time_to_rust --port ${PORT}"]
