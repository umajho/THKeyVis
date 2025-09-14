import Foundation
import ApplicationServices

// Swift implementation of permission monitoring that Rust will call
@_cdecl("swift_start_permission_monitoring")
func swiftStartPermissionMonitoring() {
    // This function will be called from the parent process (after fork)
    // Start monitoring accessibility permission in a background thread
    DispatchQueue.global(qos: .background).async {
        while true {
            let hasPermission = AXIsProcessTrusted()
            set_accessibility_permission(hasPermission)
            
            // Check every 0.5 seconds (matching original KeyMonitor frequency)
            Thread.sleep(forTimeInterval: 0.5)
        }
    }
    
    // This function should return immediately after starting the background monitoring
    // The monitoring will continue in the background thread
    print("Swift permission monitoring started in background thread")
}

// Call the Rust main function with Swift permission monitoring callback
rust_main_with_callback(swiftStartPermissionMonitoring)

// When Rust function returns, terminate the app
exit(0)