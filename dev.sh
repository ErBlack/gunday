#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$SCRIPT_DIR"

SERVER_PID=""
WASM_PID=""

cleanup() {
    echo "Shutting down..."
    if [[ -n "$SERVER_PID" ]] && ps -p "$SERVER_PID" >/dev/null 2>&1; then
        kill "$SERVER_PID" 2>/dev/null || true
    fi
    if [[ -n "$WASM_PID" ]] && ps -p "$WASM_PID" >/dev/null 2>&1; then
        kill "$WASM_PID" 2>/dev/null || true
    fi
}

trap cleanup SIGINT SIGTERM EXIT

# Check if cargo-watch is installed
if ! command -v cargo-watch &> /dev/null; then
    echo "Installing cargo-watch..."
    cargo install --locked cargo-watch
fi

npx live-server page --port=3000 --no-browser &
SERVER_PID=$!

cargo watch -s "./build-wasm.sh dev" &
WASM_PID=$!

wait