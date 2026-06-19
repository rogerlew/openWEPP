# PERFMIG01 Gate Results

Evidence: Ran.

Final gate status after PERFMIG01 closure commands.

| Gate | Command | Result |
| --- | --- | --- |
| Format | `cargo fmt --check` | passed |
| Workspace check | `cargo check --workspace` | passed |
| Contract crate check | `cargo check -p openwepp-kernel-contract` | passed |
| Orchestrator crate check | `cargo check -p openwepp-hillslope-orchestrator` | passed |
| Focused WB11 bit identity | `cargo test -p openwepp-hillslope-orchestrator perfmig01_wb11_warm_rain_indexed_writeback_is_bit_identical -- --nocapture` | passed |
| Focused scheduler indexed writeback | `cargo test -p openwepp-hillslope-orchestrator perfmig01_scheduler_applies_indexed_writeback_payload -- --nocapture` | passed |
| Focused orchestrator clippy | `cargo clippy -p openwepp-hillslope-orchestrator --tests -- -D warnings` | passed |
| Release endpoint build | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | passed |
| H2637 no-UI endpoint | `/usr/bin/time ... target/release/openwepp-cli-hill ...` | passed, `669.97s`, `228144 KB` |
| Boundary harness | `cargo run --release --manifest-path .../perfmig01-transition-boundary-bench/Cargo.toml -- 50000` | passed |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | passed |
| Workspace tests | `cargo test --workspace` | passed |
| Cargo deny | `cargo deny check` | passed |
| Markdown docs | `markdown-doc lint --path docs/decisions/0023-array-authoritative-hot-path-state.md --path docs/decisions/README.md --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260618-perfmig01-wb11-runoff-array-authoritative-production-migration-001` | passed, `12 files validated, 0 errors, 0 warnings` |
