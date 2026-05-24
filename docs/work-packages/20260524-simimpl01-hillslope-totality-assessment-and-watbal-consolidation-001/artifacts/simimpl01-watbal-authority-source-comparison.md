# Simimpl01 watbal authority source comparison

Status: phase-d-complete
Evidence mode: Static + Ran

## Static
- Comparator/provenance authority target A:
  - `/workdir/wepp-forest_260430_baseline` at commit
    `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Consolidation architecture target B:
  - `/workdir/wepp-forest` at commit
    `e926b5897ced390f7340d4760ad140429b7e2864`
  - kernel/adapters under `/workdir/wepp-forest/fpm-src/`

## Ran
- Commit verification:
  - `git -C /workdir/wepp-forest_260430_baseline rev-parse HEAD`
  - `git -C /workdir/wepp-forest rev-parse HEAD`
- Wrapper branch verification:
  - `rg -n "call watbal_hourly" /workdir/wepp-forest_260430_baseline/src/watbal.for /workdir/wepp-forest/src/watbal.for`
- Consolidated architecture probes:
  - `rg -n "requested_mode|effective_mode|wbk09_hourly_qcap_policy" /workdir/wepp-forest/fpm-src/watbal_daily_adapter.f90 /workdir/wepp-forest/fpm-src/watbal_hourly_adapter.f90 /workdir/wepp-forest/fpm-src/watbal_process_kernels.f90`

## Comparison matrix
| Dimension | Baseline authority (`wepp-forest_260430_baseline`) | Consolidated candidate (`wepp-forest/fpm-src`) | SIMIMPL01 decision |
|---|---|---|---|
| Legacy parity/comparator anchor | Stable pinned authority used by openWEPP governance | Not pinned parity comparator anchor | Keep baseline as comparator authority |
| Daily/hourly branch semantics | Explicit wrapper split with `watbal -> watbal_hourly` dispatch | Explicit adapter mode context (`requested_mode`, `effective_mode`) | Use baseline for branch semantics; use candidate for adapter modeling |
| Kernel decomposition | Split/legacy routine style | Shared-kernel family (`wbk*`) with adapters | Prefer candidate decomposition pattern for implementation design |
| Policy overlays | Legacy wrappers do not include new qcap policy module | Includes hourly policy layer (`wbk09_hourly_qcap_policy`) | Require explicit contract triage before any policy intake |
| Adoption risk | Lower for comparator equivalence | Higher if imported wholesale due mixed migration layers | Selective intake only; no wholesale import |

## Final authority posture for SIMIMPL01 follow-ons
1. Baseline physics/comparator authority remains pinned to:
- `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

2. Consolidation architecture authority is selectively admissible from:
- `/workdir/wepp-forest/fpm-src/watbal_process_kernels.f90`
- `/workdir/wepp-forest/fpm-src/watbal_daily_adapter.f90`
- `/workdir/wepp-forest/fpm-src/watbal_hourly_adapter.f90`

3. Non-authorized by default until contract-first disposition:
- Hourly qcap soft-limiter and related clamp/policy behavior in candidate sources.

## Queue impact
- This decision is encoded into SIMIMPL01 queue dependencies:
  - `simimpl03` (contract authority amendments)
  - `simimpl08` (consolidated kernel intake triage and provenance map)
  - `simimpl09+` (hourly lane and coupling closure only after triage)
