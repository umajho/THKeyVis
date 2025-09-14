- prefer `just`.
  - in the root justfile, we have `clean` which is `git clean -fdX`.
  - in `macOS/justfile`, we have `{build,run}-{debug,release}`, note that you
    should `just build-*` before `just run-*`.