#!/bin/bash

echo "Starting URVIC Development with Hot Reload..."

# Function to kill background processes on exit
cleanup() {
    echo "Stopping development servers..."
    kill $FRONTEND_PID $BACKEND_PID 2>/dev/null
    exit
}

trap cleanup EXIT

# Start Frontend with hot reload
echo "Starting React Frontend (Hot Reload)..."
cd urvic-front
npm run dev &
FRONTEND_PID=$!
cd ..

# Wait a moment for frontend to start
sleep 3

# Start Backend
echo "Starting Rust Backend..."
cargo run &
BACKEND_PID=$!

echo ""
echo "Development servers started:"
echo "Frontend (Hot Reload): http://localhost:3000"
echo "Backend API: http://localhost:20090"
echo ""
echo "Press Ctrl+C to stop both servers"

# Wait for user to stop
wait
