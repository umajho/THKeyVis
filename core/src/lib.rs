use raylib::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::process::Command;
use std::ptr;
use std::slice;

// External Swift functions
unsafe extern "C" {
    fn swift_setup_window_management();
}

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
    pub esc: bool,       // keycode 53
    pub key_0: bool,     // keycode 0 (A in QWERTY)
    pub key_1: bool,     // keycode 1 (S in QWERTY)
    pub key_2: bool,     // keycode 2 (D in QWERTY)
    pub key_3: bool,     // keycode 3 (F in QWERTY)
    pub backspace: bool, // keycode 51
    // Right side keys
    pub key_38: bool, // keycode 38 (J in QWERTY)
    pub key_40: bool, // keycode 40 (K in QWERTY)
    pub key_37: bool, // keycode 37 (L in QWERTY)
    pub key_41: bool, // keycode 41 (; in QWERTY)
    pub space: bool,  // keycode 49
    // Key labels for current layout (null-terminated strings)
    pub label_0: [u8; 8],  // Label for keycode 0 (A in QWERTY)
    pub label_1: [u8; 8],  // Label for keycode 1 (S in QWERTY)
    pub label_2: [u8; 8],  // Label for keycode 2 (D in QWERTY)
    pub label_3: [u8; 8],  // Label for keycode 3 (F in QWERTY)
    pub label_38: [u8; 8], // Label for keycode 38 (J in QWERTY)
    pub label_40: [u8; 8], // Label for keycode 40 (K in QWERTY)
    pub label_37: [u8; 8], // Label for keycode 37 (L in QWERTY)
    pub label_41: [u8; 8], // Label for keycode 41 (; in QWERTY)
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
            "a" => &mut self.key_states.label_0,  // A key -> keycode 0
            "s" => &mut self.key_states.label_1,  // S key -> keycode 1
            "d" => &mut self.key_states.label_2,  // D key -> keycode 2
            "f" => &mut self.key_states.label_3,  // F key -> keycode 3
            "j" => &mut self.key_states.label_38, // J key -> keycode 38
            "k" => &mut self.key_states.label_40, // K key -> keycode 40
            "l" => &mut self.key_states.label_37, // L key -> keycode 37
            ";" => &mut self.key_states.label_41, // ; key -> keycode 41
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
            "a" => &self.key_states.label_0,  // A key -> keycode 0
            "s" => &self.key_states.label_1,  // S key -> keycode 1
            "d" => &self.key_states.label_2,  // D key -> keycode 2
            "f" => &self.key_states.label_3,  // F key -> keycode 3
            "j" => &self.key_states.label_38, // J key -> keycode 38
            "k" => &self.key_states.label_40, // K key -> keycode 40
            "l" => &self.key_states.label_37, // L key -> keycode 37
            ";" => &self.key_states.label_41, // ; key -> keycode 41
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
            key_0: false,
            key_1: false,
            key_2: false,
            key_3: false,
            backspace: false,
            key_38: false,
            key_40: false,
            key_37: false,
            key_41: false,
            space: false,
            label_0: [0; 8],
            label_1: [0; 8],
            label_2: [0; 8],
            label_3: [0; 8],
            label_38: [0; 8],
            label_40: [0; 8],
            label_37: [0; 8],
            label_41: [0; 8],
        }
    }

    pub fn set_key_state(&mut self, keycode: u32, pressed: bool) {
        match keycode {
            53 => self.esc = pressed,       // ESC
            0 => self.key_0 = pressed,      // keycode 0 (A in QWERTY)
            1 => self.key_1 = pressed,      // keycode 1 (S in QWERTY)
            2 => self.key_2 = pressed,      // keycode 2 (D in QWERTY)
            3 => self.key_3 = pressed,      // keycode 3 (F in QWERTY)
            51 => self.backspace = pressed, // Backspace
            38 => self.key_38 = pressed,    // keycode 38 (J in QWERTY)
            40 => self.key_40 = pressed,    // keycode 40 (K in QWERTY)
            37 => self.key_37 = pressed,    // keycode 37 (L in QWERTY)
            41 => self.key_41 = pressed,    // keycode 41 (; in QWERTY)
            49 => self.space = pressed,     // Space
            _ => {}                         // Ignore other keys
        }
    }

    pub fn get_key_state(&self, keycode: u32) -> bool {
        match keycode {
            53 => self.esc,
            0 => self.key_0,
            1 => self.key_1,
            2 => self.key_2,
            3 => self.key_3,
            51 => self.backspace,
            38 => self.key_38,
            40 => self.key_40,
            37 => self.key_37,
            41 => self.key_41,
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

// Layout constants structure
struct KeyboardLayout {
    key_size: f32,
    key_spacing: f32,
    padding_x: f32,
    padding_y: f32,
    gap_multiplier: f32,
}

impl KeyboardLayout {
    fn new() -> Self {
        Self {
            key_size: 60.0,
            key_spacing: 10.0,
            padding_x: 40.0,
            padding_y: 25.0,
            gap_multiplier: 5.5,
        }
    }

    fn right_start_x(&self) -> f32 {
        self.padding_x + (self.key_size + self.key_spacing) * self.gap_multiplier
    }
}

fn get_key_label(state: &SharedState, keycode: u16) -> &str {
    match keycode {
        0 => {
            let bytes = &state.key_states.label_0;
            std::str::from_utf8(&bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())])
                .unwrap_or("A")
        }
        1 => {
            let bytes = &state.key_states.label_1;
            std::str::from_utf8(&bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())])
                .unwrap_or("S")
        }
        2 => {
            let bytes = &state.key_states.label_2;
            std::str::from_utf8(&bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())])
                .unwrap_or("D")
        }
        3 => {
            let bytes = &state.key_states.label_3;
            std::str::from_utf8(&bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())])
                .unwrap_or("F")
        }
        38 => {
            let bytes = &state.key_states.label_38;
            std::str::from_utf8(&bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())])
                .unwrap_or("J")
        }
        40 => {
            let bytes = &state.key_states.label_40;
            std::str::from_utf8(&bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())])
                .unwrap_or("K")
        }
        37 => {
            let bytes = &state.key_states.label_37;
            std::str::from_utf8(&bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())])
                .unwrap_or("L")
        }
        41 => {
            let bytes = &state.key_states.label_41;
            std::str::from_utf8(&bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())])
                .unwrap_or(";")
        }
        53 => "ESC",
        51 => "BACKSPACE",
        49 => "SPACE",
        _ => "?",
    }
}

fn draw_keyboard_layout(
    d: &mut RaylibDrawHandle,
    state: &SharedState,
    has_permission: bool,
    icons: &GameIcons,
    vertical_offset: f32,
) {
    // Use layout constants structure
    let layout = KeyboardLayout::new();
    let start_y = layout.padding_y + vertical_offset;

    // Left side keys: ESC to the left of A, then A, S, D, F in a row (using dynamic labels)
    // ESC should be at the left of A according to specification
    let left_keys = [
        (
            "ESC",
            53,
            layout.padding_x,
            start_y,
            layout.key_size,
            layout.key_size,
        ), // ESC at the left
        (
            get_key_label(state, 0), // First key (A in QWERTY)
            0,
            layout.padding_x + layout.key_size + layout.key_spacing,
            start_y,
            layout.key_size,
            layout.key_size,
        ),
        (
            get_key_label(state, 1), // Second key (S in QWERTY)
            1,
            layout.padding_x + (layout.key_size + layout.key_spacing) * 2.0,
            start_y,
            layout.key_size,
            layout.key_size,
        ),
        (
            get_key_label(state, 2), // Third key (D in QWERTY)
            2,
            layout.padding_x + (layout.key_size + layout.key_spacing) * 3.0,
            start_y,
            layout.key_size,
            layout.key_size,
        ),
        (
            get_key_label(state, 3), // Fourth key (F in QWERTY)
            3,
            layout.padding_x + (layout.key_size + layout.key_spacing) * 4.0,
            start_y,
            layout.key_size,
            layout.key_size,
        ),
    ]; // BACKSPACE - aligned with A-T, not with ESC
    let backspace_x = layout.padding_x + layout.key_size + layout.key_spacing; // Start at A position
    let backspace_width = (layout.key_size + layout.key_spacing) * 4.0 - layout.key_spacing; // Span A through T
    let backspace = (
        "BACKSPACE",
        51,
        backspace_x,
        start_y + layout.key_size + layout.key_spacing,
        backspace_width,
        layout.key_size,
    );

    // Right side keys: J, K, L, ; (using dynamic labels)
    let right_start_x = layout.right_start_x(); // Calculate from layout
    let right_keys = [
        (
            get_key_label(state, 38),
            38,
            right_start_x,
            start_y,
            layout.key_size,
            layout.key_size,
        ), // J in QWERTY
        (
            get_key_label(state, 40), // K in QWERTY
            40,
            right_start_x + layout.key_size + layout.key_spacing,
            start_y,
            layout.key_size,
            layout.key_size,
        ),
        (
            get_key_label(state, 37), // L in QWERTY
            37,
            right_start_x + (layout.key_size + layout.key_spacing) * 2.0,
            start_y,
            layout.key_size,
            layout.key_size,
        ),
        (
            get_key_label(state, 41), // ; in QWERTY
            41,
            right_start_x + (layout.key_size + layout.key_spacing) * 3.0,
            start_y,
            layout.key_size,
            layout.key_size,
        ),
    ];

    // SPACE - spanning the right side keys
    let space_width = (layout.key_size + layout.key_spacing) * 4.0 - layout.key_spacing;
    let space = (
        "SPACE",
        49,
        right_start_x,
        start_y + layout.key_size + layout.key_spacing,
        space_width,
        layout.key_size,
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

    // Draw layout name at top left, aligned with ESC key's left edge
    let layout_name = state.get_layout_name();
    if !layout_name.is_empty() {
        d.draw_text(
            &format!("Layout: {}", layout_name),
            layout.padding_x as i32,
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

    // Check if this key is unused (A or D keys don't have functions)
    let is_unused_key = keycode == 0 || keycode == 2;

    // Determine key colors based on state
    let (bg_color, border_color, text_color) = if !has_permission {
        // Red when permissions missing
        (
            Color::new(255, 200, 200, 255),
            Color::new(200, 100, 100, 255),
            Color::DARKRED,
        )
    } else if is_unused_key {
        // Grey out unused keys (A and D)
        (
            Color::new(200, 200, 200, 255),
            Color::new(150, 150, 150, 255),
            Color::new(120, 120, 120, 255),
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
        0 => state.get_key_label("a"),  // keycode 0 = A key
        1 => state.get_key_label("s"),  // keycode 1 = S key
        2 => state.get_key_label("d"),  // keycode 2 = D key
        3 => state.get_key_label("f"),  // keycode 3 = F key
        38 => state.get_key_label("j"), // keycode 38 = J key
        40 => state.get_key_label("k"), // keycode 40 = K key
        37 => state.get_key_label("l"), // keycode 37 = L key
        41 => state.get_key_label(";"), // keycode 41 = ; key
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

// Layout calculation structure
struct LayoutDimensions {
    window_width: i32,
    window_height: i32,
    base_height: i32,
    banner_height: i32,
}

// Banner layout constants and calculations
struct BannerLayout {
    banner_width: f32,
    banner_height: f32,
    banner_y: f32,
    button_width: f32,
    button_height: f32,
    button_margin: f32,
}

impl BannerLayout {
    fn new() -> Self {
        Self {
            banner_width: 570.0,
            banner_height: 50.0,
            banner_y: 20.0,
            button_width: 100.0,
            button_height: 20.0,
            button_margin: 15.0,
        }
    }

    fn banner_x(&self, window_width: f32) -> f32 {
        (window_width - self.banner_width) / 2.0
    }

    fn button_x(&self, window_width: f32) -> f32 {
        self.banner_x(window_width) + self.banner_width - self.button_width - self.button_margin
    }

    fn button_y(&self) -> f32 {
        self.banner_y + (self.banner_height - self.button_height) / 2.0
    }

    fn is_button_hovered(&self, window_width: f32, mouse_x: f32, mouse_y: f32) -> bool {
        let button_x = self.button_x(window_width);
        let button_y = self.button_y();

        mouse_x >= button_x
            && mouse_x <= button_x + self.button_width
            && mouse_y >= button_y
            && mouse_y <= button_y + self.button_height
    }
}

impl LayoutDimensions {
    fn calculate() -> Self {
        // Use the same layout constants as KeyboardLayout
        let keyboard_layout = KeyboardLayout::new();

        // Calculate keyboard dimensions
        // Right side: 4 keys (N,E,I,O)
        let right_width = keyboard_layout.key_size * 4.0 + keyboard_layout.key_spacing * 3.0;
        
        // Calculate actual keyboard span from leftmost to rightmost edge
        // Left edge: padding_x
        // Right edge: right_start_x + right_width  
        let right_start_x = keyboard_layout.right_start_x();
        let keyboard_right_edge = right_start_x + right_width;
        
        // Window width should have symmetric padding
        let window_width = (keyboard_right_edge + keyboard_layout.padding_x) as i32;

        // Calculate height
        // 2 rows of keys + backspace row + space row
        let keyboard_height = keyboard_layout.key_size * 2.0 + keyboard_layout.key_spacing * 1.0; // 2 key rows + spacing between them
        let base_height = (keyboard_height + keyboard_layout.padding_y * 2.0) as i32;

        // Banner height for permission warning
        let banner_height = 90;

        Self {
            window_width,
            window_height: base_height + banner_height,
            base_height,
            banner_height,
        }
    }
}

fn run_ui_process(shared_state: *mut SharedState) {
    // Calculate layout dimensions dynamically
    let layout = LayoutDimensions::calculate();

    let (mut rl, thread) = raylib::init()
        .size(layout.window_width, layout.window_height)
        .title("THKeyVis")
        .transparent()
        .build();
    rl.set_target_fps(120);

    // Setup window management (always-on-top, dragging, custom title) via Swift
    unsafe {
        swift_setup_window_management();
    }

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
                layout.base_height
            } else {
                layout.window_height
            };
            rl.set_window_size(layout.window_width, target_height);
            last_permission_state = has_permission;
        }

        // Get window dimensions before drawing
        let window_width = rl.get_screen_width() as f32;
        let window_height = rl.get_screen_height();

        // Calculate button area if permission banner is shown (for cursor change)
        let banner_layout = BannerLayout::new();
        let is_button_hovered = if !has_permission {
            banner_layout.is_button_hovered(window_width, mouse_pos.x, mouse_pos.y)
        } else {
            false
        };

        // Set cursor based on hover state
        if is_button_hovered {
            rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
        } else {
            rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        }

        let mut d = rl.begin_drawing(&thread);

        // Clear with transparent background
        d.clear_background(Color::new(0, 0, 0, 0));

        // Draw dark semi-transparent background (60% opacity)
        d.draw_rectangle(
            0,
            0,
            window_width as i32,
            window_height,
            Color::new(20, 20, 20, 153),
        );

        if !has_permission {
            // Permission warning banner - use consistent layout calculations
            let banner_x = banner_layout.banner_x(window_width);
            let banner_y = banner_layout.banner_y;
            let banner_width = banner_layout.banner_width;
            let banner_height = banner_layout.banner_height;
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

            // "Open Settings" button area (right side of banner) - use consistent calculations
            let button_x = banner_layout.button_x(window_width);
            let button_y = banner_layout.button_y();

            // Check if mouse is over button (use same calculation as cursor logic)
            let is_button_hovered =
                banner_layout.is_button_hovered(window_width, mouse_pos.x, mouse_pos.y);

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
            layout.banner_height as f32
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
            1 => self.refresh.as_ref(),      // S position -> Retry (Refresh icon)
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
