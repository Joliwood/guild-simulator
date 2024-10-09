#!/bin/bash
set -e

echo "Building for Windows..."

# Ensure the Windows target is installed
rustup target add x86_64-pc-windows-gnu

# Build the release for Windows
cargo build --release --target x86_64-pc-windows-gnu

echo "Build complete. Check the target/x86_64-pc-windows-gnu/release/ directory."
