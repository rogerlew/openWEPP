# Gate Results

Evidence mode: `Ran:`.

| Gate | Result |
|---|---|
| `cargo fmt --check` | pass |
| `cargo test -p openwepp-hillslope-orchestrator fq1_ --lib` | pass, `3 passed` |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | pass |
| FQ1 p1-p43 CLI validation | soil blocker cleared; `42/43` WAT/HBP; p11 downstream percolation guard |
| p11 deterministic rerun | rc `1`, `HKERNEL-WB11-PERC-E-003` at `1990-162` |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `cargo test --workspace` | pass |
| `cargo deny check` | pass with existing warnings; final line `advisories ok, bans ok, licenses ok, sources ok` |

Anti-evasion gates were not run because this package did not edit external
authority suite posture, cohort fixtures, or required-case bindings.
