//
//  KeyMonitor.swift
//  THKeyVis
//

import Cocoa
import SwiftUI
import ApplicationServices
import Foundation

class KeyMonitor: ObservableObject {
    @Published var pressedKeys = Set<String>()
    @Published var hasAccessibilityPermission = false
    private var eventTap: CFMachPort?
    private var permissionTimer: Timer?
    
    init() {
        checkAndRequestPermission()
        startPermissionMonitoring()
    }
    
    deinit {
        if let eventTap = eventTap {
            CFMachPortInvalidate(eventTap)
        }
        permissionTimer?.invalidate()
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
    
    private func keyCodeToString(keyCode: Int) -> String {
        // Map physical key codes to Colemak layout characters
        switch keyCode {
        case 57: return "esc"    // Caps Lock (remapped to Esc in system prefs)
        case 53: return "esc"    // Actual Esc key (fallback)
        case 0: return "a"       // Physical A key → Colemak A
        case 1: return "r"       // Physical S key → Colemak R
        case 2: return "s"       // Physical D key → Colemak S  
        case 3: return "t"       // Physical F key → Colemak T
        case 51: return "backspace" // Backspace
        case 38: return "n"      // Physical J key → Colemak N
        case 40: return "e"      // Physical K key → Colemak E
        case 37: return "i"      // Physical L key → Colemak I
        case 41: return "o"      // Physical ; key → Colemak O
        case 49: return "space"  // Space
        default: return "unknown"
        }
    }
}
