use raylib::prelude::*;
use std::process::Command;

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
        // Get input state before drawing
        let mouse_pos = rl.get_mouse_position();
        let mouse_clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Check accessibility permission state
        let has_permission = unsafe { PERMISSION_STATE.has_accessibility_permission };

        if !has_permission {
            // Permission warning banner - matching KeyboardView.swift exactly
            let banner_rect = Rectangle::new(20.0, 20.0, 600.0, 50.0);

            // Background with orange opacity (matching .orange.opacity(0.1))
            d.draw_rectangle_rounded(banner_rect, 0.16, 20, Color::new(255, 165, 0, 25)); // Orange with low opacity

            // Border (matching .orange.opacity(0.3))
            d.draw_rectangle_rounded_lines(banner_rect, 0.16, 20, Color::new(255, 165, 0, 76)); // Orange border

            // Triangle icon (exclamationmark.triangle.fill)
            d.draw_text("⚠", 32, 35, 16, Color::new(255, 165, 0, 255)); // Orange

            // Text: "Accessibility permission required"
            d.draw_text(
                "Accessibility permission required",
                55,
                38,
                12,
                Color::new(255, 165, 0, 255), // Orange text
            );

            // "Open Settings" button area (right side)
            let button_rect = Rectangle::new(470.0, 32.0, 100.0, 20.0);

            // Check if mouse is over button
            let is_button_hovered = mouse_pos.x >= button_rect.x
                && mouse_pos.x <= button_rect.x + button_rect.width
                && mouse_pos.y >= button_rect.y
                && mouse_pos.y <= button_rect.y + button_rect.height;

            // Button text
            let button_color = if is_button_hovered {
                Color::new(0, 0, 139, 255) // Darker blue when hovered
            } else {
                Color::new(0, 122, 255, 255) // Blue (matching SwiftUI .blue)
            };

            d.draw_text("Open Settings", 475, 38, 11, button_color);

            // Handle button click
            if is_button_hovered && mouse_clicked {
                // Open System Preferences directly from Rust
                let _ = Command::new("open")
                    .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
                    .spawn();
            }
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
