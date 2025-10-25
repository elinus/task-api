# Task Management API

REST API built with Rust, Axum, and PostgreSQL.

## Day 1 Progress
- ✅ Project setup
- ✅ Database schema
- ✅ Basic CRUD operations
- ✅ Health check endpoint

## Run Locally
```bash
docker-compose up -d
cargo run
```

## Test
```bash
curl http://localhost:3000/health
curl http://localhost:3000/api/tasks
```

## Tech Stack
- Rust 1.75+
- Axum web framework
- PostgreSQL
- SQLx
