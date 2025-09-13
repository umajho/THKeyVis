# THVisKey

## Copilot Agent Histroy

### 1

#### Prompt

```md
Please make a keystroke visualizer for me.

Requirements:

- runs on macOS.
- use swiftUI.
- It should be able to run without the window being focused, so it can work when
  the game window is focused. (properly handles the “Input Monitoring”
  permission.)
- It should always be the top window.

UI design:

- I use an ergodox-like keyboard, the keys should be splited to left and right.
- I use colemak, and replaced caps lock with esc.
- layout:
  - keys on the left, first row (ltr): `[esc]` `[a]` `[r]` `[s]` `[t]`. being
    squares. `[r]` `[s]` are greyed out since they are not actually used.
  - keys on the left, second row: [back space]. being a rectangular alining with
    `[a]` ~ `[t]`.
  - keys on the right, first row (ltr): `[n]` `[e]` `[i]` `[o]`.
  - keys on the right, second row: `[space]`.
- margins around keys.
```
