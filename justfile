# Build debug configuration
build-debug:
    xcodebuild -project THKeyVis.xcodeproj -scheme THKeyVis -configuration Debug build

# Build release configuration  
build-release:
    xcodebuild -project THKeyVis.xcodeproj -scheme THKeyVis -configuration Release build

# Run debug build
run-debug:
    #!/usr/bin/env bash
    APP_PATH=$(xcodebuild -project THKeyVis.xcodeproj -scheme THKeyVis -configuration Debug -showBuildSettings | grep "BUILT_PRODUCTS_DIR" | head -1 | sed 's/.*= //')
    open "$APP_PATH/THKeyVis.app"

# Run release build
run-release:
    #!/usr/bin/env bash
    APP_PATH=$(xcodebuild -project THKeyVis.xcodeproj -scheme THKeyVis -configuration Release -showBuildSettings | grep "BUILT_PRODUCTS_DIR" | head -1 | sed 's/.*= //')
    open "$APP_PATH/THKeyVis.app"
