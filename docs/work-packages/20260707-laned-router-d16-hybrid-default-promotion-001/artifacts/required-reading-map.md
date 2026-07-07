# Required Reading Map

Status: EXECUTED. Evidence mode: Static.

## Read

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-tier1-local-numerics-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-tier1-local-numerics-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-tier2-mesh-resolution-adjudication-001/package.md`

## Initial Boundary

This package adjudicates active-path hybrid default promotion only. It does not
claim broader no-env Lane-D active-owner default activation.

## Authority Findings

- `SC-OFEROUTE-002` lines 23-35 describe the hybrid subsystem as
  experimental/unpromoted and make promotion conditional on its acceptance
  posture.
- `SC-OFEROUTE-002` lines 286-310 keep the current selector semantics:
  `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` selects hybrid and unset means plain
  rev-27 active behavior.
- `SC-OFEROUTE-002` lines 394-398 require both the Case-4 hybrid ladder and
  named ratified fidelity/timing tolerances before promotion.
- `SC-OFEROUTE-002` lines 410-419 ratify only GAP-OFEHYB-002 exact-evaluator
  numeric dust; they explicitly do not change default or tolerance posture.
- `SC-OFEROUTE-001` lines 222-224 keep `OPENWEPP_LANED_ACTIVE=1` as the active
  owner selector and point hybrid promotion back to `SC-OFEROUTE-002`.

Conclusion: active-lane hybrid default promotion needs a current package
tolerance decision. The package stopped before code because the H2637
plain-vs-hybrid deltas are not covered by an existing ratified tolerance.
