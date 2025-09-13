//
//  ContentView.swift
//  THKeyVis
//

import SwiftUI

struct ContentView: View {
    @EnvironmentObject var keyMonitor: KeyMonitor
    @EnvironmentObject var windowManager: WindowManager
    
    var body: some View {
        KeyboardView(keyMonitor: keyMonitor)
            .padding()
            .background(.clear)
            .onAppear {
                // Setup window after content appears
                windowManager.setupWindow()
            }
    }
}

#Preview {
    ContentView()
        .environmentObject(KeyMonitor())
        .environmentObject(WindowManager())
}
