# Task Management API

REST API with JWT authentication, task dependencies, and audit logging built with Rust, Axum, and PostgreSQL.

## Features
- ✅ Full CRUD operations
- ✅ JWT authentication
- ✅ Password hashing (bcrypt)
- ✅ Role-based access control (RBAC)
- ✅ Protected routes
- ✅ Input validation
- ✅ Query filtering
- ✅ Task dependencies (DAG)
- ✅ Circular dependency detection
- ✅ Audit logging (complete history)

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

### Task Dependencies
```
POST   /api/tasks/:id/dependencies              - Add dependency
DELETE /api/tasks/:id/dependencies/:depends_on  - Remove dependency
GET    /api/tasks/:id/dependencies              - Get dependencies
GET    /api/tasks/:id/dependencies/all          - Get all transitive deps
GET    /api/tasks/:id/blocked                   - Get tasks blocked by this
```

### Audit Logs
```
GET /api/tasks/:id/history                  - Get task history
GET /api/audit/user-activity?limit=50       - Get user activity
GET /admin/audit/recent?limit=100           - Admin: all activity
```

## Dependency System

### How It Works
```
Task A: Design mockups          (no dependencies)
Task B: Write code              (depends on A)
Task C: Deploy                  (depends on B)

Dependency graph: A → B → C

Rules:
- Task B cannot be completed until Task A is completed
- Circular dependencies are rejected (A→B→A is invalid)
- Self-dependencies are rejected (A→A is invalid)


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
- Day 3: ✅ JWT Auth, Password Hashing, RBAC
- Day 4: ✅ Task Dependencies, Audit Logging, Graph Algorithms
