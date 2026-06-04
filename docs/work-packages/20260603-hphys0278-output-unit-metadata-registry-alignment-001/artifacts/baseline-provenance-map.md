# Baseline Provenance Map

Status: completed
Evidence mode: static

Static: HPHYS0278 changes output metadata authority only. No publication values,
physics equations, comparator tolerances, or WAT column names changed.

## Provenance

- Boundary-backed output units resolve through
  `crates/openwepp-sim-contract/src/units.rs` boundary registry entries tied to
  canonical `SC-*` contracts.
- Publication-only output units are explicit output metadata rows with
  rationale, contract id, and invariant id.
- Legacy WAT and watershed publication column names are preserved.

Ran: not applicable; no baseline value comparison was required for metadata-only
alignment.
