import Foundation

// Call the Rust init function on the main thread
rust_init()

// When Rust function returns, terminate the app
exit(0)