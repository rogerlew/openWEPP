# Gate Results

Status: `execution/reconstruction/review PASS / terminal workspace gates pending`.

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
| Terminal quick/frost/full workspace | `NOT RUN` | Required after post-result review |
| Dual terminal verification | `NOT RUN` | Required at exact clean closure candidate |

Retained output root size is `32,038,680,276` bytes. The complete manifest has
SHA-256 `a0e2a9ed1b08a41712980a8354b8471bf290faf1d9e7e164ab4858a43a05c4c6`.
Execution/result/checkpoint receipt hashes are recorded in the package outcome
and verified by both package tools. No TESTGATE command ran.
