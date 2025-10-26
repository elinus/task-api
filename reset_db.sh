#!/bin/bash

echo "🔄 Resetting database..."

# Stop containers
docker-compose down -v

# Start database
docker-compose up -d postgres

# Wait for startup
echo "⏳ Waiting for PostgreSQL..."
sleep 5

# Run migrations
echo "📊 Running migrations..."
sqlx migrate run

echo "✅ Database ready!"

# Verify
psql -U taskuser -d taskdb -h localhost -c "\\dt"
