import Foundation

// Call the Rust main function immediately - this will fork before any UI initialization
rust_main()

// When Rust function returns, terminate the app
exit(0)