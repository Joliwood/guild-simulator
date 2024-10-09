#!/bin/bash

set -e

# Variables for local build
BINARY="guild_simulator"        # Name of your binary
PACKAGE_NAME="guild_simulator"  # Name for the package
VERSION="v1.0.0"                # Version number, modify as needed
ASSETS_DIR="assets"             # Path to assets directory
TARGET="aarch64-apple-darwin"   # Target for macOS ARM64 (Apple Silicon)
OUT_DIR="tmp/package/${PACKAGE_NAME}.app/Contents/MacOS"  # Output directory for the build
PACKAGE_EXT=".dmg"              # Output package format

# Prepare output directories
rm -rf tmp
mkdir -p "$OUT_DIR"

# Set the deployment target for macOS
export MACOSX_DEPLOYMENT_TARGET=11.0

# Build the binary with cargo for macOS ARM64
echo "Building for macOS ARM64 (aarch64-apple-darwin)..."
cargo build --release --target="$TARGET" --no-default-features

# Move the built binary to the output directory
echo "Moving binary to output directory..."
mv target/"$TARGET"/release/"$BINARY" "$OUT_DIR/$BINARY"

# Set executable permissions for the binary
chmod +x "$OUT_DIR/$BINARY"

# Optionally copy assets (if they exist)
if [ -d "$ASSETS_DIR" ]; then
    echo "Copying assets to the package..."
    cp -r "$ASSETS_DIR" "tmp/package/${PACKAGE_NAME}.app/Contents"
fi

# Add metadata for macOS app bundle
echo "Adding macOS app metadata..."
mkdir -p "tmp/package/${PACKAGE_NAME}.app/Contents"  # Ensure the Contents directory exists
cat > "tmp/package/${PACKAGE_NAME}.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict>
        <key>CFBundleDevelopmentRegion</key>
        <string>en</string>
        <key>CFBundleDisplayName</key>
        <string>${PACKAGE_NAME}</string>
        <key>CFBundleExecutable</key>
        <string>${BINARY}</string>
        <key>CFBundleIdentifier</key>
        <string>com.example.${PACKAGE_NAME}</string>
        <key>CFBundleName</key>
        <string>${PACKAGE_NAME}</string>
        <key>CFBundleShortVersionString</key>
        <string>${VERSION}</string>
        <key>CFBundleVersion</key>
        <string>${VERSION}</string>
        <key>CFBundleInfoDictionaryVersion</key>
        <string>6.0</string>
        <key>CFBundlePackageType</key>
        <string>APPL</string>
        <key>CFBundleSupportedPlatforms</key>
        <array>
            <string>MacOSX</string>
        </array>
    </dict>
</plist>
EOF

# Create a .dmg file from the package
echo "Creating .dmg package..."
hdiutil create -fs HFS+ -volname "${PACKAGE_NAME}" -srcfolder "tmp/package" "${PACKAGE_NAME}-${VERSION}${PACKAGE_EXT}"

echo "Build and packaging for macOS ARM64 completed successfully!"
