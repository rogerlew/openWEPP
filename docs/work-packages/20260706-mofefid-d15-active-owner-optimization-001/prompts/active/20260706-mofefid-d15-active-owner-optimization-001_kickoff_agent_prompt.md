# Kickoff Prompt - MOFEFID-D15A Active Owner and Timing Optimization

Task: execute
`docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/package.md`
end to end.

Repo: `/home/workdir/openWEPP`

Base state: `main` / `origin/main` at
`94a7ac3aff003a89328701e4a6daf3abd98c8fe3` or newer, with the D15
blocker-resolution package merged. Do not create or switch branches unless the
operator explicitly asks.

Required reading:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` sections 6.1 and 7
- `docs/ROADMAP.md` section M
- D10B package and artifacts
- D11/D12/D13 packages enough to verify operand/source-shape/routed-hydrograph
  active consumer obligations
- D14 package and timing/profile artifacts
- D15 activation preflight, D15 rerun, and D15 blocker-resolution artifacts

Objective:

1. Optimize or explicitly adjudicate the D10B-corrected H2637 opt-in routed
   timing regression (`91.59 s` user / `1:31.67` wall).
2. Implement the contract-authorized opt-in active production owner path only
   when timing and authority gates are green.

The optimization portion must follow the D14 structure: intake/baseline, slot
instrumentation, empirical profile, optimization plan, behavior-preserving
implementation, and evidence/closure. Do not tune physics, loosen tolerances,
or change numerical method for performance.

Production activation constraints:

- The active routed path must own surface-water routing for opt-in lanes.
- The old DC01 daily-lump runon path must not also feed the same active lane.
- Runtime closure hard-fail must be live in active mode.
- Routed hydrograph shape must feed the D13 erosion consumer when routing owns
  the water path.
- Rev-21 friction operands and D12 source-shape limbs must be consumed by the
  real active production path.
- Subsystem-off/default behavior must remain byte-identical.
- No surrogate physics, compatibility wrapper, silent fallback, or shadow-only
  activation claim.

If any required activation or timing precondition cannot be closed, stop at
`EXECUTED-HOLD-*`, write `artifacts/hold-legitimacy-audit.md`, and do not
partially flip production activation.

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
`comparator_suite_runner`, and `timing_comparator` subagents for read-only
source/authority audit, profiling review, optimization review, active-consumer
proof review, H2637 timing/comparator execution, and heavy gate execution.
Expected outputs are compact findings, timing metrics, comparator/gate metrics,
log paths, and package-local review or verification artifact text. Write access
is read-only unless the operator assigns a bounded write set for a named
implementation fix.

Required gates are listed in `package.md`. Record all gate outcomes as `PASS`,
`FAIL`, `BLOCKED`, or `NOT RUN` in `artifacts/gate-results.md`.
