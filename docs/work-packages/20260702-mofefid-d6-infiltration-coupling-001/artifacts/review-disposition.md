# Review Disposition (Codex D6 review `4c1c6774`, hold)

All four findings **accepted and fixed**:

| # | Finding | Action |
|---|---|---|
| CX-D6-001 | Invalid rainfall/substep silently normalized or skipped | `green_ampt_excess_hyetograph` now returns `Result`: fails closed on invalid soil (`InvalidCellParameter`), non-finite/non-positive substep (`DegenerateConfiguration`), and malformed rainfall intervals — non-finite bound/rate, `end < start`, negative rate (`InvalidForcing`). The silent `substep=1.0` fallback and the `duration<=0 \|\| rate<0 { continue }` skip are removed. `run_infiltrated_cascade` propagates via `?`. Test `hyetograph_fails_closed_on_invalid_inputs` (6 cases). |
| CX-D6-002 | Contract still had contradictory re-infiltration / supersede-then-compose text | Purged the stale wording from the scope bullet, the `DC01 runon ownership` kernel-process row, the **`INV-OFEROUTE-009` core statement** (which still asserted "routed inter-OFE excess re-infiltrates" — the self-contradiction; rewritten to SUPERSEDE: infiltration on rainfall, runon surface-routed and NOT re-infiltrated), `OBL-OFEROUTE-P-004`, and the D5 test-vector row. Grep-verified no non-historical stale text remains. |
| CX-D6-003 | Revision truthfulness stale — D6 claimed rev 4 while D5 already had rev 5 | The misplaced duplicate rev-4 D6 entry is removed and re-added as **rev 6** at the end of the history, and its text now names the D5 rev-5 resolution it corrects. |
| CX-D6-004 | Unponded→ponded transition was a full-step ponded approximation, not an explicit split | `green_ampt_step` now implements the explicit Mein-Larsen transition: `r<=Ks` never ponds; ponding begins at `Fp = s/(r/Ks - 1)`; a mid-step transition infiltrates ALL rainfall pre-ponding (`F0->Fp` over `t_p=(Fp-F0)/r`) then integrates the implicit Green-Ampt relation for the remainder (`green_ampt_integrate_ponded`); an already-ponded step integrates from `F0`. The doc comment now matches the code. Test `explicit_ponding_split_conserves_and_delays_excess`. |

Post-fix gates (Ran): `ofe_routing::infiltration` 11 tests; full orchestrator
suite 182/182; clippy `-D warnings` 0; fmt clean; BEI PASS-DEFERRED; authority
guards PASS; `ofe_routing` shadow-first (grep-verified).

## Re-check disposition (Codex re-review `92a4c7a6`)

CX-D6-004 closed at re-check. The three partial closures completed — all were
the same "fixed one site, missed a sibling" pattern:

| # | Residual | Action |
|---|---|---|
| CX-D6-001 | `green_ampt_step` / `InfiltrationState` still public and normalized invalid direct inputs | Made the stateful step-level API **private** (`green_ampt_step`, `InfiltrationState`, `InfiltrationStep`) — the offered "make it internal" option. The public surface is now only the validated entry points (`green_ampt_excess_hyetograph`, `run_infiltrated_cascade`, both fail-closed) plus pure data/query types (`GreenAmptSoil`/`is_valid`, `RainfallInterval`, `ExcessHyetograph`, `infiltration_capacity_m_s`). In-module tests still exercise the private step. |
| CX-D6-002 | `cascade.rs` module header still had the D5 compose/re-infiltrate text | Rewrote the header GAP-OFEROUTE-003 note to the SUPERSEDE model (surface routing, per-OFE infiltration on rainfall, runon NOT re-infiltrated). Repo-wide sweep of `ofe_routing/` + D6 docs confirms no non-historical stale text. |
| CX-D6-003 | `package.md` still said "rev 4" in two places | Both corrected to rev 6 (the contract line and the correction sentence). |

Post-fix gates (Ran): full orchestrator suite 182/182; `ofe_routing` 34/34;
clippy `-D warnings` 0; fmt clean; BEI PASS-DEFERRED; authority guards PASS;
step-level API grep-confirmed private.
