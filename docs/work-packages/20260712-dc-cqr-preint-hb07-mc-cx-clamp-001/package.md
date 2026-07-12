# DC-CQR-HB07-001 — Muskingum-Cunge `cx` Clamp

Status: `ACTIVE`
Parent: `docs/work-packages/20260712-cqr-preint-hb07-channel-routing-diagnostics-001/package.md`

## Defect

HB-07 review found that `compute_variable_muskingum_cunge_state` silently
replaces computed `cx < -10` with `-10`. `SC-ROUTE-001#INV-ROUTE-022`
explicitly prohibits coefficient clamps, peak clips, empirical damping, and
static/dynamic fallback. Inadmissible dynamic coefficients must fail closed
with `WKERNEL-WS10-CHANNEL-E-003` before recurrence.

## Bounded Correction

- Target production file remains
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`.
- Replace the clamp with the existing typed domain-violation path and exact
  `cx` symbol; do not introduce a replacement threshold or coefficient repair.
- Add a red regression that reaches `cx < -10` and proves the old clamp admits
  the state, followed by the exact typed rejection.
- Add independent reconstruction for fresh/carried `qref`, `c0..c4`, and `q1`,
  exact shape celerity values, missing-state initialization, and the remaining
  HB-07 A–H boundary/error-priority obligations identified by review.
- Re-run focused metrics and the W11C executable consumer. Any instability in
  an existing consumer must be classified against `INV-ROUTE-022`; no clamp or
  fallback may be restored.

## Acceptance

Terminal PASS requires archived red/green evidence, exact typed error identity,
zero eligible diagnostics function below 75%, both fixed rows at CRAP at most
30, focused/full orchestrator tests, W11C consumer, scoped Clippy, formatting,
two independent final reviews, and durable A–H/lineage/metric artifacts.
