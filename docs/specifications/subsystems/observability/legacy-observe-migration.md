# Legacy Observe Migration (`OBS-MIGRATION-001`)

Status: draft-normative
Promoted by: `20260520-obs01-observability-subsystem-foundation`
Evidence mode: `Static`
Ran evidence: none in OBS01 promotion

## Purpose
- Define the authoritative migration/disposition from legacy observe toggles to typed observability interfaces. `[DIRECT]`
- Preserve explicit unsupported parser-sidecar posture for `wepp_observe*`. `[DIRECT]`

## Legacy Surface Disposition

| Legacy surface | Legacy behavior summary | OBS disposition | Evidence |
|---|---|---|---|
| `wepp_observe.on` | CWD file-presence probe activates observe logging. | unsupported parser sidecar; translated only at policy layer to typed intent. | `[DIRECT]` |
| `wepp_observe_frost.on` | Tag-prefix filter limits emitted observe lines. | unsupported parser sidecar; represented as explicit scope filters. | `[DIRECT]` |
| `wepp_observe_wb05e_target.dat` | Sidecar file sets target `year/sdate` observe gate. | unsupported parser sidecar; represented as explicit window selectors. | `[DIRECT]` |

## Migration Rules
- Parser contracts must not reintroduce `SC-INFILE-OBSERVE-FLAGS-*` compatibility surfaces. `[DIRECT]`
- Legacy control semantics are preserved only through explicit typed intent fields and replay selectors. `[DIRECT]`
- Missing or ambiguous migrated inputs are hard-fail conditions with migration notices, not silent fallback. `[DIRECT]`

## Phased Migration Plan
1. OBS01: publish canonical contracts and mapping tables. `[DIRECT]`
2. OBS02: implement typed intent/replay request parsers and migration errors. `[INFERENCE]`
3. OBS03: implement structured telemetry sinks and migration notices in runtime output. `[INFERENCE]`
4. OBS04: remove remaining operator guidance that depends on legacy observe files. `[INFERENCE]`

## HOLD Register
- `OBS-HOLD-007`: operator-facing migration CLI UX contract unresolved pending runner/replay implementation surfaces. `[DIRECT]`
