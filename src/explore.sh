#!/bin/bash

# Define the URLs and their names
names=(
    "market_info"
    "featured"
    "market"
    "coming_soon"
    "only_at_alamo"
)

urls=(
    "https://drafthouse.com/s/mother/v1/page/cclamp/nyc?useUnifiedSchedule=true"
    "https://drafthouse.com/s/mother/v2/schedule/featured/nyc"
    "https://drafthouse.com/s/mother/v2/schedule/market/nyc"
    "https://drafthouse.com/s/mother/v2/schedule/coming-soon/nyc"
    "https://drafthouse.com/s/mother/v2/schedule/collection/only-at-the-alamo/nyc"
)

# Common headers for all requests
headers=(
    -H 'accept: application/json, text/plain, */*'
    -H 'accept-language: en-US,en;q=0.9'
    -H 'referer: https://drafthouse.com/nyc'
    -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36'
)

# Fetch data for each URL
for i in "${!names[@]}"; do
    name="${names[$i]}"
    url="${urls[$i]}"
    output_file="${name}.json"
    echo "Fetching $name data..."
    curl "$url" \
        "${headers[@]}" \
        | jq "." > "data/$output_file"
    echo "Saved to $output_file"
    
    # Analyze the structure
    echo -e "\n=== $name Structure ===\n"
    echo "Top-level keys:"
    jq 'keys' "$output_file"
    echo -e "\nSample data fields:"
    jq '.data[0] | keys' "$output_file" 2>/dev/null || echo "No data array found"
    echo -e "\n"
done