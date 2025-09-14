# THKeyVis

Touhou STG Keystroke Visualizer for my own setup. 99.99% vibe coded.

Without modifications, I don't think this works for anyone else, since my setup
is not very common:

- macOS. (I tried rust+raylib at first, but the LLM can't figure out how to
  handle macOS permissions, so I gave up and turned to swift+swiftUI, which
  means this project is worthless for you if you are on other OSs.)
- ergodox-like keyboard.

## Screenshots

| Basic                        | Remap Mode                        |
| ---------------------------- | --------------------------------- |
| ![](./screenshots/basic.png) | ![](./screenshots/remap-mode.png) |

## Copilot Agent Histroy

### Session 1

[Here](./macOS/README.md#copilot-agent-histroy).

### Session 2

#### Episode 1

##### Prompt

###### Part 1

```md
Hello. Can you make the swift project in `./macOS` to hand over the main thread
to the rust's `init` function in `./core/src/lib.rs`?

When the rust function returned, terminate the app.

Don't let the swift code do any other unnecessary things, like setting up
SwiftUI.

Don't clean up the old SwiftUI related code for now. You can comment out those
code that should not run in the main path though.

You can reference:
https://gist.github.com/Jomy10/a4873dd43942ed1bf54d387dbc888795

Don't forget to make xcodebuild to run the build scripts during building.
```

…

###### Part 2

```md
After I changed core/src/lib.rs, neither:

- start the active schema in Xcode, nor
- `just build-debug` then `just run-debug`

reflects the changes.
```

```md
You are wrong. Even if I removed the target folder, and saw that cargo rebuilt
everything, this still happens.
```

```md
please don't hard code paths.
```

```md
using find doesn't seem robust to me.
```

```md
please rename the `build-rust` command in justfile to `build-debug-rust`, and
also apply the same fix to `build-rust-release` (which should be renamed to
`build-release-rust`). also, please fix `build_rust.sh`, it currently only build
the release version.
```
