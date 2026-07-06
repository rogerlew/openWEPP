# Pre-Implementation Contract Gate (D10B-S3)

Status: executed
Evidence mode: Ran

Recorded BEFORE any production scheme correction (the only production-code
changes so far are diagnostics-class instrumentation, the rev-24-bound
Manning limb + `run_iwagaki_manning` builder, and the TV diagnostic — all
additive, none altering existing scheme behavior; existing tests
unaffected).

## Contract authority

- `SC-OFEROUTE-001` rev 24 landed (see `contract-implementation-evidence.md`):
  scheme form bound (limiter branch, two-sided face dissipation, boundary
  fluxes/ledger, conservative handoff), Case-4 oracle re-anchored, gap rows
  updated. Tolerances PROPOSED (peak 5% rel, `t_peak` 1.5 s, rise 2.0 s at
  the finest sweep grid), ratification deferred to the S4 evidence per the
  package's S2 rule.

## Oracle standing (acceptance instrument)

`cargo test -p openwepp-hillslope-orchestrator --release iwagaki_oracle`:
4/4 PASS — closed-form anchors (steady state `q = vL`, exact rising limb
`q = alpha (v t)^m`), exact conservation (finite-volume residual <= 1e-10),
grid self-convergence (4000 vs 8000 cells: peak < 1%, `t_peak` < 0.5 s),
and cross-validation of the two INDEPENDENT constructions (monotone
finite-volume reference vs Lagrangian characteristics fan with
junction-shock tracking + cutoff rarefaction: peak within 3%, `t_peak`
within 1 s). Converged Case-4 oracle metrics
(`logs/s3-oracle-metrics.json`): peak `~0.00828 m^2/s`, `t_peak ~24.6 s`,
rise 10-90% `~19.65 s`. (Note: the demoted digitized enhanced-WEPP trace —
peak 0.00813, 25.98 s, 20.88 s — lands close to the true entropy solution;
the demotion stands as an authority decision, and the oracle now supplies
the clean-room acceptance the trace could not.)

## Failing baseline (contract-derived tests vs the pre-correction scheme)

`cargo test -p openwepp-hillslope-orchestrator --release d10b_reconciliation`:
**5/5 FAIL**, exactly as the gate requires:

| Test | Pre-correction failure |
|---|---|
| `case4_manning_solver_converges_to_iwagaki_oracle` | peak error 157% -> 220% -> 394% across refinement (DIVERGES; the inverted limiter's `phi < 0` amplifies `Gr` without bound at shocks once the correct Manning law is run) |
| `case4_manning_tvd_dissipation_is_mass_neutral_and_tv_diminishing` | TVD boundary leak `-4.38e-4 m^2` (~0.2% of supply) vs the 1e-12 gate |
| `solver_ledger_books_scheme_actual_boundary_fluxes` | OFE 0 booked inflow `0` vs scheme-actual `0.0145 m^2`: the predictor's forward difference admits `q_0 dt / 2` per step through the TOP face even with zero upstream inflow — a real boundary leak, not only booking |
| `handoff_injection_is_flux_integral_conservative` | left-endpoint injection vs the sampled-series integral |
| `nineteen_ofe_conservation_is_resolution_convergent` | residuals `[9.0%, 17.8%, 53.7%]` GROW under refinement (anti-convergent), reproducing the S0 ledger on the bare-`k_o` fixture |

## Authorized production corrections (S4 slate, per rev 24)

1. `phi()` -> Davis (3.20)/Mingham (31f) branch.
2. Cell-indexed boundary-exempt TVD -> two-sided FACE-BASED dissipation
   (Mingham 31a/31g), zero domain-boundary faces, exact telescoping.
3. Upstream boundary: prescribed-flux BC in BOTH sweeps (top-face flux =
   `q_up` in the predictor as well) so actual injection = `q_up dt`.
4. Downstream boundary: kinematic upwind outflow closure (predictor
   outflow face flux = `q_{n-1}`, replacing the over-discharging linear
   extrapolation ghost); ledger books the actual
   `0.5 (q_{n-1}^n + q_pred_{n-1}) dt`.
5. Ledger: booked = actual for inflow/outflow; positivity-clamp stage
   contributions at half weight.
6. Cascade handoff: exact piecewise-linear flux-integral injection
   (optional integral closure on `Forcing`; cascade supplies it).

Seven-gate check: reproduction (S0 + this baseline), mechanism (named per
item), ownership (all in `ofe_routing` solver/cascade), authority (rev 24
bindings), safety (no guard loosened; fail-closed paths untouched),
testability (the five failing tests above), validation (S4 sweeps +
tolerance ratification). GATE: PASS — proceed to S4.
