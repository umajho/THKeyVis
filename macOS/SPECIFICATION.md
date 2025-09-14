# Specification (Retrospective)

This specification documents the expected behaviors of the legacy Swift codebase
for the purpose of porting to a Rust+Swift architecture. The remap-mode
functionality is excluded as per requirements.

- [x] features that has been implemented in the Rust+Swift architecture is
      checked, like this one.

## Legacy Components Overview

The legacy implementation consists of the following Swift files (note:
`main.swift` is NOT legacy):

- `ContentView.swift` - Main application view container
- `KeyboardView.swift` - Keyboard visualization interface
- `KeyMonitor.swift` - Key monitoring and layout detection
- `KeyView.swift` - Individual key display component
- `WindowManager.swift` - Window behavior management
- `CustomTitleBar.swift` - Custom window controls

## Expected Behaviors

### 1. Permission Management

All Ported.

### 2. Global Key Monitoring

- [x] Work while other applications have focus (e.g., games)
- [x] Show immediate visual feedback when keys are pressed/released

### 3. Keyboard Layout Adaptation

- [x] Automatically detect the current keyboard layout (e.g., QWERTY, Colemak,
      etc.)
- [x] Display the correct character labels for each physical key position
- [ ] React immediately when the user switches keyboard layouts
  - Should be implemented on the swift side. See `setupInputSourceMonitoring` in
    `KeyMonitor.swift`.
- [x] Show the layout name in the UI (e.g., "Layout: Colemak")
  - At the top left corner below the title bar.
- [x] Gracefully handle complex layouts (CJK) by showing fallback labels

### 4. Window Behavior

- [ ] Always stay on top of other windows
- [x] Maintain fixed size and prevent user resizing

### 5. Window Controls

- [ ] Allow dragging the entire window by clicking and dragging anywhere
- [ ] title: "THKeyVis (https://github.com/umajho/THKeyVis)"

### 6. Keyboard Visualization Layout

#### Layout

```
Left Side:  [ESC] [A] [R] [S] [T]
                   [ BACKSPACE ]
Right Side: [N] [E] [I] [O]
             [   SPACE   ]
```

NOTE:

- BACKSPACE should be aligned with `[A]` ~ `[T]`, not `[ESC]` ~ `[T]`
- It shows `A/R/S/T` and `N/E/I/O` because I assume I'm using colemak. If I'm
  using QWERTY, it will be `A/S/D/F` and `J/K/L/;`.

#### Visual behavior

- [x] Keys light up / return to normal immediately when pressed / released
- [x] Disabled keys (A, D positions in QWERTY) show in grey
- [x] All keys show in red when permissions are missing, and are disabled
- [x] No animation delays (immediate response for gaming)

### 7. Key Display Requirements

**Each key must show:**

- [x] Main key label (center, prominent) - matches current keyboard layout
- [x] Small layout hint (top-left corner, blue) - only if different from main
      label
- [x] Functional icon (bottom) - for gaming context (arrows, retry, bomb, etc.)
  - in QWERTY position:
    - `S` for Retry
    - `F` for Bomb
    - `J/K/L/;` for `left/up/down/right` Arrows
    - Backspace for Shot
    - Space for Slow-Movement Mode (Focus Mode)

### 8. Layout Responsiveness

- [x] Maintain consistent visual layout regardless of permission state
- [x] Preserve key spacing and alignment at all times

### 10. Performance Requirements

- Respond to keystrokes as fast as possible

## Critical User Experience Goals

2. **Immediate Feedback**: Zero perceived delay between keypress and visual
   response
