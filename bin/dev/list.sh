#!/bin/bash

API_URL="http://localhost:8080"

echo -e "\n=== Listing All Jobs ==="
curl -s $API_URL/api/v0/jobs | jq '.[] | {id, name, status, result}'
