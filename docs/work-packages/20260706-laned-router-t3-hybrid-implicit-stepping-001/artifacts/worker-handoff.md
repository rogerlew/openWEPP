# Worker Handoff (T3)

Status: HANDOFF-RECORDED. I0/I1 complete; I2 landed as an experimental
opt-in and remains held. Rev-31 rerun package
`20260707-laned-router-t3-ratification-solve-cost-001` executed the open
Case-4 hybrid ladder and it FAILED the ratified peak tolerance; fidelity
ratification is therefore BLOCKED and the selector is NOT promoted.

Next actionable items, in value order:

1. **Aggressive-rule composition fix — EXECUTED (2026-07-07, WP
   `20260706-laned-router-t3-aggressive-deficit-carry-001`, rev 30):** the
   deficit-carry composition landed exactly in the shape above
   (`run_with_options_deficit_carry` + cross-span carry + fail-closed
   end-of-window disposition) and the mask is flipped to zero-source-only.
   H2637 runs green through the former failure coordinates with all rev-27
   closures at machine precision; the carry fired 6×/yr (all absorbed).
   HONEST OUTCOME: the ~1.9x prize did NOT materialize — explicit work fell
   by the full 55.5 % coverage but the implicit cell-solve cost (cold
   basin-split seeds, ~23 M scalar solves/yr) consumes it (endpoint
   `38.0-38.3 s` vs `37.9 s` plain). NEW top lever: **implicit solve-cost
   reduction** under the rev-29 determinism constraint (deterministic
   within-step warm seeding from the downstream march's own upstream cell;
   Newton on the composed cell residual; composes with Tier-1
   friction-eval cuts). See the WP's `fix-evidence.md`.
2. **Rev-31 solve-cost lever + ratification rerun — EXECUTED-HOLD
   (2026-07-07, WP
   `20260707-laned-router-t3-ratification-solve-cost-001`):** deterministic
   branch-local warm seeding and profile counters landed; H2637 active hybrid
   improved to `36.61 s` user with `274.7M` implicit map evaluations counted.
   HOLD: Case-4 hybrid ladder failed (`22.8% / 15.5% / 10.2%` peak errors vs
   `5%` tolerance). No selector promotion.
3. **Tier-1** (delegable now; scaffolded at
   `docs/work-packages/20260707-laned-router-tier1-local-numerics-001/`; specs in
   `docs/backlog/20260706-laned-router-numerics-performance-tiers.md` +
   `i1-implicit-stepper-evidence.md`): analytic celerity, Newton-α,
   `h·sqrt(h)`/pow — est. 2.5-4x on the explicit phases; composes with the
   hybrid.
4. **Tier-2** resolution adjudication (delegable; scaffolded at
   `docs/work-packages/20260707-laned-router-tier2-mesh-resolution-adjudication-001/`):
   5-cell production mesh via the existing oracle ladder, est. ~3-4x.
5. Dual review + verification for this package per
   `docs/work-packages/AGENTS.md` before merge (same lane structure as
   D15A; the D15A review artifacts are the template).

Key discoveries a future worker must not re-learn (all contract-recorded at
rev 28): the Z-shaped regime-dispatch rating + basin-split seeding +
LOW-jump→HIGH-root recovery + fail-closed double-collapse; Steffensen
determinism; the dust-floor rule on exact-ledger guards; the two mid-execution
fail-closed catches (both were REAL defects — the guard design keeps paying).
