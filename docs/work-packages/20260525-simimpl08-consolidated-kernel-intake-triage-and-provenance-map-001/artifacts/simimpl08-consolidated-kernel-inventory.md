# simimpl08 consolidated kernel inventory

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Baseline provenance authority anchor:
  - `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Consolidated candidate intake source:
  - `/workdir/wepp-forest/fpm-src@7b24677f8f174d7ee3dff989aa1530fdb1a7c60a`
- Inventory scope follows SIMIMPL01/SIMIMPL03 SIMCONS guardrails and includes
  candidate watbal kernel, adapter, and policy-related modules.

## Candidate module inventory
| candidate_id | candidate module | lines | primary exports/surfaces | baseline anchor(s) | notes |
|---|---|---:|---|---|---|
| `SIMCONS-CAND-001` | `watbal_process_types.f90` | 287 | typed timestep/geometry/input/storage/policy/flux/closure/status structures | `src/watbal.for`, `src/watbal_hourly.for` | core data model decomposition; includes mode and clamp/status classes |
| `SIMCONS-CAND-002` | `watbal_process_kernels.f90` | 2426 | `wbk01..wbk09`, `wbk19a_*`, closure diagnostics | `src/watbal.for`, `src/watbal_hourly.for` | mixed core kernels + policy overlays + probe controls |
| `SIMCONS-CAND-003` | `watbal_daily_adapter.f90` | 341 | `wb_daily_adapter_execute` | `src/watbal.for` | daily adapter scaffold around shared kernels |
| `SIMCONS-CAND-004` | `watbal_hourly_adapter.f90` | 619 | `wb_hourly_adapter_execute` | `src/watbal_hourly.for` | hourly adapter scaffold with requested/effective mode surfaces |
| `SIMCONS-CAND-005` | `watbal_closure_guard.f90` | 371 | closure residual function + runtime closure guard checks | `src/watbal.for`, `src/watbal_hourly.for` | explicit 1.0 mm residual threshold logic |
| `SIMCONS-CAND-006` | `watbal_route_types.f90` | 902 | route/impoundment type and status catalogs | `src/route.for`, `src/wsh*.for`, `src/imp*.for` | watershed/channel/impoundment lane schema; out of immediate hillslope lane scope |
| `SIMCONS-CAND-007` | `watbal_route_kernels.f90` | 6147 | route/impoundment kernel families (`wbk_route_*`, `wbk_imp_*`) | `src/route.for`, `src/wsh*.for`, `src/imp*.for` | includes many `DEFER_LEGACY`/`legacy_shim_required` pathways and env-controlled authority toggles |
| `SIMCONS-CAND-008` | `hillslope_binary_pass_legacy_adapter.f90` | 971 | `hbp_capture_legacy_day` legacy pass writer bridge | `src/wshpas.for`, `src/cpass*.inc` (pass lineage) | legacy adapter with producer tag `wepp-forest-ps04-legacy-adapter` |

## Policy/overlay and fallback surfaces discovered
- Runtime env-toggle overlays in daily/hourly/process layers:
  - `process_accounting_wbk08_mode`
  - `process_accounting_wbk01_input_probe*`
  - `process_accounting_rain_routing_kernel_probe*`
  - `process_accounting_wbk05_trace*`
  - `process_accounting_wbk05_latk_mode`
- Hourly qcap policy overlay surface:
  - `wbk09_hourly_qcap_policy`
  - `qcap_policy_enabled`, `qcap_soft_gain`, `qcap_soft_frac_max`
- Legacy-defer shim/fallback surfaces in route kernels:
  - `WBK_ROUTE_*_DEFER_LEGACY` message paths
  - `legacy_shim_required = .true.` branches
  - env-controlled D09 authority toggles (`WB34_M5_D09_AUTH`, `WB34_M6_D09_AUTH`, `WB33_M5_D09_AUTHORITY`).

## Ran
- Candidate module discovery:
  - `cd /workdir/wepp-forest/fpm-src && rg --files | sort | rg '(^|/)wbk[^/]*\.(f90|for)$|(^|/)wb[^/]*\.(f90|for)$|watbal|hourly|adapter|policy'`
- Module export and size inventory:
  - `cd /workdir/wepp-forest/fpm-src && wc -l ...`
  - `cd /workdir/wepp-forest/fpm-src && rg -n '^\s*public\s*::' ...`
- Baseline routine anchor checks:
  - `cd /workdir/wepp-forest_260430_baseline && rg -n 'subroutine watbal|call watbal_hourly|subroutine watbal_hourly|ui_run|solwpv' src/watbal.for src/watbal_hourly.for`
  - `cd /workdir/wepp-forest_260430_baseline && rg -n 'subroutine route|subroutine wsh|subroutine imp|call route|call wsh' src/route.for src/wshdrv.for src/wshrun.for src/wshpek.for src/wshcqi.for src/wshirs.for src/wshchr.for src/wshimp.for src/impday.for src/imppol.f90 src/imppow.f90 src/impris.f90`
