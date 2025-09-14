import Foundation
import ApplicationServices

// Function to monitor accessibility permission in a background thread
func startPermissionMonitoring() {
    DispatchQueue.global(qos: .background).async {
        while true {
            let hasPermission = AXIsProcessTrusted()
            set_accessibility_permission(hasPermission)
            
            // Check every 0.5 seconds (matching original KeyMonitor frequency)
            Thread.sleep(forTimeInterval: 0.5)
        }
    }
}

// Start permission monitoring thread before handing control to Rust
startPermissionMonitoring()

// Give the permission monitor a moment to initialize
Thread.sleep(forTimeInterval: 0.1)

// Call the Rust init function on the main thread
rust_init()

// When Rust function returns, terminate the app
exit(0)