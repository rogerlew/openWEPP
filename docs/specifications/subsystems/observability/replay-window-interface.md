# Replay Window Interface (`OBS-REPLAY-001`)

Status: active
Promoted by: `20260520-obs01-observability-subsystem-foundation`
Evidence mode: `Static`
Ran evidence: none in OBS01 promotion

## Purpose
- Define deterministic replay-window request/response requirements for targeted diagnostics. `[DIRECT]`
- Keep replay contract boundaries separate from production run-config argument surfaces. `[DIRECT]`

## Request Fields

| Field | Type | Required | Evidence |
|---|---|---|---|
| `schema_id` | string | yes | `[INFERENCE]` |
| `request_id` | string | yes | `[INFERENCE]` |
| `snapshot_ref` | string | yes | `[DIRECT]` |
| `window.selector_kind` | enum | yes | `[INFERENCE]` |
| `window.selector` | object | yes | `[INFERENCE]` |
| `scope` | object | yes | `[DIRECT]` |
| `execution` | object | yes | `[DIRECT]` |
| `outputs` | object | yes | `[INFERENCE]` |

## Selector Kinds
- `year_sdate` for day-level targeting. `[DIRECT]`
- `year_sdate_hour` for sub-daily anomaly targeting. `[DIRECT]`
- `absolute_step` for iteration-index targeting. `[INFERENCE]`
- `phase_range` for bounded short-pipeline replay. `[INFERENCE]`

## Required Output Artifacts
- `replay_summary.json` with selector hash and determinism metadata. `[INFERENCE]`
- `delta_report.json` with comparator/baseline delta summary. `[DIRECT]`
- `events.jsonl` when telemetry emission is enabled. `[INFERENCE]`

## Failure Semantics
- Invalid selector payload is hard-fail validation error. `[INFERENCE]`
- Missing/unreadable snapshot is hard-fail I/O error. `[INFERENCE]`
- Unsupported scope target for selected snapshot context is hard-fail compatibility error. `[INFERENCE]`
- Determinism precondition violations in strict mode are hard-fail configuration errors. `[DIRECT]`

## Determinism Rules
- Deterministic mode requires single-thread execution behavior and explicit seed/materialization metadata capture in outputs. `[DIRECT]`
- Identical inputs under same target configuration must produce deterministic ordering and equivalent semantic outputs. `[DIRECT]`

## HOLD Register
- `OBS-HOLD-006`: snapshot/HBP materialization schema contract unresolved and blocks implementation-grade conformance tests. `[INFERENCE]`
