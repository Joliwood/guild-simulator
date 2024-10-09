#!/bin/bash

# Add the target if not already added
if ! rustup target list --installed | grep -q x86_64h-apple-darwin; then
  rustup target add x86_64h-apple-darwin
fi

# Build the project
cargo build --target x86_64h-apple-darwin --release

# Check if build was successful
if [ $? -eq 0 ]; then
  echo "Build successful!"
  echo "Binary located at: target/x86_64h-apple-darwin/release/"
else
  echo "Build failed!"
fi
