//
//  WindowManager.swift
//  THKeyVis
//

import Cocoa
import SwiftUI

class WindowManager: NSObject, ObservableObject, NSWindowDelegate {
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
        
        // Set window properties
        window.isOpaque = false
        window.backgroundColor = NSColor.clear
        window.hasShadow = true
        window.titlebarAppearsTransparent = true
        window.titleVisibility = .visible
        window.title = "THKeyVis (https://github.com/umajho/THKeyVis)"
        
        // Set window delegate to handle close events
        window.delegate = self
        
        // Make window stay on top even when other apps are focused
        window.hidesOnDeactivate = false
        window.canHide = true // Allow hiding with minimize button
        window.collectionBehavior = [.canJoinAllSpaces, .fullScreenAuxiliary]
        
        // Let the window size naturally first, then adjust and lock it
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
            // Get the natural content size
            let naturalSize = window.contentView?.fittingSize ?? NSSize(width: 450, height: 200)
            
            // Set a reasonable size that fits the content
            let targetSize = NSSize(width: max(naturalSize.width, 450), height: max(naturalSize.height, 200))
            window.setContentSize(targetSize)
            
            // Now configure window style to prevent resizing
            window.styleMask = [.titled, .closable, .miniaturizable]
            
            // Lock the size
            window.minSize = targetSize
            window.maxSize = targetSize
        }
        
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
    
    // MARK: - NSWindowDelegate
    
    func windowShouldClose(_ sender: NSWindow) -> Bool {
        // Terminate the application when the close button is clicked
        NSApplication.shared.terminate(nil)
        return true
    }
    
    func windowWillResize(_ sender: NSWindow, to frameSize: NSSize) -> NSSize {
        // Prevent any resizing by returning the current size
        return sender.frame.size
    }
}
