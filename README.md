# Task Management API

REST API built with Rust, Axum, and PostgreSQL.

## Features
- ✅ Full CRUD operations
- ✅ Query filtering (status, priority, assigned_to)
- ✅ Input validation
- ✅ Proper error handling
- ✅ PostgreSQL with migrations

## API Endpoints

### Tasks
```
GET    /api/tasks              - List all tasks (with optional filters)
GET    /api/tasks/:id          - Get single task
POST   /api/tasks              - Create task
PUT    /api/tasks/:id          - Update task
DELETE /api/tasks/:id          - Delete task
```

### Query Parameters
- `status`: pending | in_progress | completed
- `priority`: low | medium | high | urgent
- `assigned_to`: UUID
- `created_by`: UUID

### Example Usage
```bash
# Create task
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "My task", "priority": "high"}'

# List high priority tasks
curl "http://localhost:3000/api/tasks?priority=high"

# Update task
curl -X PUT http://localhost:3000/api/tasks/{id} \
  -H "Content-Type: application/json" \
  -d '{"status": "completed"}'

# Delete task
curl -X DELETE http://localhost:3000/api/tasks/{id}
```

## Run
```bash
docker-compose up -d
cargo run
```

## Test
```bash
./test.sh
```

## Tech Stack
- Rust 1.75+
- Axum web framework
- PostgreSQL + SQLx
- Validator for input validation

## Progress
- Day 1: ✅ Basic CRUD
- Day 2: ✅ Update/Delete, Filters, Validation
