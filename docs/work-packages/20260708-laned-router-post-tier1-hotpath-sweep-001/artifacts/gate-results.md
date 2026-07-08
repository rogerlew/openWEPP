# Gate Results

Status: `COMPLETE`
Evidence mode: Ran.

| Gate | Result | Evidence |
|---|---|---|
| Contract-authority audit | PASS | `contract-disposition.md`: no `SC-OFEROUTE-001` amendment required for behavior-preserving loop/prework removal |
| Focused kinematic-wave tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing::kinematic_wave` -> `27` run, `27` passed, `303` skipped |
| Focused cascade tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing::cascade` -> `6` run, `6` passed, `324` skipped |
| `git diff --check` | PASS | No output |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260708-laned-router-post-tier1-hotpath-sweep-001 --format json` -> `14` files scanned, `0` errors, `0` warnings |
| `.rs` line-count disposition | PASS | `kinematic_wave.rs=2482` lines. Existing WARN band; below 3000-line hard split threshold. This package adds a small private summary and one focused unit test; a scope-safe split is not required before closure. |
| `cargo fmt --check` | PASS | No output |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Finished clean |
| `cargo nextest run --workspace --profile full` | PASS | `1437` tests run, `1437` passed, `3` skipped; wall `591.066s` |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok` |
| Release build | PASS | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` -> finished in `1m 03s` |
| Timing evidence | PASS | Median H2637 user time `11.72 s`; profiled `solver_cfl_ns=2277134095` |
| Authority anti-evasion guard | NOT RUN | No protected fixture, required-case binding, cohort, or external-authority suite posture edit |
