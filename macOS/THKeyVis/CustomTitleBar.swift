//
//  CustomTitleBar.swift
//  THKeyVis
//

import SwiftUI
import AppKit

struct CloseButton: View {
    @State private var isHovered = false
    
    var body: some View {
        Button(action: {
            NSApplication.shared.terminate(nil)
        }) {
            Circle()
                .fill(isHovered ? Color.red : Color.red.opacity(0.7))
                .frame(width: 12, height: 12)
                .overlay(
                    // X symbol when hovered
                    Image(systemName: "xmark")
                        .font(.system(size: 6, weight: .bold))
                        .foregroundColor(.white.opacity(isHovered ? 1.0 : 0.0))
                )
        }
        .buttonStyle(PlainButtonStyle())
        .onHover { hovering in
            withAnimation(.easeInOut(duration: 0.15)) {
                isHovered = hovering
            }
        }
    }
}



// Window dragging functionality
struct DraggableWindowView<Content: View>: View {
    let content: Content
    @State private var initialMouseLocation: NSPoint = .zero
    @State private var initialWindowOrigin: NSPoint = .zero
    
    init(@ViewBuilder content: () -> Content) {
        self.content = content()
    }
    
    var body: some View {
        content
            .gesture(
                DragGesture(coordinateSpace: .global)
                    .onChanged { value in
                        if let window = NSApplication.shared.windows.first {
                            // Convert SwiftUI coordinates to screen coordinates
                            let mouseLocation = NSEvent.mouseLocation
                            
                            if initialMouseLocation == .zero {
                                initialMouseLocation = mouseLocation
                                initialWindowOrigin = window.frame.origin
                            }
                            
                            let deltaX = mouseLocation.x - initialMouseLocation.x
                            let deltaY = mouseLocation.y - initialMouseLocation.y
                            
                            let newOrigin = NSPoint(
                                x: initialWindowOrigin.x + deltaX,
                                y: initialWindowOrigin.y + deltaY
                            )
                            window.setFrameOrigin(newOrigin)
                        }
                    }
                    .onEnded { _ in
                        initialMouseLocation = .zero
                        initialWindowOrigin = .zero
                    }
            )
    }
}

#Preview {
    CloseButton()
}