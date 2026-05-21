# Simulation Subsystem and Kernel Architecture

Status: Draft (ARCH02)
Evidence: Static
Ran evidence: none

## Purpose
This document defines the openWEPP simulation architecture for subsystem
ownership, dependency direction, and kernel/orchestrator boundaries.

## Governing posture
- Static: [DIRECT] ADR-0011 sets architecture-first and top-down science-contract authority (`/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md:18`, `:23`).
- Static: [DIRECT] Existing process architecture already splits hillslope and watershed CLI responsibilities (`/home/workdir/openWEPP/docs/architecture/README.md:10`, `:23`).
- Static: [INFERENCE] This architecture refines subsystem internals while preserving that two-stage process model.

## Subsystem model

| subsystem | responsibility | outputs |
| --- | --- | --- |
| contracts | canonical typed state, status taxonomy, symbol aliases | contract crates/types |
| inputs | parse and validate scenario + sidecar declarations | immutable input model |
| topology | graph assembly/validation for hillslope/watershed | validated topology graph |
| hillslope orchestrator | deterministic phase scheduling for hillslope runtime | typed hillslope flux/state traces |
| watershed orchestrator | deterministic routing/impoundment scheduling | typed watershed flux/state traces |
| hillslope kernels | pure process kernels | flux + next-state + status |
| watershed kernels | pure route/impoundment kernels | flux + next-state + status |
| legacy bridge adapters | HBP compatibility and sidecar boundary handling | typed adapter payloads + typed errors |
| reporting/comparator | summaries and comparator-ready datasets | reports, traces, comparator packets |

## Dependency direction
- `contracts` -> all subsystems.
- `inputs` -> `topology` and orchestrators.
- `topology` -> orchestrators.
- orchestrators -> kernels.
- orchestrators -> reporting.
- adapters -> orchestrators (edge integration only).
- kernels -> no adapter/parser dependencies.

## Execution boundaries

### Pre-execution
- Parse `.run`, primary input files, and required sidecars.
- Build topology graph and run closure checks.
- Produce immutable input + validated topology artifacts.

### Hillslope execution
- Deterministic phase order: normalization, storage bounds, ET, percolation/deep seepage, lateral transfer, drainage, runoff reconciliation, storage reconciliation, closure diagnostics.
- Each phase returns typed `status` with message ID.
- Non-finite/domain violations are typed failures surfaced to orchestrator.

### Watershed execution
- Deterministic dispatch sequence over validated graph nodes.
- Route and impoundment kernels are invoked through typed orchestrator inputs.
- Summary accumulation (daily/monthly/yearly/EOS) runs as separate kernelized phase.

## Error and status policy
- Static: [INFERENCE] No silent fallback for contract-required fields, missing required sidecars, or kernel non-finite/domain failures.
- Static: [INFERENCE] Status model is standardized: `ok`, `finite_ok`, `domain_ok`, `boundary_class`, `clamp_class`, `message_id`.
- Static: [INFERENCE] Orchestrators decide whether to hard-stop or propagate typed failures upstream based on contract policy.

## Symbol continuity policy
- Canonical WEPP/wepp-forest symbols remain authoritative in science-contract
  variable tables.
- Rust field names may differ but must be mapped explicitly through alias tables.
- No replacement of canonical symbols without explicit alias metadata.

## Parallelism policy
- Parallel execution is allowed only for independent partitions with explicit
  dependency order and deterministic reduction semantics.
- Any parallel slice must document ordering assumptions and closure guarantees.

## Compatibility boundary policy
- Legacy HBP/sidecar support remains explicit adapter behavior.
- Adapter failures are typed errors.
- Kernel crates do not perform direct sidecar file I/O.

## Minimum implementation checkpoints
1. Subsystem crates/modules exist with one owner per mutable state surface.
2. Deterministic phase graph for hillslope and watershed is encoded and tested.
3. Typed kernel trait contracts and unified status taxonomy are implemented.
4. Topology validation gate runs before any timestep execution.
5. Canonical symbol alias tables are present for all contracted state surfaces.
6. Comparator confidence-tier metadata is carried with architecture outputs.
