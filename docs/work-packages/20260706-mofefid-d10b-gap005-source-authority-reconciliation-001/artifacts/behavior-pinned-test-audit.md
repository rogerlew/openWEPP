# Behavior-Pinned Test Audit (D10B)

Status: executed
Evidence mode: Ran (each disposition verified by the passing post-change suite)

Rule applied: a test encoding a LAW (conservation, domain guards, physical
signatures) is preserved; a test PINNING the pre-rev-24 defect signature or
sampler mechanics is updated/replaced with the rev-25 semantics, each
individually dispositioned here.

| Test | Pre-D10B role | Disposition |
|---|---|---|
| `dval::case4_iwagaki_peak_is_resolution_sensitive_boundary` | PINNED THE DEFECT (asserted peak shift > 20% between resolutions) | REPLACED by `case4_iwagaki_peak_is_resolution_stable_after_rev24` (shift < 10%; measured 2.8%) — the defect pin inverted into a fix pin |
| `dval::case4_iwagaki_sampling_correction_exposes_timing_boundary` | Pinned the D8 timing-boundary window (t_peak 34..40 s) on the `k_o=200` diagnostic | REPLACED by `case4_iwagaki_ko_diagnostic_remains_coherent` (order-of-magnitude + substep/sampled coherence; the `k_o` config is comparator-flag diagnostic, not acceptance, per rev 24) |
| `kinematic_wave::hydrograph_sampling_interpolates_within_large_solver_steps` | Pinned the D8-2 interpolating point-sampler mechanics | REPLACED by `hydrograph_bins_are_conservative_and_rise_at_bin_scale` (bin series carries booked outflow EXACTLY + rising limb visible at bin scale) — strictly stronger conservation semantics |
| `kinematic_wave::case1_bare_surface_reaches_steady_state_and_conserves_mass` | Read steady q from the instantaneous last sample (2%) | UPDATED: authoritative steady measure = BOOKED discharge (0.5% band; measured 0.03%); sampled tail keeps a 6% ripple band (boundary ripple characterized in `iwagaki-case4-evidence.md`) |
| `cascade::case3_vegetated_strip_backs_up_more_water_than_bare` | Read steady q from single last samples | UPDATED: tail-mean of the bin-flux hydrograph (ripple-robust); the physical signature assertions (mass-balance discharge equality, veg backs up more water) unchanged |
| `dval::case2_underprediction_is_ks_operand_limited` | Ks-sensitivity bounds (0.90..0.95 adjusted ratio) | UNCHANGED and passing post-correction (adjusted ratio back in band after the celerity/boundary corrections settled) |
| `kinematic_wave::conservation_residual_converges_with_resolution` | Residual shrinks with resolution (discretization-only) | UNCHANGED and passing (booked-equals-actual keeps a resolution-dependent measurable > 0 via storage timing; still convergent) |
| D8 low-`k_o` regression, friction/regime/gating tests, fail-closed domain tests, seam fixtures A/B, infiltration coupling | Laws | UNCHANGED, all passing |

Evidence: `cargo test -p openwepp-hillslope-orchestrator --release ofe_routing`
= 61 passed / 0 failed. D10's rejected limiter-flip trial is superseded: it
was judged against the demoted digitized-trace oracle and against the two
defect-pinning tests dispositioned above.
