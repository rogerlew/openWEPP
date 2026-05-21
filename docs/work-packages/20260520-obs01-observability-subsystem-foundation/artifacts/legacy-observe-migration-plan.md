# Legacy Observe Migration Plan (`OBS-MIGRATION-001`)

Status: draft
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in OBS01 kickoff

## Goal
- Migrate off ad-hoc legacy observe sidecar toggles to typed observability interfaces. `[DIRECT]`
- Preserve diagnostic intent while removing parser-sidecar compatibility dependency. `[DIRECT]`

## Legacy Surfaces (Inventory)

| Legacy surface | Legacy behavior | Source evidence | OBS disposition | Evidence |
|---|---|---|---|---|
| `wepp_observe.on` | Enables observe logging by cwd file presence probe. | `/home/workdir/wepp-forest/src/wepp_observe.for:26-31` | unsupported parser sidecar; replaced by typed intent activation | `[DIRECT]` |
| `wepp_observe_frost.on` | Filters emitted tags to frost/snow-related prefixes. | `/home/workdir/wepp-forest/src/wepp_observe.for:32-40` | unsupported parser sidecar; replaced by typed scope target filters | `[DIRECT]` |
| `wepp_observe_wb05e_target.dat` | Optional year/sdate target-window gating for observe flow. | `/home/workdir/wepp-forest/src/watbal_process_probe.f90:107-124` | unsupported parser sidecar; replaced by replay/window selector fields | `[DIRECT]` |
| `process_accounting_*` env vars | Enable/shape multiple CSV traces and authority controls. | `/home/workdir/wepp-forest/src/watbal_process_probe.f90:147-219` | migrate to typed intent and replay request fields | `[DIRECT]` |

## Parser Governance Lock
- Input-surface registry already marks all `wepp_observe*` files as `unsupported` parser sidecars. `[DIRECT]`
- Parser contracts must not add compatibility shims for `wepp_observe*` surfaces. `[DIRECT]`

## Migration Mapping

| Legacy control pattern | New control pattern | Migration behavior | Evidence |
|---|---|---|---|
| CWD sentinel file toggles (`wepp_observe.on`) | `telemetry.enabled=true` in typed intent | hard-fail on legacy file dependence; emit migration notice | `[DIRECT]` |
| Tag-prefix filter via `wepp_observe_frost.on` | `scope.targets` and optional event-kind filter in typed intent | preserve semantic filtering via explicit selectors | `[DIRECT]` |
| Target date file (`wepp_observe_wb05e_target.dat`) | `window.selector_kind=year_sdate` + selector payload | preserve target-window semantics with explicit fields | `[DIRECT]` |
| Env-var trace paths and enables | `telemetry.sink` and event-kind subscriptions | convert to validated declarative config | `[INFERENCE]` |

## Migration Phases
1. Phase `OBS01` (this package). `[DIRECT]`
- Publish canonical contracts and migration mapping. `[DIRECT]`
- Mark unresolved schema/taxonomy blockers as `HOLD`. `[DIRECT]`

2. Phase `OBS02`. `[INFERENCE]`
- Implement typed intent parser and replay-window request parser with strict validation errors. `[INFERENCE]`
- Add explicit migration diagnostic message when legacy surfaces are detected. `[INFERENCE]`

3. Phase `OBS03`. `[INFERENCE]`
- Implement structured trace emission and sink validation. `[INFERENCE]`
- Add comparator-facing replay output bundles for targeted windows. `[INFERENCE]`

4. Phase `OBS04`. `[INFERENCE]`
- Remove remaining legacy-observe operational guidance from runbooks; replace with typed-intent examples only. `[INFERENCE]`

## Failure Posture
- If legacy observe sidecar files are requested as parser inputs, return typed unsupported-surface error with migration hint. `[DIRECT]`
- Do not silently ignore legacy observe controls when intent is ambiguous; fail explicitly. `[DIRECT]`
- No default trace sink path inference from cwd for migrated workflows. `[DIRECT]`

## Compatibility Guarantees
- Diagnostic scope parity is maintained through explicit scope and window fields, not sidecar toggles. `[INFERENCE]`
- Existing legacy tags can be represented as structured `source` and `payload` metadata in event schema for audit continuity. `[INFERENCE]`

## Risks
- Operators familiar with sentinel toggles may need explicit CLI/documentation transition support. `[INFERENCE]`
- Without snapshot schema lock-in, migration implementation cannot guarantee cross-role replay portability yet. `[INFERENCE]`

## HOLD Register
- `OBS-HOLD-007`: migration-facing CLI UX and error-message wording are not yet bound to an implemented runner/replay command surface. `[DIRECT]`
