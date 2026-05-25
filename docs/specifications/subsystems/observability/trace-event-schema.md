# Trace Event Schema (`OBS-TRACE-001`)

Status: active
Promoted by: `20260520-obs01-observability-subsystem-foundation`
Evidence mode: `Static`
Ran evidence: none in OBS01 promotion

## Purpose
- Define schema-versioned structured events for diagnostics and replay analysis. `[DIRECT]`
- Replace ad-hoc text probe output as the primary observability contract. `[DIRECT]`

## Event Envelope

| Field | Type | Required | Evidence |
|---|---|---|---|
| `schema_id` | string | yes | `[INFERENCE]` |
| `schema_version` | semver | yes | `[INFERENCE]` |
| `event_id` | string | yes | `[INFERENCE]` |
| `intent_id` | string | yes | `[INFERENCE]` |
| `run_id` | string | yes | `[INFERENCE]` |
| `event_time_utc` | RFC3339 string | yes | `[INFERENCE]` |
| `binary_role` | enum (`hillslope`, `watershed`, `replay`) | yes | `[DIRECT]` |
| `scope_level` | enum (`kernel`, `phase`, `surface`) | yes | `[DIRECT]` |
| `event_kind` | enum | yes | `[INFERENCE]` |
| `severity` | enum | yes | `[INFERENCE]` |
| `window_selector` | object | yes | `[INFERENCE]` |
| `source` | object | yes | `[INFERENCE]` |
| `payload` | object | yes | `[INFERENCE]` |

## Required Event Kinds

| Event kind | Purpose | Evidence |
|---|---|---|
| `intent_accepted` | Records validated intent acceptance. | `[INFERENCE]` |
| `intent_rejected` | Records typed intent validation failure. | `[INFERENCE]` |
| `kernel_io` | Captures kernel inputs/outputs for diagnostics. | `[INFERENCE]` |
| `closure_summary` | Captures closure totals/residual surfaces. | `[DIRECT]` |
| `guard_violation` | Captures invariant violation context. | `[INFERENCE]` |
| `window_begin` | Marks deterministic replay/stimulation window start. | `[INFERENCE]` |
| `window_end` | Marks replay/stimulation window completion. | `[INFERENCE]` |
| `migration_notice` | Captures legacy mapping notices during migration paths. | `[INFERENCE]` |

## Emission Rules
- Event emission is opt-in via typed intent, not cwd sentinel discovery. `[DIRECT]`
- Strict-schema mode must fail fast when event shape is invalid. `[INFERENCE]`
- Per-window event ordering must be deterministic for the same within-config deterministic run conditions. `[INFERENCE]`

## Sink Requirements
- OBS01 minimum sink contract is line-delimited JSON output (`jsonl`/`ndjson`). `[INFERENCE]`
- Future sink formats must preserve schema IDs and semantic field parity. `[INFERENCE]`

## HOLD Register
- `OBS-HOLD-005`: units-manifest binding strategy for payload fields remains unresolved pending crate/module layout lock. `[DIRECT]`
