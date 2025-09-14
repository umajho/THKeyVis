# Specification (Retrospective)

This specification documents the expected behaviors of the legacy Swift codebase
for the purpose of porting to a Rust+Swift architecture. The remap-mode
functionality is excluded as per requirements.

### Legacy Components Overview

The legacy implementation consists of the following Swift files (note:
`main.swift` is NOT legacy):

- `ContentView.swift` - Main application view container
- `KeyboardView.swift` - Keyboard visualization interface
- `KeyMonitor.swift` - Key monitoring and layout detection
- `KeyView.swift` - Individual key display component
- `WindowManager.swift` - Window behavior management
- `CustomTitleBar.swift` - Custom window controls

### Expected Behaviors

#### 1. Permission Management

**The application must:**

- Detect when macOS Accessibility permission is granted/revoked
- Show a warning banner when permissions are not available
- Automatically begin key monitoring when permissions are granted
- Stop monitoring and clear displayed key states when permissions are revoked
- Handle permission changes that occur while the app is running
- Handle permission changes that occur when the app regains focus

#### 2. Global Key Monitoring

**The application must:**

- Monitor keystrokes from any application (global scope)
- Work while other applications have focus (e.g., games)
- Detect key press and key release events separately
- Show immediate visual feedback when keys are pressed/released
- Continue monitoring until the application is closed or permissions revoked

#### 3. Keyboard Layout Adaptation

**The application must:**

- Automatically detect the current keyboard layout (e.g., QWERTY, Colemak, etc.)
- Display the correct character labels for each physical key position
- React immediately when the user switches keyboard layouts
- Show the layout name in the UI (e.g., "Layout: Colemak")
- Gracefully handle complex layouts (CJK) by showing fallback labels
- Never crash or become unresponsive due to layout changes

#### 4. Window Behavior

**The application must:**

- Always stay on top of other windows (including fullscreen games)
- Remain visible when other applications have focus
- Never hide automatically or minimize
- Maintain fixed size and prevent user resizing
- Position itself at the bottom-center of the main screen
- Survive application switches and desktop changes

#### 5. Window Controls

**The application must:**

- Provide a red close button that terminates the entire application
- Show an 'X' symbol when hovering over the close button
- Allow dragging the entire window by clicking and dragging anywhere
- Display a title with clickable link: "THKeyVis
  (https://github.com/umajho/THKeyVis)"
- Open the GitHub link in the default browser when clicked
- Show appropriate cursor changes when hovering over interactive elements

#### 6. Keyboard Visualization Layout

**The application must display:**

```
Header: [Close Button] [Title] [Layout Info]
Warning: [Accessibility Permission Required] (when applicable)
Left Side:  [ESC] [A] [R] [S] [T]
           [    BACKSPACE    ]
Right Side: [N] [E] [I] [O]
           [   SPACE   ]
```

**Visual behavior requirements:**

- Keys light up blue immediately when pressed
- Keys return to normal color immediately when released
- Disabled keys (R, S positions) show in grey
- All keys show in red when permissions are missing
- No animation delays (immediate response for gaming)

#### 7. Key Display Requirements

**Each key must show:**

- Main key label (center, prominent) - matches current keyboard layout
- Small layout hint (top corner, blue) - only if different from main label
- Functional icon (bottom) - for gaming context (arrows, retry, bomb, etc.)
- Appropriate colors based on state (normal/pressed/disabled/error)

#### 8. Layout Responsiveness

**The application must:**

- Maintain consistent visual layout regardless of permission state
- Keep the same window size whether warnings are shown or hidden
- Preserve key spacing and alignment at all times
- Show consistent behavior across different screen sizes and resolutions

#### 9. Error Handling

**The application must:**

- Never crash due to permission changes
- Gracefully handle keyboard layout detection failures
- Continue functioning if some keys cannot be monitored
- Recover automatically when system conditions improve
- Provide clear visual feedback for error states

#### 10. Performance Requirements

**The application must:**

- Respond to keystrokes within milliseconds (gaming requirement)
- Use minimal CPU resources when idle
- Not interfere with other applications' performance
- Handle rapid key sequences without lag or dropped events

### Critical User Experience Goals

1. **Gaming Compatibility**: Must work seamlessly while games have focus
2. **Immediate Feedback**: Zero perceived delay between keypress and visual
   response
3. **Layout Flexibility**: Support for any keyboard layout without
   reconfiguration
4. **Reliability**: Never crash or become unresponsive during normal use
5. **Simplicity**: No configuration required beyond system permissions

This specification defines the expected user experience that must be preserved
when porting to the new architecture.
