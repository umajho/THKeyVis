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
    // Current keyboard layout name (null-terminated string, max 63 chars + null terminator)
    pub current_layout_name: [u8; 64],
    // Key states for the specific keys we monitor
    pub key_states: KeyStates,
}

// Specific keys we monitor based on SPECIFICATION.md
#[repr(C)]
pub struct KeyStates {
    // Left side keys
    pub esc: bool,
    pub key_a: bool, // A position (keycode 0)
    pub key_r: bool, // R position (keycode 1)
    pub key_s: bool, // S position (keycode 2)
    pub key_t: bool, // T position (keycode 3)
    pub backspace: bool,
    // Right side keys
    pub key_n: bool, // N position (keycode 38)
    pub key_e: bool, // E position (keycode 40)
    pub key_i: bool, // I position (keycode 37)
    pub key_o: bool, // O position (keycode 41)
    pub space: bool,
    // Key labels for current layout (null-terminated strings)
    pub label_a: [u8; 8], // Label for A position
    pub label_r: [u8; 8], // Label for R position
    pub label_s: [u8; 8], // Label for S position
    pub label_t: [u8; 8], // Label for T position
    pub label_n: [u8; 8], // Label for N position
    pub label_e: [u8; 8], // Label for E position
    pub label_i: [u8; 8], // Label for I position
    pub label_o: [u8; 8], // Label for O position
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            has_accessibility_permission: false,
            current_layout_name: [0; 64],
            key_states: KeyStates::new(),
        }
    }

    pub fn set_layout_name(&mut self, name: &str) {
        // Clear the array first
        self.current_layout_name = [0; 64];

        // Copy the string bytes, ensuring we don't exceed the buffer size
        let bytes = name.as_bytes();
        let copy_len = std::cmp::min(bytes.len(), 63); // Leave room for null terminator
        self.current_layout_name[..copy_len].copy_from_slice(&bytes[..copy_len]);
        // Array is already null-terminated due to initialization with zeros
    }

    pub fn get_layout_name(&self) -> String {
        // Find the null terminator
        let null_pos = self
            .current_layout_name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(64);
        String::from_utf8_lossy(&self.current_layout_name[..null_pos]).to_string()
    }

    pub fn set_key_label(&mut self, key_position: &str, label: &str) {
        let target_array = match key_position {
            "a" => &mut self.key_states.label_a,
            "r" => &mut self.key_states.label_r,
            "s" => &mut self.key_states.label_s,
            "t" => &mut self.key_states.label_t,
            "n" => &mut self.key_states.label_n,
            "e" => &mut self.key_states.label_e,
            "i" => &mut self.key_states.label_i,
            "o" => &mut self.key_states.label_o,
            _ => return,
        };

        // Clear the array first
        *target_array = [0; 8];

        // Copy the string bytes, ensuring we don't exceed the buffer size
        let bytes = label.as_bytes();
        let copy_len = std::cmp::min(bytes.len(), 7); // Leave room for null terminator
        target_array[..copy_len].copy_from_slice(&bytes[..copy_len]);
    }

    pub fn get_key_label(&self, key_position: &str) -> String {
        let source_array = match key_position {
            "a" => &self.key_states.label_a,
            "r" => &self.key_states.label_r,
            "s" => &self.key_states.label_s,
            "t" => &self.key_states.label_t,
            "n" => &self.key_states.label_n,
            "e" => &self.key_states.label_e,
            "i" => &self.key_states.label_i,
            "o" => &self.key_states.label_o,
            _ => return "?".to_string(),
        };

        let null_pos = source_array.iter().position(|&b| b == 0).unwrap_or(8);
        String::from_utf8_lossy(&source_array[..null_pos]).to_string()
    }
}

impl KeyStates {
    pub fn new() -> Self {
        Self {
            esc: false,
            key_a: false,
            key_r: false,
            key_s: false,
            key_t: false,
            backspace: false,
            key_n: false,
            key_e: false,
            key_i: false,
            key_o: false,
            space: false,
            label_a: [0; 8],
            label_r: [0; 8],
            label_s: [0; 8],
            label_t: [0; 8],
            label_n: [0; 8],
            label_e: [0; 8],
            label_i: [0; 8],
            label_o: [0; 8],
        }
    }

    pub fn set_key_state(&mut self, keycode: u32, pressed: bool) {
        match keycode {
            53 => self.esc = pressed,       // ESC
            0 => self.key_a = pressed,      // A position
            1 => self.key_r = pressed,      // R position (S in QWERTY)
            2 => self.key_s = pressed,      // S position (D in QWERTY)
            3 => self.key_t = pressed,      // T position (F in QWERTY)
            51 => self.backspace = pressed, // Backspace
            38 => self.key_n = pressed,     // N position (J in QWERTY)
            40 => self.key_e = pressed,     // E position (K in QWERTY)
            37 => self.key_i = pressed,     // I position (L in QWERTY)
            41 => self.key_o = pressed,     // O position (; in QWERTY)
            49 => self.space = pressed,     // Space
            _ => {}                         // Ignore other keys
        }
    }

    pub fn get_key_state(&self, keycode: u32) -> bool {
        match keycode {
            53 => self.esc,
            0 => self.key_a,
            1 => self.key_r,
            2 => self.key_s,
            3 => self.key_t,
            51 => self.backspace,
            38 => self.key_n,
            40 => self.key_e,
            37 => self.key_i,
            41 => self.key_o,
            49 => self.space,
            _ => false,
        }
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
                if let Some(keycode) = rdev_key_to_keycode(key) {
                    state.key_states.set_key_state(keycode, true);
                }
            }
            rdev::EventType::KeyRelease(key) => {
                if let Some(keycode) = rdev_key_to_keycode(key) {
                    state.key_states.set_key_state(keycode, false);
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
// Convert rdev Key to macOS keycode
fn rdev_key_to_keycode(key: rdev::Key) -> Option<u32> {
    match key {
        rdev::Key::Escape => Some(53),
        rdev::Key::KeyA => Some(0),
        rdev::Key::KeyS => Some(1), // This is the R position in Colemak
        rdev::Key::KeyD => Some(2), // This is the S position in Colemak
        rdev::Key::KeyF => Some(3), // This is the T position in Colemak
        rdev::Key::Backspace => Some(51),
        rdev::Key::KeyJ => Some(38), // This is the N position in Colemak
        rdev::Key::KeyK => Some(40), // This is the E position in Colemak
        rdev::Key::KeyL => Some(37), // This is the I position in Colemak
        rdev::Key::SemiColon => Some(41), // This is the O position in Colemak
        rdev::Key::Space => Some(49),
        _ => None,
    }
}

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

fn draw_keyboard_layout(
    d: &mut RaylibDrawHandle,
    state: &SharedState,
    has_permission: bool,
    icons: &GameIcons,
    vertical_offset: f32,
) {
    // Layout constants - matching SPECIFICATION.md layout
    const KEY_SIZE: f32 = 60.0;
    const KEY_SPACING: f32 = 10.0;
    const START_X: f32 = 40.0; // Reduced padding for symmetric layout
    let start_y = 25.0 + vertical_offset; // Reduced top padding

    // Left side keys: ESC to the left of A, then A, R, S, T in a row
    // ESC should be at the left of A according to specification
    let left_keys = [
        ("ESC", 53, START_X, start_y, KEY_SIZE, KEY_SIZE), // ESC at the left
        (
            "A",
            0,
            START_X + KEY_SIZE + KEY_SPACING,
            start_y,
            KEY_SIZE,
            KEY_SIZE,
        ),
        (
            "R",
            1,
            START_X + (KEY_SIZE + KEY_SPACING) * 2.0,
            start_y,
            KEY_SIZE,
            KEY_SIZE,
        ),
        (
            "S",
            2,
            START_X + (KEY_SIZE + KEY_SPACING) * 3.0,
            start_y,
            KEY_SIZE,
            KEY_SIZE,
        ),
        (
            "T",
            3,
            START_X + (KEY_SIZE + KEY_SPACING) * 4.0,
            start_y,
            KEY_SIZE,
            KEY_SIZE,
        ),
    ];

    // BACKSPACE - aligned with A-T, not with ESC
    let backspace_x = START_X + KEY_SIZE + KEY_SPACING; // Start at A position
    let backspace_width = (KEY_SIZE + KEY_SPACING) * 4.0 - KEY_SPACING; // Span A through T
    let backspace = (
        "BACKSPACE",
        51,
        backspace_x,
        start_y + KEY_SIZE + KEY_SPACING,
        backspace_width,
        KEY_SIZE,
    );

    // Right side keys: N, E, I, O
    let right_start_x = START_X + (KEY_SIZE + KEY_SPACING) * 6.0; // Leave gap after left side
    let right_keys = [
        ("N", 38, right_start_x, start_y, KEY_SIZE, KEY_SIZE),
        (
            "E",
            40,
            right_start_x + KEY_SIZE + KEY_SPACING,
            start_y,
            KEY_SIZE,
            KEY_SIZE,
        ),
        (
            "I",
            37,
            right_start_x + (KEY_SIZE + KEY_SPACING) * 2.0,
            start_y,
            KEY_SIZE,
            KEY_SIZE,
        ),
        (
            "O",
            41,
            right_start_x + (KEY_SIZE + KEY_SPACING) * 3.0,
            start_y,
            KEY_SIZE,
            KEY_SIZE,
        ),
    ];

    // SPACE - spanning the right side keys
    let space_width = (KEY_SIZE + KEY_SPACING) * 4.0 - KEY_SPACING;
    let space = (
        "SPACE",
        49,
        right_start_x,
        start_y + KEY_SIZE + KEY_SPACING,
        space_width,
        KEY_SIZE,
    );

    // Draw all keys
    for &(label, keycode, x, y, width, height) in &left_keys {
        draw_key(
            d,
            label,
            keycode,
            x,
            y,
            width,
            height,
            state,
            has_permission,
            icons,
        );
    }

    for &(label, keycode, x, y, width, height) in &right_keys {
        draw_key(
            d,
            label,
            keycode,
            x,
            y,
            width,
            height,
            state,
            has_permission,
            icons,
        );
    }

    // Draw special keys
    draw_key(
        d,
        backspace.0,
        backspace.1,
        backspace.2,
        backspace.3,
        backspace.4,
        backspace.5,
        state,
        has_permission,
        icons,
    );
    draw_key(
        d,
        space.0,
        space.1,
        space.2,
        space.3,
        space.4,
        space.5,
        state,
        has_permission,
        icons,
    );

    // Draw layout name at top left, positioned relative to keyboard
    let layout_name = state.get_layout_name();
    if !layout_name.is_empty() {
        d.draw_text(
            &format!("Layout: {}", layout_name),
            20,
            (start_y - 25.0) as i32,
            12,
            Color::DARKGRAY,
        );
    }
}

fn draw_key(
    d: &mut RaylibDrawHandle,
    default_label: &str,
    keycode: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    state: &SharedState,
    has_permission: bool,
    icons: &GameIcons,
) {
    let key_rect = Rectangle::new(x, y, width, height);
    let is_pressed = state.key_states.get_key_state(keycode);

    // Determine key colors based on state
    let (bg_color, border_color, text_color) = if !has_permission {
        // Red when permissions missing
        (
            Color::new(255, 200, 200, 255),
            Color::new(200, 100, 100, 255),
            Color::DARKRED,
        )
    } else if is_pressed {
        // Pressed state - highlighted
        (
            Color::new(150, 200, 255, 255),
            Color::new(100, 150, 200, 255),
            Color::BLACK,
        )
    } else {
        // Normal state
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

    // Get the main label for this key (from layout or default)
    let main_label = get_key_main_label(default_label, keycode, state);

    // Draw main label (center, prominent)
    let text_size = if width > 100.0 { 16 } else { 20 };
    let text_width = d.measure_text(&main_label, text_size);
    let text_x = (x + width / 2.0 - text_width as f32 / 2.0) as i32;
    let text_y = (y + height / 2.0 - text_size as f32 / 2.0) as i32;
    d.draw_text(&main_label, text_x, text_y, text_size, text_color);

    // Draw QWERTY hint (top-left, blue, small) - shows QWERTY position for non-special keys
    let qwerty_hint = get_qwerty_hint(keycode);
    if !qwerty_hint.is_empty() && main_label.to_uppercase() != qwerty_hint.to_uppercase() {
        d.draw_text(
            &qwerty_hint,
            (x + 3.0) as i32,
            (y + 3.0) as i32,
            8,
            Color::BLUE,
        );
    }

    // Draw functional icon (bottom) for gaming context
    if let Some(icon_texture) = icons.get_icon_texture(keycode) {
        // Icon size (small, bottom of key)
        let icon_size = 16.0;
        let icon_x = x + width / 2.0 - icon_size / 2.0;
        let icon_y = y + height - icon_size - 2.0;

        // Draw the icon with appropriate tint based on key state
        let icon_tint = if has_permission {
            Color::new(100, 100, 100, 200) // Semi-transparent dark gray
        } else {
            Color::new(150, 50, 50, 150) // Semi-transparent dark red when no permission
        };

        d.draw_texture_ex(
            icon_texture,
            Vector2::new(icon_x, icon_y),
            0.0,                                   // rotation
            icon_size / icon_texture.width as f32, // scale to fit icon_size
            icon_tint,
        );
    }
}

fn get_key_main_label(default_label: &str, keycode: u32, state: &SharedState) -> String {
    // Get label from current keyboard layout stored in shared state
    let layout_label = match keycode {
        0 => state.get_key_label("a"),
        1 => state.get_key_label("r"),
        2 => state.get_key_label("s"),
        3 => state.get_key_label("t"),
        38 => state.get_key_label("n"),
        40 => state.get_key_label("e"),
        37 => state.get_key_label("i"),
        41 => state.get_key_label("o"),
        _ => String::new(),
    };

    if !layout_label.is_empty() && layout_label != "?" {
        layout_label.to_uppercase()
    } else {
        default_label.to_string()
    }
}

fn get_qwerty_hint(keycode: u32) -> &'static str {
    // Returns the QWERTY character for the physical key position
    match keycode {
        0 => "A",  // A position
        1 => "S",  // S position in QWERTY
        2 => "D",  // D position in QWERTY
        3 => "F",  // F position in QWERTY
        38 => "J", // J position in QWERTY
        40 => "K", // K position in QWERTY
        37 => "L", // L position in QWERTY
        41 => ";", // ; position in QWERTY
        _ => "",   // No hint for special keys
    }
}

fn run_ui_process(shared_state: *mut SharedState) {
    // Constants for layout
    const BANNER_HEIGHT: i32 = 90; // Banner + margin space
    const BASE_HEIGHT: i32 = 180; // Reduced height for symmetric vertical padding
    const WINDOW_WIDTH: i32 = 770; // Calculated: keyboard width (690) + symmetric padding (40×2)

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, BASE_HEIGHT + BANNER_HEIGHT)
        .title("THKeyVis")
        .build();
    rl.set_target_fps(120);

    // Load game icons
    let mut icons = GameIcons::new();
    icons.load_icons(&mut rl, &thread);

    let mut last_permission_state = false;

    while !rl.window_should_close() {
        // Get input state before drawing
        let mouse_pos = rl.get_mouse_position();
        let mouse_clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        // Read shared state once per frame
        let state = unsafe { &*shared_state };

        // Check accessibility permission state from shared memory
        let has_permission = state.has_accessibility_permission;

        // Dynamically resize window based on permission state
        if has_permission != last_permission_state {
            let target_height = if has_permission {
                BASE_HEIGHT
            } else {
                BASE_HEIGHT + BANNER_HEIGHT
            };
            rl.set_window_size(WINDOW_WIDTH, target_height);
            last_permission_state = has_permission;
        }

        // Get window dimensions before drawing
        let window_width = rl.get_screen_width() as f32;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        if !has_permission {
            // Permission warning banner - centered horizontally
            let window_width = window_width;
            let banner_width = 570.0; // Fits well in 770px window with padding
            let banner_height = 50.0;
            let banner_x = (window_width - banner_width) / 2.0;
            let banner_y = 20.0;
            let banner_rect = Rectangle::new(banner_x, banner_y, banner_width, banner_height);

            // Background with orange opacity (matching .orange.opacity(0.1))
            d.draw_rectangle_rounded(banner_rect, 0.16, 20, Color::new(255, 165, 0, 25)); // Orange with low opacity

            // Border (matching .orange.opacity(0.3))
            d.draw_rectangle_rounded_lines(banner_rect, 0.16, 20, Color::new(255, 165, 0, 76)); // Orange border

            // Triangle icon (exclamationmark.triangle.fill) - positioned relative to banner
            d.draw_text(
                "⚠",
                (banner_x + 12.0) as i32,
                (banner_y + 15.0) as i32,
                16,
                Color::new(255, 165, 0, 255),
            ); // Orange

            // Text: "Input Monitoring permission required" - positioned relative to banner
            d.draw_text(
                "Input Monitoring permission required",
                (banner_x + 35.0) as i32,
                (banner_y + 18.0) as i32,
                12,
                Color::new(255, 165, 0, 255), // Orange text
            );

            // "Open Settings" button area (right side of banner)
            let button_width = 100.0;
            let button_height = 20.0;
            let button_x = banner_x + banner_width - button_width - 15.0;
            let button_y = banner_y + (banner_height - button_height) / 2.0;
            let button_rect = Rectangle::new(button_x, button_y, button_width, button_height);

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

            d.draw_text(
                "Open Settings",
                (button_x + 5.0) as i32,
                (button_y + 6.0) as i32,
                11,
                button_color,
            );

            // Handle button click
            if is_button_hovered && mouse_clicked {
                // Open System Preferences directly from Rust
                let _ = Command::new("open")
                    .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ListenEvent")
                    .spawn();
            }
        }

        // Draw keyboard layout according to SPECIFICATION.md
        // Calculate vertical offset based on whether banner is shown
        let keyboard_offset_y = if has_permission {
            0.0
        } else {
            BANNER_HEIGHT as f32
        };
        draw_keyboard_layout(&mut d, state, has_permission, &icons, keyboard_offset_y);
    }
}

// Embedded icon data using include_bytes!
const ARROW_LEFT_PNG: &[u8] = include_bytes!("../../icons/MaterialSymbolsArrowBack.png");
const ARROW_RIGHT_PNG: &[u8] = include_bytes!("../../icons/MaterialSymbolsArrowForward.png");
const ARROW_UP_PNG: &[u8] = include_bytes!("../../icons/MaterialSymbolsArrowUpward.png");
const ARROW_DOWN_PNG: &[u8] = include_bytes!("../../icons/MaterialSymbolsArrowDownward.png");
const BOMB_PNG: &[u8] = include_bytes!("../../icons/MaterialSymbolsBomb.png");
const FOCUS_PNG: &[u8] = include_bytes!("../../icons/MaterialSymbolsFilterCenterFocus.png");
const REFRESH_PNG: &[u8] = include_bytes!("../../icons/MaterialSymbolsRefresh.png");
const SHOOT_PNG: &[u8] = include_bytes!("../../icons/EosIconsTroubleshooting.png");

// Icon storage for gaming icons
struct GameIcons {
    arrow_left: Option<Texture2D>,
    arrow_right: Option<Texture2D>,
    arrow_up: Option<Texture2D>,
    arrow_down: Option<Texture2D>,
    bomb: Option<Texture2D>,
    focus: Option<Texture2D>,
    refresh: Option<Texture2D>,
    shoot: Option<Texture2D>,
}

impl GameIcons {
    fn new() -> Self {
        Self {
            arrow_left: None,
            arrow_right: None,
            arrow_up: None,
            arrow_down: None,
            bomb: None,
            focus: None,
            refresh: None,
            shoot: None,
        }
    }

    fn load_icons(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        println!("Loading embedded icons...");

        // Load textures from embedded byte data
        if let Ok(image) = Image::load_image_from_mem(".png", ARROW_LEFT_PNG) {
            if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                self.arrow_left = Some(texture);
            }
        }
        if let Ok(image) = Image::load_image_from_mem(".png", ARROW_RIGHT_PNG) {
            if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                self.arrow_right = Some(texture);
            }
        }
        if let Ok(image) = Image::load_image_from_mem(".png", ARROW_UP_PNG) {
            if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                self.arrow_up = Some(texture);
            }
        }
        if let Ok(image) = Image::load_image_from_mem(".png", ARROW_DOWN_PNG) {
            if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                self.arrow_down = Some(texture);
            }
        }
        if let Ok(image) = Image::load_image_from_mem(".png", BOMB_PNG) {
            if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                self.bomb = Some(texture);
            }
        }
        if let Ok(image) = Image::load_image_from_mem(".png", FOCUS_PNG) {
            if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                self.focus = Some(texture);
            }
        }
        if let Ok(image) = Image::load_image_from_mem(".png", REFRESH_PNG) {
            if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                self.refresh = Some(texture);
            }
        }
        if let Ok(image) = Image::load_image_from_mem(".png", SHOOT_PNG) {
            if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                self.shoot = Some(texture);
            }
        }
    }

    fn get_icon_texture(&self, keycode: u32) -> Option<&Texture2D> {
        match keycode {
            2 => self.refresh.as_ref(),      // S position -> Retry (Refresh icon)
            3 => self.bomb.as_ref(),         // F position -> Bomb
            38 => self.arrow_left.as_ref(),  // J position -> Left Arrow
            40 => self.arrow_up.as_ref(),    // K position -> Up Arrow
            37 => self.arrow_down.as_ref(),  // L position -> Down Arrow
            41 => self.arrow_right.as_ref(), // ; position -> Right Arrow
            51 => self.shoot.as_ref(),       // Backspace -> Shot (EosIconsTroubleshooting)
            49 => self.focus.as_ref(),       // Space -> Focus Mode
            _ => None,
        }
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

// FFI functions for keyboard layout management
#[unsafe(no_mangle)]
pub extern "C" fn set_layout_name(name_ptr: *const std::os::raw::c_char) {
    unsafe {
        if !SHARED_STATE_PTR.is_null() && !name_ptr.is_null() {
            let name_cstr = std::ffi::CStr::from_ptr(name_ptr);
            if let Ok(name_str) = name_cstr.to_str() {
                (*SHARED_STATE_PTR).set_layout_name(name_str);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_key_label(
    position_ptr: *const std::os::raw::c_char,
    label_ptr: *const std::os::raw::c_char,
) {
    unsafe {
        if !SHARED_STATE_PTR.is_null() && !position_ptr.is_null() && !label_ptr.is_null() {
            let position_cstr = std::ffi::CStr::from_ptr(position_ptr);
            let label_cstr = std::ffi::CStr::from_ptr(label_ptr);

            if let (Ok(position_str), Ok(label_str)) = (position_cstr.to_str(), label_cstr.to_str())
            {
                (*SHARED_STATE_PTR).set_key_label(position_str, label_str);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_layout_name(buffer: *mut std::os::raw::c_char, buffer_size: usize) {
    unsafe {
        if !SHARED_STATE_PTR.is_null() && !buffer.is_null() && buffer_size > 0 {
            let layout_name = (*SHARED_STATE_PTR).get_layout_name();
            let name_bytes = layout_name.as_bytes();
            let copy_len = std::cmp::min(name_bytes.len(), buffer_size - 1);

            std::ptr::copy_nonoverlapping(name_bytes.as_ptr(), buffer as *mut u8, copy_len);
            *((buffer as *mut u8).add(copy_len)) = 0; // Null terminator
        }
    }
}

// Export the init function with C ABI for Swift interop
#[unsafe(no_mangle)]
pub extern "C" fn rust_init() {
    init();
}
