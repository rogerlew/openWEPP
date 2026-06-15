# Worker Handoff

Status: complete pending package commit and push.

Current row: CQR21.

Package:
`docs/work-packages/20260615-cqr21-climate-runtime-adapter-complexity-001/`.

Target file:
`crates/openwepp-climate-runtime-adapter/src/lib.rs`.

Final target:
`SharedClimateRuntimeInputError::fmt`.

Final metric:
CRAP `2.0`.

Implemented changes:

- Added focused characterization for every shared climate runtime input error
  code and display string.
- Split `SharedClimateRuntimeInputError::fmt` into code-prefix writing and a
  private `fmt_message` branch helper.
- Removed the production `too_many_lines` suppression from the CQR21 target.

Required next actions:

1. Run and record final required gates.
2. Commit the CQR21 package write set, excluding unrelated dirty `AGENTS.md`.
3. Push `main` to `origin`.
4. Only after successful push, update
   `docs/work-packages/cqr-burndown-execplan.md` for CQR21 with package path,
   commit SHA, branch, date, and final CRAP.
5. Commit and push the tracker update.

First actionable follow-up if blocked: inspect the failing final gate output and
fix only CQR21-owned files unless the failure proves a pre-existing external
issue.
