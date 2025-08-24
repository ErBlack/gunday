#!/bin/bash

cleanup() {
    echo "Shutting down..."
    if [ ! -z "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null
    fi
    if [ ! -z "$WASM_PID" ]; then
        kill $WASM_PID 2>/dev/null
    fi
    exit 0
}

trap cleanup SIGINT SIGTERM EXIT

npx serve page &
SERVER_PID=$!

cargo watch -s "wasm-pack build --target web --out-dir page --dev" &
WASM_PID=$!

wait