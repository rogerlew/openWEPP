# Conflict And Disposition Ledger

Status: queued.
Evidence mode: static scaffold.

## Pre-Code Conflict Check

Static:

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-051` does not authorize production
  `physics_bulk`; it keeps the model candidate-only and opt-in. No direct
  contract contradiction with the 2026-06-25 melt decision was found.
- `SC-SNOWFREEZE-001` lacks the melt-modernization authority requested by the
  operator: shortwave/albedo operands, albedo state, opt-in melt selector,
  no-radiation-tuning rule, and explicit negative-benchmark disposition for
  degree-day snowbench variants.
- `docs/work-packages/20260625-snowdensity-04-offline-adjudication-loop-001/artifacts/worker-handoff.md`
  conflicts with the 2026-06-25 decision by routing next work to
  `dense_slow_melt_v1` runtime opt-in. Treat it as superseded evidence, not
  active authority.
- `docs/work-packages/README.md` must describe the supersession so future agents
  do not follow the stale handoff.
- The current Rust/test melt-term convention stores raw melt as
  `amelt + bmelt + cmelt + dmelt` (`clim05_snow_runtime_kernel_contract.rs`),
  while the operator decision and WEPP Chapter 3 prose write
  `amelt - bmelt + cmelt + dmelt`. The execution package must resolve this as a
  sign/alias convention in `SC-SNOWFREEZE-001` before changing formula code.

## Required Disposition During Execution

- Accept the missing contract authority and resolve it by amendment before code.
- Reject production promotion of `dense_slow_melt_v1`.
- Reject any unreviewed `bmelt` sign flip; require contract-bound alias proof.
- Preserve `SNOWDENSITY-04` artifacts as historical evidence; do not rewrite
  their closeout numbers to fit the new decision.
