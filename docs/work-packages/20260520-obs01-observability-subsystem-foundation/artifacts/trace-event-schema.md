# Trace Event Schema (`OBS-TRACE-001`)

Status: draft
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in OBS01 kickoff

## Purpose
- Define machine-parseable, schema-versioned observability events for kernel and short-pipeline diagnostics. `[DIRECT]`
- Replace ad-hoc string-line probe output as the primary observability contract surface. `[DIRECT]`

## Authority Anchors
- Legacy probe logging currently emits formatted text lines to `wepp_observe.log` with fixed tag/value layout. `[DIRECT]`
- Legacy process probes emit CSV files gated by environment variables and fixed header strings. `[DIRECT]`
- OBS01 requires structured trace/event schema requirements as a first-class subsystem capability. `[DIRECT]`

## Event Envelope

| Field | Type | Required | Description | Evidence |
|---|---|---|---|---|
| `schema_id` | string | yes | `openwepp-observability-event-v1` | `[INFERENCE]` |
| `schema_version` | semver string | yes | Event schema version. | `[INFERENCE]` |
| `event_id` | string | yes | Stable unique event identifier. | `[INFERENCE]` |
| `intent_id` | string | yes | Links event to validated observability intent. | `[INFERENCE]` |
| `run_id` | string | yes | Parent simulation/replay execution ID. | `[INFERENCE]` |
| `event_time_utc` | RFC3339 string | yes | Event timestamp. | `[INFERENCE]` |
| `binary_role` | enum | yes | `hillslope`, `watershed`, `replay`. | `[DIRECT]` |
| `scope_level` | enum | yes | `kernel`, `phase`, `surface`. | `[DIRECT]` |
| `event_kind` | enum | yes | Semantic event class. | `[INFERENCE]` |
| `severity` | enum | yes | `debug`, `info`, `warn`, `error`, `fatal`. | `[INFERENCE]` |
| `window_selector` | object | yes | Replayed or stimulated window identity. | `[INFERENCE]` |
| `source` | object | yes | Module/routine identifiers and phase context. | `[INFERENCE]` |
| `payload` | object | yes | Event-specific structured payload. | `[INFERENCE]` |

## Event Kinds

| Event kind | Required payload keys | Purpose | Evidence |
|---|---|---|---|
| `intent_accepted` | `scope`, `window`, `entry_class` | Confirms validated intent for execution. | `[INFERENCE]` |
| `intent_rejected` | `error_code`, `error_message`, `field_path` | Records typed validation failure. | `[INFERENCE]` |
| `kernel_io` | `kernel_id`, `inputs`, `outputs`, `units_manifest_ref` | Captures routine input/output vectors for diagnostics. | `[INFERENCE]` |
| `closure_summary` | `inputs_total`, `outputs_total`, `storage_delta`, `residual` | Captures closure surfaces analogous to legacy water-balance probe tags. | `[DIRECT]` |
| `guard_violation` | `invariant_id`, `violation_class`, `state_slice` | Captures hard-fail invariant context. | `[INFERENCE]` |
| `window_begin` | `window_hash`, `selector` | Marks deterministic replay/stimulation window entry. | `[INFERENCE]` |
| `window_end` | `window_hash`, `status`, `duration_ms` | Marks window completion and status. | `[INFERENCE]` |
| `migration_notice` | `legacy_surface`, `mapped_field`, `policy` | Operator-facing mapping record for migration path. | `[INFERENCE]` |

## Payload Conventions
- Numeric fields must carry explicit units via either field suffix or `units` object references. `[INFERENCE]`
- Missing numeric values must use explicit null semantics instead of sentinel magic constants in new event payloads. `[INFERENCE]`
- If legacy sentinel values are imported for provenance, the payload must carry `legacy_sentinel=true` metadata and original symbol context. `[INFERENCE]`

## Example Event

```json
{
  "schema_id": "openwepp-observability-event-v1",
  "schema_version": "1.0.0",
  "event_id": "evt-7f0a",
  "intent_id": "obs-intent-123",
  "run_id": "run-abc",
  "event_time_utc": "2026-05-20T23:59:00Z",
  "binary_role": "replay",
  "scope_level": "kernel",
  "event_kind": "closure_summary",
  "severity": "info",
  "window_selector": {"selector_kind": "year_sdate", "year": 1, "sdate": 83},
  "source": {"module": "watbal", "routine": "wb05e", "phase": "hourly"},
  "payload": {
    "inputs_total_mm": 12.2,
    "outputs_total_mm": 12.1,
    "storage_delta_mm": 0.1,
    "residual_mm": 0.0
  }
}
```

## Emission Rules
- Emission must be explicit opt-in by typed intent; no implicit enablement by cwd file presence. `[DIRECT]`
- Strict mode must fail fast on invalid event shape when `telemetry.strict_schema=true`. `[INFERENCE]`
- Event ordering within one execution window must be deterministic for the same pinned configuration. `[INFERENCE]`

## Sink Requirements
- Minimum required sink for OBS01 is line-delimited JSON (`jsonl`/`ndjson`) to preserve append-friendly behavior with schema preservation. `[INFERENCE]`
- Future Parquet sink may be added but must preserve schema IDs and comparable field semantics. `[INFERENCE]`

## HOLD Register
- `OBS-HOLD-005`: canonical units-manifest reference strategy for event payload fields is unresolved pending concrete crate/module layout. `[DIRECT]`
