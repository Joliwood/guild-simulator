#!/bin/bash

# Variables
GAME_EXECUTABLE_NAME="guild_simulator"
VERSION=$1

if [ -z "$VERSION" ]; then
  echo "Error: Version not provided."
  echo "Usage: ./build_web.sh <version>"
  exit 1
fi

# Ensure trunk is installed
trunk --version &> /dev/null
if [ $? -ne 0 ]; then
  echo "Installing trunk..."
  cargo install trunk
fi

# Add wasm target
echo "Adding wasm target..."
rustup target add wasm32-unknown-unknown

# Build the web release
echo "Building web release..."
trunk build --release

# Optimize wasm
echo "Optimizing wasm..."
wasm-opt -O3 dist/*.wasm -o dist/optimized.wasm

# Zip the release
echo "Creating zip package..."
zip -r "${GAME_EXECUTABLE_NAME}_web_${VERSION}.zip" dist/

echo "Web build complete: ${GAME_EXECUTABLE_NAME}_web_${VERSION}.zip"
