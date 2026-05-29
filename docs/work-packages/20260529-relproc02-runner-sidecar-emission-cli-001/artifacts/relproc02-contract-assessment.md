# relproc02-contract-assessment

Status: complete  
Evidence mode: Static

## Contract Baseline

1. `docs/work-packages/20260529-relproc01-openwepp-release-procedure-draft-001/artifacts/worker-handoff.md`
   declared a dedicated sidecar-emission CLI as immediate next action.
2. `docs/contracts/openwepp-runner-contract.md` previously required
   `run-hillslope` and `release lint`, but did not define a sidecar-emission
   command surface.
3. `docs/governance/openwepp-release-procedure-draft.md` used a manual Python
   workaround for sidecar generation.

## Contract Amendments Applied

- `docs/contracts/openwepp-runner-contract.md`
  - Added required command surface:
    `open_wepp_runner release sidecar --binary <path> --role <role>`.
  - Added `release sidecar` requirements:
    explicit args, role domain, sidecar validation, hard-failure behavior.
  - Recorded `RELMD-E-001..005` failure IDs for metadata emission.
- `docs/contracts/openwepp-binary-release-contract.md`
  - Added canonical sidecar-emission command surface and requirements.
- `docs/governance/openwepp-release-procedure-draft.md`
  - Replaced manual scripted sidecar generation with runner command usage.
