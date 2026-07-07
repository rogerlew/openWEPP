# Feasible Evidence

Status: EXECUTED-HOLD-COHORT-AUTHORITY. Evidence mode: Static + Ran.

## Reused D16 Evidence

D16 remains the only active-runnable current-mesh H2637 evidence:

- Active plain: `39.73 s` user / `0:39.75` wall.
- Active explicit hybrid: `33.45 s` user / `0:33.47` wall.
- Case-4 hybrid ladder: PASS.
- Blocking deltas: `-0.4396 %` routed outlet and `-6.474 %` pass sediment
  sums.

Source artifacts:

- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/timing-and-fidelity.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/hold-legitimacy-audit.md`

## New Evidence In This Package

- Exact runner binary provenance:
  `artifacts/binary-provenance.txt`.
- owcmp env preflight:
  `artifacts/owcmp-env-preflight.log`.
- owcmp manifest run preflight:
  `artifacts/owcmp-manifest-run-preflight.log`.
- repo runfile inventory:
  `artifacts/repo-runfile-inventory.txt`.
- active routing-coefficient search:
  `artifacts/routing-coefficients-search.txt`.
- copied-fixture active preflight failures:
  `artifacts/active-preflight/*.log`.

## Evidence Interpretation

No new active plain-vs-hybrid cohort comparison could be run because no
available cohort member passed active Lane-D preflight. This package therefore
does not amend `SC-OFEROUTE-002`, does not ratify tolerance thresholds, and
does not flip selector defaults.
