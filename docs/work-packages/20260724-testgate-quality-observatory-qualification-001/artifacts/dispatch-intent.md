# Dispatch Intent

Evidence class: Static.

- Workflow: `.github/workflows/testgate-shadow.yml`
- Ref: `main`
- Base/active authority: the commit that first records the exact authoritative
  package status `ACTIVE`; its SHA will be bound after that correction commits.
- Intent package:
  `docs/work-packages/20260724-testgate-quality-observatory-qualification-001/package.md`
- Dispatch count authorized: one attempt per corrected stable head
- Forest runner: exact `openwepp`, `forest1`, `trusted` label set
- Concurrency: `openwepp-forest1-testgate`

Attempt 1 bound head
`37eeee9a045ad15e3afe2c534ec132551dfbc81c` to provider run
[`30164861346`](https://github.com/rogerlew/openWEPP/actions/runs/30164861346).
It failed closed before planning with
`GATE-PACKAGE-CHAIN-ANCHOR-INACTIVE`: the dispatched scaffold base
`907222635e281a2e135b7f83bdf41eef9656a2d6` still recorded this package as
`QUEUED`. The authenticated recovery artifact retained the same typed cause.
The unchanged head will not be rerun.

Attempt 2 will use the first commit whose package text records the exact
planner-authoritative status `ACTIVE`. Its base SHA, final head, and provider
run ID will be recorded only after focused gates pass, the correction is
pushed, and current forest1 occupancy is clear.

No QA, coverage, CRAP, or CQR workflow is authorized.
