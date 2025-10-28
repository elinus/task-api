#!/bin/bash

API="http://localhost:3000"

echo "🔗 Testing Task Dependencies System"
echo ""

# Login
echo "--------------- Login"
TOKEN=$(curl -s -X POST $API/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "alice@example.com", "password": "password123"}' | jq -r '.token')

echo "Token: ${TOKEN:0:20}..."
echo ""

# Create tasks for dependency testing
echo "--------------- Creating Tasks"

TASK_A=$(curl -s -X POST $API/api/tasks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title": "Task A: Design mockups", "priority": "high"}' | jq -r '.id')
echo "Task A (Design): $TASK_A"

TASK_B=$(curl -s -X POST $API/api/tasks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title": "Task B: Write code", "priority": "high"}' | jq -r '.id')
echo "Task B (Code): $TASK_B"

TASK_C=$(curl -s -X POST $API/api/tasks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title": "Task C: Deploy", "priority": "medium"}' | jq -r '.id')
echo "Task C (Deploy): $TASK_C"

TASK_D=$(curl -s -X POST $API/api/tasks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title": "Task D: Documentation", "priority": "low"}' | jq -r '.id')
echo "Task D (Docs): $TASK_D"
echo ""

# Create valid dependencies
echo "--------------- Adding Valid Dependencies"

echo "B depends on A (Code needs Design):"
curl -s -X POST $API/api/tasks/$TASK_B/dependencies \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"depends_on\": \"$TASK_A\"}" | jq

echo ""
echo "C depends on B (Deploy needs Code):"
curl -s -X POST $API/api/tasks/$TASK_C/dependencies \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"depends_on\": \"$TASK_B\"}" | jq

echo ""
echo "D depends on A (Docs need Design):"
curl -s -X POST $API/api/tasks/$TASK_D/dependencies \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"depends_on\": \"$TASK_A\"}" | jq
echo ""

# Check dependencies
echo "--------------- Getting Dependencies for Task C"
curl -s -X GET $API/api/tasks/$TASK_C/dependencies \
  -H "Authorization: Bearer $TOKEN" | jq
echo ""

# Check what tasks are blocked by A
echo "--------------- What tasks are blocked by Task A?"
curl -s -X GET $API/api/tasks/$TASK_A/blocked \
  -H "Authorization: Bearer $TOKEN" | jq
echo ""

# Try creating circular dependency (should fail)
echo "--------------- Try Creating Circular Dependency (A→B→C→A) - Should FAIL"
curl -s -X POST $API/api/tasks/$TASK_A/dependencies \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"depends_on\": \"$TASK_C\"}" | jq
echo ""

# Try self-dependency (should fail at database level)
echo "--------------- Try Self-Dependency (A depends on A) - Should FAIL"
curl -s -X POST $API/api/tasks/$TASK_A/dependencies \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"depends_on\": \"$TASK_A\"}" | jq
echo ""

# Remove a dependency
echo "--------------- Remove Dependency (C no longer depends on B)"
curl -s -X DELETE $API/api/tasks/$TASK_C/dependencies/$TASK_B \
  -H "Authorization: Bearer $TOKEN"
echo "Deleted"
echo ""

# Verify removal
echo "--------------- Verify Dependency Removed"
curl -s -X GET $API/api/tasks/$TASK_C/dependencies \
  -H "Authorization: Bearer $TOKEN" | jq
echo ""

echo "✅ Dependency tests complete!"
echo ""
echo "Dependency Graph Created:"
echo "  A (Design)"
echo "  ├── B (Code) → [deleted] → C (Deploy)"
echo "  └── D (Docs)"