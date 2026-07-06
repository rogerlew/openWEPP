# Seam-Decomposed Conservation Ledger (D10B-S0)

Status: executed
Evidence mode: Ran (all numbers below are from executed runs this package)

## Baseline reproduction (Case 4 + H2637 surface)

- Case-4 D10 baselines reproduce EXACTLY: `NS_trace=0.2626773293282393`,
  peak ratio `0.836598358891877`, sampled `t_peak=37.0 s` at
  `(cells=120, sample_dt=1.0, max_dt=0.5)`; `NS_trace=0.1932963706` at
  `(240, 0.25, 0.25)`; `NS_trace=0.1012443141` at `(480, 0.125, 0.125)`.
  Logs: `logs/s0-case4-*.log`. Commands identical to the D10
  `command-log.json` rows.
- The D10-era H2637 executed-vector shadow test
  (`h2637_executed_vector_shadow_on_off`) NO LONGER EXISTS: D11 rev-20 made
  `OPENWEPP_LANED_SHADOW=1` fail closed without native
  `routing_coefficients`, and the surviving test on the legacy H2637
  fixture is `h2637_legacy_shadow_fails_closed_without_routing_coefficients`
  (Ran: `cargo nextest list -p openwepp --test laned_shadow_h2637`). The
  resolution-sensitivity class is instead reproduced below on an
  H2637-SHAPED cascade fixture at the exact recorded sweep points, using
  the same `run_cascade` path and the shadow's cells/OFE working
  resolution (10), per `laned_shadow.rs` constants and its recorded sweep
  note (`(900,300) -> 6.0%`, `(900,120) -> 10.0%`, `(120,300) -> 22.1%`).

## Instrumentation added (diagnostics class, additive)

- `CascadeResult.per_ofe_solver_mass: Vec<MassBalance>` — per-OFE solver
  ledgers exposed (`cascade.rs`).
- `MassBalance` gains three scheme-actual diagnostic terms
  (`kinematic_wave.rs`): `scheme_inflow_m2` (`0.5 (q_up + q_0) dt` — the
  discrete scheme's true upstream injection), `scheme_outflow_m2`
  (`0.5 (q_ghost + q_pred_out) dt` — the true downstream discharge through
  the predictor's extrapolated ghost and the corrector's outlet flux), and
  `tvd_boundary_leak_m2` (the TVD term's non-telescoping boundary
  remainder `[gr_{n-2} Δh_top - gr_0 Δh_bottom] dx`).
- Harness: `examples/cascade_seam_ledger.rs` — 19-OFE steep (0.25..0.61)
  H2637-shaped cascade, 20 m OFEs, 10 cells/OFE, hourly-pulse source
  (2/6/2 mm over hours 6-8), window = last active hour + 6 h drain (the
  shadow's clip rule), swept over `(sample_dt, max_dt)`.

## Decomposition identity

The cascade conservation residual decomposes EXACTLY (gap <= 6e-14 m^3 on
every sweep point) as:

    residual = ofe_internal + seam_sampling + seam_injection + terminal_quadrature

Results (`logs/s0-seam-ledger-decomposed.json`):

| (sample_dt, max_dt) | residual_rel | ofe_internal | seam_sampling | seam_injection | terminal_quad | inflow_booking | outflow_booking | tvd_leak |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| (900, 300) | 9.0% | 14.53 | -4.01 | 0.21 | -0.52 | -5.50 | -18.13 | -1.89 |
| (900, 120) | 7.0% | 14.80 | -6.65 | 0.06 | -0.27 | -5.52 | -18.39 | -1.91 |
| (120, 300) | 20.2% | 20.59 | 1.71 | 0.56 | 0.21 | -4.68 | -21.00 | -4.16 |
| (120, 120) | 17.8% | 19.22 | 0.13 | 0.47 | 0.43 | -4.78 | -20.13 | -3.78 |
| (60, 30) | 35.9% | 27.88 | 11.64 | 1.20 | 0.18 | -3.66 | -23.27 | -7.89 |
| (15, 5) | 53.7% | 51.77 | 1.19 | 5.33 | 2.93 | -0.67 | -31.65 | -12.42 |

(All volumes m^3 against ~114 m^3 of source. `ofe_internal` further
decomposes as `inflow_booking - outflow_booking - tvd_leak +
scheme_identity`; the scheme-identity residual closes to <= 0.4% of source
at the shadow's operating resolutions and leaves ~14% of the finest-point
residual unattributed — candidate: positivity-clamp stage double-booking,
see Mechanism 5.)

## Named mechanisms (evidence-ranked)

1. **Outflow ghost over-discharge (DOMINANT).** The predictor's
   linear-extrapolation downstream ghost (`kinematic_wave.rs`
   `2 q_{n-1} - q_{n-2}`) makes the scheme discharge 18-32 m^3 (16-28% of
   source) MORE than the committed-state trapezoid the ledger books. This
   is the "boundary/handoff treatment" surface named by the D10 hold, now
   ranked first by measurement.
2. **TVD boundary-cell exemption leak (ANTI-CONVERGENT ENGINE).** The
   cell-indexed TVD application exempts cells 0 and n-1, so its
   telescoping sum leaves a per-step boundary remainder. Under CFL-active
   stepping `Cr = 0.9` is pinned, so `Cf` saturates at `0.25` and the
   per-step leak does NOT shrink as dt shrinks while the step count grows
   — the residual therefore GROWS under refinement (-1.9 -> -12.4 m^3).
   This explains the recorded dt-non-monotone behavior. The face-based
   two-sided dissipation form (Davis 1984 eqs. 3.16-3.18; the form's
   coefficients live at faces k+1/2) telescopes exactly with zero
   boundary-face flux — the source-faithful Leg-A form and the
   conservative form are the same edit.
3. **Inflow booking mismatch.** The scheme actually injects
   `0.5 (q_up + q_0) dt` (the predictor's forward difference sees cell 0's
   own flux, not the BC; only the corrector sees `q_up`) while the ledger
   books `q_up dt` (-5.5 m^3 at the operating point).
4. **Handoff sampling/injection + terminal quadrature (SECONDARY).**
   Left-endpoint rectangle injection of the interpolated upstream sampled
   hydrograph, sampled-grid volume quadrature at seams and the terminal
   outlet: real, sign-varying, but second-order relative to 1-3 at the
   operating resolutions.
5. **Positivity-clamp stage double-booking (MINOR, suspected).** Stage
   clamps in the predictor/corrector are booked at full weight but enter
   the committed state at half weight (the commit averages the two
   stages); candidate for the unattributed remainder at the finest sweep
   point. To be resolved by the S4 ledger correction and retested.

## Rejected alias formulas (anti-tautology record)

- `total_source_m3 - total_routed_outlet_m3` as "the loss": conflates
  storage still on the hillslope at window end with non-conservation.
- Sampled-hydrograph trapezoid volume as the authoritative outflow: it is
  a MEASUREMENT quadrature of the committed state, not the scheme's
  discharged mass (Mechanism 1 is invisible to it).
- `residual + clamp ~ 0` self-consistency: holds only when the ledger
  books the scheme's actual fluxes; the current ledger does not.

## Consequence for GAP-OFEROUTE-005

The H2637 "conservation" figure is dominated by solver-internal
scheme-vs-ledger flux mismatches and a real, unbooked TVD boundary leak —
NOT by the inter-OFE handoff (the package's carried-in aliasing hypothesis
is DEMOTED to secondary by this measurement). All named mechanisms lie
inside the Lane D numerical-method envelope, and their correction classes
are bound by the conservation hard gate plus the Davis face-form authority
(Leg A) — no Papanicolaou material is needed. The scaffold's Finding
ranking is corrected by this artifact; the package Surprises section
records the reversal.
