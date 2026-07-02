# D8 Execution Report

Evidence classes:

- `Static:` source/code/contract inspection.
- `Ran:` command execution in this package.

## Summary Verdicts

| Item | Verdict | Evidence |
|---|---|---|
| D8-1 | corrected | R-63 SI `I` convention pinned by low-`k_o` regression; negative `I` no longer silently clamped. |
| D8-2 | corrected metric + declared boundary | Solver samples now interpolate to sample times; Case 4 shock peak/timing remains resolution-sensitive and is recorded as `GAP-OFEROUTE-005`. |
| D8-3 | Case 2 operand-limited; Case 3 declared boundary | Case 2 closes with plausible `Ks` sensitivity; Case 3 enhanced trace exceeds recorded rainfall-length ceiling. |
| D8-4 | operand-limited | Routing-only response is fast; slow limb is Green-Ampt operand sensitivity. |

## What Changed

Code:

- `friction.rs`: removed `rainfall_intensity_m_s.max(0.0)` from
  `skin_resistance_shen_li`; active callers already validate forcing. Added
  low-`k_o` SI unit regression and negative-intensity non-normalization test.
- `kinematic_wave.rs`: sampled hydrograph output now linearly interpolates
  between solver step endpoints. Added a regression for sample-time attribution.
  This is a cross-cutting shadow-surface correction: every routing run that
  consumes sampled outlet hydrographs, including D4/D5/D6 validation surfaces
  and cascade handoff interpolation, now receives interpolated values at the
  requested sample time. Conservation ledgers and CFL checks use solver
  internals and are unaffected; the D4/D5/D6/orchestrator suites were rerun
  through the package gates after the correction.
- `dval.rs`: `DvalRun::peak_m2_s` and `time_to_peak_s` now mean sampled
  hydrograph metrics, matching `compare_dval.py`; sub-step peak/time are
  retained as diagnostics. Added Case 2/3/4 and Case 1 attribution tests.
- `dval_case.rs`: stderr summary reports sampled and sub-step diagnostics; Case
  4 accepts optional cell/sample/max-dt diagnostics.

Contract:

- `SC-OFEROUTE-001` rev 9:
  - `INV-OFEROUTE-002` now says D8 closes the local SI `I` convention against
    R-63 but primary coefficient provenance remains `GAP-OFEROUTE-002`.
  - `INV-OFEROUTE-011` records D8 per-case verdicts and zero clean
    reproductions.
  - `GAP-OFEROUTE-001` no longer claims Case 4 confirms limiter fidelity.
  - `GAP-OFEROUTE-005` records the Iwagaki shock numerical boundary.

## Per-Item Closure

### D8-1

Static: R-63 explicitly states rainfall intensity `I` in `m/s` for the Shen &
Li equation and `Re = q/nu`.

Ran: `shen_li_low_ko_vector_pins_si_rainfall_intensity` exercises `I=100 mm/h`
converted to `m/s`, `k_o=1`, `Re=100`. Passing raw `100` as `I` would inflate
the numerator by more than 400x. The regression distinguishes the unit error.

Closure: corrected. D8 closes the local SI convention and removes silent
negative-intensity normalization. It does not claim a primary Shen & Li audit.

### D8-2

Static: Iwagaki forcing remains no-rain lateral supply; `I=0` is correct.

Ran:

- Before D8 sampler correction, Case 4 `k_o=200` reported sampled `t_peak=28 s`
  and rise `20.6 s`.
- After sampler correction, the same run reports sampled `t_peak=37 s` and rise
  `29.4 s`; sub-step peak time is `36.98 s`, reconciled within one sample.
- Refined diagnostic `240 cells / 0.25 s sample / 0.25 s max_dt` gives a peak
  materially different from the default, proving resolution sensitivity.

Closure: the metric discrepancy is corrected; the shock-capture peak/timing
shortfall is a declared boundary in `GAP-OFEROUTE-005`. A separate defect-shaped
follow-on must close shock numerics with TVD primary/source authority,
convergence criteria, and Iwagaki operand bounds.

### D8-3

Static: R-63 equations (4)-(6) match local form/wave/vegetation kernels. The
forcing channels are correct: rainfall drives Green-Ampt and skin `I`; excess
drives routing.

Ran: Case 2 improves from `NS_trace=0.454`, peak ratio `0.747` at `Ks=20 mm/h`
to `NS_trace=0.961`, peak ratio `0.922` at `Ks=10 mm/h`, without friction
kernel changes.

Ran: Case 3 enhanced peak is greater than `I * L` for the recorded rainfall and
length. Even `Ks=0` cannot reach the enhanced peak.

Closure: Case 2 is operand-limited, no friction defect. Case 3 is a declared
comparator-surface/operand boundary.

### D8-4

Static: Case 1 Green-Ampt operands are texture-derived, not measured case
operands.

Ran: routing-only (`Ks=0`) rise is `77.4 s`; default Green-Ampt rise is
`4999.7 s` vs enhanced `3579.9 s`. Therefore routing celerity is not the slow
limb mechanism.

Closure: operand-limited. The transient lag is attributed to Green-Ampt operand
uncertainty, not a routing/cascade defect.

## HOLD Legitimacy Audit

D8 has two declared boundaries:

- **Case 3 comparator surface:** the enhanced trace peak exceeds the recorded
  rainfall-length ceiling under D01 operands. In-envelope correction would be
  tuning kernels toward a physically impossible comparand. Closure needs a
  corrected source/cut-point/geometry authority.
- **Case 4 shock numerics:** after forcing and sampling correction, peak/timing
  are resolution-sensitive. In-envelope alpha-iteration adjustment was tried
  and rejected because it broke existing steady/cascade/conservation tests.
  Closure needs a separate shock numerics DC package with TVD primary/source
  authority and convergence criteria.

Neither boundary is a request for a trace-only follow-up; both name the missing
authority and the next defect-shaped target.

## Supersession Note

Static: the D7 execution report was written before the D8 sampler correction
and says Case 4 timing/rise reproduce near `k_o=200`. That claim is superseded
by this D8 report, `SC-OFEROUTE-001` rev 9, and `GAP-OFEROUTE-005`: after
sample-time interpolation, Case 4 has `NS_trace=0.262677`, sampled
`t_peak=37 s` versus reference `25.98 s`, and remains a shock-capture numerical
boundary rather than a reproduction.
