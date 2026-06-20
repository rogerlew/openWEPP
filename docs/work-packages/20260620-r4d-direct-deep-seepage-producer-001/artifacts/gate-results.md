# R4D Gate Results

Status: complete.
Evidence mode: Ran.

| Gate | Result | Evidence |
|---|---|---|
| Pre-implementation contract gate | PASS | `producer-selection.md`, `r4d-process-span-contract.md`, `operand-lineage.md`, and `pre-implementation-contract-gate.md` completed before Rust edits. |
| `cargo fmt --check` | PASS | Ran after implementation. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after implementation. |
| `cargo test --workspace` | PASS | Ran after implementation. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Focused R4D tests | PASS | `cargo test -p openwepp-hillslope-orchestrator r4d_ -- --nocapture`: 2 passed. |
| Focused R4B tests | PASS | `cargo test -p openwepp-hillslope-orchestrator r4b_ -- --nocapture`: 3 passed. |
| Aggregate R2A direct tests | PASS | `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`: 3 passed. |
| Runner direct-runtime counter tests | PASS | `cargo test -p openwepp-runner r2a_ -- --nocapture`: 2 passed. |
| No-compatibility proof | PASS | Forbidden-token scan found no direct-runtime compatibility storage/request/writeback/symbol tokens; scheduler no-diff; runtime counters clean. |
| Release build | PASS | `/usr/bin/time -f 'release_build\t%e\t%M' cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: `57.84 s`, `1109092 KB`. |
| Default-disabled H2637 median `<= 676.67 s` | PASS | `635.94/650.91/645.47 s`; median `645.47 s`. |
| Protected output identity | PASS | HBP/loss/PASS/plot/WAT hashes recorded; PASS DuckDB row equivalence: `12419` vs `12419`, zero differences, `17` columns. |
| Markdown lint | PASS | Scoped package/roadmap/catalog lint passed after execution docs. |
| `git diff --check` | PASS | Ran with untracked package files included through intent-to-add. |
