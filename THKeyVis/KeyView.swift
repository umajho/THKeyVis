//
//  KeyView.swift
//  THKeyVis
//

import SwiftUI

struct KeyView: View {
    let keyName: String
    let isPressed: Bool
    let isDisabled: Bool
    let isPermissionDisabled: Bool
    let width: CGFloat
    let height: CGFloat
    let qwertyLabel: String?
    let iconName: String?
    let iconDescription: String?
    
    init(keyName: String, isPressed: Bool, isDisabled: Bool = false, isPermissionDisabled: Bool = false, width: CGFloat = 50, height: CGFloat = 50, qwertyLabel: String? = nil, iconName: String? = nil, iconDescription: String? = nil) {
        self.keyName = keyName
        self.isPressed = isPressed
        self.isDisabled = isDisabled
        self.isPermissionDisabled = isPermissionDisabled
        self.width = width
        self.height = height
        self.qwertyLabel = qwertyLabel
        self.iconName = iconName
        self.iconDescription = iconDescription
    }
    
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8)
                .fill(backgroundColor)
                .stroke(borderColor, lineWidth: 2)
                .frame(width: width, height: height)
            
            VStack(spacing: 2) {
                // QWERTY label at top (if provided)
                if let qwertyLabel = qwertyLabel {
                    HStack {
                        Text(qwertyLabel.uppercased())
                            .font(.system(size: 10, weight: .medium))
                            .foregroundColor(secondaryTextColor)
                        Spacer()
                    }
                } else {
                    HStack {
                        Spacer()
                    }
                    .frame(height: 12)
                }
                
                // Main key name (centered)
                Text(keyName.uppercased())
                    .font(.system(size: 12, weight: .semibold, design: .monospaced))
                    .foregroundColor(textColor)
                
                // Icon at bottom (if provided, no labels)
                if let iconName = iconName {
                    Image(systemName: iconName)
                        .font(.system(size: 10))
                        .foregroundColor(textColor)
                } else {
                    Spacer()
                        .frame(height: 12)
                }
            }
            .padding(4)
        }
        .shadow(color: isPressed ? .blue.opacity(0.3) : .black.opacity(0.1), radius: isPressed ? 4 : 2)
    }
    
    private var backgroundColor: Color {
        if isPermissionDisabled {
            return .red.opacity(0.2)
        } else if isDisabled {
            return .gray.opacity(0.3)
        } else if isPressed {
            return .blue.opacity(0.7)
        } else {
            return .white.opacity(0.9)
        }
    }
    
    private var borderColor: Color {
        if isPermissionDisabled {
            return .red.opacity(0.6)
        } else if isDisabled {
            return .gray.opacity(0.5)
        } else if isPressed {
            return .blue
        } else {
            return .gray.opacity(0.7)
        }
    }
    
    private var textColor: Color {
        if isPermissionDisabled {
            return .red.opacity(0.7)
        } else if isDisabled {
            return .gray.opacity(0.6)
        } else if isPressed {
            return .white
        } else {
            return .black.opacity(0.8)
        }
    }
    
    private var secondaryTextColor: Color {
        if isPermissionDisabled {
            return .red.opacity(0.5)
        } else if isDisabled {
            return .gray.opacity(0.4)
        } else if isPressed {
            return .white.opacity(0.7)
        } else {
            return .blue.opacity(0.7)
        }
    }
}

#Preview {
    VStack(spacing: 10) {
        KeyView(keyName: "A", isPressed: false)
        KeyView(keyName: "S", isPressed: true)
        KeyView(keyName: "R", isPressed: false, isDisabled: true)
        KeyView(keyName: "T", isPressed: false, isPermissionDisabled: true)
        KeyView(keyName: "SPACE", isPressed: false, width: 120, height: 50)
    }
    .padding()
    .background(.black.opacity(0.1))
}
