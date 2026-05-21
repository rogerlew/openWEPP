# Topology Validation Gate

Status: Draft (ARCH04)
Evidence: Static
Ran evidence: none

## Purpose

Specify the deterministic pre-execution topology validation gate and typed
failure surfaces for topology closure/invariant enforcement.

Implementation path:
`/home/workdir/openWEPP/crates/openwepp-topology/src/lib.rs`.

## Gate Interface

Input:
- `TopologyGraph`

Output:
- `TopologyValidationReport`
  - `status: SimulationStatus`
  - `violations: Vec<ClosureViolation>`

Status phase is `pre_execution_validation`.

## Status Policy

### Pass
- `classification: Nominal`
- `boundary_class: OK`
- `message_id: TOPOLOGY-OK-001`

### Fail
- `classification: Failure`
- `boundary_class: TOPOLOGY_INVALID`
- `message_id: TOPOLOGY-E-VALIDATION-FAILED`
- one or more typed closure violations are emitted

No silent fallback is allowed for topology closure failures.

## Invariant and Diagnostic Map

| invariant/check id | failure message id | rule |
| --- | --- | --- |
| `INV-TOPO-001` | `TOPO-E-001` | `hillslope_count >= 1` |
| `INV-TOPO-002` | `TOPO-E-002` | declared vs observed channel count closure |
| `INV-TOPO-003` | `TOPO-E-003` | declared vs observed impoundment count closure |
| `INV-TOPO-004` | `TOPO-E-004` | each downstream node has at least one contributor |
| `INV-TOPO-005` | `TOPO-E-005` | channel/impoundment directed cycle count must be zero |
| `INV-TOPO-006` | `TOPO-E-006` | hillslope contributor reference domain |
| `INV-TOPO-006` | `TOPO-E-007` | channel contributor reference domain |
| `INV-TOPO-006` | `TOPO-E-008` | impoundment contributor reference domain |
| `INV-TOPO-007` | `TOPO-E-009` | unresolved channel contributor reference |
| `INV-TOPO-008` | `TOPO-E-010` | unresolved impoundment contributor reference |

## Integration with ARCH03 Contract Substrate

- Uses `openwepp-sim-contract::status` for gate status taxonomy.
- Uses `openwepp-sim-contract::closure` primitives for count/domain checks.
- Uses typed `ClosureViolation` diagnostics for orchestration-facing failure
  routing.

## ARCH04 Test Linkage

Covered by:
- `/home/workdir/openWEPP/tests/integration/topology_graph_validation_gate.rs`
