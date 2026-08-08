# Calibration Readiness Matrix

Status: complete for authority-only intent

Evidence mode: Static

```text
science_implementation_status = AUTHORITY_MISSING
calibration_evidence_status = NOT_CALIBRATION_READY
identifiability_status = NOT_ASSESSED
```

| Obligation | Disposition | Evidence | Rationale |
|---|---|---|---|
| typed/enumerable parameter surface | PASS | `SC-VEGETATION-001` constants/parameters | Required classes, units, digest, provenance, and assumption labeling are defined; no values admitted. |
| observation operator with units and scale | NOT_APPLICABLE | contract calibration section | No equation, runtime output, dataset, or calibration intent exists in this package. |
| deterministic candidate execution | NOT_APPLICABLE | gap 008 | No implementation/candidate execution is authorized. |
| objective reconstruction | NOT_APPLICABLE | calibration section | No objective or observation role is admitted. |
| sensitivity analysis | NOT_APPLICABLE | calibration section | No executable parameterized model exists. |
| identifiability/confounding analysis | NOT_APPLICABLE | calibration section | `NOT_ASSESSED`; future family-specific authority required. |
| boundary, saturation, and failure reporting | PASS | branch/guard, gaps, vectors | All boundary failures and non-promotable gaps are explicit; no numeric saturation is admitted. |
| equifinality/uncertainty retention | NOT_APPLICABLE | calibration section | No parameter ensemble or empirical claim exists. |
| synthetic recovery | NOT_APPLICABLE | gap 008 | Structurally meaningless before an executable parameterized model and observation operator. |
| additional-data inventory | PASS | gaps 002-009 and implementation handoff | Future root, process, snow, component, and held-out observation needs are named. |

The `NOT_APPLICABLE` rows are package-intent dispositions, not readiness passes
for a later implementation or calibration package. Missing physiological
authority remains an explicit production blocker.
