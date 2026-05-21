# Debug Intent Schema (`OBS-INTENT-001`)

Status: draft-normative
Promoted by: `20260520-obs01-observability-subsystem-foundation`
Evidence mode: `Static`
Ran evidence: none in OBS01 promotion

## Purpose
- Define typed observability/debug intent for activation, scope, window, stimulation, and telemetry. `[DIRECT]`
- Replace ad-hoc sentinel-driven activation with validated contract fields. `[DIRECT]`

## Schema Identity
- `schema_id`: `openwepp-observability-intent-v1` `[INFERENCE]`
- `schema_version`: `1.0.0` `[INFERENCE]`

## Required Fields

| Field | Type | Required | Contract rule | Evidence |
|---|---|---|---|---|
| `schema_id` | string | yes | Must match canonical ID. | `[INFERENCE]` |
| `schema_version` | semver | yes | Major mismatch is hard-fail. | `[INFERENCE]` |
| `intent_id` | string | yes | Stable request correlation ID. | `[INFERENCE]` |
| `mode` | enum | yes | `production-parity` or `diagnostic`. | `[INFERENCE]` |
| `scope.level` | enum | yes | `kernel`, `phase`, or `surface`. | `[DIRECT]` |
| `scope.targets` | array<string> | yes | Non-empty target list. | `[INFERENCE]` |
| `window` | object | yes | Deterministic selector object per kind rules. | `[DIRECT]` |
| `stimulation.entry_class` | enum | yes | `kernel-single`, `phase-slice`, `surface-check`. | `[INFERENCE]` |
| `telemetry.enabled` | bool | yes | Explicit enable/disable only. | `[DIRECT]` |
| `telemetry.strict_schema` | bool | yes | If true, emit-time schema violations are hard-fail. | `[INFERENCE]` |
| `telemetry.sink.kind` | enum | yes | `jsonl`, `ndjson`, or future supported kind. | `[INFERENCE]` |
| `telemetry.sink.path` | string | yes | Explicit sink path required; no cwd defaults. | `[DIRECT]` |
| `replay.enabled` | bool | yes | Controls replay snapshot mode. | `[DIRECT]` |
| `replay.snapshot_ref` | string/null | conditional | Required when replay enabled. | `[INFERENCE]` |

## Validation Errors (Draft)

| Error code | Condition | Posture | Evidence |
|---|---|---|---|
| `OBS_INTENT_E001` | Unknown schema ID/version | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E002` | Missing required field | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E003` | Invalid scope vs stimulation class | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E004` | Invalid or incomplete window selector | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E005` | Replay enabled without snapshot reference | hard-fail | `[INFERENCE]` |
| `OBS_INTENT_E006` | Telemetry sink path missing/implicit | hard-fail | `[DIRECT]` |
| `OBS_INTENT_E007` | Legacy observe sidecar activation request | hard-fail + migration notice | `[DIRECT]` |

## Legacy Mapping Policy
- `wepp_observe.on` maps to explicit telemetry enablement in typed intent only at policy layer; parser compatibility remains unsupported. `[DIRECT]`
- `wepp_observe_frost.on` maps to explicit scope targeting filters in typed intent. `[DIRECT]`
- `wepp_observe_wb05e_target.dat` maps to explicit `window` selector fields. `[DIRECT]`

## Guard Rules
- Missing required intent fields must never be silently defaulted. `[DIRECT]`
- Sentinel-file discovery must never activate observability in this contract. `[DIRECT]`

## HOLD Register
- `OBS-HOLD-004`: final error-code namespace lock deferred until crate/module bootstrap. `[DIRECT]`
