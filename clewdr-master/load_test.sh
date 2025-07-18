#!/bin/bash

# Base URL of your proxy endpoint
BASE_URL="http://127.0.0.1:8484/v1/v1beta/models/gemini-2.5-flash-preview-04-17"
API_KEY="1145141919870"

# Function to handle each request in a separate process
send_request() {
    local request_id=$1

    # Generate random number (0-9) for content
    local random_num=$((RANDOM % 10))

    # Randomly choose between streaming and non-streaming endpoint
    local use_streaming=$((RANDOM % 2))

    local endpoint
    if [ $use_streaming -eq 1 ]; then
        endpoint="streamGenerateContent?key=$API_KEY&alt=sse"
        local mode="streaming"
    else
        endpoint="generateContent?key=$API_KEY"
        local mode="non-streaming"
    fi

    # Full URL for this request
    local full_url="$BASE_URL:$endpoint"

    # Create payload with random number in content
    local payload='{
    "contents": [
      {
        "parts": [
          {
            "text": "I am testing my API proxy, tell me a random fact about the number '"$random_num"', longer better."
          }
        ]
      }
    ],
    "generationConfig": {
      "temperature": 0.7,
      "topP": 0.95,
      "topK": 40
    }
  }'

    echo "Request $request_id started at $(date) - Mode: $mode, Number: $random_num"

    # Create a unique log file for this request
    local log_file="request_${request_id}_${mode}_${random_num}.log"

    # Send the request and save the response
    curl -X POST \
        -H "Content-Type: application/json" \
        -d "$payload" \
        "$full_url" >"$log_file" 2>&1

    echo "Request $request_id completed at $(date), response saved to $log_file"
}

# Counter for request IDs
request_id=1

# Loop to send requests every 1 second
while true; do
    # Launch request in background but still capture output
    send_request $request_id &

    # Increment request counter
    ((request_id++))

    # Wait for 1 second before the next request
    sleep 1
done
