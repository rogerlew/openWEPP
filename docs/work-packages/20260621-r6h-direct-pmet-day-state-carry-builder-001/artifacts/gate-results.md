# R6H Gate Results

Status: queued.

| Gate | Command/evidence | Result | Notes |
|---|---|---|---|
| Formatting | `cargo fmt --check` | Queued |  |
| Focused check | `cargo check -p openwepp-runner -p openwepp-hillslope-orchestrator` | Queued |  |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Queued |  |
| Focused interleaved day-input tests | Queued | Queued |  |
| Focused WAT parity tests | Queued | Queued |  |
| CLI cutover fail-closed/public-write contract | Queued | Queued |  |
| Multi-OFE/lane anti-alias fixture | Queued | Queued |  |
| WAT id authority evidence | Queued | Queued |  |
| Static no-compatibility scan | Queued | Queued |  |
| Independent WAT reconstruction | Queued | Queued |  |
| Workspace tests | `cargo test --workspace` | Queued |  |
| Dependency policy | `cargo deny check` | Queued |  |
| Line count | `wc -l` over touched `.rs` files | Queued |  |
