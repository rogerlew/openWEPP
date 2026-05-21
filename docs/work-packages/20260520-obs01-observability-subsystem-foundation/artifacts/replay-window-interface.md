# Replay Window Interface (`OBS-REPLAY-001`)

Status: draft
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in OBS01 kickoff

## Purpose
- Define deterministic replay-window interface requirements for targeted kernel and phase stimulation. `[DIRECT]`
- Keep replay surface separate from production CLI argument surfaces per replay ADR boundaries. `[DIRECT]`

## Interface Contract Summary
- Replay execution is driven from `openwepp-replay` as a dedicated binary role. `[DIRECT]`
- Replay requests must use explicit structured window selectors and snapshot references. `[INFERENCE]`
- Replay outputs must remain distinguishable from production simulation outputs. `[DIRECT]`

## Input Contract (Replay Request)

| Field | Type | Required | Description | Evidence |
|---|---|---|---|---|
| `schema_id` | string | yes | `openwepp-replay-window-request-v1` | `[INFERENCE]` |
| `request_id` | string | yes | Stable request identity. | `[INFERENCE]` |
| `snapshot_ref` | string | yes | Snapshot/HBP-derived source artifact. | `[DIRECT]` |
| `window.selector_kind` | enum | yes | `year_sdate`, `year_sdate_hour`, `absolute_step`, or `phase_range`. | `[INFERENCE]` |
| `window.selector` | object | yes | Selector payload for chosen kind. | `[INFERENCE]` |
| `scope` | object | yes | Target scope (`kernel`, `phase`, `surface`) and targets. | `[DIRECT]` |
| `execution` | object | yes | Determinism and strictness toggles (single-thread, seed pinning, strict schema). | `[DIRECT]` |
| `outputs` | object | yes | Replay output directory/prefix and requested artifacts. | `[INFERENCE]` |

## Selector Semantics
- `year_sdate` selector mirrors existing legacy target semantics while removing sidecar activation coupling. `[DIRECT]`
- `year_sdate_hour` supports hourly anomaly localization used in legacy process probes. `[DIRECT]`
- `absolute_step` supports deterministic iteration index selection where day/hour conversion is unavailable. `[INFERENCE]`
- `phase_range` supports bounded short pipeline replay for phase-scoped diagnosis. `[INFERENCE]`

## Output Contract

| Output artifact | Required | Description | Evidence |
|---|---|---|---|
| `replay_summary.json` | yes | Status, selector hash, timing, determinism metadata. | `[INFERENCE]` |
| `events.jsonl` | conditional | Structured event stream if telemetry enabled. | `[INFERENCE]` |
| `delta_report.json` | yes | Comparator or baseline deltas for requested surfaces. | `[DIRECT]` |
| `state_slice_dump.json` | conditional | Optional minimal state dump for failed guard or divergence analysis. | `[INFERENCE]` |

## Determinism Requirements
- Replay must run single-threaded when deterministic mode is requested. `[DIRECT]`
- Seed and selector fields must be captured in output metadata for exact rerunability. `[INFERENCE]`
- Identical replay request inputs on the same target must produce deterministic event ordering and equivalent numeric surfaces under semantic-parity policies. `[DIRECT]`

## Failure Semantics

| Failure ID | Condition | Outcome | Evidence |
|---|---|---|---|
| `OBS_REPLAY_E001` | Invalid selector fields for selector kind | hard-fail validation error | `[INFERENCE]` |
| `OBS_REPLAY_E002` | Snapshot reference missing/unreadable | hard-fail I/O error | `[INFERENCE]` |
| `OBS_REPLAY_E003` | Requested scope target not available in snapshot context | hard-fail compatibility error | `[INFERENCE]` |
| `OBS_REPLAY_E004` | Determinism preconditions not met for strict mode | hard-fail configuration error | `[DIRECT]` |
| `OBS_REPLAY_E005` | Output path ambiguous or collides with production output naming policy | hard-fail output contract error | `[DIRECT]` |

## Relationship to Subprocess Model
- Replay remains a dedicated binary role and does not alter subprocess-per-hillslope orchestration rules for production pathways. `[DIRECT]`
- Any replay-internal subprocess use must follow explicit argument-array rules and avoid shell interpolation. `[DIRECT]`

## HOLD Register
- `OBS-HOLD-006`: snapshot/HBP-to-replay-state materialization schema remains unspecified and blocks implementation-grade conformance tests. `[INFERENCE]`
