# Gate Results

Status: `executed-hold`

Evidence mode: `Ran:` local validation plus `Static:` hold disposition.

| Gate | Status | Evidence |
|------|--------|----------|
| `cargo fmt --check` | PASS | Ran after supervisor/test edit. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after retained production/test edit. Focused runner clippy also passed: `cargo clippy -p openwepp-runner --test watershed_cli_behavior_contract -- -D warnings`. |
| `cargo nextest run --workspace --profile full` | NOT RUN | W7 is held before complete closure; no accepted sediment-active fixture exists. |
| `cargo deny check` | NOT RUN | W7 is held before complete closure. |
| focused W7 path regression | PASS | `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw7_watershed_cli_generated_mode_accepts_relative_run_dir -- --nocapture`: `1 passed`. |
| release CLI build | PASS | `cargo build -p openwepp-runner --release --bins`: finished release build. |
| W7 sediment-active fixture gate | BLOCKED | No inspected fixture produced production-generated nonzero sediment; see `sediment-fixture-inventory.md`. |
| W7 output-identity gate | BLOCKED | No accepted sediment-active fixture exists; see `output-identity-evidence.md`. |
| W7 conservation-reconstruction gate | BLOCKED | No nonzero produced sediment signal exists; see `conservation-reconstruction.md`. |
| fixture checksum manifest validation | NOT RUN | No new W7 fixture was adopted. |
| scoped docs lint | PASS | `markdown-doc lint --path ...`: `54 files validated, 0 errors, 0 warnings`. |
| `git diff --check` | PASS | Clean. |

Complete closure is intentionally not claimed.
