# Simulation Status Taxonomy

Status: Draft (ARCH03)
Evidence: Static
Ran evidence: none

## Purpose

Define the typed, deterministic status taxonomy used at simulation subsystem
boundaries for pre-execution checks, kernel execution, and summary phases.

This taxonomy is implemented in `crates/openwepp-sim-contract/src/status.rs`.

## Core Status Surface

`SimulationStatus` fields:

| field | type | semantics |
| --- | --- | --- |
| `phase` | `SimulationPhase` | Lifecycle phase producing the status (`pre_execution_validation`, `hillslope_kernel`, `watershed_kernel`, `summary_accumulator`, `compatibility_adapter`). |
| `ok` | `bool` | Producer-declared success bit. |
| `finite_ok` | `bool` | Non-finite gate (`NaN`, `Inf`) result. |
| `domain_ok` | `bool` | Domain/bounds gate result. |
| `boundary_class` | `BoundaryClass` | Machine-readable boundary outcome class. |
| `clamp_class` | `ClampClass` | Explicit clamp/adjustment class (`NONE` when no clamp applied). |
| `message_id` | `String` | Stable message contract ID for auditability and routing. |

## Deterministic Classification Rules

`SimulationStatus::classification()` returns one of:
- `Nominal`
- `Advisory`
- `Failure`

Rule order:
1. If any of `ok == false`, `finite_ok == false`, or `domain_ok == false`:
   classification is `Failure`.
2. Else if `boundary_class` is a failure class:
   classification is `Failure`.
3. Else if `boundary_class` is advisory OR `clamp_class != NONE`:
   classification is `Advisory`.
4. Else:
   classification is `Nominal`.

Severity mapping is deterministic:
- `Nominal` -> `Ok`
- `Advisory` -> `Warning`
- `Failure` -> `Error`

## Boundary Classes

Baseline boundary classes include legacy WEPP-aligned values plus
architecture-level failure classes.

| boundary_class | class |
| --- | --- |
| `OK` | nominal |
| `DRY` | advisory |
| `SATURATED` | advisory |
| `CAP_BINDING` | advisory |
| `NEGATIVE_INPUT` | failure |
| `ZERO_GEOMETRY` | failure |
| `MODE_MISMATCH` | failure |
| `TOPOLOGY_INVALID` | failure |
| `CLOSURE_VIOLATION` | failure |
| `DOMAIN_VIOLATION` | failure |
| `NON_FINITE` | failure |
| `MISSING_REQUIRED_INPUT` | failure |

## Clamp Classes

| clamp_class | semantics |
| --- | --- |
| `NONE` | no clamp applied |
| `LOWER_BOUND_CLAMP` | lower bound clamp was applied |
| `UPPER_BOUND_CLAMP` | upper bound clamp was applied |
| `QCAP_SOFT_LIMIT` | runoff soft-cap policy clamp |
| `PROFILE_SHORTFALL` | profile shortfall clamp |

## Policy Constraints

- `message_id` must be non-empty.
- Advisory constructor paths reject failure boundary classes.
- No silent fallback: failures remain typed and explicit.

## ARCH03 Test Linkage

Covered by:
- `tests/integration/sim_contract_status_taxonomy.rs`
