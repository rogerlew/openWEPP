# Gate results

Ran: terminal observation-only Q cut.

| Requirement | Result | Evidence |
| --- | --- | --- |
| P custody and Q recovery | PASS | P final bundle verified; Q final capture/verify pass; failed overlapping-destination attempt retained separately. |
| Fixed additive accounting tests | PASS | `raw/gates/q-compact-tests.log`: 3/3 pass including lifecycle, stale/non-LIFO/live/overflow, clock reversal, errors, and off neutrality. |
| Release build | PASS | `raw/gates/terminal-build.log`; exact binary in source manifest. |
| Authentic semantic admission | PASS | final `raw/series-05`: all 16 processes valid; exact protected identity and work counts. |
| Balanced measurements | PASS | two warmups/posture, six off/on pairs, CPU0, no retry; six on profiles reconcile at 0 ns. |
| Offline reconstruction | PASS | `raw/analysis-final-cut3.json`; receipt/log hashes checked by portable analyzer. |
| Detailed recomputation capture | PASS | exact-source `raw/recompute-trace-cut3`, 400/400/200/72 lifecycle, no errors or dropped records. |
| Conditional >=5% new prototype gate | PASS (negative decision) | no complete effective-input key or distinct interchangeable repeat demonstrated; no implementation authorized. |
| Rustfmt | PASS | `raw/gates/terminal-rustfmt.log`, exit 0. |
| Warnings-denied scoped Clippy | **FAIL** | The frozen required gate is not green: identical P and Q commands both exit 101 with 1,017 diagnostics. Independent comparison of diagnostic heading, attributed file, and emitted source-line snippet found identical multisets. This supports “no observed new diagnostic,” but does not satisfy the required gate. |
| Line-count governance | PASS with inherited WARN | no file >=3000; four inherited files are 2000-2999 and receive WARN, no expansion architecture work. |
| Dual independent review | PASS for truthful HOLD publication | Both reviews closed documentation findings and support the characterization/no-new-prototype result; neither waives Clippy FAIL. |
| Dual terminal verification | PASS for truthful HOLD publication | Both verifiers independently reconstructed custody, identities, accounting, analysis, and recomputation evidence. Clippy remains FAIL. |

The frozen plan expected green scoped Clippy based on P's published handoff.
Re-executing the stated warnings-denied three-crate command against P itself
exposes inherited failures. No pre-implementation amendment removed the
requirement, so matched P/Q diagnostics cannot retroactively replace it. The
package reaches executed HOLD even though its bounded characterization and
negative prototype decision remain usable. No production Rust changed.

The mistakenly executed primary-worktree rustfmt/compact commands are retained as
`rustfmt.log` and `compact-tests.log`; they are not Q evidence. Exact Q evidence
uses the `q-*`/`terminal-*` logs named above.
