#!/bin/bash

API_URL="http://localhost:8080"

echo "=== Creating Jobs ==="

# Create job 1
JOB1=$(curl -s -X POST $API_URL/api/v0/jobs \
  -H "Content-Type: application/json" \
  -d '{
    "name": "send_email",
    "payload": {"api_url":"https://httpbin.org/post","data":{"sample_key":"sample_value"}}
  }')
echo "Job 1: $(echo $JOB1 | jq -r '.id')"

# Create job 2
JOB2=$(curl -s -X POST $API_URL/api/v0/jobs \
  -H "Content-Type: application/json" \
  -d '{
    "name": "process_data",
    "payload": {"api_url":"https://httpbin.org/post","data":{"sample_key":"sample_value"}}
  }')
echo "Job 2: $(echo $JOB2 | jq -r '.id')"

# Create job 3
JOB3=$(curl -s -X POST $API_URL/api/v0/jobs \
  -H "Content-Type: application/json" \
  -d '{
    "name": "generate_report",
    "payload": {"api_url":"https://httpbin.org/post","data":{"sample_key":"sample_value"}}
  }')
echo "Job 3: $(echo $JOB3 | jq -r '.id')"

echo -e "\n=== Listing All Jobs ==="
curl -s $API_URL/api/v0/jobs | jq '.[] | {id, name, status}'

echo -e "\n=== Listing Pending Jobs ==="
curl -s "$API_URL/api/v0/jobs?status=Pending" | jq '.[] | {id, name, status}'

echo -e "\n=== Getting Job 1 Details ==="
JOB1_ID=$(echo $JOB1 | jq -r '.id')
curl -s $API_URL/api/v0/jobs/$JOB1_ID | jq .

echo -e "\n=== Health Check ==="
curl -s $API_URL/api/v0/health | jq .
