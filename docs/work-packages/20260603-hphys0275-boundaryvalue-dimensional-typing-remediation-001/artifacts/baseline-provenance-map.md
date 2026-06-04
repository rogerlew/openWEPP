# Baseline Provenance Map

Status: completed
Evidence mode: static

Static: HPHYS0275 changes unit typing at runtime boundaries and does not change
baseline-authoritative physics equations, constants, or numerical conversion
lineage. Production numeric values remain the same values already emitted by
the climate runtime adapter and SIMIMPL28 forcing synthesis.

## Provenance

- `SC-CLIMATE-001`: daily climate `prcp`, `radly`, temperature, wind speed,
  elapsed storm timing, and hyetograph intensity units.
- `SC-SNOWFREEZE-001`: SIMIMPL28 hourly winter radiation, air temperature,
  cloud fraction, hourly rain, and hourly snowfall boundary families.
- `docs/specifications/unit-governance.md`: `BoundaryValue::scalar` is not
  final closure for migrated high-risk dimensional runtime surfaces.
- `docs/specifications/units/boundary-symbol-unit-registry.md`: canonical
  unit registry and HPHYS0275 migrated/follow-up typed posture.

Ran: not-run; static provenance only.
