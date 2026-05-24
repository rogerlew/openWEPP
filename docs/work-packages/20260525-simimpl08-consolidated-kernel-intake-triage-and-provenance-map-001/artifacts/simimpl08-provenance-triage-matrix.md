# simimpl08 provenance triage matrix

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Classification vocabulary is contract-authoritative:
  - `adopt`: intake permitted for bounded downstream implementation scope.
  - `defer`: retain candidate for future package wave; not intake-authorized now.
  - `reject`: do not intake into openWEPP production scope (violates/risks
    SIMCONS guardrails for current wave).
- Guard authority:
  - `SC-WATBAL-001` `INV-WATBAL-021` / `HS-SIMCONS-E-001`
  - `SC-SYSTEM-001` `INV-SYSTEM-021` / `WS-SIMCONS-E-001`

## Triage matrix
| candidate_surface | baseline provenance anchor | contract touchpoint(s) | classification | risk class | dependency/next wave | rationale |
|---|---|---|---|---|---|---|
| `SIMCONS-CAND-001` `watbal_process_types` core state model (`requested_mode`, `effective_mode`, structured status) | `src/watbal.for`, `src/watbal_hourly.for` | `INV-WATBAL-019`, `INV-SYSTEM-019`, `INV-WATBAL-021` | `adopt` | medium | `SIMIMPL09` | Provides modular type boundaries aligned with mode-propagation closure needs and does not require wholesale policy intake by itself. |
| `SIMCONS-CAND-002A` `watbal_process_kernels` core kernel set (`wbk01..wbk08`, diagnostics) | `src/watbal.for`, `src/watbal_hourly.for` | `INV-WATBAL-019`, `INV-WATBAL-021` | `adopt` | medium | `SIMIMPL09` then `SIMIMPL10` | Shared-kernel decomposition is a strong architecture intake candidate, but only as bounded subset with explicit typed-guard mapping and no policy/probe overlays. |
| `SIMCONS-CAND-002B` `wbk19a_*` runoff/runon/depression partition helpers | `src/watbal.for` + route/runon couplings | `INV-WATBAL-021`, SIMIMPL02 deferred coupling crosswalk | `defer` | medium-high | `SIMIMPL10` / later runoff-partition wave | Requires broader coupling closure and test authority beyond current SIMIMPL09 hourly-foundation boundary. |
| `SIMCONS-CAND-003` `watbal_daily_adapter` adapter skeleton | `src/watbal.for` | `INV-WATBAL-021` | `adopt` | medium | `SIMIMPL09` | Adapter decomposition is useful for lane orchestration, but only after removing runtime env-mode bypass (`process_accounting_wbk08_mode`). |
| `SIMCONS-CAND-004` `watbal_hourly_adapter` adapter skeleton with requested/effective mode fields | `src/watbal_hourly.for` | `INV-WATBAL-019`, `INV-SYSTEM-019`, `INV-WATBAL-021` | `adopt` | medium-high | `SIMIMPL09` | Mode-aware signature is aligned with SIMMODE closure, but intake must exclude probe toggles and untriaged qcap policy overlays. |
| `SIMCONS-CAND-005` `watbal_closure_guard_core` residual formula and threshold structure | `src/watbal.for`, `src/watbal_hourly.for` | `INV-WATBAL-001/002` family, `INV-WATBAL-021` | `adopt` | medium | `SIMIMPL09` | Closure residual formulation is valuable as authority/check surface; direct `error stop` behavior must be translated to typed error propagation in openWEPP. |
| `SIMCONS-CAND-006` `watbal_route_types` watershed/channel/impoundment schema | `src/route.for`, `src/wsh*.for`, `src/imp*.for` | `INV-SYSTEM-021` | `defer` | high | watershed/route package (post-SIMIMPL10) | Outside immediate hillslope SIMIMPL09 scope and tightly coupled to deferred watershed lane architecture. |
| `SIMCONS-CAND-007` `watbal_route_kernels` route/imp kernels | `src/route.for`, `src/wsh*.for`, `src/imp*.for` | `INV-SYSTEM-021` | `defer` | high | watershed/route package (post-SIMIMPL10) | Module is large, contains many deferred-legacy pathways and env-toggle authority controls; requires dedicated watershed triage/contract-test wave. |
| `SIMCONS-CAND-008` `hillslope_binary_pass_legacy_adapter` | `src/wshpas.for`, `src/cpass*.inc` lineage | `INV-SYSTEM-021` + binary pass contract family | `defer` | medium-high | pass-format bridge package | Valuable for pass lineage but not part of SIMIMPL09 hourly-lane kernel intake objective; retain for dedicated pass adapter governance. |
| `SIMCONS-POL-001` `wbk09_hourly_qcap_policy` and qcap parameters | no baseline parity anchor (new overlay) | `INV-WATBAL-021`, `INV-SYSTEM-021` | `reject` | high | none in current queue | Explicit qcap-style clamp overlay is non-authorized by default and is specifically called out as forbidden without separate contract disposition. |
| `SIMCONS-POL-002` env-driven identity-mode bypass (`process_accounting_wbk08_mode`) | no baseline parity anchor | `INV-WATBAL-021` | `reject` | high | none in current queue | Runtime env toggles that alter closure behavior are not admissible for production intake in this wave. |
| `SIMCONS-POL-003` probe/trace env surfaces (`process_accounting_wbk01_input_probe*`, `process_accounting_rain_routing_kernel_probe*`, `WB33_*TRACE*`) | no baseline parity anchor | `INV-SYSTEM-021` | `reject` | medium-high | observability-specific package only | These are instrumentation overlays, not kernel authority; intake would blur production behavior ownership and provenance. |
| `SIMCONS-POL-004` legacy shim/defer pathways (`legacy_shim_required`, `WBK_ROUTE_*_DEFER_LEGACY`) | partial legacy proxy only | `INV-SYSTEM-021` | `reject` | high | none in current queue | Fallback/defer wrappers conflict with openWEPP no-silent-fallback posture for promoted production paths. |
| `SIMCONS-POL-005` env authority toggles (`WB34_M5_D09_AUTH`, `WB34_M6_D09_AUTH`, `WB33_M5_D09_AUTHORITY`) | no baseline parity anchor | `INV-SYSTEM-021` | `reject` | high | none in current queue | Environment-driven authority switching is not acceptable for deterministic contract-governed production intake. |

## Summary counts
- `adopt`: 5
- `defer`: 4
- `reject`: 5

## Ran
- Candidate behavior probes used to support classification:
  - `cd /workdir/wepp-forest/fpm-src && rg -n 'get_environment_variable|legacy_shim_required|DEFER_LEGACY|qcap|clamp|process_accounting' watbal_daily_adapter.f90 watbal_hourly_adapter.f90 watbal_process_kernels.f90 watbal_route_kernels.f90`
  - `cd /workdir/wepp-forest/fpm-src && nl -ba watbal_process_kernels.f90 | sed -n '2045,2150p'`
  - `cd /workdir/wepp-forest/fpm-src && nl -ba watbal_hourly_adapter.f90 | sed -n '1,230p'`
  - `cd /workdir/wepp-forest/fpm-src && nl -ba watbal_route_kernels.f90 | sed -n '3528,4160p'`
  - `cd /workdir/wepp-forest/fpm-src && nl -ba watbal_route_kernels.f90 | sed -n '4880,4965p'`
