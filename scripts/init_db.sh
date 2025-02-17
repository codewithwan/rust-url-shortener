#!/bin/sh
set -e

echo "Running database migration..."
PGPASSWORD=$POSTGRES_PASSWORD psql -h db -U $POSTGRES_USER -d $POSTGRES_DB -f /usr/src/app/migrations/2025-02-14-create-shortlink-table.sql

echo "Starting application..."
exec ./time_to_rust --port ${PORT}
