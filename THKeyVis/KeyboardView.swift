//
//  KeyboardView.swift
//  THKeyVis
//
//  Created by Umaĵo on 2025/9/13.
//

import SwiftUI

struct KeyboardView: View {
    @ObservedObject var keyMonitor: KeyMonitor
    
    var body: some View {
        HStack(spacing: 40) {
            // Left side
            VStack(spacing: 10) {
                // First row: ESC A R S T
                HStack(spacing: 8) {
                    KeyView(
                        keyName: "ESC",
                        isPressed: keyMonitor.pressedKeys.contains("esc")
                    )
                    KeyView(
                        keyName: "A",
                        isPressed: keyMonitor.pressedKeys.contains("a")
                    )
                    KeyView(
                        keyName: "R",
                        isPressed: keyMonitor.pressedKeys.contains("r"),
                        isDisabled: true
                    )
                    KeyView(
                        keyName: "S",
                        isPressed: keyMonitor.pressedKeys.contains("s"),
                        isDisabled: true
                    )
                    KeyView(
                        keyName: "T",
                        isPressed: keyMonitor.pressedKeys.contains("t")
                    )
                }
                
                // Second row: BACKSPACE (aligned with A-T)
                HStack {
                    Spacer()
                        .frame(width: 58) // ESC width + spacing
                    
                    KeyView(
                        keyName: "⌫",
                        isPressed: keyMonitor.pressedKeys.contains("backspace"),
                        width: 200, // Width to align with A through T
                        height: 50
                    )
                }
            }
            
            // Right side
            VStack(spacing: 10) {
                // First row: N E I O
                HStack(spacing: 8) {
                    KeyView(
                        keyName: "N",
                        isPressed: keyMonitor.pressedKeys.contains("n")
                    )
                    KeyView(
                        keyName: "E",
                        isPressed: keyMonitor.pressedKeys.contains("e")
                    )
                    KeyView(
                        keyName: "I",
                        isPressed: keyMonitor.pressedKeys.contains("i")
                    )
                    KeyView(
                        keyName: "O",
                        isPressed: keyMonitor.pressedKeys.contains("o")
                    )
                }
                
                // Second row: SPACE
                KeyView(
                    keyName: "SPACE",
                    isPressed: keyMonitor.pressedKeys.contains("space"),
                    width: 200, // Width to span across N-O
                    height: 50
                )
            }
        }
        .padding(20)
        .background(
            RoundedRectangle(cornerRadius: 16)
                .fill(.black.opacity(0.85))
                .stroke(.gray.opacity(0.3), lineWidth: 1)
        )
        .shadow(color: .black.opacity(0.3), radius: 10)
    }
}

#Preview {
    KeyboardView(keyMonitor: KeyMonitor())
        .preferredColorScheme(.dark)
}
