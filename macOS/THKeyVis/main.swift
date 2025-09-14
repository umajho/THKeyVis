import Foundation
import ApplicationServices
import AppKit

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

// Swift function that Rust calls to open System Preferences
@_cdecl("swift_open_system_preferences")
func swiftOpenSystemPreferences() {
    DispatchQueue.main.async {
        if let url = URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility") {
            NSWorkspace.shared.open(url)
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