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
            .background(.clear)
            .onAppear {
                // Ensure window stays on top
                windowManager.bringToFront()
            }
    }
}

#Preview {
    ContentView()
        .environmentObject(KeyMonitor())
        .environmentObject(WindowManager())
}
