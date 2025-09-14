- prefer `just`.
  - in the root justfile, we have `clean` which is `git clean -fdX`.
  - in `macOS/justfile`, we have `{build,run}-{debug,release}`, note that you
    should `just build-*` before `just run-*`.
  - don't forget `cd` to the right directory.
- `./core` is the main project.
- `./macOS` is the entry point for macOS. It also contains the legacy version of
  this project. 