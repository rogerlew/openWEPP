# R6G Blocker Ledger

Status: executed-held.

| Iteration | Marker/blocker | Output family | Reduced unit | In envelope? | Action | Result |
|---|---|---|---|---|---|---|
| 1 | `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP` | WAT | `wepp_id`, `year`, `Es`, `Total-Soil`, `SoilWaterTotal`, `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore` | Yes | Bind direct WAT identity/profile/process producers from parsed static inputs, climate request, private scheduler seed surface, direct runtime state, and residual-inclusive layer projection. | Closed for the inherited current fixture's identity/profile/first-day ET-storage fields; HBP identity green. Full canonical WAT id and lane-dimensional authority remain follow-up. |
| 2 | Direct projection storage under-count | WAT | First-day `Total-Soil`, `SoilWaterTotal` | Yes | Correct direct projection aggregate storage to include unfrozen residual liquid water from layer residual theta. | Closed; first WAT row now matches compatibility exactly for ET/storage. |
| 3 | `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` | WAT | Day-2 `Es`, `Total-Soil`, `SoilWaterTotal` | Boundary | Stop without aliasing WB13/runtime values; require dynamic/interleaved PMET day input builder that constructs ET component operands after the prior direct day commits carried layer state. | Held. |

## Current-Fixture R6F Field Reductions

- `wepp_id`: direct WAT publication now uses the current fixture's WAT output
  id constant instead of direct hillslope identity. Review disposition records
  full canonical multi-OFE WAT id authority as follow-up.
- `year`: direct WAT publication maps calendar year to simulation year using
  the first direct publication day.
- `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`:
  direct publication builds profile operands from parsed layer/profile symbols
  and authoritative layer derivation.
- First-day `Es`, `Total-Soil`, `SoilWaterTotal`: direct publication now
  reaches bit identity after residual liquid water is included in aggregate
  direct storage.

## Current Reduced Fields

The exact residual field set is:

1. `Es`
2. `Total-Soil`
3. `SoilWaterTotal`

The R6G marker is reserved for this exact set only. If any additional WAT field
is present, or if `Dp`/`P` joins the mismatch, the marker must not fire.

## Fixture Evidence

Current-fixture first WAT row after R6G:

| Field | Direct | Compatibility | Result |
|---|---:|---:|---|
| `Es` | `1.0115699107918512` | `1.0115699107918512` | Match |
| `Total-Soil` | `103.76254155138196` | `103.76254155138196` | Match |
| `SoilWaterTotal` | `103.76254155138196` | `103.76254155138196` | Match |
| `Dp` | `0` | `0` | Match |
| `latqcc` | `0` | `0` | Match |
| `Tile` | `0` | `0` | Match |

Current-fixture second WAT row after R6G:

| Field | Direct | Compatibility | Delta |
|---|---:|---:|---:|
| `Es` | `0.8341925321233935` | `0.7677601843722608` | `0.0664323477506327` |
| `Total-Soil` | `102.92834901925858` | `102.99478136700971` | `-0.06643234775113` |
| `SoilWaterTotal` | `102.92834901925858` | `102.99478136700971` | `-0.06643234775113` |

The storage delta tracks the ET component delta, so the remaining blocker is not
a WAT writer profile/identity alias. It is the missing direct day-input builder
that can construct PMET operands from direct-carried layer state after each
prior day commits.

## Accepted Follow-Up Boundaries

- Prove canonical WAT id semantics for non-trivial OFE/lane cases before full
  R6 cutover.
- Replace day-global publication inputs with lane-dimensional dynamic day
  inputs before claiming multi-lane authority.
- Add an allowlisted direct-symbol lineage ledger for the private seed surface
  and dynamic PMET operands before final no-compatibility closure.
