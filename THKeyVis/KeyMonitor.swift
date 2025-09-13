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
    }
    
    deinit {
        if let eventTap = eventTap {
            CFMachPortInvalidate(eventTap)
        }
        permissionTimer?.invalidate()
        layoutCheckTimer?.invalidate()
        
        // Remove all notification observers
        DistributedNotificationCenter.default().removeObserver(self)
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
        // Check permission status every 2 seconds
        permissionTimer = Timer.scheduledTimer(withTimeInterval: 2.0, repeats: true) { _ in
            let accessEnabled = AXIsProcessTrusted()
            
            DispatchQueue.main.async {
                let wasEnabled = self.hasAccessibilityPermission
                self.hasAccessibilityPermission = accessEnabled
                
                // If permission was just granted, setup key monitoring
                if !wasEnabled && accessEnabled {
                    self.setupKeyTap()
                }
                // If permission was revoked, clean up
                else if wasEnabled && !accessEnabled {
                    if let eventTap = self.eventTap {
                        CFMachPortInvalidate(eventTap)
                        self.eventTap = nil
                    }
                    self.pressedKeys.removeAll()
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
                    keyMonitor.handleKeyEvent(type: type, event: event)
                }
                return Unmanaged.passRetained(event)
            },
            userInfo: Unmanaged.passUnretained(self).toOpaque()
        )
        
        if let eventTap = eventTap {
            let runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, eventTap, 0)
            CFRunLoopAddSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes)
            CGEvent.tapEnable(tap: eventTap, enable: true)
            print("Key monitoring enabled successfully")
        } else {
            print("Failed to create event tap")
        }
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
}
