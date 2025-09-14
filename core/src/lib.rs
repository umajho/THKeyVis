use raylib::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::process::Command;
use std::ptr;
use std::slice;

// Shared memory structure for cross-process communication
#[repr(C)]
pub struct SharedState {
    pub has_accessibility_permission: bool,
    // Key states array - index corresponds to key codes
    // For now we'll use a simple approach: index 0 = A key
    pub key_states: [bool; 256], // Support for 256 different keys
}

impl SharedState {
    const A_KEY_INDEX: usize = 0;

    pub fn new() -> Self {
        Self {
            has_accessibility_permission: false,
            key_states: [false; 256],
        }
    }

    pub fn set_a_key(&mut self, pressed: bool) {
        self.key_states[Self::A_KEY_INDEX] = pressed;
    }

    pub fn is_a_key_pressed(&self) -> bool {
        self.key_states[Self::A_KEY_INDEX]
    }
}

// Legacy Swift FFI compatibility
#[repr(C)]
pub struct PermissionState {
    pub has_accessibility_permission: bool,
}

// Global shared state - will be set by Swift and read by Rust
static mut PERMISSION_STATE: PermissionState = PermissionState {
    has_accessibility_permission: false,
};

// Type definition for permission monitoring callback
type PermissionMonitoringCallback = unsafe extern "C" fn();

// This is the new main entry point that forks early with permission monitoring callback
#[unsafe(no_mangle)]
pub extern "C" fn rust_main_with_callback(callback: Option<PermissionMonitoringCallback>) {
    // Create shared memory file
    let shared_mem_path = "/tmp/thkeyvis_shared_state";
    let shared_state = create_shared_memory(shared_mem_path);

    // Set global pointer for Swift FFI access
    unsafe {
        SHARED_STATE_PTR = shared_state;
    }

    // Fork the process BEFORE any UI initialization
    match unsafe { libc::fork() } {
        -1 => {
            eprintln!("Failed to fork process");
            std::process::exit(1);
        }
        0 => {
            // Child process: Run pure Rust UI
            run_ui_process(shared_state);
        }
        child_pid => {
            // Parent process: Run key monitoring and permission checking
            run_key_monitor_process(shared_state, child_pid, callback);
        }
    }
}

// Compatibility function that calls rust_main_with_callback with no callback
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() {
    rust_main_with_callback(None);
}

pub fn init() {
    // Legacy function - now just calls the new main with no callback
    rust_main();
}

fn create_shared_memory(path: &str) -> *mut SharedState {
    // Create/open file for shared memory
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Failed to create shared memory file");

    // Write initial data to set file size
    let initial_state = SharedState::new();
    let state_bytes = unsafe {
        slice::from_raw_parts(
            &initial_state as *const _ as *const u8,
            std::mem::size_of::<SharedState>(),
        )
    };
    file.write_all(state_bytes)
        .expect("Failed to write initial state");
    file.flush().expect("Failed to flush file");

    // Memory map the file
    let fd = file.as_raw_fd();
    let size = std::mem::size_of::<SharedState>();

    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        panic!("Failed to create memory mapping");
    }

    ptr as *mut SharedState
}

fn run_key_monitor_process(
    shared_state: *mut SharedState,
    child_pid: i32,
    callback: Option<PermissionMonitoringCallback>,
) {
    println!("Parent process: Starting key monitoring...");

    // Set up signal handler to clean up when child exits
    extern "C" fn signal_handler(_: i32) {
        std::process::exit(0);
    }
    unsafe {
        libc::signal(libc::SIGCHLD, signal_handler as usize);
    }

    // Start permission monitoring - use Swift callback if provided, otherwise use Rust fallback
    if let Some(permission_callback) = callback {
        println!("Parent process: Starting Swift permission monitoring...");
        unsafe {
            permission_callback();
        }
    } else {
        println!("Parent process: Starting fallback Rust permission monitoring...");
        // Start a separate thread for permission checking (fallback)
        let permission_shared_state = shared_state as usize; // Convert to usize for thread safety
        std::thread::spawn(move || {
            let shared_ptr = permission_shared_state as *mut SharedState;
            loop {
                // Check accessibility permission every 500ms
                let has_permission = check_accessibility_permission();
                unsafe {
                    (*shared_ptr).has_accessibility_permission = has_permission;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        });
    }

    // Start rdev listener - this is the parent process so no thread safety issues
    if let Err(error) = rdev::listen(move |event| {
        let state = unsafe { &mut *shared_state };

        match event.event_type {
            rdev::EventType::KeyPress(key) => {
                if key == rdev::Key::KeyA {
                    state.set_a_key(true);
                }
            }
            rdev::EventType::KeyRelease(key) => {
                if key == rdev::Key::KeyA {
                    state.set_a_key(false);
                }
            }
            _ => {} // Ignore other events
        }
    }) {
        eprintln!("Key monitoring error: {:?}", error);
    }

    // Wait for child to exit
    let mut status = 0;
    unsafe {
        libc::waitpid(child_pid, &mut status, 0);
    }
}

// Check accessibility permission using system call
fn check_accessibility_permission() -> bool {
    use std::process::Command;

    // Use AppleScript to check accessibility permission
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to return true")
        .output();

    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}

fn run_ui_process(shared_state: *mut SharedState) {
    let (mut rl, thread) = raylib::init().size(640, 480).title("THKeyVis").build();
    rl.set_target_fps(120);

    while !rl.window_should_close() {
        // Get input state before drawing
        let mouse_pos = rl.get_mouse_position();
        let mouse_clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        // Read shared state once per frame
        let state = unsafe { &*shared_state };

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Check accessibility permission state from shared memory
        let has_permission = state.has_accessibility_permission;

        if !has_permission {
            // Permission warning banner - matching KeyboardView.swift exactly
            let banner_rect = Rectangle::new(20.0, 20.0, 600.0, 50.0);

            // Background with orange opacity (matching .orange.opacity(0.1))
            d.draw_rectangle_rounded(banner_rect, 0.16, 20, Color::new(255, 165, 0, 25)); // Orange with low opacity

            // Border (matching .orange.opacity(0.3))
            d.draw_rectangle_rounded_lines(banner_rect, 0.16, 20, Color::new(255, 165, 0, 76)); // Orange border

            // Triangle icon (exclamationmark.triangle.fill)
            d.draw_text("⚠", 32, 35, 16, Color::new(255, 165, 0, 255)); // Orange

            // Text: "Input Monitoring permission required"
            d.draw_text(
                "Input Monitoring permission required",
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
                    .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ListenEvent")
                    .spawn();
            }
        }

        // Draw the A key representation
        let a_key_pressed = state.is_a_key_pressed();
        let key_rect = Rectangle::new(100.0, 150.0, 60.0, 60.0);

        // Color based on key state
        let (bg_color, border_color, text_color) = if a_key_pressed {
            // Pressed state - darker colors
            (
                Color::new(200, 200, 255, 255),
                Color::new(100, 100, 200, 255),
                Color::BLACK,
            )
        } else {
            // Unpressed state - lighter colors
            (
                Color::new(240, 240, 240, 255),
                Color::new(180, 180, 180, 255),
                Color::BLACK,
            )
        };

        // Draw key background
        d.draw_rectangle_rounded(key_rect, 0.1, 10, bg_color);

        // Draw key border
        d.draw_rectangle_rounded_lines(key_rect, 0.1, 10, border_color);

        // Draw the "A" text in the center
        let text_size = 24;
        let text_width = d.measure_text("A", text_size);
        let text_x = (key_rect.x + key_rect.width / 2.0 - text_width as f32 / 2.0) as i32;
        let text_y = (key_rect.y + key_rect.height / 2.0 - text_size as f32 / 2.0) as i32;

        d.draw_text("A", text_x, text_y, text_size, text_color);
    }
}

// Global shared memory pointer - will be set during init
static mut SHARED_STATE_PTR: *mut SharedState = ptr::null_mut();

// C FFI function to set permission state from Swift
#[unsafe(no_mangle)]
pub extern "C" fn set_accessibility_permission(has_permission: bool) {
    unsafe {
        if !SHARED_STATE_PTR.is_null() {
            (*SHARED_STATE_PTR).has_accessibility_permission = has_permission;
        } else {
            // Fallback to legacy global state if shared memory not initialized
            PERMISSION_STATE.has_accessibility_permission = has_permission;
        }
    }
}

// C FFI function to get permission state (for debugging)
#[unsafe(no_mangle)]
pub extern "C" fn get_accessibility_permission() -> bool {
    unsafe {
        if !SHARED_STATE_PTR.is_null() {
            (*SHARED_STATE_PTR).has_accessibility_permission
        } else {
            PERMISSION_STATE.has_accessibility_permission
        }
    }
}

// Export the init function with C ABI for Swift interop
#[unsafe(no_mangle)]
pub extern "C" fn rust_init() {
    init();
}
