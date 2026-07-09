# Gate Results

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

| Gate | Status | Evidence |
|---|---|---|
| Package scaffold exists | PASS | `package.md`, `artifacts/`, `prompts/active/`, and `prompts/archived/` exist. |
| Pre-implementation contract gate | PASS | `artifacts/pre-implementation-contract-gate.md` records no required contract amendment before code. |
| `cargo fmt --check` | PASS | Ran after implementation; exit 0. |
| `cargo check` touched crates | PASS | `cargo check -p openwepp-hillslope-orchestrator -p openwepp-runner -p openwepp-hillslope-output -p openwepp-sim-contract`; exit 0. |
| Focused groundwater recurrence tests | PASS | `cargo test -p openwepp-hillslope-orchestrator gwbaseflow -- --nocapture`; 3 passed. |
| Focused WAT projection test | PASS | `cargo test -p openwepp-runner r6a_direct_projection_consumers_read_publication_frame_operands -- --nocapture`; 1 passed. |
| Focused WAT writer/schema tests | PASS | `cargo test -p openwepp-hillslope-output hillslope_wat -- --nocapture`; 4 passed. |
| Unit-registry smoke | PASS | `cargo test -p openwepp-sim-contract units -- --nocapture`; 0 selected tests, crate built successfully. |
| Workspace clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings`; exit 0 after adding a scoped `too_many_lines` allow to an enlarged test fixture. |
| `cargo nextest` quick | PASS | Initial run failed the direct-frame bounded-size guard (`DirectDayFrame=15536` > old `15488`). After updating the documented bound for `DirectGroundwaterDayOutput`, reran `cargo nextest run --workspace --profile quick`; 1387 passed, 26 skipped. |
| `cargo nextest` full | PASS | `cargo nextest run --workspace --profile full`; 1462 passed, 3 skipped. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Export closure | HOLD | `gwbfv` has real WAT watershed consumer proof; generated `gwdsv` and `bftharea` channel-threshold consumer remain outside this package. |
