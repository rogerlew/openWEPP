# Iwagaki Case-4 Evidence (D10B S4, rev-24 acceptance surface)

Status: executed
Evidence mode: Ran (all numbers verbatim from executed runs;
`logs/s3-oracle-metrics.json`, `logs/s4-oracle-solver-final.json`)

## Oracle (acceptance instrument)

Monotone finite-volume reference (exact conservation <= 1e-13 at every
grid) + independent characteristics fan (junction shock tracked, cutoff
rarefaction seeded):

| Construction | Resolution | peak (m^2/s) | t_peak (s) | rise 10-90 (s) |
|---|---|---:|---:|---:|
| upwind | 2000 | 0.00814291 | 24.66 | 19.62 |
| upwind | 4000 | 0.00820783 | 24.64 | 19.64 |
| upwind | 8000 | 0.00825080 | 24.62 | 19.65 |
| upwind | 16000 | 0.00827536 | 24.62 | 19.66 |
| characteristics fan | 1200 particles | 0.00829174 | 24.52 | 19.66 |

Richardson extrapolation of the upwind sequence (observed order ratio
~0.57-0.66): peak limit ~= 0.00831 m^2/s. The two independent
constructions agree within 3% peak / 1 s timing — the oracle's own
correctness evidence (plus closed-form anchors and exact conservation;
`numerics-convergence-evidence.md`).

Comparator-flag note (ADR-0017): the DEMOTED digitized enhanced-WEPP
trace (peak 0.00813, t_peak 25.98 s, rise 20.88 s) lands within ~2% /
~1.4 s of the oracle — close to the true entropy solution, recorded as a
flag observation. The demotion stands as an authority decision (clean
room, transcription-error spec, `k_o` confound).

## Solver under the primary's law (Manning `n = 0.009`, rev-24 config)

| cells | sample_dt (s) | max_dt (s) | peak (m^2/s) | t_peak (s) | rise (s) | max Courant | TV transient (m^2/s) |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 120 | 0.25 | 0.125 | 0.00841955 | 24.6250 | 19.5735 | 0.8022 | 4.770e-4 |
| 240 | 0.125 | 0.0625 | 0.00846005 | 24.5625 | 19.7493 | 0.8023 | 4.436e-4 |
| 480 | 0.0625 | 0.03125 | 0.00852457 | 24.5312 | 19.7694 | 0.8016 | 5.224e-4 |
| 960 | 0.03125 | 0.015625 | 0.00844436 | 24.6406 | 19.6854 | 0.8009 | 4.804e-4 |

Errors vs the extrapolated oracle limit (~0.00831 / 24.62 s / 19.66 s):
peak +1.3% .. +2.6% (bounded wobble, non-diverging), `t_peak` within
0.09 s, rise within 0.11 s. PRE-correction baseline for contrast: peak
error 157% -> 394% DIVERGING, `t_peak` late by ~12 s
(`pre-implementation-contract-gate.md`).

## Ratified tolerances (rev-24 -> ratified rev 25)

- Peak: |solver - extrapolated-oracle| / oracle < **5%** at every ladder
  resolution, non-diverging (finest <= coarsest + 2.5%). Measured: <= 2.6%.
- Sampled `t_peak`: < **1.5 s**. Measured: <= 0.09 s.
- 10-90% rise: < **2.0 s**. Measured: <= 0.11 s.
- Booked-ledger conservation identity: exact (<= 1e-9 relative). Measured:
  <= ~1e-13.
- TV transient (homogeneous steps, uniform-material faces): bounded <
  **1e-3 m^2/s**. Measured 4.4-5.2e-4 (~6% of peak flux, a one-step-scale
  transient at shock formation / boundary-adjacent cells).

Enforced by `ofe_routing::d10b_reconciliation_tests` (5/5 green; failing
5/5 against the pre-rev-24 scheme).

## Residual items (named, bounded, non-blocking)

1. **Strict-TVD gap:** the scheme is essentially-non-oscillatory, not
   strictly TVD — max single-step homogeneous TV(q) increase ~5e-4 m^2/s
   (~6% of peak) at shock formation. The uniform-coefficient Davis/Mingham
   TVD theorem does not cover the variable-coefficient + frozen-alpha +
   boundary-closure variant. Bounded by test; refinement candidate.
   MONITOR SCOPE caveat (review A MINOR-8): the diagnostic measures
   homogeneous steps only, over uniform-material faces only, pre-commit —
   junction-face and forced-step oscillation is outside the monitored
   functional, so "essentially-non-oscillatory" must not be over-read;
   acceptance rests on the independently enforced oracle-convergence and
   conservation surfaces.
2. **Boundary flux ripple:** the one-sided outflow-closure pair supports a
   slow, zero-mean, mass-exact ripple in the instantaneous outlet flux at
   coarse grids (characterized via `examples/steady_probe.rs`: booked
   steady discharge exact to 0.03% at all grids while the raw last-cell
   state wobbles). The exported hydrograph is bin-mean boundary flux,
   which bounds the visible ripple; steady-state acceptance reads the
   booked discharge.
3. **Shock-peak wobble:** +-1.3% grid-to-grid around the oracle limit at
   the shock; inside the ratified peak tolerance.

Surface-pairing note (review B m12): the solver metrics are bin-mean
boundary-flux values while the oracle's are near-instantaneous (0.02 s
recording); the ladder refines `sample_dt` proportionally (0.25 -> 0.03125
s), so the comparison limit is consistent and the mismatch is well inside
the 5% peak tolerance at the acceptance resolutions.
