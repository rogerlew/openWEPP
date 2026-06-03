# Unit Governance Gate Requirements

Status: completed
Evidence mode: static

Static: HPHYS0273 defines these contract-derived gates for follow-up
implementation packages. The gates are derived from
`docs/specifications/unit-governance.md`.

## Required Gates

1. **Contract unit coverage gate**
   - Every touched dimensional `SC-*` symbol has an explicit unit.
   - Every touched alias-map row includes a unit check.
   - Missing unit rows keep kernel-affecting disposition in `HOLD`.
2. **Boundary registry gate**
   - Every touched dimensional runtime symbol has a registry entry or explicit
     registry gap.
   - Registry entry includes canonical symbol, aliases, unit, dimension,
     domain, producer/consumer, owning contract, typed-wrapper posture, and
     publication mapping.
3. **Typed-boundary gate**
   - New or migrated high-risk dimensional runtime seams use typed
     `BoundaryValue` variants where wrappers exist.
   - Remaining dimensional `BoundaryValue::scalar` surfaces require explicit
     scalar-exception rationale.
4. **Conversion helper gate**
   - Dimensional conversions use named directional helpers.
   - Raw conversion literals require allowlist provenance and follow-up
     disposition.
5. **Output metadata gate**
   - Published unit metadata traces to the same authority as runtime symbols.
   - Publication unit conversions are named and tested.
6. **Review disposition gate**
   - Dual reviews complete before final package closure.
   - Every finding is dispositioned and verified.

## Follow-Up Owners

| Gate | First implementation owner |
| --- | --- |
| Contract unit coverage | HPHYS0279 |
| Boundary registry | HPHYS0274 |
| Typed-boundary | HPHYS0275 |
| Conversion helper | HPHYS0276 |
| Output metadata | HPHYS0278 |
| High hourly radiation guard | HPHYS0277 |

Ran: not-run; this artifact is static gate-authoring evidence.
