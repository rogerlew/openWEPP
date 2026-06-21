# R6F Output Family Burndown

Status: executed-held.

| Family | Required gate | Current state | Evidence | Next action |
|---|---|---|---|---|
| HBP | Byte identity plus field/operand lineage. | Inherited near-zero fixture closed; full nonzero peak-runoff/event-duration fixture remains open. | `r6f-hbp-byte-diff.md`; `r6f_cutover_candidate_hbp_identity_exposes_wat_producer_gap`. | Keep current fixture green and add nonzero fixture before final R6 cutover. |
| WAT | Arrow row, schema, value, and metadata parity. | Held at `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`. | `r6f-blocker-ledger.md`; runner and CLI tests. | R6G must add parsed-input typed producer binding. |
| PASS | Arrow row, schema, value, and metadata parity with PASS fixture coverage. | Not reached after WAT hold. | Fail-closed cutover prevents public writes. | Resume after WAT is green. |
| Loss | JSON identity. | Compatibility comparison passed before WAT in current gate ordering; full direct loss authority not accepted. | No final cutover evidence because WAT blocks. | Resume after WAT/PASS. |
| Manifest | Direct provenance/checksum parity. | Not reached; existing gate still protects manifest cutover. | `r6f-manifest-cutover-evidence.md`. | Resume after WAT/PASS/loss. |
| Public writes | Direct cutover writes all required output files. | Blocked. | CLI test proves no public files are written under hold. | Enable only after all parity gates pass. |

## Iteration Notes

| Date | Family | Change | Result |
|---|---|---|---|
| 2026-06-21 | HBP | Added direct near-zero runoff `peakro`/`watdur` operands. | HBP byte identity green for the inherited near-zero fixture. |
| 2026-06-21 | WAT | Fixed climate precipitation unit projection (`day.prcp` is already mm in runner climate projection). | WAT `P`, `RM`, `Q`, `QOFE` now match on fixture. |
| 2026-06-21 | WAT | Added direct-runtime typed process input slots, layer carry, and profile-depth/porosity projection. | Runtime can receive and publish missing operands when supplied; production runner producer remains absent. |
