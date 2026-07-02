# Gate Results

Status: `EXECUTED`

| Gate | Result | Evidence |
| --- | --- | --- |
| Handoff prompt authored | `PASS` | `prompts/active/kickoff.md` |
| Package scaffold authored | `PASS` | `package.md` |
| Scaffold docs lint | `PASS` | Prior scaffold evidence: `23 files validated, 0 errors, 0 warnings` |
| Scaffold diff whitespace | `PASS` | Prior scaffold evidence: `git diff --check` |
| Correction authority envelope recorded | `PASS` | `package.md`, `artifacts/correction-authority-envelope.md` |
| Seven-gate bar recorded | `PASS` | `artifacts/seven-gate-bar.md` |
| Current defect reproduced | `PASS` | Prior static evidence showed public CLI used compatibility projection. |
| Attempted shortcut reviewed | `PASS` | Blocking review findings accepted; shortcut removed. |
| Direct physics implementation | `PASS` | `kernel/direct.rs` calls WS11, WS12, WS18, and shared WS20 routing helpers over typed frame state. |
| Frame-native typed dispatch implemented | `PASS` | `execute_watershed_dispatch_with_frame`. |
| Production old-surface routing removed | `PASS` | Public CLI and direct kernel source scans found no old surface/request/writeback markers. |
| Typed publication fail-closed operands implemented | `PASS` | `publish_typed_routing_report` errors on missing routed channel/impoundment state. |
| Public CLI behavior | `PASS` | `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture`: 24 passed. |
| WS10/WS11/WS12 physics contracts | `PASS` | Focused WS10, WS11, and WS12 integration contracts passed. |
| Committed carnivorous parser fixture | `PASS` | `carnivorous_adobo_committed_fixture_is_repo_local_32_hillslope_gate`: passed. |
| Committed carnivorous output identity | `NOT-APPLICABLE` | Fixture README states it is not a current CLI E2E fixture with TOML/HBP bindings. No output identity claim recorded. |
| Protected public output evidence | `PASS` | Behavior contract decodes required Parquet outputs and proves jobs=1/jobs=N row identity. |
| Conservation/magnitude audit | `PASS-LIMITED` | Public tests prove non-zero generated pass payload consumption and stable decoded output rows; no carnivorous output audit claimed. |
| Source guards | `PASS` | W4 guard checks public CLI, typed dispatch body, direct kernel old-marker absence, and direct physics call markers. |
| `cargo fmt --check` | `PASS` | Ran successfully. |
| `cargo clippy --workspace --all-targets -- -D warnings` | `PASS` | Ran successfully. |
| `cargo nextest run --workspace --profile full` | `PASS` | 1284 tests passed, 1 skipped. |
| `cargo deny check` | `PASS` | advisories, bans, licenses, and sources all ok. |
| `git diff --check` | `PASS` | Ran successfully. |
| Final disposition recorded | `PASS` | `artifacts/disposition.md`. |
