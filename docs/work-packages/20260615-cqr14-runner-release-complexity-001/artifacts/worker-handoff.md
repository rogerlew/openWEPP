# Worker Handoff

Status: complete.

Static: CQR14 package path:
`docs/work-packages/20260615-cqr14-runner-release-complexity-001/`.

Static: production write set:
`crates/openwepp-runner/src/release.rs`.

Static: final target CRAP:
`lint_release_directory` -> `4.0`.

Static: package is ready for commit and push on the current branch. After push,
update `docs/work-packages/cqr-burndown-execplan.md` for CQR14 with the pushed
commit SHA, branch, date, package path, and final CRAP, then commit and push the
tracker update.

WARN: root `AGENTS.md` is modified outside this package and must remain
unstaged for CQR14.
