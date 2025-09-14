//
//  THKeyVisApp.swift
//  THKeyVis
//

import SwiftUI

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
