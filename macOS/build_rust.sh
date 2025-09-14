#!/bin/bash

# build_rust.sh - Script to build Rust library for macOS

set -e

# Get build type from first argument, default to debug
BUILD_TYPE="${1:-debug}"

if [ "$BUILD_TYPE" != "debug" ] && [ "$BUILD_TYPE" != "release" ]; then
    echo "Error: Build type must be 'debug' or 'release'"
    echo "Usage: $0 [debug|release]"
    exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Navigate to the core Rust project directory
CORE_DIR="${SCRIPT_DIR}/../core"

echo "Building Rust library in $BUILD_TYPE mode..."

# Build the Rust library for the target architecture
cd "${CORE_DIR}"

# Build for the current architecture (library only)
if [[ $(uname -m) == 'arm64' ]]; then
    echo "Building for arm64 (Apple Silicon)..."
    if [ "$BUILD_TYPE" = "release" ]; then
        cargo build --release --lib --target aarch64-apple-darwin
        RUST_LIB_PATH="${CORE_DIR}/target/aarch64-apple-darwin/release/libcore.a"
    else
        cargo build --lib --target aarch64-apple-darwin
        RUST_LIB_PATH="${CORE_DIR}/target/aarch64-apple-darwin/debug/libcore.a"
    fi
else
    echo "Building for x86_64 (Intel)..."
    if [ "$BUILD_TYPE" = "release" ]; then
        cargo build --release --lib --target x86_64-apple-darwin
        RUST_LIB_PATH="${CORE_DIR}/target/x86_64-apple-darwin/release/libcore.a"
    else
        cargo build --lib --target x86_64-apple-darwin
        RUST_LIB_PATH="${CORE_DIR}/target/x86_64-apple-darwin/debug/libcore.a"
    fi
fi

echo "Rust library built at: ${RUST_LIB_PATH}"

# Copy the library to known locations for Xcode to link against
XCODE_LIB_DIR="${SCRIPT_DIR}/lib"
mkdir -p "${XCODE_LIB_DIR}"
cp "${RUST_LIB_PATH}" "${XCODE_LIB_DIR}/libcore.a"

# Also copy to Xcode's derived data directory if we can determine it
if [ -n "${BUILT_PRODUCTS_DIR}" ]; then
    echo "Copying to Xcode build directory: ${BUILT_PRODUCTS_DIR}"
    cp "${RUST_LIB_PATH}" "${BUILT_PRODUCTS_DIR}/libcore.a"
fi

echo "Rust library copied to: ${XCODE_LIB_DIR}/libcore.a"