#!/bin/bash

API="http://localhost:3000"

echo "🔐 Testing Authentication System"
echo ""

echo "1️⃣  Register User"
REGISTER_RESPONSE=$(curl -s -X POST $API/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "alice@example.com",
    "password": "password123"
  }')
echo "$REGISTER_RESPONSE" | jq

TOKEN=$(echo "$REGISTER_RESPONSE" | jq -r '.token')
echo "Token: $TOKEN"
echo ""

echo "2️⃣  Try Creating Task WITHOUT Token (should fail)"
curl -s -X POST $API/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Test task"}' | jq
echo ""

echo "3️⃣  Create Task WITH Token (should succeed)"
curl -s -X POST $API/api/tasks \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"title": "Authenticated task", "priority": "high"}' | jq
echo ""

echo "4️⃣  Login with Same User"
LOGIN_RESPONSE=$(curl -s -X POST $API/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "alice@example.com",
    "password": "password123"
  }')
echo "$LOGIN_RESPONSE" | jq

NEW_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')
echo ""

echo "5️⃣  List Tasks with New Token"
curl -s -X GET $API/api/tasks \
  -H "Authorization: Bearer $NEW_TOKEN" | jq
echo ""

echo "6️⃣  Try Login with Wrong Password (should fail)"
curl -s -X POST $API/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "alice@example.com",
    "password": "wrongpassword"
  }' | jq
echo ""

echo "7️⃣  Try Invalid Token (should fail)"
curl -s -X GET $API/api/tasks \
  -H "Authorization: Bearer invalid-token-12345" | jq
echo ""

echo "✅ Auth tests complete!"
