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

### 2

#### Prompt

```md
- it should work after the `Accessibility` permission is granted immediaately
  without restart being involved.
- If the `Accessibility` persimmsion is not granted, the UI should warn about
  that, and the icons should all be disabled.
```

### 3

#### Prompt

```md
Window control buttons (`[x]` and `[-]`) don't work, please fix it.
```

### 4

#### Prompt

```md
Write a justfile for `build-debug`, `build-release`, `run-debug`, and
`run-release`.
```

```md
Please just write the 4 commands I mentioned for now, and don't use absolute
path, find a proper way to get the output app's location.
```

### 5

#### Prompt

```md
Make the close button actually close (kill) the application.
```

### 6

#### Prompt

```md
It seems that even if my layout is colemak, the app still reads qwerty. (For
example, it doesn't think I pressed `T` if the key I pressed is `T` in colemak
(`F` in qwerty), and it thinks that I pressed `T` if the key I pressed is `G` in
colemak (`T` in qwerty). )

Also, please use `just` from now on for building/running the app.
```

### 7

#### Prompt

```md
Please use text instead of icons to represent key names.

Also, Please disable the animation, to make feedbacks fast.

Note: you need to run `just build-*` before `just run-*`.
```
