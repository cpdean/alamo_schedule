#!/bin/bash

# Get local IP address
IP=$(ipconfig getifaddr en0)

# Check if IP was found
if [ -z "$IP" ]; then
    echo "Could not find IP address. Are you connected to WiFi?"
    exit 1
fi

PORT=8000

echo ""
echo "Starting server..."
echo ""
echo "Access from this device:    http://localhost:$PORT"
echo "Access from other devices:  http://$IP:$PORT"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Start Python HTTP server
python3 -m http.server $PORT
