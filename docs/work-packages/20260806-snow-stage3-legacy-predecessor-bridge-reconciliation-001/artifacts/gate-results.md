# Gate Results

Status: `technical closure PASS / dual terminal verification pending`.

Evidence mode: `Ran`.

| Gate | Result | Evidence |
| --- | --- | --- |
| Exact clean reviewed execution head | `PASS` | `cb31e6f4d06fd66a3ef5b3a7711a095b3f3d84f4` |
| Endpoint matrix execution | `PASS` | `999.08 s`; log `target/snow_stage3_legacy_predecessor_bridge_reconciliation-logs/01-runner-execute.log` |
| Independent endpoint reconstruction | `PASS` | `1795.79 s`; log `02-reconstruct.log` |
| Conditional checkpoint execution | `PASS / not triggered` | `13.30 s`; exact 14-checkpoint inventory retained, zero selected lanes |
| Independent checkpoint reconstruction | `PASS / not triggered` | `188.03 s`; explicit no-op result |
| Runner retained verifier | `PASS` | `212.47 s`; endpoint matrix custody verified |
| Consumer retained verifier | `PASS` | `319.59 s`; `110747` retained artifacts verified |
| Package model-free tests | `PASS` | `42/42` at the post-result candidate |
| Focused contract tests | `PASS` | `12/12` at the post-result candidate |
| Package and roadmap Markdown | `PASS` | `39` files, zero errors or warnings |
| Assurance validate/plan | `PASS / DRAFT` | Three selected reports; snow/frost generation `7d1a3ba1`; public report count zero |
| Assurance staged build/check | `PASS` | All three governed reports assembled and checked only in an owned temporary root |
| Governed review-draft drift | `PASS` | Canonical renderer synchronized three stale v130-bound files, then exact check passed |
| Assurance export/release guard | `PASS` | Zero public reports/documents; vendoring false; no transition performed |
| Post-result science/Rust/consumer review | `PASS/PASS/PASS` | Science and custody pass at `dd7c1a3cf`; consumer hold corrected and recheck passes at `039ee78e2` |
| First terminal quick attempt | `FAIL / finding accepted` | Exact clean `de3c14933`; 908 passed, 2 stale v129 version-pin failures, then fail-fast at 910 run; no science/runtime failure |
| First 37-binary focused reconciliation | `FAIL / finding accepted` | 34 passed, 1 stale registry-index v129/narrative assertion failed, 123 not run after fail-fast; registry-row repair prospectively scoped |
| Final 37-binary stale-guard reconciliation | `PASS` | Dedicated binary `6/6`; all 37 changed binaries `158/158` in `282.131 s` with no fail-fast |
| Terminal quick workspace | `PASS` | Exact clean `5b620524a`; `2235/2235`, 40 skipped, `2305.216 s` |
| Terminal frost workspace | `PASS` | Exact clean `5b620524a`; `360/360`, 1969 skipped, `534.602 s` |
| Terminal full workspace | `PASS` | Exact clean `5b620524a`; `2284/2284`, 33 skipped, `2293.316 s` |
| Formatting/Clippy/doctests/dependency policy | `PASS` | Workspace all-target warnings denied; doctests and `cargo deny check` pass |
| Exact-head focused/static closure | `PASS` | Contracts `12/12`; stale guards `158/158`; package `42/42`; Binding Exposure, SC units, Markdown `41` files, seven changed JSON documents, diff hygiene, staged assurance build/check, review-draft drift, and export guard pass |
| Dual terminal verification | `NOT RUN` | Required at exact clean closure candidate |

Retained output root size is `32,038,680,276` bytes. The complete manifest has
SHA-256 `a0e2a9ed1b08a41712980a8354b8471bf290faf1d9e7e164ab4858a43a05c4c6`.
Execution/result/checkpoint receipt hashes are recorded in the package outcome
and verified by both package tools. No TESTGATE command ran.

The first terminal quick attempt is retained as contrary evidence under
`artifacts/terminal-verification/`. It exposed a repository-wide historical
test-coupling defect: 37 integration tests asserted that the canonical
contract's latest version remained v129. The package prospectively expanded
its test-only write set before any correction. The admitted mechanical repair
keeps each test's invariant, obligation, behavior, and package markers while
binding canonical contract identity instead of a perpetually stale latest
revision number. The fresh complete quick/frost/full run passes.

The comparator batch accidentally queued `cargo test --workspace` after the
canonical Nextest profiles. It was deliberately interrupted after `222.146 s`
and is recorded as an aborted, non-applicable legacy-harness observation: repo
policy requires that harness only for libtest-specific behavior, which this
package does not change. It is not reported as a gate pass or failure.

The full-tree raw-unit observation reported 12 existing literals in two
production Rust files outside the 98-path package diff. No production Rust or
conversion literal changed, and the package requires the passing scoped SC
unit gate rather than remediation of unrelated baseline debt. This observation
is recorded, not reported as a package gate pass. Terminal logs are under
`target/local-ci-history/predecessor-bridge-5b620524a/`.
