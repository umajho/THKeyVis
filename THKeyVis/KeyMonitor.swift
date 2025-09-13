//
//  KeyMonitor.swift
//  THKeyVis
//
//  Created by Umaĵo on 2025/9/13.
//

import Cocoa
import SwiftUI
import ApplicationServices

class KeyMonitor: ObservableObject {
    @Published var pressedKeys = Set<String>()
    private var eventTap: CFMachPort?
    
    init() {
        requestInputMonitoringPermission()
        setupKeyTap()
    }
    
    deinit {
        if let eventTap = eventTap {
            CFMachPortInvalidate(eventTap)
        }
    }
    
    private func requestInputMonitoringPermission() {
        let options: NSDictionary = [kAXTrustedCheckOptionPrompt.takeRetainedValue() as String: true]
        let accessEnabled = AXIsProcessTrustedWithOptions(options)
        
        if !accessEnabled {
            print("Input monitoring permission is required. Please grant permission in System Preferences.")
        }
    }
    
    private func setupKeyTap() {
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
        // Map key codes to our specific keys (Colemak layout)
        switch keyCode {
        case 57: return "esc"    // Caps Lock (remapped to Esc in system prefs)
        case 53: return "esc"    // Actual Esc key (fallback)
        case 0: return "a"       // A (same in Colemak)
        case 15: return "r"      // R (same position in Colemak)
        case 1: return "s"       // S (same position in Colemak)  
        case 17: return "t"      // T (same position in Colemak)
        case 51: return "backspace" // Backspace
        case 45: return "n"      // N (same position in Colemak)
        case 14: return "e"      // E (different position in Colemak - F key)
        case 34: return "i"      // I (different position in Colemak - U key)
        case 31: return "o"      // O (different position in Colemak - Y key)
        case 49: return "space"  // Space
        default: return "unknown"
        }
    }
}
