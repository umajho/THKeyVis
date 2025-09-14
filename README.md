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

##### Prompts

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

#### Episode 2

##### Prompts

###### Part 1

```md
You should be able to find the logic for handling Accesibility permission in the
old swift code.

This time, spawn a thread in swift to monitor whether the permission is granted,
and use shared memory to let the rust side know the state. In rust, at every
frame, it checks the value in the shared memory, and if the permission is not
granted, it warns like what is done in the old code.
```

###### Part 2

```md
You should check `macOS/THKeyVis/KeyboardView.swift` line 99 for the ui
implementation.
```

```md
Please make the implementation exactly like the original one
(`KeyboardView.swift:99`). For example, you should use a button the guide the
user to the settings page. Secondly, don't add other texts that is not in the
original implementation.
```

###### Part 3

```md
Since opening the settings is just opening a URL, can't we do it on the rust
side directly?
```

#### Episode 3

##### Prompts

###### Part 1

```md
On the rust side:

Add a square with text `A` to represent the `A` key.

Use `rdev` to detect whether that key is pressed, and visualize it with the
square.

Remember that:

> **The process running the blocking listen function (loop) needs to be the
> parent process (no fork before).** The process needs to be granted access to
> the Accessibility API (ie. if you’re running your process inside Terminal.app,
> then Terminal.app needs to be added in System Preferences \> Security &
> Privacy \> Privacy \> Accessibility) If the process is not granted access to
> the Accessibility API, MacOS will silently ignore rdev’s listen calleback and
> will not trigger it with events. No error will be generated.
```

````md
Bad news:

When the app has the Accessibility permission, and when I pressed `A`, nothing
happened.

When I preseed a modifier key, no matther whether the app has the Accessibility
perrmision, it always crashed:

```
Crashed Thread: 6

Exception Type: EXC_BREAKPOINT (SIGTRAP) Exception Codes: 0x0000000000000001,
0x0000000186ca2588

Termination Reason: Namespace SIGNAL, Code 5 Trace/BPT trap: 5 Terminating
Process: exc handler [75905]

Thread 6 Crashed:
0   libdispatch.dylib             	       0x186ca2588 _dispatch_assert_queue_fail + 120
1   libdispatch.dylib             	       0x186cd478c dispatch_assert_queue$V2.cold.1 + 116
2   libdispatch.dylib             	       0x186ca250c dispatch_assert_queue + 108
3   HIToolbox                     	       0x1929ef90c islGetInputSourceListWithAdditions + 160
4   HIToolbox                     	       0x192b6857c isValidateInputSourceRef + 88
5   HIToolbox                     	       0x192b685f8 TSMGetInputSourceProperty + 36
6   THKeyVis.debug.dylib          	       0x104d46338 rdev::macos::keyboard::Keyboard::string_from_code::hbd31cc6a5ee574a9 + 108 (keyboard.rs:95)
7   THKeyVis.debug.dylib          	       0x104d462c0 rdev::macos::keyboard::Keyboard::create_string_for_key::h715bad6aab8aecb4 + 84 (keyboard.rs:86)
8   THKeyVis.debug.dylib          	       0x104d439a4 rdev::macos::common::convert::h7cfa5cdcc5229c67 + 1252 (common.rs:141)
9   THKeyVis.debug.dylib          	       0x104d41a20 rdev::macos::listen::raw_callback::ha0cca8ea018f4334 + 248 (listen.rs:24)
10  SkyLight                      	       0x18d02bbb4 processEventTapData(void*, unsigned int, unsigned int, unsigned int, unsigned char*, unsigned int) + 560
11  SkyLight                      	       0x18d2c27d4 _XPostEventTapData + 344
12  SkyLight                      	       0x18d02c448 eventTapMessageHandler(__CFMachPort*, void*, long, void*) + 168
13  CoreFoundation                	       0x186f6e284 __CFMachPortPerform + 240
14  CoreFoundation                	       0x186f42250 __CFRUNLOOP_IS_CALLING_OUT_TO_A_SOURCE1_PERFORM_FUNCTION__ + 60
15  CoreFoundation                	       0x186f42178 __CFRunLoopDoSource1 + 508
16  CoreFoundation                	       0x186f40b78 __CFRunLoopRun + 2200
17  CoreFoundation                	       0x186f3fc58 CFRunLoopRunSpecific + 572
18  CoreFoundation                	       0x186fb9714 CFRunLoopRun + 64
19  THKeyVis.debug.dylib          	       0x104d41788 rdev::macos::listen::listen::h0b18a6fbd6242fe3 + 428 (listen.rs:62)
20  THKeyVis.debug.dylib          	       0x104d3f5bc rdev::listen::h31140b46a33b5565 + 16 (lib.rs:272)
21  THKeyVis.debug.dylib          	       0x104d3c228 core::init::_$u7b$$u7b$closure$u7d$$u7d$::hff68e7ac839e1766 + 16 (lib.rs:27)
…
```
````

```md
Your new approach doesn't change anything, and using catch_unwind smells.

I'm thinking… Can we fork the process, and use the child process for the UI. The
two processes use mmap to communicate: A part of the mmap is an array, each
element represents a key code. The parent process running `dev::listen` sets the
elements, and the child UI process checks the elements of the array at each
frame.

The Accessibility permission state should also be on the mmap.
```

````md
```
Crashed Thread:        0  Dispatch queue: com.apple.main-thread

Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000

Termination Reason:    Namespace OBJC, Code 1 

Application Specific Information:
*** multi-threaded process forked ***
crashed on child side of fork pre-exec


Thread 0 Crashed::  Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib        	       0x186e218cc __abort_with_payload + 8
1   libsystem_kernel.dylib        	       0x186e482d4 abort_with_payload_wrapper_internal + 104
2   libsystem_kernel.dylib        	       0x186e4826c abort_with_reason + 32
3   libobjc.A.dylib               	       0x186a9d934 _objc_fatalv(unsigned long long, unsigned long long, char const*, char*) + 128
4   libobjc.A.dylib               	       0x186a9d8b4 _objc_fatal(char const*, ...) + 44
5   libobjc.A.dylib               	       0x186a81064 performForkChildInitialize(objc_class*, objc_class*) + 400
6   libobjc.A.dylib               	       0x186a667d8 initializeNonMetaClass + 628
7   libobjc.A.dylib               	       0x186a66600 initializeNonMetaClass + 156
8   libobjc.A.dylib               	       0x186a82a64 initializeAndMaybeRelock(objc_class*, objc_object*, locker_mixin<lockdebug::lock_mixin<objc_lock_base_t>>&, bool) + 176
9   libobjc.A.dylib               	       0x186a65f8c lookUpImpOrForward + 292
10  libobjc.A.dylib               	       0x186a65b84 _objc_msgSend_uncached + 68
11  THKeyVis.debug.dylib          	       0x10100767c _glfwInitCocoa + 100
12  THKeyVis.debug.dylib          	       0x100ff6f7c glfwInit + 224
13  THKeyVis.debug.dylib          	       0x100ee7cf8 InitPlatform + 72
14  THKeyVis.debug.dylib          	       0x100ee9c54 InitWindow + 396
15  THKeyVis.debug.dylib          	       0x100e23430 raylib::core::init_window::h7df9a6540e4a76e1 + 336
16  THKeyVis.debug.dylib          	       0x100e24dc4 raylib::core::RaylibBuilder::build::habc395e9a18f39c6 + 264
17  THKeyVis.debug.dylib          	       0x100d6a268 core::run_ui_process::hb635d31d7f72e270 + 140
18  THKeyVis.debug.dylib          	       0x100d6ada4 core::init::hab1295be5df1ed94 + 204
19  THKeyVis.debug.dylib          	       0x100d6ab74 rust_init + 12
20  THKeyVis.debug.dylib          	       0x100d65a24 __debug_main_executable_dylib_entry_point + 112 (main.swift:26)
21  dyld                          	       0x186ab6b98 start + 6076
```
````

###### Part 2

```md
I still need the swift side to monitor the Accessibility permission. Maybe you
should pass `startPermissionMonitoring` from the swift side to the rust side,
and invoke that function after the fork in the parent process and before
`rdev::listen`?
```

###### Part 3

```md
Now, the elephant in the room: when I pressed `A`, still no thing happens.

I found the root-cause: The permission should be granted is not Accessibility,
but Input Monitoring.

Here is a way to detect if it is granted:
https://stackoverflow.com/questions/79010369/how-to-check-the-status-of-the-input-monitoring-permission-in-my-swift-based-m

Also, you need to update the banner.
```

#### Episode 4

##### Prompts

```md
create a retrospective sepcification for the legacy code from the agent histroy
in this README (./macos/README.md). Ignore the remap-mode part. It will be used
for porting the legacy swift code to rust+swift. (note that only `main.swift` is
not legacy, other swift files are legacy.)
```

```md
please focus on expected behaviors, not implementation details.
```

#### Episode 5

##### Prompt

###### Part 1

```md
Read macOS/SPECIFICATION.md, implement the features related to keystroke
monitoring and visualization in the new rust+swift architecture. You can ignore
requirements about window related functionality for now.
```

```md
It can correctly the keys I'm pressing no matter the keyboard layout, but:

- the UI layout is off (see the screenshot).
- the window size is bad.
- esc should be at the left of `A`.
- space/backspace/esc should be as high as the normal keys.
- all the icons are shown as `?`.
- The QWERTY-hint small blue letters (at the top-left corner of the squares) are
  missing.
- When I switched the keyboard layout, it didn't change the key names
  accordingly. (It seems they are hardcoded for colemak for now.)
```
