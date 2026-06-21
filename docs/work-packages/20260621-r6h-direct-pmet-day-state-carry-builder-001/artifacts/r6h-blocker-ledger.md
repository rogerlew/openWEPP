# R6H Blocker Ledger

Status: queued.

| Iteration | Marker/blocker | Output family | Reduced unit | In envelope? | Action | Result |
|---|---|---|---|---|---|---|
| 0 | `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` | WAT | Day-2 `Es`, `Total-Soil`, `SoilWaterTotal` | Yes | Replace precomputed PMET day inputs with an interleaved direct day-input builder that reads committed direct-carried layer/state after the prior day. | Queued |

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
