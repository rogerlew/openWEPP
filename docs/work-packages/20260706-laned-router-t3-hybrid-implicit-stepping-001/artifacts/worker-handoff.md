# Worker Handoff (T3)

Status: HANDOFF-RECORDED. I0/I1 complete; I2 landed as an experimental
opt-in (strict rule) with all closure gates green.

Next actionable items, in value order:

1. **Aggressive-rule composition fix** (the 55.5 %-coverage prize, est.
   ~1.9x active endpoint): extend the hybrid span composition so a
   front-arrival terminal-bin deficit from a SHORT explicit span carries
   into the next span's bins instead of failing closed
   (`NegativeOutletBin`, H2637 lane 17 day 54). Shape: return the terminal
   deficit from the solver run (or expose it on `RoutingResult`) and let
   `route_single_ofe_hybrid` absorb it in the global bin composition with
   the exact-total rule. Then flip the mask predicate to zero-source-only
   (one expression, already written and reverted in-history) and re-run the
   H2637 evidence + closure gates.
2. **Fidelity ratification** (rev-28 follow-on): pick the implicit-phase dt
   (900 vs 300) against a named per-bin tolerance using the I1 ladder +
   H2637 hydrograph-surface deltas; ratify in SC-OFEROUTE-001 and graduate
   the selector from evidence-gathering.
3. **Tier-1** (delegable now; specs in
   `docs/backlog/20260706-laned-router-numerics-performance-tiers.md` +
   `i1-implicit-stepper-evidence.md`): analytic celerity, Newton-α,
   `h·sqrt(h)`/pow — est. 2.5-4x on the explicit phases; composes with the
   hybrid.
4. **Tier-2** resolution adjudication (delegable): 5-cell production mesh
   via the existing oracle ladder, est. ~3-4x.
5. Dual review + verification for this package per
   `docs/work-packages/AGENTS.md` before merge (same lane structure as
   D15A; the D15A review artifacts are the template).

Key discoveries a future worker must not re-learn (all contract-recorded at
rev 28): the Z-shaped regime-dispatch rating + basin-split seeding +
LOW→HIGH→Filippov closure; Steffensen determinism; the dust-floor rule on
exact-ledger guards; the two mid-execution fail-closed catches (both were
REAL defects — the guard design keeps paying).
