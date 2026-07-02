# Review Disposition

Review: `review-codex.md` (`a2ca5747`, hold). All four blockers **accepted
and fixed**:

| # | Blocker | Action |
|---|---|---|
| D4-CX-001 | Non-finite forcing / invalid parameters not fail-closed (NaN forcing returned Ok) | Added `RoutingError::NonFiniteForcing` (checked per step on rainfall excess, intensity, upstream inflow) and `InvalidCellParameter` (per-cell domain validation at run start: finite, non-negative, `lambda <= 1`). Tests: `nan_forcing_fails_closed`, `nan_inflow_fails_closed`, `invalid_cell_parameter_fails_closed`. |
| D4-CX-002 | Predictor/corrector depth clamps hidden from the ledger, so `positivity_clamp_m2 == 0` didn't prove no clamp mass | Clamp accumulation now covers **all three** clamp sites (predictor, corrector, committed `h_next`). The convergence test's `clamp == 0` assertion therefore proves no clamping fired anywhere, at either resolution — the discretization-only conservation claim is now sound. |
| D4-CX-003 | `sample_dt_s <= 0` could hang the solver | `run()` fails closed on non-finite / non-positive `sample_dt_s` (and re-validates `cell_length`, `end_time`, `max_dt` for finiteness). Test: `nonpositive_sample_dt_fails_closed_not_hang`. |
| D4-CX-004 | Contract assigned Ef evidence to the D4 single-OFE solver row, but D4 defers Ef | `SC-OFEROUTE-001` rev 2: the `OFEROUTE-KWE-TVD-SOLVER` BEI note now states D4's evidence surface is PHYSICS validation (not case-Ef); `INV-OFEROUTE-011` explicitly assigns the Ef acceptance to the D-val stage at D5/integration (Cases 1-3 need SC-RUNOFFPART infiltration; Case 4 needs digitized observed data). No invariant/physics change. |

Post-fix gates (Ran): `ofe_routing` 17/17 (13 + 4 fail-closed); full
orchestrator suite green; clippy `-D warnings` 0; fmt clean; BEI
PASS-DEFERRED; authority guards PASS; solver still shadow-first.

## Re-check disposition (Codex re-review `f7850cc4`)

Two accepted at re-check (clamp ledger-complete; `sample_dt_s<=0` fails
closed). Two partial closures completed:

| # | Residual | Action |
|---|---|---|
| CX-001 | Finite negative excess/inflow/intensity silently normalized to zero | `RoutingError::NonFiniteForcing` renamed to `InvalidForcing`; `is_valid_forcing(x) = finite && x >= 0`; all three forcing channels now fail closed on negative OR non-finite, with the `.max(0.0)` normalization removed (values are validated non-negative). Test `negative_forcing_fails_closed_not_zeroed` (negative excess and negative inflow). |
| CX-004 | Test-Vector Obligations "Single-OFE solver" row still assigned `Ef` evidence to D4 | Row rewritten to D4 physics vectors (conservation/CFL/steady-state/shock/fail-closed); a new "D-val Ef acceptance" row assigns the `Ef` reproduction to D5/integration (infiltration-coupled forcing + digitized observed series). The `INV-OFEROUTE-011` row and `OFEROUTE-KWE-TVD-SOLVER` BEI note (rev 2) already carried the stage boundary; the obligations table now matches. |

Post-fix gates (Ran): `ofe_routing` 18/18; orchestrator 166/166; clippy
`-D warnings` 0; fmt clean; BEI PASS-DEFERRED; authority guards PASS.
