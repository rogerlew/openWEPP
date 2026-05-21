# Kernel Stimulation Use Cases

Status: draft
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in OBS01 kickoff

## Purpose
- Define concrete observability-driven stimulation scenarios that avoid full end-to-end runs. `[DIRECT]`
- Provide acceptance-oriented scenario IDs for follow-on implementation slices. `[INFERENCE]`

## Preconditions
- Replay binary exists as a dedicated execution surface (`openwepp-replay`). `[DIRECT]`
- Routine interface requires explicit `describe/validate_inputs/run/validate_output` behaviors suitable for bounded execution. `[DIRECT]`
- Legacy probes currently bind activation to sentinel files and fixed windows, demonstrating current limitations to replace. `[DIRECT]`

## Use-Case Matrix

| Use Case ID | Scenario | Trigger | Required Inputs | Expected Outputs | Confidence Tier Signal | Evidence |
|---|---|---|---|---|---|---|
| `OBS-UC-001` | Single-kernel closure investigation | Developer requests one kernel with fixture | kernel ID, fixture state slice, tolerance profile | kernel output, closure residuals, guard status events | Tier-A daily surface if daily water-balance scoped | `[INFERENCE]` |
| `OBS-UC-002` | Phase-slice replay (daily) | Replay request for one day and phase chain | replay snapshot, day selector, phase chain | deterministic rerun outputs + diff report | Tier-A when single OFE daily surfaces are targeted | `[DIRECT]` |
| `OBS-UC-003` | Hourly anomaly isolation | Comparator delta investigation on hourly surface | snapshot, hourly selector, kernel chain | event stream with first divergence context | investigation signal only (not standalone rejection) | `[DIRECT]` |
| `OBS-UC-004` | Watershed-channel micro-window | Channel routing discrepancy investigation | HBP shard subset, channel selector, routing window | routing trace events + delta packet | lower-confidence investigation signal | `[DIRECT]` |
| `OBS-UC-005` | Guard-violation replay | Hard-fail invariant reproduction | fixture/snapshot, invariant selector, strict mode | typed guard violation event with state slice | gate-blocking when invariant hard-fail class | `[INFERENCE]` |
| `OBS-UC-006` | Legacy-observe intent translation check | Operator migrates old debug toggle workflow | translated typed intent (no sidecar file) | equivalent diagnostic scope + explicit migration metadata | migration readiness indicator | `[INFERENCE]` |

## Scenario Details

### `OBS-UC-001` Single-kernel closure investigation
- Must execute without launching hillslope/watershed end-to-end orchestration. `[DIRECT]`
- Must emit `kernel_io` and `closure_summary` events for one routine invocation. `[INFERENCE]`
- Must reject missing units/field mappings at intent validation stage. `[INFERENCE]`

### `OBS-UC-002` Phase-slice replay (daily)
- Window selectors must include `year` and `sdate` equivalent selectors, matching existing legacy target semantics but via typed fields. `[DIRECT]`
- Replay rerun must include explicit `window_selector_hash` for deterministic audit replay. `[INFERENCE]`

### `OBS-UC-003` Hourly anomaly isolation
- Hourly and watershed deltas are investigation signals, not standalone rejection outcomes. `[DIRECT]`
- Event payload must include comparator-tier metadata so downstream gates avoid false hard-fail conclusions. `[INFERENCE]`

### `OBS-UC-004` Watershed-channel micro-window
- Must preserve subprocess-orchestrated boundaries and avoid shell interpolation in any spawned command path. `[DIRECT]`
- Channel replay outputs must remain distinguishable from production outputs per replay ADR guidance. `[DIRECT]`

### `OBS-UC-005` Guard-violation replay
- Requires strict-schema trace mode and invariant ID projection in emitted events. `[INFERENCE]`
- Must preserve typed error emission (no silent fallback/no defaulting). `[DIRECT]`

### `OBS-UC-006` Legacy-observe intent translation check
- Translation mechanism is policy-layer only; parser-sidecar compatibility remains unsupported. `[DIRECT]`
- Translation must capture legacy provenance in metadata for operator traceability. `[INFERENCE]`

## Acceptance Gates for Stimulation Layer
- Every use case must declare required inputs and deterministic window selectors. `[INFERENCE]`
- Every use case must define expected structured outputs and gate semantics. `[INFERENCE]`
- No use case may require runtime file-presence toggles in cwd. `[DIRECT]`

## HOLD Register
- `OBS-HOLD-003`: fixture/snapshot interchange schema ID is unresolved and blocks strict conformance tests for use cases `OBS-UC-001` through `OBS-UC-005`. `[INFERENCE]`
