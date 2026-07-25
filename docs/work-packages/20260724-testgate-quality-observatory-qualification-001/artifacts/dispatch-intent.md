# Dispatch Intent

Evidence class: Static.

- Workflow: `.github/workflows/testgate-shadow.yml`
- Ref: `main`
- Base/qualification scaffold:
  `907222635e281a2e135b7f83bdf41eef9656a2d6`
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

Attempt 2 bound head
`3e2a22eb6572b28042bda47b9097b41bbfe63041` to provider run
[`30165078755`](https://github.com/rogerlew/openWEPP/actions/runs/30165078755).
Execution, independent verification, and authority all passed. Its exact
receipt `8d6559916ae0b3cd9529f4cd5816f8e476bed1265e0d90d79495c6b145fa1582`
truthfully records `LOCAL_UNTRUSTED` and `DEFERRED_TO_QUALITY_CI`, but the
activation base narrowed the increment to three documentation files and
selected only documentation lint. The passing receipt therefore does not
close the package's declared critical changed-head qualification.

Attempt 3 will retain the original scaffold base and use the corrected exact
status grammar to authenticate its `QUEUED / ORDER-6` anchor. That preserves
the full quality-separation increment for selection. Its final head and
provider run ID will be recorded only after focused gates pass, the correction
is pushed, and current forest1 occupancy is clear.

No QA, coverage, CRAP, or CQR workflow is authorized.
