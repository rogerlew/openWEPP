# Process Span Contract

Static: R5C adds two one-phase direct spans after R5B storage bounds:

1. `DIRECT_R5C_DECOMPOSITION_SPAN = [DecompositionTransition]`
2. `DIRECT_R5C_RESIDUE_PARTITION_SPAN = [ResiduePartitionTransition]`

## Phase Identity

Static:

- `Normalization -> StorageBounds -> DecompositionTransition ->
  ResiduePartitionTransition -> AnnualGrowthTransition` remains the canonical
  order.
- `DecompositionTransition` and `ResiduePartitionTransition` move from
  lifecycle `Hold` to lifecycle `Executed`.
- `AnnualGrowthTransition` and `PerennialGrowthTransition` remain lifecycle
  `Hold` until R5D.

## Span Counters

Static:

- Each R5C phase span records exactly one phase entry, one direct compute, one
  state mutation, one downstream operand production, one shadow projection, and
  zero compatibility-edge invocations.
- The direct executor day-span count increases from 16 to 18.
- Per-day direct phase-entry count increases by two.

## Upstream And Downstream Contract

Static:

- `DecompositionTransition` requires successful R5B `StorageBounds` shadow
  projection.
- `ResiduePartitionTransition` requires successful R5C
  `DecompositionTransition` shadow projection.
- Both spans fail closed with typed `DirectRuntimeError` when required upstream
  state is missing or invalid.

## Protected Boundaries

Static:

- No scheduler phase-order change.
- No public output cutover.
- No default runtime activation.
- No direct-runtime use of compatibility symbols, request payloads, writeback
  surfaces, or hot symbol tables.
