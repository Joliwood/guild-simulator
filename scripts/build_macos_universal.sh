#!/bin/bash

# Set environment variables
GAME_EXECUTABLE_NAME="guild_simulator"
GAME_OSX_APP_NAME="guild_simulator"
VERSION="$1"  # Pass the version as the first argument

if [ -z "$VERSION" ]; then
  echo "Error: Version not provided."
  echo "Usage: ./build-macos.sh <version>"
  exit 1
fi

# Ensure macOS SDK is set
export MACOSX_DEPLOYMENT_TARGET=11.0

# Clean up any previous builds
echo "Cleaning up old builds..."
rm -rf target/
rm -rf build/macos/src

# Install rust toolchain for Apple Silicon
echo "Installing rust toolchain for Apple Silicon..."
rustup target add aarch64-apple-darwin

# Build for Apple Silicon
echo "Building release for Apple Silicon..."
SDKROOT=$(xcrun -sdk macosx --show-sdk-path)
cargo build --profile dist --target=aarch64-apple-darwin

# Install rust toolchain for x86_64
echo "Installing rust toolchain for x86_64 Apple..."
rustup target add x86_64-apple-darwin

# Build for x86 Apple
echo "Building release for x86_64..."
SDKROOT=$(xcrun -sdk macosx --show-sdk-path)
cargo build --profile dist --target=x86_64-apple-darwin

# Create a universal binary
echo "Creating Universal Binary..."
lipo -create -output target/dist/${GAME_EXECUTABLE_NAME} \
  target/aarch64-apple-darwin/dist/${GAME_EXECUTABLE_NAME} \
  target/x86_64-apple-darwin/dist/${GAME_EXECUTABLE_NAME}

# Create the release directory structure
echo "Creating macOS app structure..."
mkdir -p build/macos/src/Game.app/Contents/MacOS/assets
mkdir -p build/macos/src/Game.app/Contents/MacOS/credits

# Copy assets and executable
echo "Copying assets and executable..."
cp -r assets/ build/macos/src/Game.app/Contents/MacOS/assets
cp -r credits/ build/macos/src/Game.app/Contents/MacOS/credits
cp target/dist/${GAME_EXECUTABLE_NAME} build/macos/src/Game.app/Contents/MacOS/

# Rename the app
mv build/macos/src/Game.app build/macos/src/${GAME_OSX_APP_NAME}.app

# Create a symbolic link to /Applications
ln -s /Applications build/macos/src/

# Package everything into a .dmg
echo "Creating .dmg package..."
hdiutil create -fs HFS+ -volname "${GAME_OSX_APP_NAME}" -srcfolder build/macos/src ${GAME_EXECUTABLE_NAME}_${VERSION}_macOS.dmg

echo "Build and packaging complete. DMG file: ${GAME_EXECUTABLE_NAME}_${VERSION}_macOS.dmg"
