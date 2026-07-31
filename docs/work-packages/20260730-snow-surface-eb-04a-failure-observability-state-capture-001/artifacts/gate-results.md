# Terminal Gate Results

Evidence: `Ran`

| Gate | Result |
| --- | --- |
| Focused Stage 0/Stage 3/trace contracts | PASS — `19/19` |
| Typed replay regressions | PASS |
| Diagnostic replay and anti-alias self-check | PASS — `24/24` |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| Nextest quick | PASS — `2110/2110`, 36 skipped |
| Nextest frost | PASS — `324/324`, 1,876 skipped |
| Nextest full | PASS — `2159/2159`, 29 skipped |
| Figure/sidecar inventory and SVG parse | PASS — `2/2` |
| Visual figure inspection | PASS |
| `git diff --check` | PASS |
| Security and line-count governance | PASS with two pre-existing WARNs |

An initial quick attempt exposed an anti-wiring failure because replay was
implemented in the generic guard include. It moved into the authorized Stage 3
module; the focused test and complete terminal quick run then passed.
