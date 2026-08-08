# Calibration Readiness Matrix

Status: complete

Evidence mode: Static

Canonical status:

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

| Obligation | Disposition | Evidence path | Rationale |
|---|---|---|---|
| typed/enumerable parameter surface | PASS | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#constants-and-parameters-with-provenance-anchors` | no empirical parameter admitted |
| observation operator with units and scale | NOT_APPLICABLE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#calibration-and-identifiability` | no calibration authorized |
| deterministic candidate execution | NOT_APPLICABLE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#calibration-and-identifiability` | no candidate estimation |
| objective reconstruction | NOT_APPLICABLE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#calibration-and-identifiability` | no objective |
| sensitivity analysis | NOT_APPLICABLE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#calibration-and-identifiability` | no calibratable surface |
| identifiability/confounding analysis | NOT_APPLICABLE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#calibration-and-identifiability` | no empirical parameter |
| boundary, saturation, and failure reporting | PASS | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#branch-and-guard-table` | guards and gaps are explicit |
| equifinality/uncertainty retention | NOT_APPLICABLE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#calibration-and-identifiability` | no fitted ensemble |
| synthetic recovery | NOT_APPLICABLE | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#calibration-and-identifiability` | no calibration algorithm |
| additional-data inventory | PASS | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md#gap-register-and-promotability-labels` | required authority evidence is named; no data fit |
