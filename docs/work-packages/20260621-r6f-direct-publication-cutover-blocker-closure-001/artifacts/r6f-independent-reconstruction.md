# R6F Independent Reconstruction

Status: executed-held.

## Reconstruction Ledger

| Output family | Field/key | Reconstruction source | Writer source | Agreement evidence | Status |
|---|---|---|---|---|---|
| HBP | `peakro`, `watdur` | Direct `q_runoff_m` plus WB16 near-zero thresholds/floor. | Direct HBP candidate from `DirectPublicationDayRow`. | HBP payload and byte identity test passed. | Accepted for inherited near-zero fixture only; nonzero event reconstruction remains open. |
| WAT | `P`, `RM`, `Q`, `QOFE` | Direct climate/liquid/runoff publication operands. | Direct WAT row builder. | R6F WAT reduction test asserts agreement with compatibility for these fields. | Accepted for current fixture. |
| WAT | `Es`, storage/profile fields | Direct runtime typed-input fixture with independent synthetic layer state and PMET operands. | Direct publication row projection. | `r6f_publication_capture_accepts_typed_process_inputs_and_carries_layers` passed. | Runtime receiving surface accepted; production producer still missing. |
| PASS | Pending | Not reached after WAT hold. | Pending | Pending | Blocked behind WAT. |
| Loss | Pending | Not accepted as final cutover because WAT blocks. | Pending | Pending | Blocked behind WAT. |
| Manifest | Pending | Not reached after WAT hold. | Pending | Pending | Blocked behind WAT. |

## Commands

| Date | Command | Result | Notes |
|---|---|---|---|
| 2026-06-21 | `cargo test -p openwepp-hillslope-orchestrator r6f_publication_capture_accepts_typed_process_inputs_and_carries_layers -- --nocapture` | Passed | Independent one-layer direct runtime fixture reconstructs ET/storage/profile operands without writer compatibility rows. |
| 2026-06-21 | `cargo test -p openwepp-runner r6f_cutover_candidate_hbp_identity_exposes_wat_producer_gap -- --nocapture` | Passed | HBP bytes equal on current fixture; WAT accepted fields agree; remaining fields differ. |

## Boundary

R6F cannot independently reconstruct production WAT `Es`/storage/profile parity
because the production parsed-input producer is absent. The runtime can publish
those operands when supplied, but runner cutover currently supplies only
climate/calendar.
