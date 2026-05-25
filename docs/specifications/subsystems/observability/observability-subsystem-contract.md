# Observability Subsystem Contract (`OBS-CONTRACT-001`)

Status: active
Promoted by: `20260520-obs01-observability-subsystem-foundation`
Evidence mode: `Static`
Ran evidence: none in OBS01 promotion

## Purpose
- Define first-class observability subsystem boundaries and responsibilities for openWEPP. `[DIRECT]`
- Enable kernel/phase/surface stimulation and deterministic replay diagnostics without requiring full end-to-end runs. `[DIRECT]`

## Normative Scope
- Typed observability intent acceptance and validation. `[DIRECT]`
- Bounded stimulation entry classes (`kernel-single`, `phase-slice`, `surface-check`). `[INFERENCE]`
- Structured trace/event emission interfaces and strict-schema mode expectations. `[DIRECT]`
- Replay-window request/response contract alignment for `openwepp-replay`. `[DIRECT]`
- Migration governance from legacy `wepp_observe*` sidecars. `[DIRECT]`

## Out of Scope
- Full implementation rollout for all kernels in this contract revision. `[DIRECT]`
- Parser-sidecar compatibility restoration for `wepp_observe*`. `[DIRECT]`

## Authority Anchors
- `docs/planning/openwepp-observability-subsystem-assessment.md` establishes required subsystem capabilities and no-sidecar carry-forward policy. `[DIRECT]`
- `docs/specifications/wepp-input-files/input-surface-registry.md` marks observe sidecars as unsupported parser surfaces. `[DIRECT]`
- `docs/decisions/0006-three-binaries-incl-replay.md` defines replay as a dedicated binary boundary. `[DIRECT]`
- `/home/workdir/wepp-forest/src/wepp_observe.for` and `/home/workdir/wepp-forest/src/watbal_process_probe.f90` show legacy sentinel/env-var probe behavior being replaced. `[DIRECT]`

## Boundary Contract

| Boundary ID | Name | Responsibility | Inputs | Outputs | Evidence |
|---|---|---|---|---|---|
| `OBS-B-001` | Intent Boundary | Validate typed intent and reject unsupported legacy activation patterns. | intent document | validated intent or typed error | `[DIRECT]` |
| `OBS-B-002` | Stimulation Boundary | Execute bounded kernel/phase/surface stimulation over fixture or snapshot state. | validated intent + fixture/snapshot | state deltas + diagnostic statuses | `[INFERENCE]` |
| `OBS-B-003` | Trace Boundary | Emit schema-versioned structured events to declared sinks. | structured events | JSONL/NDJSON event stream (Parquet future) | `[INFERENCE]` |
| `OBS-B-004` | Replay Boundary | Execute deterministic replay windows from replay request spec. | replay request + snapshot ref | replay summary + delta report + events | `[DIRECT]` |
| `OBS-B-005` | Migration Boundary | Enforce no-sidecar policy and provide migration mapping semantics. | legacy mapping policy | explicit migration notices + rejection posture | `[DIRECT]` |

## Required Invariants

| Invariant ID | Requirement | Failure posture | Evidence |
|---|---|---|---|
| `OBS-INV-001` | Activation must be explicit typed intent, never cwd sentinel discovery. | hard-fail config error | `[DIRECT]` |
| `OBS-INV-002` | Stimulation must support kernel/phase/surface scopes without full end-to-end run dependency. | hard-fail unsupported-scope error | `[DIRECT]` |
| `OBS-INV-003` | Replay-window executions must preserve within-config determinism expectations. | hard-fail determinism contract error | `[DIRECT]` |
| `OBS-INV-004` | Telemetry events must be schema-versioned structured payloads in strict mode. | hard-fail schema validation error | `[DIRECT]` |
| `OBS-INV-005` | Parser subsystem must not reintroduce `SC-INFILE-OBSERVE-FLAGS-*` compatibility. | governance hold | `[DIRECT]` |

## Normative Stimulation Scenarios

| Scenario ID | Scope | Minimum contract expectation | Evidence |
|---|---|---|---|
| `OBS-UC-001` | `kernel` | Single-kernel execution with closure/guard event emission. | `[INFERENCE]` |
| `OBS-UC-002` | `phase` | Daily phase-slice replay with deterministic selector hash and delta output. | `[DIRECT]` |
| `OBS-UC-003` | `surface` | Hourly anomaly isolation with comparator-tier metadata attached to outputs. | `[DIRECT]` |
| `OBS-UC-004` | `phase` | Watershed/channel micro-window replay keeping replay outputs distinct from production outputs. | `[DIRECT]` |
| `OBS-UC-005` | `surface` | Guard-violation reproduction with invariant ID projection and state-slice payload. | `[INFERENCE]` |
| `OBS-UC-006` | `kernel`/`phase` | Legacy intent translation path with explicit no-sidecar parser compatibility. | `[DIRECT]` |

## Ownership Model
- Replay binary owns replay-window request handling and bounded rerun execution surfaces. `[DIRECT]`
- Hillslope/watershed binaries own production-path event emission only when explicit intent enables telemetry. `[INFERENCE]`
- Parser subsystem ownership remains separate from observability subsystem and retains unsupported disposition for observe sidecars. `[DIRECT]`

## HOLD Register
- `OBS-HOLD-001`: snapshot envelope schema ID unresolved for strict cross-binary replay conformance. `[INFERENCE]`
- `OBS-HOLD-002`: stable typed error namespace unresolved pending crate bootstrap and module naming lock-in. `[DIRECT]`
