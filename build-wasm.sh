#!/bin/bash

# Build script for Gunday WASM version
# This script builds the game for web deployment

echo "🎮 Building Gunday for Web (WASM)..."
echo "=================================="

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Check if wasm32-unknown-unknown target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "📦 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build the WASM version
echo "🔨 Building WASM package..."
wasm-pack build --target web --out-dir page --no-typescript

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📁 Generated files in ./page/:"
    ls -la page/
    echo ""
    echo "🌐 To test the game:"
    echo "   Open ./page/index.html in a web browser"
    echo "   Or run: python3 -m http.server 8000 (from ./page directory)"
else
    echo "❌ Build failed!"
    exit 1
fi
