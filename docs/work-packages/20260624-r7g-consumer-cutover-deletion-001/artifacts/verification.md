# Verification

Status: COMPLETE.

Ran:
- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed after the final test-module split.
- `cargo deny check`: passed.
- `git diff --check`: passed.
- `markdown-doc lint --path docs/work-packages/20260624-r7g-consumer-cutover-deletion-001 --no-ignore`: passed, 12 files.
- `markdown-doc lint --path docs/work-packages/README.md`: passed, 1 file.
- Focused:
  - `cargo test -p openwepp-hillslope-orchestrator r7b_constructor_type_size_layout_is_bounded -- --nocapture`: passed.
  - `cargo test -p openwepp-hillslope-orchestrator r7g_ -- --nocapture`: passed.
  - `cargo test -p openwepp-runner r7g_direct_production -- --nocapture`: passed.
  - `cargo test -p openwepp-hillslope-orchestrator direct_runtime_r3c_r4b -- --nocapture`: passed.
- Production deletion scan: no matches for deleted bridge symbols outside
  intentional tests.
