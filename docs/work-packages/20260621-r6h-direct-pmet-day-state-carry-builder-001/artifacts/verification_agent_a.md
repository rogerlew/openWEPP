# R6H Verification Agent A

Status: complete.

Source: Curie delegated QA verification.

Evidence class: Ran + Static.

| Check | Evidence reviewed | Result | Notes |
|---|---|---|---|
| Interleaved PMET builder | `run_publication_capture_with_interleaved_day_inputs`, `DirectPublicationDayInputBuilder`, focused orchestrator test | PASS | Builder constructs day/lane inputs after prior direct commit and focused test proves call order. |
| WAT parity | Focused R6H runner tests and CLI contract | HELD | R6G marker cleared; remaining residual is exactly `Es` and maps to `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`. |
| No compatibility authority | Static scan and `r6h-no-compatibility-proof.md` | PASS for producer path; HELD for final cutover | Direct WAT producer reads `DirectRunPublicationFrame`; compatibility rows remain only for fail-closed parity comparison before public writes. |
| Required gates | `cargo fmt --check`, check, clippy, focused tests, CLI test, `cargo test --workspace`, `cargo deny check`, line counts | PASS/HELD as recorded | Rust gates pass; package is held because WAT parity/WAT id/multi-OFE closure cannot truthfully pass yet. |

## Verdict

Verified executed-held disposition. Curie's queued-artifact and gate-table
findings were accepted and addressed in final artifacts.
