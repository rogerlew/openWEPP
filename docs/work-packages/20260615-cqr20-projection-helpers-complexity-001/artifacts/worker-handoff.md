# Worker Handoff

Status: complete pending package commit and push.

Current row: CQR20.

Package:
`docs/work-packages/20260615-cqr20-projection-helpers-complexity-001/`.

Target file:
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`.

Final target:
`project_annual_extension_controls`.

Final metric:
CRAP `9.0`.

Implemented changes:

- Added focused characterization tests for annual extension control success
  branches, mismatch branches, unsupported annual `resmgt`, and representative
  day/fraction domain errors.
- Split `project_annual_extension_controls` into private per-branch helper
  functions without changing public runtime projection behavior.
- Removed the production `too_many_lines` suppression from the CQR20 target.

Required next actions:

1. Commit the CQR20 package write set, excluding unrelated dirty `AGENTS.md`.
2. Push `main` to `origin`.
3. Only after successful push, update
   `docs/work-packages/cqr-burndown-execplan.md` for CQR20 with package path,
   commit SHA, branch, date, and final CRAP.
4. Commit and push the tracker update.

First actionable follow-up if blocked: inspect the failing final gate output and
fix only CQR20-owned files unless the failure proves a pre-existing external
issue.
