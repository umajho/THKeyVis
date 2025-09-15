# THKeyVis macOS

## Copilot Agent History

### 0

The project is created with Xcode.

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

#### Prompts

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

### 8

#### Prompts

```md
Now:

- add small gray `asdf/jkl;` at the top left `arst/neio`.
- add icons below … representing:
  - a: retry
  - t: bomb (for STG games)
  - n: left
  - e: up
  - i: down
  - o: right
  - backspace: shoot
  - space: slow (holding space enters the slow-movement mode)
```

```md
The icons and gray texts seem off, the key names are also not aligned anymore,
also although I'm not sure it is introduced in the changes this time, the
background is also not half-transparent anymore.

also, no need for icon labels.

<!-- screenshot -->
```

### ⑨

#### Prompt

```md
The position of the gray texts/icons are correct only if the height is minimum,
but when the app is opened, it is not minimum in height.

Also:

- please don't make the icons gray, just use the same color as the key names.
- please make the gray text a little larger, and should be capitalized. It seems
  that gray is not very accessible, consider changing it to another secondary
  color.
```

### ⑩

#### Prompts

```md
Is it possible to determine the key names dynamically by the layout the user is
using? (If it is possible, it should also be reactive to the user's layout
switching.)
```

```md
The key names didn't change? they still seem to be hardcoded.
```

```md
The key names become `aprg/kfuy` for english layouts (colemak/ABC Extended), and
become `?`s for CJK layouts.
```

```md
The key names are correct now. But for some reason, there is a small chance that
when I switch layouts, the app doesn't notice the layout change.
```

### 11

#### Prompts

```md
- Don't show the blue small text if it is the same with the key name.
- Make the window always be the minimum size and unresizable.
```

```md
The window is too small that it breaks the layout. Can you find another
approach, for example, open the window the old way and then shrink the window,
and then disable the resize functionality?

<!--screenshot-->
```

```md
I can still resize the window. And now the `Layout: xxx` texts are duplicated.
(please keep the old one.)

I also forgot to mention that you should account for the layout change by things
like the “Accessibility permission required” alert.
```

### 12

#### Prompt

```md
I found that when the permission is revoked, the app still thinks it has the
permission, please fix that.
```

### 13

#### Prompts

```md
Please set the title to `THKeyVis (https://github.com/umajho/THKeyVis)`.
```

```md
Weird, titlear is empty. I thought the title bar were always set to an empty
text. it seems that there is a bug that makes the title not showing the text.
```

### 14

#### Prompts

```md
I wonder: since you alread made the KeyboardView round and have a transparnent
margin to the window frame, can you just:

- remove the window frame.
- fake the title bar with a native-like red closing button and with the title
  text.
- make the whole body draggable (to move the window).
```

```md
- while left-pressing and moving the cursor, the window doesn't move.
- I want the close button and title at the top of the body, no need to create a
  separate bar.
```

```md
The close button and the title text is outside of the body. I want them to be
inside. Also, since you make the link clickable, maybe consider changing the
cursor style while the cursor is hovering on the link?

<!--screenshot-->
```

```md
The top padding is a little large, please make it smaller.

Also, only the `https://…` part should be a link, the `THKeyVis` part (and the
parens) should just be regular texts.

<!--screen-->
```

### 15

#### Prompts

##### Part 1

```md
Please implement “remap mode”:

- Add a toggle at the same line of “layout: XXX”, on the right side, to control
  whether this mode is enabled.
- When we are in the remap mode:
  - remap `S` (QWERTY position, also apply to others at below) to `R`
  - remap `F` to `X`
  - remap `J`/`K`/`L`/`;` to right/up/down/left arrows
  - remap backsapce to `Z`
  - while space is pressed, simulate pressing left shift
  - icons below key names other than those for arrow keys become:
    `<icon> = <mapped key name>`
```

```md
Nothing is remapped.

Also, I got left and right wrong: `J` should be for left, and `;` should be for
right. Sorry.
```

```md
For `JKL;`, even if the remap mode is on, the icon line should still just
contain the icon itself. Showing things like
`<left arrow icon> = <left arrow key>` is unnecessary.
```

```md
It seems the font size (line height?) of the icon line affects the layout, which
makes the layout between enabling/disabling the remap mode inconsistent. Please
fix that.
```

##### Part 2

```md
Now, the UI part is done, you should start implementing the remapping
functionality.
```

##### Part 3

```md
Hmm, it doesn't seem that space->lshift works.
```

```md
Please stop. I don't think simply mapping the shift key using keycode will ever
work. You know that the shift key is a modifier key right?
```

##### Part 4

```md
The remapping for the shift key works until I pressed any other key while
holding the space key.
```

```md
The implementation is broken: When I held the shift key, other keys are no
longer mapped. For example, while holding the space key, when I pressed `J` (in
QWERTY), instead of remapping it to be the left arrow, it bceame the uppercased
`N` (since I'm using colemak).
```

##### Part 5

```md
This solution works well in the VSCode Editor, is broken in the VSCode terminal,
and is half-broken (laggy) in the game I tested. Well, maybe simulating modifier
keys is not a good idea.

- add `(broken)` after the shift icon in the remap mode.
- add a square representing the left shift below the `esc` square:
  - it should only appear in remap mode.
  - it should be as large as the `esc` square. Since `LEFT SHIFT`, `LSHIFT`,
    `SHIFT` are all too long, use an icon instead for the key's name.
```

```md
- don't add `(broken)` around the “Remap Mode” label.
- pressing the shift key doesn't reflect on the UI (on the remap mode). This
  should be fixed.
- The shift icons in the shift square are duplicated.
```

```md
- You should keep the upper part of the shift icon, instead of the lower part.
- holding the left shift key still doesn't work.
```

```md
Holding shift works now.

But for the first issue: It is not about make the icon filled, what I mean is
that the key name should be the icon, and the icon line should be empty.
```
