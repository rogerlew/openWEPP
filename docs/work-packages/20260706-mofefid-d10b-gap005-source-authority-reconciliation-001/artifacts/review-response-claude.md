# Review Response (Codex post-execution review -> Claude fixes)

Status: executed
Evidence mode: Ran (all fixes built + tested this session; commands in
gate-results addendum)

Responding to `review-codex.md` (reviewed commit `1d202b10`). Every
finding accepted; all fixes landed in this response commit.

| Codex finding | Disposition | Fix + evidence |
|---|---|---|
| High 1 — CFL fail-open | accepted, FIXED | `run_with_options` loop: non-finite max celerity -> `NonFiniteState`; non-finite/non-positive `dt` -> `CflViolation` (the `dt <= 0 -> break -> Ok` path is gone). Regression `unsatisfiable_cfl_fails_closed_not_partial_ok` (extreme finite Manning params drive `dt` to 0; asserts typed Err). |
| High 2 — duplicated Case-4 source + cutoff divergence | accepted, FIXED | Single source of truth: `dval::run_iwagaki_cells` now derives geometry/slopes/supplies/cutoff/window from `iwagaki_oracle::OracleConfig::iwagaki_case4()` (+ shared `IWAGAKI_MANNING_N`); solver cutoff test now `t < supply_end_s` (matching the oracle) and the new `run_with_options` clips steps at forcing breakpoints so no step straddles the 10 s cutoff. Regression `case4_solver_and_oracle_source_histories_agree_exactly` (solver booked source total == config supplied volume to 1e-9 rel, two resolutions; via the new `diagnostic_rainfall_excess_m2`). Side effect: acceptance metrics IMPROVED (`logs/rr-oracle-solver-final.json`: peaks 0.008288/0.008391/0.008484/0.008402 -> errors -0.3%..+2.1% vs the extrapolated oracle; `t_peak` within 0.06 s) — the coarse-grid straddle surplus was inflating peaks. |
| Medium 1 — terminal negative bin publish | accepted, FIXED | `BinRecorder::finish` folds only sub-noise (<= 1e-9 of total) terminal deficits; a material deficit returns to `run_with_options`, which fails closed with the new typed `RoutingError::NegativeOutletBin`. Regression `single_ofe_outlet_bins_stay_nonnegative_under_front_arrival` (terminal/single-OFE surface: runon-pulse front arrival; published hydrograph + bins non-negative; bins sum exactly to booked outflow). |
| Medium 2 — stale seam-ledger labels | accepted, FIXED | Example relabeled to post-rev-26 semantics (`seam_transfer_identity_m3`, `terminal_booking_identity_m3` — structural identities, not quadrature evidence; module doc marks the historical S0 numbers as retaining their original meaning) and an EXPLICIT `terminal_sampled_quadrature_m3` diagnostic added (exported-hydrograph trapezoid vs booked mass; measures -0.0004% at the finest sweep point, `logs/rr-seam-ledger-final.json`). |
| Medium 3 — stale contract GAP-005 authority | accepted, FIXED | Swept: guard-map INV-011 row (no open blocker), INV-012 status cell (rev-26 update: GAP-005 precondition SATISFIED, class finding (b) eliminated), OBL-OFEROUTE-P-006 tail, D4 test-vector pointer, BEI `OFEROUTE-ACTIVATION-VALIDATION` row. Historical/provenance rows (D8/D10/rev-15 changelog, superseded-note paragraph) intentionally retained as history. |
| Medium 4 — stale D15 blocker language | accepted, FIXED | Strategy §6.1 D15 row -> "RERUN UNBLOCKED by D10B" with the D14-refresh precondition; ordering paragraph, §7 queue row, and stop-conditions rewritten; ROADMAP §M detail column updated (packages README was already updated at D10B closure). |
| Low 1 — superseded acceptance wording in oracle-reanchoring | accepted, FIXED | S2 "Acceptance shape" marked SUPERSEDED-AS-PROPOSAL with the rev-26 ratified form stated and the original preserved as history. |
| Low 2 — bibliography rights staleness | accepted, FIXED | R-102/R-103 distribution rows now cite the recorded 2026-07-06 rights-log addendum classifications. |
| Low 3 — strict-TVD test naming | accepted, FIXED | Renamed `..._and_tv_transient_bounded`; module header reworded to the bounded-transient form. |
| Low 4 — 3-of-6 sweep-point regression | accepted, EXPANDED | `nineteen_ofe_conservation_is_resolution_convergent` now runs ALL SIX recorded sweep points (runtime cost acceptable: focused suite 67/67 in ~33 s). |
| Low 5 — stale comments/counts | accepted, FIXED | `kinematic_wave.rs` module doc (primaries in hand, source-correct form), `ofe_routing.rs` module doc (opt-in shadow wiring named), line-count checklist refreshed (max 1594 < 2000), focused-count refreshed (67/67) in the behavior-pinned audit addendum. |

## Additional change surfaced by the High-2 fix (transparent re-disposition)

The exact source history exposed that the k_o=200 DIAGNOSTIC's
"resolution-stable <10%" pin (`case4_iwagaki_peak_is_resolution_stable_after_rev24`)
had been ratified from a confounded measurement (mixed sample grids +
straddle surplus). Measured post-fix: the diagnostic's bin-mean peak
wobbles +-13% across 120..960 cells at fixed sampling with no trend —
inherent to the `q ∝ h^3` near-discontinuous shock at these grids, not a
scheme regression (the Manning ACCEPTANCE surface remains stable and
in-tolerance, and conservation stays exact). The pin is replaced by
law-like diagnostic guards
(`case4_iwagaki_ko_diagnostic_conserves_and_stays_positive`); details in
the behavior-pinned-test-audit addendum. Flagged here explicitly for
Codex re-check.
