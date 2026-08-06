# Review Agent A

Status: complete / changes required / findings dispositioned

Evidence mode: Static independent domain-science review by
`/root/meltout_strategy_review` on 2026-08-05. No tracked edits.

## Findings

1. **Blocker — terminal integrator under-specified.** A `1 minute` outer
   cadence plus enthalpy does not remove cooling stiffness as snow mass tends
   to zero. The contract must define an implicit or error-controlled terminal
   integrator, tolerances, event bracketing, convergence behavior, and flux
   reevaluation. Libsnobal supports the threshold/conversion precedent, not an
   exact-event algorithm.
2. **Blocker — receiving surface over-specified.** Snow disappearance does not
   prove bare or persistently wet soil. Residue, vegetation, frost,
   infiltration, and ponding control the new surface. `SC-WATBAL-001` is not
   soil-surface energy authority; `SC-SOIL-001`, `SC-RUNOFFPART-001`,
   `SC-EVAP-001`, and pinned `tmpadj`/frost sources require reconciliation.
3. **Blocker — persistent snow-only state cannot support seasonal claims.**
   Post-divergence soil heat, evaporation, infiltration, soil water/frost, and
   surface-water state feed later ground flux and snow reappearance. A
   seasonal claim requires a coupled shadow of those states; otherwise
   acceptance must be event-local.
4. **Blocker — terminal water and infiltration-first routing omitted.** Snow
   drainage is not hillslope runoff. At exhaustion, retained snow liquid must
   be released exactly once to surface liquid, then routed through
   infiltration, soil storage, ponding/overflow, evaporation, and residual
   runoff.
5. **Blocker — event chronology must include vapor exhaustion.** The earliest
   solid-exhaustion event can arise from melt, sublimation, or their
   combination. Snow-surface vapor flux cannot be reserved for a full substep
   after earlier meltout; simultaneous precipitation/deposition and snow
   reappearance require explicit chronology.
6. **Blocker — authority hold occurs too late.** Complete receiving-surface and
   coupled-state authority must be a Phase-1 pre-implementation go/no-go gate,
   not a hold after terminal production code is written.

## Reviewer Disposition

`CHANGES REQUIRED`. The direct transfer of snow-computed
`Q_unallocated_after_exhaustion` to soil is correctly rejected because those
fluxes use invalid snow-surface properties after the regime change. Before the
corrections, the package was executable only to a Phase-1 authority `HOLD`.
