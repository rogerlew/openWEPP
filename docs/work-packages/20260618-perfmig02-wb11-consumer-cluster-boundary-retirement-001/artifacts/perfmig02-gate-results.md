# PERFMIG02 Gate Results

Static: checked package acceptance criteria and local instructions.

Ran: focused Rust gates, release build, H2637 endpoint, and transition-boundary bench.

## Gate Table

| Gate | Result | Evidence |
|---|---|---|
| Reader set mapped | PASS | `perfmig02-reader-map.md` |
| Readers migrated to dense `SymbolId` reads | PASS | Dense-first scalar helper changes in `state_access.rs`; scheduler proof test |
| Internal logical materialization dropped/moved | PASS for six internal symbols | Scheduler skip-id policy plus stale-removal apply policy |
| No dual-read on normal retired-symbol path | PASS | `perfmig02_scheduler_keeps_retired_symbols_indexed_only_between_phases` |
| PERFMIG01 focused fixture identity | PASS | Focused tests below |
| H2637 HBP/WAT/PASS identity | PASS with PASS Arrow-equality | `perfmig02-bit-identity.md` |
| H2637 endpoint improvement vs 669.97 s | FAIL/REDIRECT | Final binary: 672.14 s and 675.00 s, +2.17 to +5.03 s |
| `apply_indexed` retired-boundary cost drops | FAIL | skip-6 apply: 105.460510 us vs materialize-all 104.752336 us |
| Determinism/conservation regressions | PASS on exercised H2637 manifest closure fields | manifest closure fields unchanged; output identity evidence |
| Full workspace closure gates | PASS | commands below |

## Focused Commands Already Run

```text
cargo fmt --check
cargo test -p openwepp-kernel-contract indexed_apply_can_skip_selected_logical_materialization -- --nocapture
cargo test -p openwepp-hillslope-orchestrator perfmig02_scheduler_keeps_retired_symbols_indexed_only_between_phases -- --nocapture
cargo check --workspace
cargo test -p openwepp-hillslope-orchestrator perfmig01_wb11_warm_rain_indexed_writeback_is_bit_identical -- --nocapture
cargo test -p openwepp-hillslope-orchestrator perfmig01_scheduler_applies_indexed_writeback_payload -- --nocapture
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
markdown-doc lint --path docs/work-packages/20260618-perfmig02-wb11-consumer-cluster-boundary-retirement-001 --path docs/ROADMAP.md --path docs/work-packages/README.md
```

Result: all passed.

## Timing Commands Already Run

```text
/usr/bin/time -f "h2637_same\t%e\t%M" target/release/openwepp-cli-hill ...
/usr/bin/time -f "h2637_same_repeat\t%e\t%M" target/release/openwepp-cli-hill ...
cargo run --release --manifest-path .../perfmig02-transition-boundary-bench/Cargo.toml -- 50000
```

Result: H2637 endpoint was negative on both final-binary runs, and the apply-boundary microbench did not drop.

## Gate Non-Deferral Disposition

Because the package explicitly defines flat/negative endpoint after boundary retirement as REDIRECT, the
package closes as `executed-redirect`, not `CONTINUE`. Identity and code gates pass; the strategy gate fails.
