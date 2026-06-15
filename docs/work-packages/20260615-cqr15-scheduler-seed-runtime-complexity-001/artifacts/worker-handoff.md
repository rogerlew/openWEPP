# Worker Handoff

Status: complete.

Current state:

- CQR15 production refactor is implemented.
- Characterization tests are added and passing.
- Before/after LCOV and CRAP artifacts are generated.
- Target and new helpers are CRAP `<= 30`.
- Package artifacts, reviews, verification, disposition, and final gates are
  completed.

Required next steps:

1. Stage only CQR15 package files, `docs/work-packages/README.md`, and touched
   runner source/test files. Exclude unrelated root `AGENTS.md`.
2. Commit CQR15 package write set with a terse CQR15-specific message.
3. Push `main` to origin.
4. Only after package push succeeds, check off CQR15 in
   `docs/work-packages/cqr-burndown-execplan.md` and record package path,
   commit SHA, branch, date, and final target CRAP.
5. Commit and push the tracker update.

First follow-up if interrupted: stage, commit, and push the package write set.
