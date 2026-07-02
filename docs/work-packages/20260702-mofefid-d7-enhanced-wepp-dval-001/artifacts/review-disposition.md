# D7 Execution Review Disposition (Codex `review-codex-execution.md`)

All three blocking findings **accepted and fixed**. The corrections make the
outcome humbler and more truthful: **zero cases cleanly reproduce.**

| # | Finding | Action |
|---|---|---|
| 1 (critical) | `run_iwagaki` fed the lateral-supply rate into `rainfall_intensity_m_s`, but Iwagaki has NO rain — spuriously inflating the skin term and invalidating the solver-side shock attribution | Set `I = 0` in `run_iwagaki` (lateral supply stays as excess). Reran: at k_o~200 the timing (28 s vs 26 s) and 10-90% rise (20.6 s vs 20.9 s) **reproduce**; residual is peak −20% / NS ~0.30. **`GAP-OFEROUTE-004` WITHDRAWN** (forcing-bug artifact); Case 4 reclassified **operand-limited** (unspecified flume k_o). Committed test corrected. |
| 2 | Case 1 marked REPRODUCES without the required shape gate; rise time differs materially | Applied the rise-limb shape gate: openWEPP 10-90% rise ~5000 s vs enhanced ~3580 s (~40% slow). Case 1 **downgraded to PARTIAL** (steady magnitude reproduces, transient does not). Test renamed `case1_bare_reproduces_steady_magnitude`; execution-report states the shape-gate failure. |
| 3 | D7-S2 (skin `I`/`ν` convention) deferred while the contract claimed D-val confirmation | The skin term is k_o-dominated (Case-1 `I`-term ≈ 60 vs k_o 500), so Case-1 reproduction never validated the convention — and finding #1 shows the `I` path was mis-exercised. `INV-OFEROUTE-002` and `GAP-OFEROUTE-002` corrected to **unconfirmed / audit OPEN**; execution-report S2 states D7 does not close it. |

Contract: `SC-OFEROUTE-001` rev 8 (GAP-004 withdrawn; INV-011 corrected —
Case 1 PARTIAL, zero clean reproductions; skin-convention reconciled).

Post-fix gates (Ran): `ofe_routing::dval` 2/2 (corrected); full orchestrator
suite; clippy `-D warnings`; fmt; BEI PASS-DEFERRED; authority guards; harness
Case 1/4 re-run. `dval` shadow/analysis-only.

## Note on the value of the review

Finding #1 is the important one: a forcing bug produced a *plausible* solver
GAP that I attributed with confidence. The lesson is the truthfulness-with-
competence one — a hypothesis (solver-side lag) was delivered as a conclusion
(a contract GAP) before the forcing was verified. The corrected result has no
manufactured defect.
