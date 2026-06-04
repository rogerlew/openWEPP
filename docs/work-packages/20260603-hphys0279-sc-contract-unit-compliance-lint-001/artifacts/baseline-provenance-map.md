# Baseline Provenance Map

Status: completed
Evidence mode: static

Static: HPHYS0279 changes documentation/tooling governance only. No process
physics, runtime behavior, comparator tolerances, publication values, or legacy
baseline equations changed.

Provenance:

- Contract unit lint authority derives from
  `docs/specifications/unit-governance.md`.
- Contract-profile shape derives from
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Symbol alias/unit rules derive from
  `docs/specifications/science-contract-authoring-procedure.md`.
- Executable registry cross-checks parse
  `crates/openwepp-sim-contract/src/units.rs`.

Ran: not applicable; no baseline physics comparison was required.
