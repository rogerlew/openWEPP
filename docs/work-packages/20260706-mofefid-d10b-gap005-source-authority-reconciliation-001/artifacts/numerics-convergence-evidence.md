# Numerics Convergence Evidence (D10B)

Status: executed
Evidence mode: Ran

## Oracle self-evidence

`cargo test -p openwepp-hillslope-orchestrator --release iwagaki_oracle`
(4/4 PASS):

- `upwind_single_reach_matches_closed_form`: steady `q = vL` within 0.5%,
  exact rising limb `q = alpha (v t)^m` within 2% at `t_c/2`,
  conservation <= 1e-10 (Iwagaki experiment-(A) configuration).
- `upwind_case4_conserves_and_peaks_after_cutoff`: exact conservation
  (<= 1e-10), post-cutoff in-window peak.
- `upwind_case4_self_convergence`: 4000 vs 8000 cells — peak < 1%,
  `t_peak` < 0.5 s.
- `characteristics_fan_cross_validates_upwind_reference`: the two
  INDEPENDENT constructions (monotone FV vs Lagrangian characteristics
  with Rankine-Hugoniot shock tracking + cutoff rarefaction fan) agree:
  peak < 3%, `t_peak` < 1 s; fan mass residual 7.0e-4; junction shocks
  tracked (`shocks_tracked = 1` merged front).

Upwind grid sequence (`logs/s3-oracle-metrics.json`): peaks
0.00814291 / 0.00820783 / 0.00825080 / 0.00827536 at 2000/4000/8000/16000
cells; refinement ratios 0.66 / 0.57 (consistent first-order-at-shock
family); Richardson limit ~0.00831.

## Solver convergence ladder (rev-24 corrected scheme)

`logs/s4-oracle-solver-final.json`: 120/240/480/960 cells with
proportionally refined `(sample_dt, max_dt)` — peak errors vs the
extrapolated oracle +1.3/+1.8/+2.6/+1.3%, `t_peak` errors <= 0.09 s,
rise errors <= 0.11 s, max Courant 0.80 at every grid (true-celerity CFL).
Verdict: CONVERGED within the ratified tolerances with a bounded,
non-diverging shock-peak wobble; supports CORRECTION acceptance (not
hold).

## Multi-OFE resolution sweep

`logs/s4-seam-ledger-final.json`: the 19-OFE steep cascade's clamp-adjusted
conservation residual is IDENTICALLY ZERO (decomposition gap <= 3.4e-14,
scheme identity <= 2e-12) at every `(sample_dt, max_dt)` sweep point,
including the three shadow-recorded points (900,300)/(900,120)/(120,300)
that measured 6.0%/10.0%/22.1% pre-correction. Strictly stronger than the
resolution-convergence the contract requires.
