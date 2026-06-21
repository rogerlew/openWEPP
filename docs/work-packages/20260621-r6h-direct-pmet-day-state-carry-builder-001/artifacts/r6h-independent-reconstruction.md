# R6H Independent Reconstruction

Status: executed-held.

Record independent WAT reconstruction that does not restate the direct producer
formula with the same operands.

| Field group | Reconstruction source | Writer source | Agreement evidence | Status |
|---|---|---|---|---|
| PMET `Es` | Direct PMET diagnostics from day-2 seed surface and WAT row comparison | Direct WAT row builder | Mismatch only: direct `0.7677601843722605` mm vs compatibility `0.7677601843722608` mm. | Held |
| Storage totals | Direct hydrology projection aggregate from direct ET layer state | Direct hydrology projection and WAT row builder | `Total-Soil` and `SoilWaterTotal` bit-identical in focused R6H fixture. | Complete |
| WAT id/calendar | Direct row identity and compatibility WAT comparator | Direct WAT row builder | Current fixture identity fields match; broader WAT id semantics remain held. | Partial |

## Anti-Tautology Requirement

The accepted reconstruction must compare independently produced operands or
outputs and must reject plausible wrong formulas, including day-global inputs,
stale prior-day state, WB13 aliases, and fixture-only identity constants.

## Reconstruction Notes

- `r6h_cutover_candidate_hbp_identity_reduces_wat_to_pmet_layer_ulp_gap`
  compares direct WAT rows built from `DirectRunPublicationFrame` against
  compatibility WAT rows built from WB13 after direct artifacts are already
  produced.
- `reduced_wat_mismatch_fields` returns exactly `["Es"]`.
- The implementation-time diagnostic reduced the `Es` mismatch to PMET
  `wfevp`/`etkr` values computed from carried direct surface-layer water, not
  stale day-global PMET input construction.
