# Calibration Readiness Matrix

Status: not ready / out of scope

- `science_implementation_status`: `IMPLEMENTED` for the bounded mechanics-only
  persistent evaluation operator.
- `calibration_evidence_status`: `NOT_APPLICABLE`; this package expressly
  excludes calibration and measured SWE remains diagnostic-only.
- `identifiability_status`: `NOT_APPLICABLE`; this package evaluates mechanics
  continuity, not parameter identification.

| Dimension | Status | Reason |
|---|---|---|
| mechanics continuity | ready | typed state, restart, chronology, and closure evidence |
| observation authority | not ready | measured SWE remains diagnostic-only |
| forcing applicability | not ready | wind/exposure and canopy linkage unresolved |
| terminal recipient | not ready | land-surface recipient is explicitly censored |
| parameter fitting | prohibited | this package introduces no calibration authority |

Mechanics completion does not change calibration readiness.

| Contract obligation | Disposition | Evidence / rationale |
|---|---|---|
| `INV-SNOWFREEZE-100` | PASS | typed opt-in state, lane/order identity, restart equivalence, and production isolation tests |
| `OBL-SNOWFREEZE-P-072` | PASS | default-off stateful runner path and schema-v7 mechanics trace |
| `OBL-SNOWFREEZE-C-014` | PASS | rejecting real JSONL consumer independently reconstructs daily/cumulative custody |
| `TOL-SNOWFREEZE-021` | PASS | scale-aware water and energy closure guards and negative poisoned-residual test |

The readiness obligations from `science-contract-spec.md` are orthogonal to
the mechanics implementation and are dispositioned explicitly:

| Readiness obligation | Disposition | Evidence path | Rationale |
|---|---|---|---|
| typed/enumerable parameter surface | NOT_APPLICABLE | `package.md` Excluded Scope | no calibration parameter surface is introduced |
| observation operator with units and scale | NOT_APPLICABLE | `package.md` Implementation Intent | measured SWE is diagnostic-only and carries no package result |
| deterministic candidate execution | NOT_APPLICABLE | `package.md` Excluded Scope | no calibration candidate execution is authorized |
| objective reconstruction | NOT_APPLICABLE | `package.md` Excluded Scope | no calibration objective is defined |
| sensitivity analysis | NOT_APPLICABLE | `package.md` Excluded Scope | no fitted parameter or candidate grid exists |
| identifiability/confounding analysis | NOT_APPLICABLE | `package.md` Excluded Scope | parameter identification is outside this mechanics experiment |
| boundary, saturation, and failure reporting | NOT_APPLICABLE | `package.md` Validation And Exit Criteria | calibration-boundary reporting is out of scope; mechanics failures remain typed and tested |
| equifinality/uncertainty retention | NOT_APPLICABLE | `package.md` Excluded Scope And Claim Limits | no calibrated ensemble or uncertainty claim is produced |
| synthetic recovery | NOT_APPLICABLE | `package.md` Purpose | restart equivalence proves state mechanics, not parameter recovery |
| additional-data inventory | NOT_APPLICABLE | `package.md` Excluded Scope And Claim Limits | physical evidence gaps are claim limits, not a calibration workstream |
