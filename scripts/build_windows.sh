#!/bin/bash

# Variables
GAME_EXECUTABLE_NAME=guild_simulator
VERSION=${1:-v1.0.0}  # You can provide the version as an argument, default is v1.0.0
TARGET_DIR="target/dist"
ASSETS_DIR="assets"
CREDITS_DIR="credits"
ZIP_FILE="${GAME_EXECUTABLE_NAME}_windows_${VERSION}.zip"

# Clean up any previous build artifacts
echo "Cleaning up previous build artifacts..."
rm -rf "$TARGET_DIR"
rm -f "$ZIP_FILE"

# Step 1: Build the Rust project for Windows
echo "Building the Rust project for Windows..."
cargo build --profile dist --target x86_64-pc-windows-gnu

# Step 2: Prepare the release files
echo "Preparing release files..."
mkdir -p "${TARGET_DIR}/assets"
mkdir -p "${TARGET_DIR}/credits"
cp -r "$ASSETS_DIR" "${TARGET_DIR}"
cp -r "$CREDITS_DIR" "${TARGET_DIR}"
cp "target/x86_64-pc-windows-gnu/dist/${GAME_EXECUTABLE_NAME}.exe" "${TARGET_DIR}/"

# Step 3: Create a zip file containing the game executable, assets, and credits
echo "Creating a zip file..."
zip -r "$ZIP_FILE" "$TARGET_DIR"

echo "Windows package has been created: $ZIP_FILE"
