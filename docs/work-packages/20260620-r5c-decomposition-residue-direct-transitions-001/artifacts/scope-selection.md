# Scope Selection

Static: selected `DecompositionTransition` and `ResiduePartitionTransition`
as the R5C grouped phase family.

## Selected Scope

R5C owns two canonical phases from `DirectPhaseKind::ORDERED`:

- `DecompositionTransition`
- `ResiduePartitionTransition`

The implementation target is the direct-runtime typed frame path only. The
selected slice must run after R5B `StorageBounds` and before growth hold phases.
It must not edit scheduler dispatch, public output writers, CLI activation, or
compatibility request/writeback paths.

## Authority Inputs

Static:

- `docs/work-packages/r5-burndown-execplan.md` assigns decomposition/residue to
  R5C and requires typed inputs, direct compute, state mutation, downstream
  operands, and shadow projection for both phases.
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
  authorizes the PL17 tracked decomposition seed-pool update, finite and
  non-negative residue/root pools, bounded event fractions, non-negative
  `oratea`/`orater`, and typed hard failures for invalid domains.
- Existing compatibility dispatch in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/05_pl_phase_dispatch.rs`
  is discovery/provenance only. R5C must not call it from the direct runtime
  because it depends on compatibility boundary symbols and request contexts.

## Excluded Scope

Static:

- Annual and perennial growth transitions remain R5D.
- Full residue cover/publication cutover remains R6/R5E endpoint scope.
- Hydrology equations remain unchanged.
- Public WB13/WAT/PASS/loss/manifest output authority remains compatibility
  authoritative.
