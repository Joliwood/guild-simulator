#!/bin/bash

# Set environment variables
GAME_EXECUTABLE_NAME="bevy_game"
TARGET_DIR="dist"

# Print the current operation
echo "🚀 Starting web build..."

# Check if trunk is installed
if ! command -v trunk &> /dev/null
then
    echo "⚠️ trunk could not be found, installing trunk..."
    cargo install trunk
fi

# Add the wasm32 target if not already added
if ! rustup target list | grep wasm32-unknown-unknown | grep installed &> /dev/null
then
    echo "⚙️ Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Clean up previous builds
echo "🧹 Cleaning up previous builds..."
rm -rf $TARGET_DIR

# Build the release for the web
echo "🏗️ Building the web release with trunk..."
trunk build --release

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

# Optimize the WASM file using wasm-opt (optional)
if command -v wasm-opt &> /dev/null
then
    echo "⚙️ Optimizing WASM file..."
    wasm-opt -O3 -o $TARGET_DIR/${GAME_EXECUTABLE_NAME}_optimized.wasm $TARGET_DIR/*.wasm
    mv $TARGET_DIR/${GAME_EXECUTABLE_NAME}_optimized.wasm $TARGET_DIR/${GAME_EXECUTABLE_NAME}.wasm
else
    echo "⚠️ wasm-opt not found, skipping optimization."
fi

# Serve the web version locally
echo "🌐 Serving the web version using trunk..."
trunk serve

