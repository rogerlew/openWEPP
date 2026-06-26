# SNOWDENSITY-05 Kickoff Agent Prompt

You are working in `/home/workdir/openWEPP`. Execute
`docs/work-packages/20260625-snowdensity-05-melt-modernization-contract-first-001/`
end-to-end unless a declared HOLD boundary is reached.

Read first:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/work-packages/20260625-snowdensity-05-melt-modernization-contract-first-001/package.md`
- `docs/planning/snow-frost-fidelity-strategy.md` sections 2, 4, 5, 7, and 10
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

Critical constraints:

- Contract-first is mandatory. Amend `SC-SNOWFREEZE-001` and add
  contract-derived tests before production code.
- Do not promote `dense_slow_melt_v1` or any degree-day
  `snowbench_physics_bulk.rs` variant into production.
- Keep melt separate from density. SNOWDENSITY-05 modernizes production CoE
  melt only; SNOWDENSITY-06 owns density compaction.
- Keep the new melt path opt-in. Default behavior remains legacy CoE melt and
  must have rollback evidence.
- Do not tune or rescale the shared radiation forcing. Calibration handles, if
  any, live on contract-authorized melt coefficients only.
- Use physical defaults from cited authority, not the five SNOTEL sites.
- Do not silently clamp or default missing/invalid snow, melt, albedo, or
  radiation operands. Use typed fail-closed errors unless the contract
  explicitly authorizes bounded normalization.

Required execution order:

1. Complete required reading and update `artifacts/required-reading-map.md`.
2. Fill `artifacts/conflict-and-disposition-ledger.md` before code.
3. Amend `SC-SNOWFREEZE-001` with melt-energy operands, albedo state, opt-in
   melt selector, no-radiation-tuning guard, and negative-benchmark disposition.
4. Add and run contract-derived tests.
5. Record `artifacts/pre-implementation-contract-gate.md`; production edits are
   forbidden before this gate is green.
6. Implement opt-in CoE shortwave/albedo melt modernization in the winter-column
   hydrology path.
7. Prove default rollback identity, opt-in behavior, conservation, anti-alias
   operands, and SNOTEL rubric profile evidence.
8. Run final gates, dual reviews, dual verification, line-count governance, and
   close with truthful disposition.

If a declared HOLD boundary is reached, stop production edits, record the exact
blocker, and make the first worker-handoff item a concrete hold-lift action.
