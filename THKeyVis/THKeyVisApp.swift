//
//  THKeyVisApp.swift
//  THKeyVis
//
//  Created by Umaĵo on 2025/9/13.
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
        .windowStyle(.hiddenTitleBar)
        .windowResizability(.contentSize)
    }
}
