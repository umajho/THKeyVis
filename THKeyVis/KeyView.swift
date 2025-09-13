//
//  KeyView.swift
//  THKeyVis
//
//  Created by Umaĵo on 2025/9/13.
//

import SwiftUI

struct KeyView: View {
    let keyName: String
    let isPressed: Bool
    let isDisabled: Bool
    let width: CGFloat
    let height: CGFloat
    
    init(keyName: String, isPressed: Bool, isDisabled: Bool = false, width: CGFloat = 50, height: CGFloat = 50) {
        self.keyName = keyName
        self.isPressed = isPressed
        self.isDisabled = isDisabled
        self.width = width
        self.height = height
    }
    
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8)
                .fill(backgroundColor)
                .stroke(borderColor, lineWidth: 2)
                .frame(width: width, height: height)
            
            Text(keyName.uppercased())
                .font(.system(size: 12, weight: .semibold, design: .monospaced))
                .foregroundColor(textColor)
        }
        .shadow(color: isPressed ? .blue.opacity(0.3) : .black.opacity(0.1), radius: isPressed ? 4 : 2)
        .scaleEffect(isPressed ? 0.95 : 1.0)
        .animation(.easeInOut(duration: 0.1), value: isPressed)
    }
    
    private var backgroundColor: Color {
        if isDisabled {
            return .gray.opacity(0.3)
        } else if isPressed {
            return .blue.opacity(0.7)
        } else {
            return .white.opacity(0.9)
        }
    }
    
    private var borderColor: Color {
        if isDisabled {
            return .gray.opacity(0.5)
        } else if isPressed {
            return .blue
        } else {
            return .gray.opacity(0.7)
        }
    }
    
    private var textColor: Color {
        if isDisabled {
            return .gray.opacity(0.6)
        } else if isPressed {
            return .white
        } else {
            return .black.opacity(0.8)
        }
    }
}

#Preview {
    VStack(spacing: 10) {
        KeyView(keyName: "A", isPressed: false)
        KeyView(keyName: "S", isPressed: true)
        KeyView(keyName: "R", isPressed: false, isDisabled: true)
        KeyView(keyName: "SPACE", isPressed: false, width: 120, height: 50)
    }
    .padding()
    .background(.black.opacity(0.1))
}
