//
//  ContentView.swift
//  THKeyVis
//
//  Created by Umaĵo on 2025/9/13.
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
