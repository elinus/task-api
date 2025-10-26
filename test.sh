#!/bin/bash

API="http://localhost:3000"

echo "🧪 Testing Task API..."
echo ""

echo "1️⃣  Health Check"
curl -s $API/health | jq
echo -e "\n"

echo "2️⃣  Create Task 1 (High Priority)"
TASK1=$(curl -s -X POST $API/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Complete Day 2", "priority": "high", "status": "in_progress"}' | jq -r '.id')
echo "Created task: $TASK1"
echo ""

echo "3️⃣  Create Task 2 (Low Priority)"
TASK2=$(curl -s -X POST $API/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Review code", "priority": "low", "status": "pending"}' | jq -r '.id')
echo "Created task: $TASK2"
echo ""

echo "4️⃣  List All Tasks"
curl -s $API/api/tasks | jq
echo ""

echo "5️⃣  Filter by Priority (high)"
curl -s "$API/api/tasks?priority=high" | jq
echo ""

echo "6️⃣  Update Task 1"
curl -s -X PUT $API/api/tasks/$TASK1 \
  -H "Content-Type: application/json" \
  -d '{"status": "completed"}' | jq
echo ""

echo "7️⃣  Delete Task 2"
curl -s -X DELETE $API/api/tasks/$TASK2
echo "Deleted"
echo ""

echo "8️⃣  Final Task List"
curl -s $API/api/tasks | jq
echo ""

echo "✅ All tests complete!"
