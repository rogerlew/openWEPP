# Verification - Codex

Status: **EXECUTED**.

Evidence mode: Static + Ran.

## Verification

Ran:

- `cargo nextest run -p openwepp-runner hillslope::laned_shadow --no-capture`
  - PASS, `7/7`.
- `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --no-capture`
  - PASS, `67/67`, `522.335 s`.
- `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture`
  - PASS after correction, `1/1`, `484.578 s`.
- Release timing:
  - default/off PASS, `2.49 s` user / `0:02.51` wall.
  - shadow-on PASS, `91.59 s` user / `1:31.67` wall.
  - shadow-profile PASS, `94.87 s` user / `1:34.99` wall.
- `cargo fmt --check`
  - PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS after fixing the new test's `float_cmp` warning.
- `git diff --check`
  - PASS.
- Markdown/doc lint
  - PASS.

Not run:

- `cargo nextest run --workspace --profile full`
  - Package closes as hold before activation; focused current-scope gates ran.
- `cargo deny check`
  - Started, then interrupted by the operator while asking for blocker
    explanation; not rerun after the package was adjudicated as a hold.
