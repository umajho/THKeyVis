import Foundation
import ApplicationServices
import IOKit.hid
import Carbon
import Cocoa

// Global variables for keyboard layout monitoring
var currentInputSource: TISInputSource?
var lastLayoutName: String = ""

// Helper function to get character for a specific key code in current layout
func getCharacterForKeyCode(keyCode: Int, inputSource: TISInputSource) -> String? {
    // Get the keyboard layout data
    guard let layoutDataRef = TISGetInputSourceProperty(inputSource, kTISPropertyUnicodeKeyLayoutData) else {
        return nil
    }
    
    let layoutData = Unmanaged<CFData>.fromOpaque(layoutDataRef).takeUnretainedValue()
    let keyboardLayoutPtr = CFDataGetBytePtr(layoutData)
    
    var deadKeyState: UInt32 = 0
    var actualStringLength = 0
    var unicodeString = [UniChar](repeating: 0, count: 4)
    
    let status = UCKeyTranslate(
        UnsafePointer<UCKeyboardLayout>(OpaquePointer(keyboardLayoutPtr)),
        UInt16(keyCode),
        UInt16(kUCKeyActionDisplay),
        0, // no modifier keys
        UInt32(LMGetKbdType()),
        OptionBits(kUCKeyTranslateNoDeadKeysBit),
        &deadKeyState,
        4,
        &actualStringLength,
        &unicodeString
    )
    
    guard status == noErr && actualStringLength > 0 else {
        return nil
    }
    
    return String(utf16CodeUnits: unicodeString, count: actualStringLength)
}

// Helper function to update Rust with current layout information
func updateRustLayoutInfo(name: String, inputSource: TISInputSource) {
    // Send layout name to Rust
    set_layout_name(name)
    
    // Update key labels for each monitored key position
    let keyPositions = [
        ("a", 0),   // A key (keycode 0)
        ("s", 1),   // S key (keycode 1) 
        ("d", 2),   // D key (keycode 2)
        ("f", 3),   // F key (keycode 3)
        ("j", 38),  // J key (keycode 38)
        ("k", 40),  // K key (keycode 40)
        ("l", 37),  // L key (keycode 37)
        (";", 41),  // ; key (keycode 41)
    ]
    
    for (position, keyCode) in keyPositions {
        let label = getCharacterForKeyCode(keyCode: keyCode, inputSource: inputSource) ?? position.uppercased()
        print("Setting key label for position \(position) (keyCode \(keyCode)): \(label)")
        set_key_label(position, label)
    }
}

// Function to update current input source and notify Rust
func updateCurrentInputSource() {
    currentInputSource = TISCopyCurrentKeyboardInputSource().takeRetainedValue()
    
    // Get the layout name for display
    if let inputSource = currentInputSource {
        let nameRef = TISGetInputSourceProperty(inputSource, kTISPropertyLocalizedName)
        if let nameRef = nameRef {
            let name = Unmanaged<CFString>.fromOpaque(nameRef).takeUnretainedValue() as String
            if name != lastLayoutName {
                lastLayoutName = name
                print("Keyboard layout changed to: \(name)")
                
                // Update Rust with the new layout information
                updateRustLayoutInfo(name: name, inputSource: inputSource)
            }
        }
    }
}

// Function to setup keyboard layout monitoring
func setupKeyboardLayoutMonitoring() {
    // Initial update
    updateCurrentInputSource()
    
    // Listen for multiple types of input source changes for better reliability
    DistributedNotificationCenter.default().addObserver(
        forName: NSNotification.Name(kTISNotifySelectedKeyboardInputSourceChanged as String),
        object: nil,
        queue: .main
    ) { _ in
        updateCurrentInputSource()
    }
    
    // Also listen for input source enabled/disabled changes
    DistributedNotificationCenter.default().addObserver(
        forName: NSNotification.Name(kTISNotifyEnabledKeyboardInputSourcesChanged as String),
        object: nil,
        queue: .main
    ) { _ in
        updateCurrentInputSource()
    }
    
    // Set up a periodic check as fallback for missed notifications
    Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { _ in
        // Periodic check in case notifications are missed
        let currentSource = TISCopyCurrentKeyboardInputSource().takeRetainedValue()
        
        if let nameRef = TISGetInputSourceProperty(currentSource, kTISPropertyLocalizedName) {
            let name = Unmanaged<CFString>.fromOpaque(nameRef).takeUnretainedValue() as String
            
            if name != lastLayoutName {
                print("Layout change detected via periodic check: \(lastLayoutName) -> \(name)")
                currentInputSource = currentSource
                lastLayoutName = name
                
                // Update Rust with the new layout information
                updateRustLayoutInfo(name: name, inputSource: currentSource)
            }
        }
    }
}

// MARK: - Window Management Functions

// Function to find the raylib window and configure it
func findAndConfigureRaylibWindow() {
    // Wait a bit for raylib to create the window
    DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
        // Find all windows and look for our raylib window
        for window in NSApplication.shared.windows {
            if window.title == "THKeyVis" || window.contentView != nil {
                setupWindowProperties(window: window)
                break
            }
        }
        
        // If not found, try again after a longer delay
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
            for window in NSApplication.shared.windows {
                if window.contentView != nil {
                    setupWindowProperties(window: window)
                    break
                }
            }
        }
    }
}

// Configure window properties for always-on-top, dragging, and proper title
func setupWindowProperties(window: NSWindow) {
    // Set custom title
    window.title = "THKeyVis (https://github.com/umajho/THKeyVis)"
    
    // Make window always stay on top
    window.level = NSWindow.Level(rawValue: Int(CGWindowLevelForKey(.maximumWindow)))
    
    // Configure window behavior
    window.hidesOnDeactivate = false
    window.canHide = false
    window.collectionBehavior = [.canJoinAllSpaces, .fullScreenAuxiliary]
    
    // Enable transparency and dark background with 60% opacity
    window.backgroundColor = NSColor.clear
    window.isOpaque = false
    window.hasShadow = true
    
    // Enable dragging anywhere on the window
    window.isMovableByWindowBackground = true
    
    print("Window configured: Always-on-top, draggable, custom title set")
}



// Function that can be called from Rust to setup window management
@_cdecl("swift_setup_window_management") 
func swiftSetupWindowManagement() {
    findAndConfigureRaylibWindow()
}

// Swift implementation of system monitoring (permissions + keyboard layout) that Rust will call
@_cdecl("swift_start_system_monitoring")
func swiftStartSystemMonitoring() {
    // First, request Input Monitoring permission (this will show the dialog if needed)
    let initialRequest = IOHIDRequestAccess(kIOHIDRequestTypeListenEvent)
    print("Initial Input Monitoring permission request result: \(initialRequest)")
    
    // Setup keyboard layout monitoring immediately
    setupKeyboardLayoutMonitoring()
    
    // Force an initial layout update after a short delay to ensure everything is initialized
    DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
        updateCurrentInputSource()
    }
    
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
    print("Swift system monitoring (permissions + keyboard layout) started")
}

// Call the Rust main function with Swift system monitoring callback
rust_main_with_callback(swiftStartSystemMonitoring)

// When Rust function returns, terminate the app
exit(0)