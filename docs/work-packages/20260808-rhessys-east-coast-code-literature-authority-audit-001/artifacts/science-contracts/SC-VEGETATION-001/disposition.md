# Contract Disposition

Status: `PASS`

Evidence mode: `Static`

Canonical contract:
`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` at base
commit `86faf6fd22421372c6d9874b7bd0b7e1cabd439f`, with working-tree identity
recorded in `contract_ref.md`.

The version-2 amendment admits licensed provenance and strict schema identity,
not equations, constants, profile values, calibration, or runtime behavior.
All review and terminal-verification findings were accepted and repaired. Both
independent terminal verifiers passed the final contract bytes and gates.

| Finding ID | Source | Severity | Decision | Action | Artifact | Rationale |
| --- | --- | --- | --- | --- | --- | --- |
| `A-SC-001` | Review A | high | accepted | added `BEI-VEGETATION-002` and qualified audit dependency | canonical contract | active binding residue must be indexed |
| `B-SC-001` | Review B | high | accepted | reconciled v2 wording while preserving historical guard semantics | canonical contract/index | current lifecycle and existing contract guards must agree |
| `TV-A-SC-001` | Terminal A | critical | accepted | restored exact historical guard strings, ran affected suite and strict BEI check | gate results | contract-derived tests are non-deferrable |
