# PERFHO01 Verdict

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** (timings, manifests, GDB stack samples) + **Static** (source-path interpretation)

## Verdict

The H2637 high-OFE cost is **not acceptable as-is** for the
subprocess-per-hillslope architecture.

Current measured same-binary H2637 timing is `978.55 s` for one 19-OFE,
34-simulation-year hillslope. Prior FARPOINT01 legacy timing for the same
substrate is about `9-12 s`. The current openWEPP cost remains in the observed
`~80-110x` gap regime.

This is not primarily an output-write problem. The H2637 full run is almost
entirely user CPU (`977.99 user s`, `0.42 sys s`), and GDB stack sampling during
the 19-OFE daily loop found no samples in Parquet writers. The dominant sampled
cost is symbol-keyed runtime-surface metadata/control overhead inside the
per-OFE daily scheduler path.

## Named Hot Path

Primary hot path:

`openwepp_runner::hillslope::scheduler_trace::execute_persistent_scheduler_kernel_lifecycle`
and the callees it drives:

- `BTreeMap` clone/insert/remove/lookup of symbol-keyed runtime surfaces.
- `openwepp_kernel_contract::lib_mod::writeback::{apply_kernel_writeback,evaluate_kernel_writeback,collect_field_violations}`.
- Hydrology support helpers that repeatedly format and compare symbol keys:
  `require_state_scalar_for_symbol`, `hourly_symbol`,
  `compute_active_frost_coupling`, `require_shadow_fine_state_domains`, and
  decomposition guard scans.

Sample-backed split:

- Runtime-surface map lifecycle/access: `8/15` samples (`53.3%`).
- Writeback validation/sort/allocation/detail construction: `3/15` samples
  (`20.0%`).
- Hydrology/frost guard and symbol formatting overhead outside writeback:
  `4/15` samples (`26.7%`).

## Recommendation

Open a follow-on optimization package:

`PERFOPT01-runtime-surface-map-churn-and-writeback-validation`

Scope for that package:

1. Replace or bypass repeated success-path `BTreeMap<BoundarySymbol, BoundaryValue>`
   clone/insert/remove/lookup in the per-OFE daily loop with a deterministic,
   stable-index representation or reusable per-lane scratch state.
2. Make writeback validation detail collection lazy where the detail is needed
   only for failures, while preserving the same fail-closed errors and messages
   when a violation occurs.
3. Remove repeated symbol-string formatting from success-path hydrology support
   helpers where the symbol set is statically known.
4. Re-run PERFHO01's H2637 and 2-5 OFE ladder after the change, then compare
   output bytes or exact parquet row values against the pre-optimization baseline.

Expected gain bound:

- The GDB sample upper bound for the named success-path metadata/validation work
  is `15/15` samples because all samples were in that broad class, but the
  directly named runtime-surface map and writeback components account for
  `11/15` samples (`73.3%`).
- If PERFOPT01 removes half to two-thirds of that sampled overhead, the expected
  practical gain is roughly `1.5-2.5x`.
- The Amdahl upper bound for completely eliminating the `73.3%` named component
  is about `3.75x`, so this follow-on should be treated as the first necessary
  optimization, not as a guaranteed full closure of the `80-110x` gap.

## Determinism Constraint

Any optimization follow-on must preserve openWEPP's within-config determinism:

- Same target, same inputs, same seed: bit-reproducible outputs.
- No reordering of floating-point reductions or per-OFE transfer sequencing.
- No weakening of WB13 conservation gates, writeback validation, or fail-closed
  behavior.
- Diagnostic/error detail may be made lazy only if the emitted detail is still
  available and equivalent on the failure path.

This constraint follows `docs/numerics/README.md` and the package boundary:
PERFHO01 is characterization only and lands no production or contract change.

## Closure Notes

- Production code edits: none.
- Contract edits: none.
- Added reproducibility artifacts only: `artifacts/runfiles/*.run`.
- Required PERFHO01 deliverables are present:
  - `artifacts/perf-profile-evidence.md`
  - `artifacts/perf-scaling-curve.md`
  - `artifacts/perfho01-verdict.md`
