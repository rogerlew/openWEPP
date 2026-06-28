# Kernel Profile Compliance

Status: complete
Evidence mode: Static + Ran

| Checklist Item | Status | Evidence |
|---|---|---|
| Canonical `SC-*` file updated before production edits | met | `SC-SNOWFREEZE-001` v103 added `INV-SNOWFREEZE-074`, `OBL-SNOWFREEZE-P-049`, shallow threshold variable, and 10.3.17 addendum. |
| Required schema sections touched | met | References, variables, invariants, obligations, boundary disposition, addendum, and revision history updated. |
| Algorithm/branch behavior updated for changed density selector | met | `PhysicsBulkShallowGuardV1` opt-in branch added; absent/empty default remains `PhysicsBulkDensityCompactionV1`; `legacy_wepp` rollback remains. |
| Guard/error mapping aligned with implementation | met | Unknown density selectors still fail closed through `HillslopeCliError::RuntimeSurfaceFailure`. |
| Unit-governance impact recorded | met | New threshold is `0.25 m`; no new unit conversion path or output schema field was added. |
| Test-vector obligations reflected in tests and evidence | met | Focused test covers contract markers, selector confinement, shallow-vs-deep behavior, report disposition, and protected boundaries. |
| Production activation | not authorized | Coupled WAT gates failed; candidate remains opt-in diagnostic only. |
