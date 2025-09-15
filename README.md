# THKeyVis

Touhou STG Keystroke Visualizer for my own setup. 99.99% vibe coded.

Without modifications, I don't think this works for anyone else, since my setup
is not very common:

- macOS. (It is not entirely impossible to port to other platforms though.)
- ergodox-like keyboard.

The core is in rust, and uses raylib for UI. A relatively smaller part of the
code is in swift, to support macOS.

There is also a legacy version, which is entirely in swift, and uses SwiftUI for
UI. It was deprecated due to not being responsive enough. (~1 frame slower than
the game. )

## Caveats

- For the same configuration (Debug/Release), when you built a new app, you need
  to revoke the Input Monitoring permission for the app first, then open the
  newly built one.
- If you want to switch to a different configuration (Debug/Release), you not
  only need to revoke the Input Monitoring permission of the app you built
  earlier, you also need to add the new app to the list manually, otherwise, for
  some reason, macOS will think that it granted the permission to the app at the
  old location, and won't recognize the permission for the new app it just
  granted.

## Screenshots

| New Version                |
| -------------------------- |
| ![](./screenshots/new.png) |

| Basic (Legacy Version)              | Remap Mode (Legacy Version)              |
| ----------------------------------- | ---------------------------------------- |
| ![](./screenshots/legacy-basic.png) | ![](./screenshots/legacy-remap-mode.png) |

## Copilot Agent History

- Season 1: [Here](./macOS/README.md#copilot-agent-history).
- Season 2: [Here](./COPILOT_AGENT_HISTORY.md#season-2)
