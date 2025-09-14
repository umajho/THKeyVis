#!/bin/bash

# build_rust.sh - Script to build Rust library for macOS

set -e

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Navigate to the core Rust project directory
CORE_DIR="${SCRIPT_DIR}/../core"

echo "Building Rust library..."

# Build the Rust library for the target architecture
cd "${CORE_DIR}"

# Build for the current architecture (library only)
if [[ $(uname -m) == 'arm64' ]]; then
    echo "Building for arm64 (Apple Silicon)..."
    cargo build --release --lib --target aarch64-apple-darwin
    RUST_LIB_PATH="${CORE_DIR}/target/aarch64-apple-darwin/release/libcore.a"
else
    echo "Building for x86_64 (Intel)..."
    cargo build --release --lib --target x86_64-apple-darwin
    RUST_LIB_PATH="${CORE_DIR}/target/x86_64-apple-darwin/release/libcore.a"
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