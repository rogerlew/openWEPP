# Kernel Writeback Contract

Status: Draft (ARCH07)
Evidence: Static
Ran evidence: none

## Purpose

Specify typed kernel invocation and orchestrator-controlled writeback semantics
for hillslope and watershed execution.

Implementation paths:
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`

## Contract Types

Requests:
- `HillslopeKernelRequest`
  - `phase_name`
  - `state_surface`
  - `flux_surface`
- `WatershedKernelRequest`
  - `node_kind`, `node_id`
  - `dependency_nodes`
  - `contributor_hillslopes`
  - `state_surface`
  - `flux_surface`

Response:
- `KernelRunResponse`
  - `status: SimulationStatus`
  - `writeback: KernelWritebackPayload`

Writeback fields:
- `WritebackField { symbol, value, minimum?, maximum? }`
- `KernelWritebackPayload { state_updates, flux_updates }`

## Decision and Apply Surface

Decision:
- `evaluate_kernel_writeback(phase, payload) -> KernelWritebackDecision`
- outputs:
  - `outcome: Accept | Reject`
  - `status: SimulationStatus`
  - `violations: Vec<ClosureViolation>`

Apply:
- `apply_kernel_writeback(phase, decision, payload, state, flux)`
- precondition: `decision.outcome == Accept`
- outputs:
  - `KernelWritebackApplyResult`
  - `outcome: Apply`
  - typed status + applied symbol lists

Error:
- `WritebackError::DecisionNotAccept { outcome }`
- `WritebackError::Status(StatusError)`

## Deterministic Message-ID Map

| outcome class | message id | boundary class |
| --- | --- | --- |
| accept | `KWRITEBACK-ACCEPT-001` | `OK` |
| apply | `KWRITEBACK-APPLY-001` | `OK` |
| reject non-finite | `KWRITEBACK-E-NON-FINITE` | `NON_FINITE` |
| reject domain | `KWRITEBACK-E-DOMAIN-VIOLATION` | `DOMAIN_VIOLATION` |

## Invariants

- `INV-KWRITEBACK-001`: every writeback scalar must pass finite check.
- `INV-KWRITEBACK-002`: bounded scalar with min/max must satisfy closed range.
- `INV-KWRITEBACK-003`: min-only scalar must satisfy lower bound.
- `INV-KWRITEBACK-004`: max-only scalar must satisfy upper bound.
- `INV-KWRITEBACK-005`: kernels do not mutate orchestrator runtime state maps.
- `INV-KWRITEBACK-006`: orchestrator applies updates only for `Accept` decisions.
- `INV-KWRITEBACK-007`: reject paths must leave orchestrator writeback surfaces unchanged.

## Status Phase Rules

- Hillslope kernel/writeback statuses must use phase `hillslope_kernel`.
- Watershed kernel/writeback statuses must use phase `watershed_kernel`.
- Status-phase mismatch is a typed failure (`MODE_MISMATCH`) at orchestrator
  boundary enforcement points.

## No-Fallback Policy

Rejected writeback proposals, non-finite payloads, domain violations, and
phase mismatches are explicit typed failures. Default-value substitution and
silent mutation suppression without status/diagnostic emission are forbidden.

