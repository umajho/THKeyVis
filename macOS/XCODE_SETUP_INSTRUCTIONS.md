# Xcode Configuration Instructions for Rust Integration

This file contains the manual steps needed to configure the Xcode project to use
the Rust library.

## 1. Add Files to Project

1. Open THKeyVis.xcodeproj in Xcode
2. Right-click on the THKeyVis folder in the project navigator
3. Select "Add Files to 'THKeyVis'"
4. Navigate to and add:
   - `rust_bridge.h` (bridging header)
   - `main.swift` (new main file)

## 2. Configure Build Settings

### Add Library Search Paths

1. Select the THKeyVis project in the navigator
2. Select the THKeyVis target
3. Go to Build Settings tab
4. Find "Library Search Paths"
5. Add: `$(BUILT_PRODUCTS_DIR)`

### Add Other Linker Flags

1. In Build Settings, find "Other Linker Flags"
2. Add: `-lcore`

### Add Framework Dependencies

1. Select the THKeyVis target
2. Go to "Build Phases" tab
3. Expand "Link Binary With Libraries"
4. Click "+" and add these frameworks:
   - Cocoa.framework
   - OpenGL.framework
   - IOKit.framework
   - CoreVideo.framework
   - CoreGraphics.framework

### Set Objective-C Bridging Header

1. In Build Settings, find "Objective-C Bridging Header"
2. Set to: `THKeyVis/rust_bridge.h`

## 3. Add Build Script Phase

1. In Build Phases tab, click "+" button
2. Select "New Run Script Phase"
3. Move this phase to the top (before "Compile Sources")
4. **IMPORTANT**: Check "For install builds only" to avoid sandbox issues during
   development
5. Set the script to:
   ```bash
   # Inline script to avoid sandbox issues
   echo "Building Rust library..."

   # Navigate to core directory
   CORE_DIR="${SRCROOT}/../core"
   cd "${CORE_DIR}"

   # Build for current architecture
   if [[ $(uname -m) == 'arm64' ]]; then
       echo "Building for arm64..."
       cargo build --release --lib --target aarch64-apple-darwin
       RUST_LIB_PATH="${CORE_DIR}/target/aarch64-apple-darwin/release/libcore.a"
   else
       echo "Building for x86_64..."
       cargo build --release --lib --target x86_64-apple-darwin
       RUST_LIB_PATH="${CORE_DIR}/target/x86_64-apple-darwin/release/libcore.a"
   fi

   # Copy library to build directory
   cp "${RUST_LIB_PATH}" "${BUILT_PRODUCTS_DIR}/libcore.a"
   echo "Rust library copied to: ${BUILT_PRODUCTS_DIR}/libcore.a"
   ```
6. Add Input Files:
   - `${SRCROOT}/../core/src/lib.rs`
   - `${SRCROOT}/../core/Cargo.toml`
7. Add Output Files:
   - `${BUILT_PRODUCTS_DIR}/libcore.a`

## 4. Files Created

- `rust_bridge.h` - C header exposing rust_init() function
- `main.swift` - New main file that calls Rust instead of SwiftUI
- `build_rust.sh` - Standalone script to build Rust library
- `xcode_build_rust.sh` - Xcode-specific build script
- `THKeyVisApp.swift` - Modified to comment out SwiftUI @main

## 5. Troubleshooting Sandbox Issues

If you get sandbox permission errors when running the build script, you have
several options:

### ✅ SOLUTION: Use Justfile (Recommended)

The easiest solution is to use the provided justfile which handles the Rust
library build externally:

```bash
cd macOS
just build-debug    # Builds Rust first, then Xcode project
just run-debug      # Runs the built app
```

This approach:

- Builds the Rust library outside of Xcode's sandbox
- Copies the library to the correct DerivedData directory where Xcode expects it
- Builds the Xcode project successfully

### Alternative: Manual Build Process

```bash
cd macOS
./build_rust.sh     # Build Rust library first
# Copy to where Xcode looks for it:
cp lib/libcore.a /Users/um/Library/Developer/Xcode/DerivedData/THKeyVis-*/Build/Products/Debug/
xcodebuild -project THKeyVis.xcodeproj -scheme THKeyVis -configuration Debug
```

### Advanced: Disable Sandbox in Xcode

1. In Xcode, go to the build script phase
2. Check "For install builds only"
3. Or replace the external script call with the inline script provided above

## 6. ✅ SUCCESS - Build Complete

The integration is now working! The Swift app successfully:

- Calls the Rust `rust_init()` function on the main thread
- Displays the raylib window with "Hello, world!" text
- Terminates when the Rust function returns

### Build Commands:

```bash
cd macOS
just build-debug    # Build and link everything
just run-debug      # Launch the app
```

### What Happens:

1. Rust library is compiled to `libcore.a`
2. Swift `main.swift` calls `rust_init()` directly
3. Rust raylib window opens and runs until closed
4. App terminates when Rust function returns
