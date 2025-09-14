import Foundation
import ApplicationServices
import IOKit.hid

// Swift implementation of permission monitoring that Rust will call
@_cdecl("swift_start_permission_monitoring")
func swiftStartPermissionMonitoring() {
    // First, request Input Monitoring permission (this will show the dialog if needed)
    let initialRequest = IOHIDRequestAccess(kIOHIDRequestTypeListenEvent)
    print("Initial Input Monitoring permission request result: \(initialRequest)")
    
    // This function will be called from the parent process (after fork)
    // Start monitoring Input Monitoring permission in a background thread
    DispatchQueue.global(qos: .background).async {
        while true {
            // Check for Input Monitoring permission using IOHIDCheckAccess
            let accessType = IOHIDCheckAccess(kIOHIDRequestTypeListenEvent)
            let hasPermission = (accessType == kIOHIDAccessTypeGranted)
            set_accessibility_permission(hasPermission)
            
            // Check every 0.5 seconds (matching original KeyMonitor frequency)
            Thread.sleep(forTimeInterval: 0.5)
        }
    }
    
    // This function should return immediately after starting the background monitoring
    // The monitoring will continue in the background thread
    print("Swift Input Monitoring permission monitoring started in background thread")
}

// Call the Rust main function with Swift permission monitoring callback
rust_main_with_callback(swiftStartPermissionMonitoring)

// When Rust function returns, terminate the app
exit(0)