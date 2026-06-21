# R6H Blocker Ledger

Status: executed-held.

| Iteration | Marker/blocker | Output family | Reduced unit | In envelope? | Action | Result |
|---|---|---|---|---|---|---|
| 0 | `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` | WAT | Day-2 `Es`, `Total-Soil`, `SoilWaterTotal` | Yes | Replace precomputed PMET day inputs with an interleaved direct day-input builder that reads committed direct-carried layer/state after the prior day. | Cleared. Focused tests now prove the R6G marker does not fire. |
| 1 | `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY` | WAT | Day-2 `Es` only | Partially; direct builder fixed, exact ET layer-state bit parity remains | Preserve fail-closed cutover with a new exact marker and hand off PMET layer-state ulp parity as the next implementation boundary. | Held. Direct `Es=0.7677601843722605` mm, compatibility `Es=0.7677601843722608` mm; storage totals are bit-identical. |

## Starting Evidence From R6G

- Current-fixture HBP byte identity is green.
- First direct WAT row matches compatibility for `Es`, `Total-Soil`,
  `SoilWaterTotal`, `Dp`, `latqcc`, and `Tile`.
- R6G residual fields are exactly `Es`, `Total-Soil`, and `SoilWaterTotal` on
  day 2.
- The residual storage delta tracks the ET component delta.
- WAT id, lane-dimensional day inputs, and allowlisted symbol lineage are
  accepted R6G follow-up boundaries that must be handled before R6H can claim
  WAT cutover.

## R6H Reduction Evidence

- The runner no longer constructs a multi-day `DirectPublicationDayInput`
  vector before direct runtime commits. `DirectPublicationFrameCutover` now
  invokes an interleaved day/lane builder from the direct executor loop.
- Day `n+1` PMET seed surfaces overlay direct-carried lane layer state after
  day `n` commits.
- Current-fixture HBP identity remains green.
- Current-fixture WAT storage parity is restored: `Total-Soil` and
  `SoilWaterTotal` are bit-identical.
- The remaining reduced field is exactly `Es`. Diagnostic comparison during
  implementation showed PMET `wfevp` and `etkr` differ only at ulp scale:
  direct `pmet.wfevp_mm=11.93838347586016`, compatibility
  `pmet.wfevp_mm=11.938383475860162`; direct `pmet.es_m=0.0007677601843722604`,
  compatibility `pmet.es_m=0.0007677601843722608`.
