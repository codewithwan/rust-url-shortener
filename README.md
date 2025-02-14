# URL Shortener

This project is a URL shortener service built with Rust, Warp, and PostgreSQL. It allows users to shorten long URLs and redirect to the original URLs using the shortened links.

## Features

- Shorten long URLs
- Redirect to original URLs using shortened links
- Rate limiting to prevent abuse
- Input validation to prevent malicious URLs

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Docker (optional, for running PostgreSQL in a container)

## Getting Started

### Clone the repository

```sh
git clone https://github.com/codewithwan/rust-url-shortener.git
cd rust-url-shortener
```

### Setup PostgreSQL

You can either install PostgreSQL locally or use Docker to run a PostgreSQL container.

#### Using Docker

```sh
docker run --name url-shortener-db -e POSTGRES_PASSWORD=your_password -e POSTGRES_DB=shortlink -p 5432:5432 -d postgres
```

#### Locally

Install PostgreSQL and create a database named `shortlink`.

### Configure Environment Variables

Create a `.env` file in the root directory with the following content:

```properties
DATABASE_URL=postgres://postgres:password@localhost:5432/shortlink
BASE_URL=http://localhost:3030
RUST_ENV=development
```

### Run Database Migrations

Create the `shortlink` table in your PostgreSQL database:

```sql
CREATE TABLE shortlink (
    id SERIAL PRIMARY KEY,
    short_code VARCHAR(8) NOT NULL UNIQUE,
    original_url TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

Alternatively, you can use the provided SQL migration file:

```sh
psql -U postgres -d shortlink -f migrations/2025-02-14-create-shortlink-table.sql
```

### Build and Run the Project

```sh
cargo build
cargo run
```

The server will start running at `http://localhost:3030`.

## Usage

### Shorten a URL

Send a POST request to `/shorten` with a JSON body containing the URL to be shortened.

```sh
curl -X POST http://localhost:3030/shorten -H "Content-Type: application/json" -d '{"url": "https://example.com"}'
```

### Redirect to Original URL

Access the shortened URL in your browser or send a GET request to the shortened URL.

```sh
curl http://localhost:3030/<short_code>
```

## License

This project is licensed under the MIT License.
