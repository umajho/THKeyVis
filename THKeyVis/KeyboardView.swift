//
//  KeyboardView.swift
//  THKeyVis
//

import SwiftUI
import AppKit

struct KeyboardView: View {
    @ObservedObject var keyMonitor: KeyMonitor
    
    // Helper function to get remapped key name
    private func getRemappedKeyName(for qwertyLabel: String?) -> String? {
        guard keyMonitor.isRemapModeEnabled, let qwertyLabel = qwertyLabel else { return nil }
        
        switch qwertyLabel.lowercased() {
        case "s": return "R"
        case "f": return "X"
        case "j": return "←"
        case "k": return "↑"
        case "l": return "↓"
        case ";": return "→"
        case "backspace": return "Z"
        case "space": return "⇧ (broken)"
        default: return nil
        }
    }
    
    // Helper function to check if key should show remap
    private func shouldShowRemap(for qwertyLabel: String?) -> Bool {
        guard keyMonitor.isRemapModeEnabled, let qwertyLabel = qwertyLabel else { return false }
        return ["s", "f", "j", "k", "l", ";", "backspace", "space"].contains(qwertyLabel.lowercased())
    }
    
    var body: some View {
        VStack(spacing: 12) {
            // Header with close button and title
            HStack {
                // Close button (red dot like native macOS)
                CloseButton()
                
                Spacer()
                
                // Title with mixed text and clickable link
                HStack(spacing: 0) {
                    Text("THKeyVis (")
                        .font(.system(size: 11, weight: .medium))
                        .foregroundColor(.primary.opacity(0.7))
                    
                    Button(action: {
                        if let url = URL(string: "https://github.com/umajho/THKeyVis") {
                            NSWorkspace.shared.open(url)
                        }
                    }) {
                        Text("https://github.com/umajho/THKeyVis")
                            .font(.system(size: 11, weight: .medium))
                            .foregroundColor(.blue.opacity(0.8))
                            .underline()
                    }
                    .buttonStyle(PlainButtonStyle())
                    .onHover { isHovering in
                        if isHovering {
                            NSCursor.pointingHand.push()
                        } else {
                            NSCursor.pop()
                        }
                    }
                    
                    Text(")")
                        .font(.system(size: 11, weight: .medium))
                        .foregroundColor(.primary.opacity(0.7))
                }
                
                Spacer()
                
                // Invisible spacer on the right to balance the close button
                Color.clear
                    .frame(width: 16)
            }
            .frame(height: 18)
            
            // Layout indicator with remap mode toggle
            HStack {
                Text("Layout: \(keyMonitor.currentLayoutName)")
                    .font(.system(size: 10, weight: .medium))
                    .foregroundColor(.secondary)
                Spacer()
                HStack(spacing: 6) {
                    Text("Remap Mode")
                        .font(.system(size: 10, weight: .medium))
                        .foregroundColor(.secondary)
                    Toggle("", isOn: $keyMonitor.isRemapModeEnabled)
                        .toggleStyle(SwitchToggleStyle(tint: .blue))
                        .scaleEffect(0.7)
                }
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
                        iconName: "arrow.clockwise",
                        remappedKeyName: getRemappedKeyName(for: "s")
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
                        iconName: "burst.fill",
                        remappedKeyName: getRemappedKeyName(for: "f")
                    )
                }
                
                // Second row: LEFT SHIFT (only in remap mode) and BACKSPACE
                HStack(spacing: 8) {
                    if keyMonitor.isRemapModeEnabled {
                        KeyView(
                            keyName: "⇧",
                            isPressed: keyMonitor.pressedKeys.contains("leftshift"),
                            isPermissionDisabled: !keyMonitor.hasAccessibilityPermission
                        )
                    } else {
                        Spacer()
                            .frame(width: 50) // Same width as ESC key when not showing shift
                    }
                    
                    KeyView(
                        keyName: "BACKSPACE",
                        isPressed: keyMonitor.pressedKeys.contains("backspace"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        width: 200, // Width to align with A through T
                        height: 50,
                        qwertyLabel: "backspace",
                        iconName: "scope",
                        remappedKeyName: getRemappedKeyName(for: "backspace")
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
                        iconName: "arrow.left",
                        remappedKeyName: getRemappedKeyName(for: "j")
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 40), // K position (E in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("e"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: "k",
                        iconName: "arrow.up",
                        remappedKeyName: getRemappedKeyName(for: "k")
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 37), // L position (I in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("i"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: "l",
                        iconName: "arrow.down",
                        remappedKeyName: getRemappedKeyName(for: "l")
                    )
                    KeyView(
                        keyName: keyMonitor.getCharacterForUIKey(keyCode: 41), // ; position (O in Colemak)
                        isPressed: keyMonitor.pressedKeys.contains("o"),
                        isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                        qwertyLabel: ";",
                        iconName: "arrow.right",
                        remappedKeyName: getRemappedKeyName(for: ";")
                    )
                }
                
                // Second row: SPACE
                KeyView(
                    keyName: "SPACE",
                    isPressed: keyMonitor.pressedKeys.contains("space"),
                    isPermissionDisabled: !keyMonitor.hasAccessibilityPermission,
                    width: 200, // Width to span across N-O
                    height: 50,
                    qwertyLabel: "space",
                    iconName: "tortoise.fill",
                    remappedKeyName: getRemappedKeyName(for: "space")
                )
            }
            }
        }
        .padding(.horizontal, 20)
        .padding(.top, 12)
        .padding(.bottom, 20)
        .background(
            RoundedRectangle(cornerRadius: 16)
                .fill(.black.opacity(0.6))
                .stroke(.gray.opacity(0.3), lineWidth: 1)
        )
        .shadow(color: .black.opacity(0.3), radius: 10)
    }
}

#Preview {
    KeyboardView(keyMonitor: KeyMonitor())
        .preferredColorScheme(.dark)
}
