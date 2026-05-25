# Legacy Sidecar Bridge Boundary

Status: Draft (ARCH08)
Evidence: Static
Ran evidence: none

## Purpose

Define the adapter-only boundary for legacy sidecar and HBP compatibility so
kernel/orchestrator crates remain free of legacy policy branching.

Implementation path:
- `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/lib.rs`

## Boundary Ownership

- Adapter crate ownership (`openwepp-legacy-bridge`):
  - strict/compat policy handling
  - sidecar filename canonicalization and alias gating
  - HBP magic compatibility gating
  - typed warning/error surfaces for compatibility outcomes
- Orchestrator ownership:
  - calling the adapter with run-directory discoveries and policy selection
  - deciding fail-stop/propagation behavior from typed adapter failures
- Kernel ownership:
  - no sidecar file discovery
  - no HBP compatibility branching
  - no legacy alias resolution logic

## Non-Leakage Rules

1. Legacy sidecar compatibility policy is adapter-local and must not be
   duplicated in kernels.
2. HBP magic acceptance/rejection policy is adapter-local and must not be
   duplicated in kernels.
3. Unknown sidecars and legacy aliases never silently disappear:
   - unknown sidecars are ignored with typed warning `LSB-W-002` in both strict
     and compat modes
   - legacy aliases fail in strict mode and emit typed warning `LSB-W-001` in
     compat mode when accepted
4. Required sidecars are explicit contract entries; absence is a typed failure.

## Strict vs Compat Semantics

### Sidecar adapter

- strict:
  - reject legacy alias usage
  - ignore unknown sidecars and emit `LSB-W-002`
  - reject missing required sidecars
- compat:
  - accept configured legacy aliases and emit `LSB-W-001`
  - ignore unknown sidecars and emit `LSB-W-002`
  - still reject missing required sidecars

### HBP adapter

- strict:
  - accept canonical HBP magic only
  - reject legacy magic alias with typed error
- compat:
  - accept configured legacy magic alias and emit `HBP-W-001`
  - reject unknown/short shards as typed failures

## Invariants

- `INV-LSB-001`: canonical sidecar bindings are deterministic by `sidecar_id`.
- `INV-LSB-002`: strict policy never accepts alias-only sidecar matches.
- `INV-LSB-003`: unknown discovered sidecars are warning-only (`LSB-W-002`)
  and never become typed hard failures by policy mode.
- `INV-LSB-004`: required sidecar absence is always a typed failure.
- `INV-LSB-005`: HBP shard shorter than contract `minimum_bytes` is always a
  typed failure.
- `INV-LSB-006`: strict policy never accepts legacy HBP magic aliases.

## ARCH08 Test Linkage

Crate-local tests in:
- `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/sidecar.rs`
- `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/hbp.rs`

Coverage includes:
- strict canonical acceptance
- strict typed failure branches
- compat warning branches
- duplicate contract/discovery validation paths
