#!/bin/bash
set -e

# Wait for database to be ready
if [ -n "$DATABASE_URL" ]; then
  echo "Waiting for database..."
  until pg_isready -d "$DATABASE_URL"; do
    sleep 2
  done
fi

# Run migrations
if [ -f ./setup ]; then
  echo "Running migrations..."
  ./setup
fi

# Start the server
exec ./level_server
