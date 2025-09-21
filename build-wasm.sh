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
PROFILE=${1:-release}
FAST=${FAST:-0}
COMMON_FLAGS=(--target web --out-dir page/pkg --no-typescript)

case "$PROFILE" in
    dev)
        echo "🔨 Building WASM package (dev profile, fastest builds, skipping wasm-opt)..."
        WASM_PACK_NO_OPT=1 wasm-pack build --dev "${COMMON_FLAGS[@]}"
        ;;
    profiling|profile)
        echo "🔨 Building WASM package (profiling profile, skipping wasm-opt)..."
        WASM_PACK_NO_OPT=1 wasm-pack build --profiling "${COMMON_FLAGS[@]}"
        ;;
    release)
        if [ "$FAST" = "1" ]; then
            echo "🔨 Building WASM package (release profile, FAST=1 so skipping wasm-opt)..."
            WASM_PACK_NO_OPT=1 wasm-pack build --release "${COMMON_FLAGS[@]}"
        else
            echo "🔨 Building WASM package (release profile)..."
            wasm-pack build --release "${COMMON_FLAGS[@]}"
        fi
        ;;
    *)
        echo "❌ Unknown profile '$PROFILE'. Use one of: dev | profiling | release"
        exit 1
        ;;
esac

# wasm-bindgen 0.2.99 emits a bare "env" import that browsers can't resolve without an import map.
# Provide a tiny ES module so the specifier can be mapped safely.
cat <<'EOF' > page/pkg/env.js
// Auto-generated helper to satisfy the bare "env" import emitted by wasm-bindgen
// when targeting the web profile. Browsers resolve it via the import map in index.html.
export {};
EOF

node <<'NODE'
const fs = require('fs');
const path = 'page/pkg/gunday.js';
const original = "    imports['env'] = __wbg_star0;";
const replacement = `    imports['env'] = new Proxy(__wbg_star0, {\n        get(target, prop) {\n            if (prop in target) {\n                return target[prop];\n            }\n            const fn = (...args) => {\n                console.warn(\`wasm env import \${String(prop)} not implemented; using noop\`, args);\n                return 0;\n            };\n            return fn;\n        }\n    });`;

try {
    const contents = fs.readFileSync(path, 'utf8');
    if (!contents.includes(replacement)) {
        if (!contents.includes(original)) {
            console.error('[build-wasm] Unable to patch env proxy; signature not found');
            process.exit(1);
        }
        const updated = contents.replace(original, replacement);
        fs.writeFileSync(path, updated, 'utf8');
    }
} catch (err) {
    console.error('[build-wasm] Failed to patch env proxy:', err);
    process.exit(1);
}
NODE

