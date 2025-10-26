# Task Management API

REST API JWT authentication built with Rust, Axum, and PostgreSQL.

## Features
- ✅ Full CRUD operations
- ✅ JWT authentication
- ✅ Password hashing (bcrypt)
- ✅ Role-based access control
- ✅ Protected routes
- ✅ Input validation
- ✅ Query filtering

### Public (No Authentication)
```
GET  /health                   - Health check
POST /auth/register            - Register new user
POST /auth/login               - Login user
```

### Protected (Requires JWT Token)
```
GET    /api/tasks              - List all tasks (with filters)
GET    /api/tasks/:id          - Get single task
POST   /api/tasks              - Create task
PUT    /api/tasks/:id          - Update task
DELETE /api/tasks/:id          - Delete task (own tasks only)
```

### Admin Only
```
DELETE /admin/tasks/:id        - Delete any task
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
