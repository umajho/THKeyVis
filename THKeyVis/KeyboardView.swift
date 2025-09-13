//
//  KeyboardView.swift
//  THKeyVis
//

import SwiftUI
import AppKit

struct KeyboardView: View {
    @ObservedObject var keyMonitor: KeyMonitor
    
    var body: some View {
        VStack(spacing: 15) {
            // Layout indicator
            HStack {
                Text("Layout: \(keyMonitor.currentLayoutName)")
                    .font(.system(size: 10, weight: .medium))
                    .foregroundColor(.secondary)
                Spacer()
            }
            
            // Permission warning banner
            if !keyMonitor.hasAccessibilityPermission {
                HStack {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.orange)
                    Text("Accessibility permission required")
                        .font(.system(size: 12, weight: .medium))
                        .foregroundColor(.orange)
                    Spacer()
                    Button("Open Settings") {
                        if let url = URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility") {
                            NSWorkspace.shared.open(url)
                        }
                    }
                    .font(.system(size: 11, weight: .medium))
                    .foregroundColor(.blue)
                }
                .padding(.horizontal, 12)
                .padding(.vertical, 8)
                .background(.orange.opacity(0.1))
                .cornerRadius(8)
                .overlay(
                    RoundedRectangle(cornerRadius: 8)
                        .stroke(.orange.opacity(0.3), lineWidth: 1)
                )
            }
            
            // Keyboard layout
            HStack(spacing: 40) {
            // Left side
            VStack(spacing: 10) {
                // First row: ESC A R S T
                HStack(spacing: 8) {
                    KeyView(
                        keyName: "ESC",
                        isPressed: keyMonitor.pressedKeys.contains("esc"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 0), // A position
                        isPressed: keyMonitor.pressedKeys.contains("a"),
                        isDisabled: true,
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: "a",
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 1), // S position (R in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("r"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: "s",
                        iconName: "arrow.clockwise"
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 2), // D position (S in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("s"),
                        isDisabled: true,
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: "d"
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 3), // F position (T in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("t"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: "f",
                        iconName: "burst.fill"
                    )
                }
                
                // Second row: BACKSPACE (aligned with A-T)
                HStack {
                    Spacer()
                        .frame(width: 58) // ESC width + spacing
                    
                    KeyView(
                        keyName: "BACKSPACE",
                        isPressed: keyMonitor.pressedKeys.contains("backspace"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        width: 200, // Width to align with A through T
                        height: 50,
                        iconName: "scope"
                    )
                }
            }
            
            // Right side
            VStack(spacing: 10) {
                // First row: N E I O
                HStack(spacing: 8) {
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 38), // J position (N in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("n"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: "j",
                        iconName: "arrow.left"
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 40), // K position (E in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("e"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: "k",
                        iconName: "arrow.up"
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 37), // L position (I in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("i"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: "l",
                        iconName: "arrow.down"
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 41), // ; position (O in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("o"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: ";",
                        iconName: "arrow.right"
                    )
                }
                
                // Second row: SPACE
                KeyView(
                    keyName: "SPACE",
                    isPressed: keyMonitor.pressedKeys.contains("space"),
                    isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                    width: 200, // Width to span across N-O
                    height: 50,
                    iconName: "tortoise.fill"
                )
            }
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
