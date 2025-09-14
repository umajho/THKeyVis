use raylib::prelude::*;

// Shared memory structure for permission state
#[repr(C)]
pub struct PermissionState {
    pub has_accessibility_permission: bool,
}

// Global shared state - will be set by Swift and read by Rust
static mut PERMISSION_STATE: PermissionState = PermissionState {
    has_accessibility_permission: false,
};

pub fn init() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("THKeyVis").build();

    rl.set_target_fps(120);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Check accessibility permission state
        let has_permission = unsafe { PERMISSION_STATE.has_accessibility_permission };

        if has_permission {
            d.draw_text("Permission granted - ready!", 12, 12, 20, Color::GREEN);
        } else {
            d.draw_text("Accessibility permission required", 12, 12, 20, Color::RED);
            d.draw_text(
                "Please grant permission in System Preferences",
                12,
                40,
                16,
                Color::RED,
            );
            d.draw_text("Security & Privacy > Accessibility", 12, 60, 16, Color::RED);
        }
    }
}

// C FFI function to set permission state from Swift
#[unsafe(no_mangle)]
pub extern "C" fn set_accessibility_permission(has_permission: bool) {
    unsafe {
        PERMISSION_STATE.has_accessibility_permission = has_permission;
    }
}

// C FFI function to get permission state (for debugging)
#[unsafe(no_mangle)]
pub extern "C" fn get_accessibility_permission() -> bool {
    unsafe { PERMISSION_STATE.has_accessibility_permission }
}

// Export the init function with C ABI for Swift interop
#[unsafe(no_mangle)]
pub extern "C" fn rust_init() {
    init();
}
