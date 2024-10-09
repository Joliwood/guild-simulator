#!/bin/bash

# Set environment variables
GAME_EXECUTABLE_NAME="guild_simulator"
GAME_OSX_APP_NAME="GuildSimulator"
TARGET_DIR="target/aarch64-apple-darwin/release"
BUILD_DIR="build/macos"
DIST_DIR="dist"

# Ensure SDKROOT is set correctly
SDKROOT=$(xcrun -sdk macosx --show-sdk-path)
export SDKROOT

# Print the current operation
echo "🚀 Starting macOS ARM64 build..."

# Check if the Apple Silicon target is installed
if ! rustup target list | grep aarch64-apple-darwin | grep installed &> /dev/null
then
    echo "⚙️ Adding aarch64-apple-darwin target..."
    rustup target add aarch64-apple-darwin
fi

# Clean previous builds
echo "🧹 Cleaning up previous builds..."
rm -rf $BUILD_DIR $DIST_DIR
mkdir -p $BUILD_DIR/src/Game.app/Contents/MacOS/

# Build the release for macOS ARM64
echo "🏗️ Building the release for Apple Silicon..."
cargo build --release --target=aarch64-apple-darwin

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

# Copy the executable and assets to the macOS app structure
echo "📦 Creating macOS app structure..."
cp $TARGET_DIR/${GAME_EXECUTABLE_NAME} $BUILD_DIR/src/Game.app/Contents/MacOS/
mkdir -p $BUILD_DIR/src/Game.app/Contents/MacOS/assets
cp -r assets/ $BUILD_DIR/src/Game.app/Contents/MacOS/assets
cp -r credits/ $BUILD_DIR/src/Game.app/Contents/MacOS/credits

# Rename Game.app to your game-specific name
mv $BUILD_DIR/src/Game.app $BUILD_DIR/src/${GAME_OSX_APP_NAME}.app

# Create the DMG file
echo "💽 Creating DMG file..."
hdiutil create -fs HFS+ -volname "${GAME_OSX_APP_NAME}" -srcfolder $BUILD_DIR/src $DIST_DIR/${GAME_OSX_APP_NAME}_arm64.dmg

# Check if DMG creation was successful
if [ $? -ne 0 ]; then
    echo "❌ Failed to create DMG!"
    exit 1
fi

echo "🎉 Build complete! DMG created at: $DIST_DIR/${GAME_OSX_APP_NAME}_arm64.dmg"
