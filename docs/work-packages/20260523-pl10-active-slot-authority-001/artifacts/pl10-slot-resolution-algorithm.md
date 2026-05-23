# PL10 Slot Resolution Algorithm

Status: `complete`
Evidence mode: `Static`

Static:
- Algorithm implemented in orchestrator dispatch precondition path.

## Deterministic Algorithm

1. Read and validate integral dispatch controls:
   - `slot_count >= 1`
   - `rotation_years >= 1`
   - `rotation_repeats >= 1`
   - `1 <= year <= rotation_years * rotation_repeats`
   - `1 <= day <= 366`
2. Compute:
   - `rotation_index = ((year - 1) / rotation_years) + 1`
   - `year_in_rotation = ((year - 1) % rotation_years) + 1`
3. Scan `slot_index in 1..=slot_count` and retain slots matching:
   - `slot_ofe_index == 1`
   - `slot_rotation_index == rotation_index`
   - `slot_year_in_rotation == year_in_rotation`
4. Candidate cardinality rules:
   - `0` -> `HS-PLDISP-E-005`
   - `>1` -> `HS-PLDISP-E-006`
   - `1` -> active slot
5. Read `crop_slots` for active slot; require `crop_slots >= 1`:
   - violation -> `HS-PLDISP-E-007`
6. For each crop slot, validate schedule/growth control symbols and evaluate
   day-window activity.
7. Crop candidate cardinality rules:
   - `0` -> `HS-PLDISP-E-008`
   - `>1` -> `HS-PLDISP-E-009`
   - `1` -> active crop slot
8. Dispatch growth/decomposition preconditions against the resolved
   `slot_{i}/crop_{j}` symbol family.

## Typed Error Surface

| code | condition | boundary class |
|---|---|---|
| `HS-PLDISP-E-001` | missing required dispatch symbol | `MissingRequiredInput` |
| `HS-PLDISP-E-002` | non-finite required dispatch symbol | `NonFinite` |
| `HS-PLDISP-E-003` | non-integral required dispatch symbol | `DomainViolation` |
| `HS-PLDISP-E-004` | out-of-range required dispatch symbol | `DomainViolation` |
| `HS-PLDISP-E-005` | missing active slot for OFE/year | `DomainViolation` |
| `HS-PLDISP-E-006` | ambiguous active slot for OFE/year | `DomainViolation` |
| `HS-PLDISP-E-007` | invalid crop slot count | `DomainViolation` |
| `HS-PLDISP-E-008` | missing active crop for day | `DomainViolation` |
| `HS-PLDISP-E-009` | ambiguous active crop for day | `DomainViolation` |

## Code Anchors

- `crates/openwepp-hillslope-orchestrator/src/lib.rs:772`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:860`
