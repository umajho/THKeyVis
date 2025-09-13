//
//  KeyMonitor.swift
//  THKeyVis
//

import Cocoa
import SwiftUI
import ApplicationServices
import Foundation
import Carbon

class KeyMonitor: ObservableObject {
    @Published var pressedKeys = Set<String>()
    @Published var hasAccessibilityPermission = false
    @Published var currentLayoutName = "Unknown"
    @Published var isRemapModeEnabled = false
    private var eventTap: CFMachPort?
    private var permissionTimer: Timer?
    private var layoutCheckTimer: Timer?
    private var inputSource: TISInputSource?
    private var lastLayoutName: String = ""
    
    init() {
        setupInputSourceMonitoring()
        updateCurrentInputSource()
        checkAndRequestPermission()
        startPermissionMonitoring()
        setupApplicationNotifications()
    }
    
    private func setupApplicationNotifications() {
        // Listen for app becoming active (after permission dialogs, etc.)
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(applicationDidBecomeActive),
            name: NSApplication.didBecomeActiveNotification,
            object: nil
        )
        
        // Listen for app becoming inactive (when user goes to System Preferences)
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(applicationWillResignActive),
            name: NSApplication.willResignActiveNotification,
            object: nil
        )
    }
    
    @objc private func applicationDidBecomeActive() {
        // Check for both permission and layout changes when app becomes active (e.g., after permission dialogs)
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            self.checkPermissionStatus()
            self.checkForLayoutChange()
        }
    }
    
    @objc private func applicationWillResignActive() {
        // Store current permission state before app loses focus
        // This helps detect changes when we regain focus
        print("App losing focus - current permission state: \(hasAccessibilityPermission)")
    }
    
    private func checkPermissionStatus() {
        let accessEnabled = AXIsProcessTrusted()
        let wasEnabled = hasAccessibilityPermission
        hasAccessibilityPermission = accessEnabled
        
        if wasEnabled != accessEnabled {
            print("Permission state changed - was: \(wasEnabled), now: \(accessEnabled)")
            
            if !wasEnabled && accessEnabled {
                print("✅ Permission granted during focus change")
                setupKeyTap()
            } else if wasEnabled && !accessEnabled {
                print("❌ Permission revoked during focus change")
                if let currentEventTap = eventTap {
                    CFMachPortInvalidate(currentEventTap)
                }
                eventTap = nil
                pressedKeys.removeAll()
            }
        }
    }
    
    deinit {
        if let eventTap = eventTap {
            CFMachPortInvalidate(eventTap)
        }
        permissionTimer?.invalidate()
        layoutCheckTimer?.invalidate()
        
        // Remove all notification observers
        DistributedNotificationCenter.default().removeObserver(self)
        NotificationCenter.default.removeObserver(self)
    }
    
    private func checkAndRequestPermission() {
        let options: NSDictionary = [kAXTrustedCheckOptionPrompt.takeRetainedValue() as String: true]
        let accessEnabled = AXIsProcessTrustedWithOptions(options)
        
        DispatchQueue.main.async {
            self.hasAccessibilityPermission = accessEnabled
        }
        
        if accessEnabled {
            setupKeyTap()
        } else {
            print("Accessibility permission is required. Please grant permission in System Preferences.")
        }
    }
    
    private func startPermissionMonitoring() {
        // Check permission status more frequently (every 0.5 seconds) for better responsiveness
        permissionTimer = Timer.scheduledTimer(withTimeInterval: 0.5, repeats: true) { _ in
            let accessEnabled = AXIsProcessTrusted()
            
            DispatchQueue.main.async {
                let wasEnabled = self.hasAccessibilityPermission
                self.hasAccessibilityPermission = accessEnabled
                
                // If permission was just granted, setup key monitoring
                if !wasEnabled && accessEnabled {
                    print("✅ Accessibility permission granted - setting up key monitoring")
                    self.setupKeyTap()
                    // Also check for layout changes that might have occurred while permission dialogs were showing
                    self.checkForLayoutChange()
                }
                // If permission was revoked, clean up immediately
                else if wasEnabled && !accessEnabled {
                    print("❌ Accessibility permission revoked - cleaning up")
                    if let eventTap = self.eventTap {
                        CFMachPortInvalidate(eventTap)
                        self.eventTap = nil
                    }
                    self.pressedKeys.removeAll()
                }
                
                // Always check layout when permission state changes (dialogs may affect input source)
                if wasEnabled != accessEnabled {
                    self.checkForLayoutChange()
                }
            }
        }
    }
    
    private func setupKeyTap() {
        // Clean up existing event tap if it exists
        if let existingEventTap = eventTap {
            CFMachPortInvalidate(existingEventTap)
        }
        
        guard hasAccessibilityPermission else {
            print("Cannot setup key tap: Accessibility permission not granted")
            return
        }
        
        let eventMask = (1 << CGEventType.keyDown.rawValue) | (1 << CGEventType.keyUp.rawValue)
        
        eventTap = CGEvent.tapCreate(
            tap: .cgSessionEventTap,
            place: .headInsertEventTap,
            options: .defaultTap,
            eventsOfInterest: CGEventMask(eventMask),
            callback: { (proxy, type, event, refcon) -> Unmanaged<CGEvent>? in
                if let refcon = refcon {
                    let keyMonitor = Unmanaged<KeyMonitor>.fromOpaque(refcon).takeUnretainedValue()
                    return keyMonitor.processKeyEvent(type: type, event: event)
                }
                return Unmanaged.passRetained(event)
            },
            userInfo: Unmanaged.passUnretained(self).toOpaque()
        )
        
        if let eventTap = eventTap {
            let runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, eventTap, 0)
            CFRunLoopAddSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes)
            CGEvent.tapEnable(tap: eventTap, enable: true)
            print("✅ Key monitoring enabled successfully")
            
            // Validate that the event tap is actually working
            DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
                self.validateEventTap()
            }
        } else {
            print("❌ Failed to create event tap - permission may have been revoked")
            // If we can't create the event tap but AXIsProcessTrusted says we have permission,
            // force a permission state update
            DispatchQueue.main.async {
                self.hasAccessibilityPermission = false
            }
        }
    }
    
    private func validateEventTap() {
        // Check if event tap is still valid and enabled
        if let currentEventTap = eventTap {
            let isEnabled = CGEvent.tapIsEnabled(tap: currentEventTap)
            if !isEnabled {
                print("⚠️ Event tap is disabled - permission may have been revoked")
                DispatchQueue.main.async {
                    self.hasAccessibilityPermission = false
                    if let eventTap = self.eventTap {
                        CFMachPortInvalidate(eventTap)
                    }
                    self.eventTap = nil
                    self.pressedKeys.removeAll()
                }
            }
        } else if hasAccessibilityPermission {
            print("⚠️ Event tap is nil but permission flag is true - correcting state")
            DispatchQueue.main.async {
                self.hasAccessibilityPermission = false
                self.pressedKeys.removeAll()
            }
        }
    }
    
    private func processKeyEvent(type: CGEventType, event: CGEvent) -> Unmanaged<CGEvent>? {
        let originalKeyCode = event.getIntegerValueField(.keyboardEventKeycode)
        
        // Always update our internal state for visualization
        handleKeyEvent(type: type, event: event)
        
        // Special case: Handle space -> shift remapping
        if isRemapModeEnabled && originalKeyCode == 49 { // Space key
            // Create a shift modifier event instead of a regular key event
            if let shiftEvent = CGEvent(keyboardEventSource: nil,
                                      virtualKey: 56, // Left shift key code
                                      keyDown: type == .keyDown) {
                
                // Set the shift modifier flag
                if type == .keyDown {
                    shiftEvent.flags.insert(.maskShift)
                } else {
                    shiftEvent.flags.remove(.maskShift)
                }
                
                print("🔄 Remapping space to shift: \(type == .keyDown ? "down" : "up")")
                return Unmanaged.passRetained(shiftEvent)
            }
        }
        
        // Check if this key should be remapped (regular keys)
        if let remappedKeyCode = getRemappedKeyCode(for: Int(originalKeyCode)) {
            // Create a new event with the remapped key code
            if let newEvent = CGEvent(keyboardEventSource: nil, 
                                    virtualKey: CGKeyCode(remappedKeyCode), 
                                    keyDown: type == .keyDown) {
                
                // Copy any modifier flags from the original event
                newEvent.flags = event.flags
                print("🔄 Remapping key: \(originalKeyCode) -> \(remappedKeyCode)")
                return Unmanaged.passRetained(newEvent)
            }
        }
        
        // No remapping needed, pass through original event
        return Unmanaged.passRetained(event)
    }
    
    private func handleKeyEvent(type: CGEventType, event: CGEvent) {
        let keyCode = event.getIntegerValueField(.keyboardEventKeycode)
        let keyString = keyCodeToString(keyCode: Int(keyCode))
        
        DispatchQueue.main.async {
            switch type {
            case .keyDown:
                self.pressedKeys.insert(keyString)
            case .keyUp:
                self.pressedKeys.remove(keyString)
            default:
                break
            }
        }
    }
    
    private func setupInputSourceMonitoring() {
        // Listen for multiple types of input source changes for better reliability
        DistributedNotificationCenter.default().addObserver(
            self,
            selector: #selector(inputSourceChanged),
            name: NSNotification.Name(kTISNotifySelectedKeyboardInputSourceChanged as String),
            object: nil
        )
        
        // Also listen for input source enabled/disabled changes
        DistributedNotificationCenter.default().addObserver(
            self,
            selector: #selector(inputSourceChanged),
            name: NSNotification.Name(kTISNotifyEnabledKeyboardInputSourcesChanged as String),
            object: nil
        )
        
        // Set up a periodic check as fallback for missed notifications
        layoutCheckTimer = Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { _ in
            self.checkForLayoutChange()
        }
    }
    
    @objc private func inputSourceChanged() {
        updateCurrentInputSource()
    }
    
    private func checkForLayoutChange() {
        // Periodic check in case notifications are missed
        let currentSource = TISCopyCurrentKeyboardInputSource().takeRetainedValue()
        
        if let nameRef = TISGetInputSourceProperty(currentSource, kTISPropertyLocalizedName) {
            let name = Unmanaged<CFString>.fromOpaque(nameRef).takeUnretainedValue() as String
            
            if name != lastLayoutName {
                print("Layout change detected via periodic check: \(lastLayoutName) -> \(name)")
                inputSource = currentSource
                DispatchQueue.main.async {
                    self.currentLayoutName = name
                }
                lastLayoutName = name
            }
        }
    }
    
    private func updateCurrentInputSource() {
        inputSource = TISCopyCurrentKeyboardInputSource().takeRetainedValue()
        
        // Get the layout name for display
        if let inputSource = inputSource {
            let nameRef = TISGetInputSourceProperty(inputSource, kTISPropertyLocalizedName)
            if let nameRef = nameRef {
                let name = Unmanaged<CFString>.fromOpaque(nameRef).takeUnretainedValue() as String
                DispatchQueue.main.async {
                    self.currentLayoutName = name
                }
                lastLayoutName = name
                print("Keyboard layout changed to: \(name)")
            }
        }
    }
    
    private func keyCodeToString(keyCode: Int) -> String {
        // Handle special keys that don't have character representation
        switch keyCode {
        case 57: return "esc"    // Caps Lock (remapped to Esc in system prefs)
        case 53: return "esc"    // Actual Esc key
        case 51: return "backspace" // Backspace
        case 49: return "space"  // Space
        default: break
        }
        
        // Get the actual character from the current keyboard layout
        if let character = getCharacterForKeyCode(keyCode: keyCode) {
            return character.lowercased()
        }
        
        // Fallback to our static mapping for our specific keys
        switch keyCode {
        case 0: return "a"       
        case 1: return "r"       
        case 2: return "s"       
        case 3: return "t"       
        case 38: return "n"      
        case 40: return "e"      
        case 37: return "i"      
        case 41: return "o"      
        default: return "unknown"
        }
    }
    
    private func getCharacterForKeyCode(keyCode: Int) -> String? {
        guard let inputSource = inputSource else { return nil }
        
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
    
    // Public method to get character for specific key codes used in the UI
    func getCharacterForUIKey(keyCode: Int) -> String {
        // Try to get the character from the current layout
        if let character = getCharacterForKeyCode(keyCode: keyCode), !character.isEmpty {
            return character
        }
        
        // Fallback to physical key position labels for CJK and other complex layouts
        switch keyCode {
        case 0: return "A"  // A position
        case 1: return "S"  // S position  
        case 2: return "D"  // D position
        case 3: return "F"  // F position
        case 38: return "J" // J position
        case 40: return "K" // K position
        case 37: return "L" // L position
        case 41: return ";" // ; position
        default: return "?"
        }
    }
    
    // Map original key codes to remapped key codes based on the remap settings
    private func getRemappedKeyCode(for originalKeyCode: Int) -> Int? {
        guard isRemapModeEnabled else { return nil }
        
        switch originalKeyCode {
        case 1:  return 15  // S position -> R position (QWERTY S -> R)  
        case 3:  return 7   // F position -> X position (QWERTY F -> X)
        case 38: return 123 // J position -> Left Arrow
        case 40: return 126 // K position -> Up Arrow  
        case 37: return 125 // L position -> Down Arrow
        case 41: return 124 // ; position -> Right Arrow
        case 51: return 6   // Backspace -> Z position
        // Space (49) is handled as special case in processKeyEvent
        default: return nil
        }
    }
}
