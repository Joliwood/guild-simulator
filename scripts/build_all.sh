#!/bin/bash

# Check if version argument is provided
if [ -z "$1" ]; then
    echo "Usage: ./build_all.sh <version>"
    exit 1
fi

VERSION=$1

# Run each build script with the provided version
./scripts/build_macos_universal.sh "$VERSION"
./scripts/build_web.sh "$VERSION"
./scripts/build_windows.sh "$VERSION"

echo "All builds completed with version $VERSION"
