# R6H Anti-Alias Fixtures

Status: complete-with-hold.

| Fixture/test | Alias risk | Required distinction | Result |
|---|---|---|---|
| Interleaved PMET carry test | Day `n+1` uses stale precomputed PMET operands | Expected value differs when prior-day committed direct state is changed. | Pass: `r6h_publication_capture_builds_lane_day_inputs_after_direct_commit`. |
| WAT parity regression | R6G residual hidden by writer self-consistency | WAT parity must be compared after direct artifact production. | Pass: `r6h_cutover_candidate_hbp_identity_reduces_wat_to_pmet_layer_ulp_gap`. |
| Multi-OFE/lane fixture | Day-global input aliases lane-specific state | Two lanes/OFE values diverge and direct publication preserves the distinction. | Partial pass: orchestrator test proves lane-dimensional builder; full multi-OFE WAT parquet fixture remains follow-on. |
| WAT id authority fixture | Fixture-only `wepp_id = 1` masks wrong id semantics | Rejected identity candidates differ. | Held: not closed beyond current fixture. |
| CLI fail-closed contract | Partial outputs written while R6H gate fails | Cutover exits nonzero and writes no partial outputs unless all gates pass. | Pass: CLI contract reports `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`. |

## Rejected Aliases

- The direct publication cutover cannot pass with `HOLD-R6G-*`; tests now
  assert that marker is absent.
- The R6H marker is reserved for `Es`-only ulp drift and rejects larger `Es`
  differences, first-row mismatches, and mixed later-row residuals.
- The CLI contract verifies the output directory remains free of HBP, WAT,
  PASS, loss, and manifest files when the R6H gate fails.
