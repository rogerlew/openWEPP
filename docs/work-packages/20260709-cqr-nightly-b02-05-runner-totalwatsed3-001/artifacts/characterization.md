# Characterization

Before decomposition, public real-binary tests were added for help and exact
argument errors, explicit relative/absolute PASS/WAT precedence over invalid
defaults, and explicit missing optional soil/element errors. The focused suite
passed `6/6` tests.

Ran before production extraction: a detached worktree at scaffold `98243c6d`
received the same test-only diff and ran:

```text
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t05-predecomp-target \
  cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract \
  --profile quick
```

Exit `0`: `6` passed, `0` skipped. The detached worktree was removed after the
run. These tests consume the public binary and assert real output/error
behavior, rather than only private helper values.
