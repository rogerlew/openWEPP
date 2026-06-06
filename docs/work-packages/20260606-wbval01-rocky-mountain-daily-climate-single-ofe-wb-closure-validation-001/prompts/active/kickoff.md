# WBVAL01 Kickoff

Execution mode: package-end-to-end

Autonomy: execute end-to-end for the declared scope — enumerate hillslopes, run
`openwepp-cli-hill`, compute and classify per-hillslope conservation closure,
produce all deliverables, dual review/verification, disposition, and handoff —
without additional user intervention unless hard-blocked.

## Item 1 (do this, not the snow route)

Validate single-OFE water-balance **conservation closure** on the real CLIGEN
daily (non-breakpoint) Rocky Mountain run at
`/wc1/runs/in/indispensable-presenter` (DRIGGS ID, 23 hillslopes), per
`package.md`. This is roadmap rung-1.

Do **not** open or continue the HPHYS0298->0320 snow/`RM` comparator route. It is
suspended behind `docs/backlog/20260605-snow-code-deferred-science-review.md`
(ADR-0017). On completion, `worker-handoff.md` names **frost** as the next rung.

## Acceptance authority

- Conservation closure (`R = Σinputs - Σoutputs - ΔStorage` per hillslope/year),
  NOT comparator match and NOT snow magnitude (ADR-0017, ADR-0011).
- A population of `conservation-break` hillslopes is the expected rung-2 input,
  not a failure. Failure = an incomplete or untruthful ledger.
- Fail closed on any missing balance term; never impute zero to force closure.
- Truthful evidence mode per artifact: **Ran** for actual `openwepp-cli-hill`
  invocations, **Static** for reasoned classification.

## Required reading

- `docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/package.md`
- `docs/work-packages/README.md` (Current roadmap)
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/backlog/20260605-snow-code-deferred-science-review.md`
- `AGENTS.md`, `docs/codex_exec_plans.md`
- Precedent for driving a hillslope through the runner:
  `tests/integration/cli01_runner_hillslope_integration.rs`,
  `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
