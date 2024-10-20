#!/bin/bash

# Set environment variables
GAME_EXECUTABLE_NAME="guild_simulator"
GAME_OSX_APP_NAME="GuildSimulator"
VERSION="$1"
ICON_FILE_PATH="./build/icon.icns"

# Check if version is provided
if [ -z "$VERSION" ]; then
  echo "Error: Version not provided."
  echo "Usage: ./build-macos.sh <version>"
  exit 1
fi

# Set macOS deployment target
export MACOSX_DEPLOYMENT_TARGET=11.0

# Clean up previous build artifacts
echo "Cleaning up old builds..."
rm -rf target/
rm -rf build/macos/src

# Install the Rust toolchains for both Apple Silicon and Intel architectures
echo "Installing rust toolchains for both architectures..."
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

# Build for Apple Silicon (aarch64)
echo "Building release for Apple Silicon (aarch64)..."
SDKROOT=$(xcrun -sdk macosx --show-sdk-path)
cargo build --profile dist --target=aarch64-apple-darwin

# Build for Intel (x86_64)
echo "Building release for Intel (x86_64)..."
cargo build --profile dist --target=x86_64-apple-darwin

# Create the release directory structure
echo "Creating macOS app structure..."
mkdir -p build/macos/src/Game.app/Contents/MacOS/assets
mkdir -p build/macos/src/Game.app/Contents/MacOS/credits
mkdir -p build/macos/src/Game.app/Contents/Resources

# Combine the Apple Silicon and Intel binaries into a universal binary using lipo
echo "Creating universal binary..."
lipo -create -output build/macos/src/Game.app/Contents/MacOS/${GAME_EXECUTABLE_NAME} \
    target/aarch64-apple-darwin/dist/${GAME_EXECUTABLE_NAME} \
    target/x86_64-apple-darwin/dist/${GAME_EXECUTABLE_NAME}

# Copy assets, credits, and the universal executable
echo "Copying assets and credits..."
cp -r assets/ build/macos/src/Game.app/Contents/MacOS/assets
cp -r credits/ build/macos/src/Game.app/Contents/MacOS/credits

# Copy icon to the app
if [ -f "$ICON_FILE_PATH" ]; then
  echo "Adding icon..."
  cp "$ICON_FILE_PATH" build/macos/src/Game.app/Contents/Resources/Icon.icns
else
  echo "Warning: Icon file not found at $ICON_FILE_PATH"
fi

# Create the Info.plist
echo "Creating Info.plist..."
cat <<EOF > build/macos/src/Game.app/Contents/Info.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>CFBundleName</key>
    <string>${GAME_OSX_APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>${GAME_OSX_APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.yourcompany.${GAME_OSX_APP_NAME}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleExecutable</key>
    <string>${GAME_EXECUTABLE_NAME}</string>
    <key>CFBundleIconFile</key>
    <string>Icon</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleSupportedPlatforms</key>
    <array>
      <string>MacOSX</string>
    </array>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
  </dict>
</plist>
EOF

# Rename the app
echo "Renaming the app..."
mv build/macos/src/Game.app build/macos/src/${GAME_OSX_APP_NAME}.app

# Create a symbolic link to /Applications (optional, used for DMG creation)
ln -s /Applications build/macos/src/

# Create a DMG package
echo "Creating DMG package..."
hdiutil create -fs HFS+ -volname "${GAME_OSX_APP_NAME}" -srcfolder build/macos/src "${GAME_EXECUTABLE_NAME}_${VERSION}_macOS_universal.dmg"

echo "Build and packaging complete. DMG file: ${GAME_EXECUTABLE_NAME}_${VERSION}_macOS.dmg"
