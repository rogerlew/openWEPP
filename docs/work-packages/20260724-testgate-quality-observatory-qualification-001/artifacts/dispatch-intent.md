# Dispatch Intent

Evidence class: Static.

- Workflow: `.github/workflows/testgate-shadow.yml`
- Ref: `main`
- Base/scaffold:
  `907222635e281a2e135b7f83bdf41eef9656a2d6`
- Intent package:
  `docs/work-packages/20260724-testgate-quality-observatory-qualification-001/package.md`
- Dispatch count authorized: exactly one stable-head attempt
- Forest runner: exact `openwepp`, `forest1`, `trusted` label set
- Concurrency: `openwepp-forest1-testgate`

The final head and provider run ID will be added only after cheap gates pass,
the stable increment is pushed, current forest1 occupancy is clear, and the
dispatch succeeds.

No QA, coverage, CRAP, or CQR workflow is authorized.
