# Review Agent B (D10B) — numerical methods

Evidence class: Static (hand-derivation over the code + artifacts) + Ran
(one read-only Python fp-witness search). No cargo runs.

Recommendation: **GO-WITH-AMENDMENTS**.

Verified sound by hand: predictor/corrector telescoping and the
booked-equals-actual ledger EXACT in all branches (n=1/i=0/i=n-1, clamp
half-weights); face dissipation telescopes exactly and the r+/r- ratios,
phi, and g match Mingham (31g)/(31f)/Davis (3.17-3.20) verbatim for a
scalar state, with a positivity margin (max face coefficient 0.25);
true-celerity dq/dh sound (truncation ~0.1%, guarded floor correct in
direction); upwind oracle monotone + exactly conservative; the fan's
in-step update is the exact closed-form integral; Rankine-Hugoniot coded
correctly; bin apportioning mass-exact; Richardson mechanics valid.

CRITICAL: none.

MAJOR:
- M1: demonstrated fp zero-progress infinite loop in the bin loops
  (`record_step`, `integrate_bin_series`) for non-dyadic `sample_dt`
  (witness: sample_dt=0.003 at t=0.147) — latent (all current configs
  dyadic/integer) but caller-reachable; must force index progress.
- M2: negative booked outflow at dry-outlet front arrival on a RUNON-ONLY
  OFE (`pred_out_face = -q_{n-2}`, frozen outlet alpha 0) -> negative
  bins -> downstream `InvalidForcing` spurious cascade abort + negative
  exported bin-mean discharge. Ledger conservation itself remains exact;
  the bin/injection surface needs an adjudicated non-negative
  redistribution that preserves the exact total.
- M3: final-partial-bin under-injection when `end_time_s` is not a
  multiple of `sample_dt_s` (bin spread at mass/bin_dt over the full
  width; downstream never integrates past end_time) — the "exact at ANY
  sample resolution" claim is currently conditional on divisibility (all
  D10B evidence configs divide evenly, so the measured exactness stands).
- M4: INV-011 contract text ("monotone error decrease"; "TV non-increase")
  not amended to the ratified bounded-wobble/bounded-transient forms the
  tests enforce (same surface as Review A MAJOR-2).

MINOR: m5 booked-equals-actual test is a tautological drift guard
post-rev-24 (real enforcement = residual-exactness tests); m6 stale
`scheme_inflow_m2` doc; m7 the oracle's cutoff-rarefaction rationale is
misplaced (boundary seeding is the actual trailing-limb carrier; the fan
as coded is a near-no-op) — results unaffected (cross-validation stands);
m8 `is_break` exact f64 equality fragile against computed per-cell params;
m9 `max_courant` evidence structurally <= CFL_TARGET (same estimate as dt
selection; intra-step growth unmonitored, 10% headroom covers); m10/m11
comment drift (piecewise-linear wording; monotone-convergence claim scope);
m12 bin-mean vs instantaneous peak surface note for the evidence.

Nothing found invalidates the D10B acceptance evidence (gathered on
aligned configurations where M1-M3 cannot fire).
