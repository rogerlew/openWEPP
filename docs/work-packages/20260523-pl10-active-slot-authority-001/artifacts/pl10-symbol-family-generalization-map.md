# PL10 Symbol Family Generalization Map

Status: `complete`
Evidence mode: `Static`

Static:
- Dispatch symbol access now uses dynamic slot/crop families.

## Replacement Map

| prior coupling class | PL10 generalized form | helper |
|---|---|---|
| fixed schedule slot symbols | `pl_schedule_slot_{slot:04}_{root}` | `pl_schedule_slot_symbol` |
| fixed schedule crop symbols | `pl_schedule_slot_{slot:04}_crop_{crop:04}_{root}` | `pl_schedule_slot_crop_symbol` |
| fixed growth crop symbols | `pl_growth_slot_{slot:04}_crop_{crop:04}_{root}` | `pl_growth_slot_crop_symbol` |
| fixed decomposition crop symbols | `pl_decomp_slot_{slot:04}_crop_{crop:04}_{root}` | `pl_decomp_slot_crop_symbol` |

## Dispatch Precondition Usage

1. Decomposition dispatch resolves active slot/crop and then reads:
   - `imngmt`
   - annual path: `resmgt`
   - perennial path: `mgtopt`, `ncut`, `ncycle`
2. Growth dispatch resolves active slot/crop and then reads:
   - `imngmt`
   - annual path: `jdharv`, `jdplt`, `rw`, `resmgt`
   - perennial path: `jdharv`, `jdplt`, `rw`, `jdstop`, `mgtopt`

## Code Anchors

- `crates/openwepp-hillslope-orchestrator/src/lib.rs:684`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:1025`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:1133`
