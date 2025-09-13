//
//  WindowManager.swift
//  THKeyVis
//
//  Created by Umaĵo on 2025/9/13.
//

import Cocoa
import SwiftUI

class WindowManager: NSObject, ObservableObject {
    private var window: NSWindow?
    
    func setupWindow() {
        // Find the main window
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            if let window = NSApplication.shared.windows.first {
                self.window = window
                self.configureWindow(window)
            }
        }
    }
    
    private func configureWindow(_ window: NSWindow) {
        // Set window level to always be on top (highest level)
        window.level = NSWindow.Level(rawValue: Int(CGWindowLevelForKey(.maximumWindow)))
        
        // Make window non-activating (won't steal focus)
        window.styleMask.remove(.resizable)
        window.styleMask.insert(.nonactivatingPanel)
        
        // Set window properties
        window.isOpaque = false
        window.backgroundColor = NSColor.clear
        window.hasShadow = true
        window.titlebarAppearsTransparent = true
        window.titleVisibility = .hidden
        
        // Make window stay on top even when other apps are focused
        window.hidesOnDeactivate = false
        window.canHide = false
        window.collectionBehavior = [.canJoinAllSpaces, .fullScreenAuxiliary]
        
        // Position window at bottom-center for gaming visibility
        if let screen = NSScreen.main {
            let screenFrame = screen.visibleFrame
            let windowSize = window.frame.size
            let newOrigin = NSPoint(
                x: screenFrame.midX - windowSize.width / 2,
                y: screenFrame.minY + 50
            )
            window.setFrameOrigin(newOrigin)
        }
    }
    
    func bringToFront() {
        window?.orderFrontRegardless()
    }
}
