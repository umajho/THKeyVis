use raylib::prelude::*;

pub fn init() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    rl.set_target_fps(120);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}

// Export the init function with C ABI for Swift interop
#[unsafe(no_mangle)]
pub extern "C" fn rust_init() {
    init();
}
