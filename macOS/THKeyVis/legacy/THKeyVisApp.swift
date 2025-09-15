//
//  THKeyVisApp.swift
//  THKeyVis
//

import SwiftUI
import Foundation

// Comment out SwiftUI-based main to hand over control to Rust
/*
@main
struct THKeyVisApp: App {
    @StateObject private var keyMonitor = KeyMonitor()
    @StateObject private var windowManager = WindowManager()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(keyMonitor)
                .environmentObject(windowManager)
                .onAppear {
                    windowManager.setupWindow()
                }
        }
        .windowResizability(.contentSize)
    }
}
*/

// SwiftUI App struct preserved for reference (commented out above)
