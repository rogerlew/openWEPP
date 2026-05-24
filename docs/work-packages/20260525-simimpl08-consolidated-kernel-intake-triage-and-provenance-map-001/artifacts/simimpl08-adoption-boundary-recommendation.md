# simimpl08 adoption boundary recommendation

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Objective of this recommendation is to define a bounded, implementation-ready
  intake set for `SIMIMPL09` that satisfies `INV-WATBAL-021` and
  `INV-SYSTEM-021`.
- This is not production integration; it is a contract-governed intake boundary
  for subsequent implementation packages.

## Recommended SIMIMPL09 intake allow-list (`adopt` only)
1. `watbal_process_types` core state/flux/closure/status model.
2. `watbal_process_kernels` bounded subset:
   - `wbk01` through `wbk08` and closure diagnostics,
   - exclude `wbk09` qcap overlay,
   - exclude probe/trace runtime controls.
3. `watbal_daily_adapter` structural orchestration pattern only.
4. `watbal_hourly_adapter` structural orchestration pattern with explicit
   requested/effective mode lane closure.
5. `watbal_closure_guard` residual accounting pattern, translated to typed
   error/reporting semantics (no `error stop` runtime exits in openWEPP).

## Required exclusions for SIMIMPL09 (`reject`)
- `wbk09_hourly_qcap_policy` and qcap soft-limiter parameters.
- runtime env behavior toggles and identity-mode bypass controls.
- probe/trace CSV instrumentation controls.
- legacy shim/defer fallbacks and env-driven authority toggles.

## Deferred backlog (`defer`)
- `wbk19a_*` runoff/runon/depression helper family.
- `watbal_route_types` and `watbal_route_kernels` route/impoundment modules.
- `hillslope_binary_pass_legacy_adapter` legacy pass bridge.

## Preconditions before any adopt-surface implementation
1. Maintain contract-first order from SIMIMPL03/SIMIMPL04.
2. Preserve SIMMODE closure from SIMIMPL07 (`requested/effective/lane` tuple).
3. Encode typed guards for every intake surface; no silent fallback wrappers.
4. Extend contract-derived tests for admitted adopt surfaces before production
   path activation.

## Decision
- SIMIMPL08 adoption boundary recommendation: `GO`.
- Bounded intake set is explicit and suitable as SIMIMPL09 planning authority.
