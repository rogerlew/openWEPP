# R4E-H Producer Selection

Status: complete.

Evidence class: Static.

## Selected Producers

R4E-H selects handoff producers for the three remaining R4B
storage-reconciliation operands that had no direct upstream producer after R4D:

| Operand | Selected producer | Authority | R4E-H role |
|---|---|---|---|
| `subsurface_loss_m` / `Qd` | Direct subsurface-loss handoff | `SC-SUBHYD-001#INV-SUBHYD-009`, `INV-SUBHYD-021`, `INV-SUBHYD-023` | Validate finite nonnegative `Qd`, mutate direct state, write R4B input, shadow-project. |
| `evapotranspiration_m` / `ET` | Direct aggregate ET handoff | `SC-WATBAL-001` daily closure term, `SC-EVAP-001` ET withdrawal term | Validate finite nonnegative aggregate ET, mutate direct state, write R4B input, shadow-project. |
| `snow_coupling_m` / `S` | Direct signed snow/frost coupling handoff | `SC-WATBAL-001#INV-WATBAL-013` | Validate finite signed coupling, mutate direct state, write R4B input, shadow-project. |

## Rejected Producers

- Full WB19 `q`/`Qdd`/`Qd` compute: deferred to R4M/O.
- Full WB17 ET/root uptake compute: deferred to R4N.
- Public WB13/WAT publication fields: rejected because R4 remains shadow-only.
- Diagnostic ledger or storage residual reconstruction: rejected because those
  are non-authoritative aliases for R4B process operands.

## Boundary

The package may consume explicit typed handoff inputs and produce direct R4B
operands. It must not change scheduler activation, public output publication,
compatibility runtime, or default-disabled behavior.
