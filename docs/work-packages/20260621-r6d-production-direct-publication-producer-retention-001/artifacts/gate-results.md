# Gate Results

Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---:|---|
| Required reading | PASS | Root/work-package/science-contract/crates guidance and R6C handoff read before implementation. |
| Retained direct producer surface | PASS | `DirectPublicationFrameCutover` now builds cutover-only retained rows in the climate-day loop from parsed climate/calendar and slope geometry. |
| Cutover consumes retained frame | PASS | Cutover branch consumes `execution.retained_direct_publication`; source scan shows no skeleton/capture calls in the cutover branch. |
| Default-disabled isolation | PASS | Compatibility mode returns no retained frame; full `cargo test --workspace` includes `r2a_default_fixture_run_constructs_no_direct_runtime_skeleton` PASS. |
| Missing parity-grade producer hold | PASS | Focused unit and CLI tests pass with `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`. |
| No-compatibility proof | PASS | Source scans recorded in `no-compatibility-proof-checklist.md` show no forbidden WB13/runtime/writeback/stale sources in retained producer or cutover branch. |
| HBP/WAT/PASS/loss/manifest parity | HOLD | Not claimed. R6D holds before public writes because parity-grade direct producers are absent. |
| Line-count governance | HOLD | `00_runner_intake_and_lane_setup.rs` is `3234` lines and must be split before complete R6 closure. |
| `cargo fmt --check` | PASS | Ran successfully after final edits. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran successfully after extracting climate execution completion helper. |
| `cargo test --workspace` | PASS | Full workspace tests passed, including runner R6D and CLI cutover contract. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `wctl doc-lint --path docs/work-packages` | PASS | `960 files validated, 0 errors, 0 warnings`. |
| `git diff --check` | PASS | No whitespace errors. |
