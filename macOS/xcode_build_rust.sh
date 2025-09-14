#!/bin/bash

# xcode_build_rust.sh - Build script for Xcode to compile Rust library

set -e

echo "Building Rust library for Xcode..."

# Get the directory where this script is located  
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
CORE_DIR="${SCRIPT_DIR}/../core"

# Build the Rust library
cd "${CORE_DIR}"

# Determine target architecture from Xcode environment variables
if [ "${ARCHS}" = "arm64" ] || [ "${ARCHS}" = "arm64 x86_64" ]; then
    echo "Building for arm64..."
    cargo build --release --lib --target aarch64-apple-darwin
    RUST_LIB_PATH="${CORE_DIR}/target/aarch64-apple-darwin/release/libcore.a"
elif [ "${ARCHS}" = "x86_64" ]; then
    echo "Building for x86_64..."
    cargo build --release --lib --target x86_64-apple-darwin  
    RUST_LIB_PATH="${CORE_DIR}/target/x86_64-apple-darwin/release/libcore.a"
else
    # Default to current architecture
    if [[ $(uname -m) == 'arm64' ]]; then
        echo "Building for arm64 (default)..."
        cargo build --release --lib --target aarch64-apple-darwin
        RUST_LIB_PATH="${CORE_DIR}/target/aarch64-apple-darwin/release/libcore.a"
    else
        echo "Building for x86_64 (default)..."
        cargo build --release --lib --target x86_64-apple-darwin
        RUST_LIB_PATH="${CORE_DIR}/target/x86_64-apple-darwin/release/libcore.a"
    fi
fi

# Copy library to Xcode's build directory
XCODE_LIB_DIR="${BUILT_PRODUCTS_DIR}"
mkdir -p "${XCODE_LIB_DIR}"
cp "${RUST_LIB_PATH}" "${XCODE_LIB_DIR}/libcore.a"

echo "Rust library built and copied to: ${XCODE_LIB_DIR}/libcore.a"