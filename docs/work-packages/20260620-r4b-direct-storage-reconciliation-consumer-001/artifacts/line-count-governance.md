# R4B Line-Count Governance

Status: complete.
Evidence mode: Ran.

R4B is likely to touch `direct_runtime.rs`, which is already close to the 2000
line WARN threshold after R4A. Execution must record line counts for every
touched `.rs` file.

Policy:

- 2000+ lines: WARN, record explicit disposition and split/sunset plan.
- 3000+ lines: blocking for non-exempt files; split before closure.
- Prefer a narrow direct-runtime module split if R4B would push
  `direct_runtime.rs` past 2000 lines and the split can be kept mechanical.

Line counts after implementation:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2101 | WARN, below 3000 blocker. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 101 | OK. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 1089 | OK. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 644 | OK. |

Disposition:

R4B may close because no touched non-exempt Rust file crossed 3000 lines.
However, `direct_runtime.rs` is now in the 2000+ WARN band. The next direct
runtime package should prefer a narrow module split or extraction before adding
another large direct span, unless the split would obscure the next closure gate.
