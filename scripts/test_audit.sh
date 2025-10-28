#!/bin/bash

API="http://localhost:3000"

echo "📜 Testing Audit Log System"
echo ""

# Login
TOKEN=$(curl -s -X POST $API/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "alice@example.com", "password": "password123"}' | jq -r '.token')

echo "--------------- Create a Task"
TASK_ID=$(curl -s -X POST $API/api/tasks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title": "Audit test task", "priority": "high"}' | jq -r '.id')
echo "Created task: $TASK_ID"
echo ""

sleep 1

echo "--------------- Update the Task"
curl -s -X PUT $API/api/tasks/$TASK_ID \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"status": "in_progress", "priority": "medium"}' | jq -c '.title, .status, .priority'
echo ""

sleep 1

echo "--------------- Update Again"
curl -s -X PUT $API/api/tasks/$TASK_ID \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"status": "completed"}' | jq -c '.title, .status'
echo ""

echo "--------------- Get Task History (Audit Log)"
curl -s -X GET $API/api/tasks/$TASK_ID/history \
  -H "Authorization: Bearer $TOKEN" | jq
echo ""

echo "--------------- Get User Activity"
curl -s -X GET "$API/api/audit/user-activity?limit=10" \
  -H "Authorization: Bearer $TOKEN" | jq
echo ""

# Login as admin
ADMIN_TOKEN=$(curl -s -X POST $API/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "admin@example.com", "password": "password123"}' | jq -r '.token')

echo "--------------- Admin: Get Recent Activity (All Users)"
curl -s -X GET "$API/admin/audit/recent?limit=20" \
  -H "Authorization: Bearer $ADMIN_TOKEN" | jq
echo ""

echo "✅ Audit log tests complete!"