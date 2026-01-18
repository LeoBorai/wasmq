#!/bin/bash

API_URL="http://localhost:8080"

echo "=== Creating Jobs ==="

# Create job 1
curl -s -X POST $API_URL/api/v0/jobs \
  -H "Content-Type: application/json" \
  -d '{
    "name": "send_http_request",
    "payload": {"api_url":"https://httpbin.org/post","data":{"sample_key":"sample_value"}}
  }'
