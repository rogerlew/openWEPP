# Observability Subsystem Charter (`OBS-CONTRACT-001`)

Status: draft
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in OBS01 kickoff

## Purpose
- Define first-class observability boundaries for openWEPP so developers can stimulate kernels and short phase pipelines without full end-to-end runs. `[DIRECT]`
- Replace ad-hoc cwd sentinel activation with typed debug intent and structured telemetry contracts. `[DIRECT]`

## Authority Anchors
- `docs/planning/openwepp-observability-subsystem-assessment.md` requires typed debug intent, kernel stimulation, deterministic replay hooks, and structured traces. `[DIRECT]`
- `docs/specifications/wepp-input-files/input-surface-registry.md` marks `wepp_observe.on`, `wepp_observe_frost.on`, and `wepp_observe_wb05e_target.dat` as `unsupported` parser sidecars. `[DIRECT]`
- `docs/decisions/0006-three-binaries-incl-replay.md` locks three-binary separation and replay as an explicit execution surface. `[DIRECT]`
- `/home/workdir/wepp-forest/src/wepp_observe.for:26` and `:32` show file-presence activation via cwd sentinel files. `[DIRECT]`
- `/home/workdir/wepp-forest/src/watbal_process_probe.f90:107-124` shows target-window activation by sidecar file read and cached gating. `[DIRECT]`

## Scope
- Observability contract surfaces for `openwepp-cli-hill`, `openwepp-cli-watershed`, and `openwepp-replay`. `[DIRECT]`
- Typed debug intent schema and validation semantics. `[DIRECT]`
- Structured trace/event envelope and payload requirements. `[DIRECT]`
- Replay-window selection contract for deterministic targeted reruns. `[DIRECT]`
- Migration/disposition off legacy `wepp_observe*` flags. `[DIRECT]`

## Out of Scope
- Broad implementation of instrumentation across all kernels in OBS01. `[DIRECT]`
- UI visualization productization. `[DIRECT]`
- Parser-sidecar compatibility reintroduction for `wepp_observe*`. `[DIRECT]`

## Subsystem Boundaries

| Boundary ID | Name | Responsibility | Inputs | Outputs | Evidence |
|---|---|---|---|---|---|
| `OBS-B-001` | Intent Boundary | Accept typed observability/debug intent from orchestrator or replay CLI. | intent document | validated intent object or typed validation error | `[DIRECT]` |
| `OBS-B-002` | Stimulation Boundary | Execute one kernel or a short kernel chain against fixture or replay snapshot state. | validated intent + state fixture/snapshot | kernel outputs + state deltas + diagnostic status | `[INFERENCE]` |
| `OBS-B-003` | Trace Boundary | Emit structured events from execution sites to configured sinks. | trace events | NDJSON/JSONL/Parquet event stream (contract-defined envelope) | `[INFERENCE]` |
| `OBS-B-004` | Replay Boundary | Re-execute deterministic windows on HBP-derived or fixture-derived state slices. | replay-window spec + state snapshot | replay output bundle + comparator deltas | `[DIRECT]` |
| `OBS-B-005` | Migration Boundary | Translate legacy debug intent use-cases to typed intent without parser sidecar compatibility. | migration table + operator procedures | explicit migration steps + rejection behavior for legacy sidecars | `[DIRECT]` |

## Ownership and Responsibilities
- `openwepp-cli-hill` and `openwepp-cli-watershed` own production-path event production under explicit intent opt-in; observability remains off-path by default. `[INFERENCE]`
- `openwepp-replay` owns deterministic replay-window execution and kernel isolation invocation surfaces. `[DIRECT]`
- Orchestrator boundaries (wepppy) own passing explicit intent and replay specs, not filesystem sentinels. `[INFERENCE]`
- Parser subsystem remains explicitly separate and must continue rejecting `wepp_observe*` as parser sidecars. `[DIRECT]`

## Required Contract Invariants

| Invariant ID | Requirement | Failure posture | Evidence |
|---|---|---|---|
| `OBS-INV-001` | Observability activation must occur through typed intent, not cwd sentinel file presence. | hard-fail config error on unsupported sentinel-mode requests | `[DIRECT]` |
| `OBS-INV-002` | Observability must support kernel/phase/surface scopes without full end-to-end run dependency. | hard-fail if requested scope cannot be stimulated by declared interface | `[DIRECT]` |
| `OBS-INV-003` | Replay-window executions must be deterministic within single-thread pinned-seed configuration. | hard-fail replay contract violation | `[DIRECT]` |
| `OBS-INV-004` | Trace payload must be schema-versioned and machine-parseable; ad-hoc string-only probes are non-compliant. | hard-fail schema validation for emitted events in strict mode | `[DIRECT]` |
| `OBS-INV-005` | Parser subsystem must not add `SC-INFILE-OBSERVE-FLAGS-*` contracts for legacy observe sidecars. | governance hold on any contradictory parser-contract proposal | `[DIRECT]` |

## Kernel Stimulation Entry Classes
- `kernel-single`: run one routine descriptor (`describe/validate_inputs/run/validate_output`) with fixture state. `[INFERENCE]`
- `phase-slice`: run a bounded kernel chain for one phase window with fixed ordering and explicit state ownership. `[INFERENCE]`
- `surface-check`: run minimal kernels needed to reproduce one state-surface delta and emit closure/guard context. `[INFERENCE]`

## Acceptance Checks (OBS01)
- Canonical observability subsystem spec files are populated with normative requirements. `[DIRECT]`
- Typed intent, trace-event, replay-window, and migration specs are internally consistent on IDs and field semantics. `[INFERENCE]`
- Legacy sidecar migration plan preserves explicit unsupported parser disposition and avoids compatibility rollback. `[DIRECT]`

## HOLD Register

| Hold ID | Issue | Why hold | Unblock condition | Evidence |
|---|---|---|---|---|
| `OBS-HOLD-001` | Snapshot envelope for stimulation/replay is not yet bound to a concrete schema artifact ID. | Implementation cannot guarantee cross-binary replay payload compatibility without this lock. | Define and publish snapshot schema contract in follow-on package (`OBS02`). | `[INFERENCE]` |
| `OBS-HOLD-002` | Canonical typed error taxonomy names for observability interfaces are not yet standardized across crates (workspace still crate-empty). | Implementation would risk unstable error-surface naming before crate bootstrap. | Bind error taxonomy once crate layout is established in architecture follow-on package. | `[DIRECT]` |

## Follow-On Package Queue
1. `OBS02` snapshot schema and typed error taxonomy lock-in. `[INFERENCE]`
2. `OBS03` kernel stimulation runner skeleton in replay binary path. `[INFERENCE]`
3. `OBS04` structured event sink implementation with strict-schema validation gate. `[INFERENCE]`
4. `OBS05` migration CLI diagnostics and operator-facing rejection messaging for legacy sidecars. `[INFERENCE]`
