# Typed Observability Intent Schema (`OBS-INTENT-001`)

Status: draft
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in OBS01 kickoff

## Purpose
- Define the typed intent contract that replaces ad-hoc observe sidecar toggles. `[DIRECT]`
- Make observability activation explicit, validated, and scope-aware. `[DIRECT]`

## Schema Identity
- `schema_id`: `openwepp-observability-intent-v1` `[INFERENCE]`
- `schema_version`: `1.0.0` `[INFERENCE]`

## Top-Level Document Shape

```json
{
  "schema_id": "openwepp-observability-intent-v1",
  "schema_version": "1.0.0",
  "intent_id": "obs-intent-<uuid>",
  "mode": "diagnostic",
  "scope": {
    "level": "kernel",
    "targets": ["wb05e"]
  },
  "window": {
    "selector_kind": "year_sdate",
    "year": 1,
    "sdate": 83,
    "hour_or_step": null
  },
  "stimulation": {
    "entry_class": "kernel-single",
    "fixture_ref": "fixtures/wb05e/base.json"
  },
  "telemetry": {
    "enabled": true,
    "strict_schema": true,
    "sink": {
      "kind": "jsonl",
      "path": "./output/obs-events.jsonl"
    }
  },
  "replay": {
    "enabled": false,
    "snapshot_ref": null
  }
}
```

## Field Specification

| Field | Type | Required | Description | Validation rule | Evidence |
|---|---|---|---|---|---|
| `schema_id` | string | yes | Intent schema identifier. | Must equal `openwepp-observability-intent-v1`. | `[INFERENCE]` |
| `schema_version` | semver string | yes | Schema version. | Major version mismatch is hard error. | `[INFERENCE]` |
| `intent_id` | string | yes | Stable request ID for audit correlation. | Non-empty; UUID preferred. | `[INFERENCE]` |
| `mode` | enum | yes | `production-parity` or `diagnostic`. | Unknown mode is hard error. | `[INFERENCE]` |
| `scope.level` | enum | yes | `kernel`, `phase`, or `surface`. | Must match stimulation entry class compatibility. | `[DIRECT]` |
| `scope.targets` | array<string> | yes | Kernel/phase/surface target IDs. | Non-empty and deduplicated. | `[INFERENCE]` |
| `window` | object | yes | Deterministic window selector. | Selector completeness required by kind. | `[DIRECT]` |
| `stimulation.entry_class` | enum | yes | `kernel-single`, `phase-slice`, `surface-check`. | Must match `scope.level`. | `[INFERENCE]` |
| `stimulation.fixture_ref` | string/null | conditional | Fixture path for non-replay stimulation. | Required when replay is disabled. | `[INFERENCE]` |
| `telemetry.enabled` | bool | yes | Controls event emission request. | `false` allowed for pure replay-diff mode. | `[INFERENCE]` |
| `telemetry.strict_schema` | bool | yes | Enforce schema validation at emit time. | If `true`, invalid event is hard error. | `[INFERENCE]` |
| `telemetry.sink.kind` | enum | yes | `jsonl`, `ndjson`, or `parquet` (future). | Unsupported kind is hard error. | `[INFERENCE]` |
| `telemetry.sink.path` | string | yes | Sink location. | Must be explicit; no cwd implicit defaulting. | `[DIRECT]` |
| `replay.enabled` | bool | yes | Indicates replay snapshot mode. | When `true`, `snapshot_ref` required. | `[DIRECT]` |
| `replay.snapshot_ref` | string/null | conditional | Snapshot source for replay. | Required for replay mode. | `[INFERENCE]` |

## Enum Definitions
- `mode`: `production-parity`, `diagnostic`. `[INFERENCE]`
- `scope.level`: `kernel`, `phase`, `surface`. `[DIRECT]`
- `window.selector_kind`: `year_sdate`, `year_sdate_hour`, `absolute_step`, `date_range`. `[INFERENCE]`
- `stimulation.entry_class`: `kernel-single`, `phase-slice`, `surface-check`. `[INFERENCE]`

## Validation Error Taxonomy (Draft)

| Error code | Condition | Severity | Evidence |
|---|---|---|---|
| `OBS_INTENT_E001` | Unknown schema ID or major version mismatch | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E002` | Missing required top-level field | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E003` | Invalid scope/entry-class combination | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E004` | Missing selector fields for window kind | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E005` | Replay enabled without snapshot ref | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E006` | Telemetry sink path omitted or implicit | hard-fail | `[DIRECT]` |
| `OBS_INTENT_E007` | Legacy observe sidecar request detected | hard-fail + migration hint | `[DIRECT]` |

## Legacy Mapping (Policy Layer Only)

| Legacy surface | Typed intent mapping | Compatibility posture | Evidence |
|---|---|---|---|
| `wepp_observe.on` | `telemetry.enabled=true` and explicit `scope` required | parser-sidecar unsupported; policy translation only | `[DIRECT]` |
| `wepp_observe_frost.on` | `scope.targets` subset filtered to frost/snow domains | parser-sidecar unsupported; policy translation only | `[DIRECT]` |
| `wepp_observe_wb05e_target.dat` | `window.selector_kind=year_sdate` with explicit selector fields | parser-sidecar unsupported; policy translation only | `[DIRECT]` |

## Guard Rules
- Missing required intent data is never defaulted silently. `[DIRECT]`
- Sentinel-file presence must never trigger implicit activation in openWEPP observability interfaces. `[DIRECT]`
- Unknown fields may be rejected in strict mode to preserve deterministic behavior. `[INFERENCE]`

## HOLD Register
- `OBS-HOLD-004`: error-code namespace lock cannot be finalized until crate/module naming is established by workspace bootstrap. `[DIRECT]`
