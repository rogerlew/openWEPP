# Diagnostic Reproduction

Status: EXECUTED
Evidence mode: Static + Ran.

## Prior Evidence Reused

The immediate predecessor package localized the WA selected-cohort blocker to
active-router positivity-clamp amplification:

- `baseline_fixed10` and `dx20` completed day 1122 with source
  `4889.32122696 m3`, clamp `25.4271138404 m3`, outlet
  `4901.81626622 m3`, storage `12.932074575 m3`, and relative residual
  `1.38931116172e-15`.
- `dx5` completed day 1122 with source `4889.32122696 m3`, clamp
  `554.609197907 m3`, outlet `5431.03005677 m3`, storage
  `12.9003680932 m3`, and relative residual `6.96835462392e-16`.
- `dx2p5` failed day 1122 at `laned_active_day_cascade_residual` after
  clamp/storage operands grew to `857952342.299 m3` / `857412328.397 m3`.
- `dx1p25` failed day 1122 at the same guard after clamp/storage operands
  grew to `190055300.17 m3` / `45708292.5541 m3`.
- Completed-rung magnitude attribution found the dominant retained-default
  event at day 1418 lane 5: `baseline_fixed10` clamp
  `145554.778351 m3` on local source `2914.7262742 m3`; `dx10` clamp
  `457540698.111 m3`; `dx5` clamp `27708994361.1 m3`.
- The active hydrology rows in `H1.wat.parquet` were invariant across inspected
  rungs/days, so the defect is router-internal, not a source-producer change.

Source artifacts:

- `../20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/day1122-reproduction.md`
- `../20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/magnitude-attribution.md`
- `../20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/numerics-adjudication.md`

## Rejected Core-Solver Candidate

Ran:

```text
cargo nextest run -p openwepp-hillslope-orchestrator --lib \
  material_negative_predictor_stage_fails_closed_without_clamp_booking \
  case1_bare_surface_reaches_steady_state_and_conserves_mass \
  case4_lateral_pulse_conserves_mass_and_captures_front \
  conservation_residual_converges_with_resolution
```

An experimental, non-retained edit made material predictor/corrector negative
stages return `NegativeDepth`. The direct guard vector passed, but the retained
Case-4 shock smoke vector failed immediately. A retry-halving variant was then
tried; it preserved the fail-closed stage rule but ran the Case-4 vector for
`156.899 s` before manual interrupt, with the Case-4 oracle path still not
complete.

Decision: the core TVD-MacCormack stage semantics cannot be changed safely in
this package. A positivity-preserving or monotone solver correction needs its
own design/evidence package. The retained implementation therefore targets the
active publication guard, not the D10B solver core.

## In-Envelope Guard Chosen

The contract-authorized in-envelope closure is an active-publication hard guard:
a routed active day fails closed if booked positivity-clamp injection exceeds
the external source mass injected into the active router for that day.

This prevents clamp-adjusted algebraic closure from publishing WA's material
clamp-mass days while preserving the D10B solver/oracle behavior.
