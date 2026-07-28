# Implementation Summary

Evidence class: `Static + Ran`

Order 2 removed the gate planner and external-transaction adapter from every
prospective CAL-04B tool path.

- `execute-prefix.py` now executes the package's literal calibration argv
  arrays directly without a shell. It records fsynced per-command logs and JSON
  evidence and creates `primary-failure.json` before returning on nonzero exit,
  timeout, or launch error. It performs no cleanup.
- `publish-results.py` now inventories a fixed allowlist of package-owned result
  files and performs plan-only inspection by default. Apply mode uses
  same-directory temporary files, file fsync, atomic replacement, and directory
  fsync. Differing destinations require explicit `--replace`.
- Freeze consumes `calibration-complete.json`, not a transaction receipt.
  Custody consists of the nonempty checksum-bound freeze plus two direct
  read-only verifier records; capabilities, attestations, dispatch claims,
  workflow identity, and planner receipts are gone.
- Holdout requires `bubblewrap`. The repository, Harvard, calibration outputs,
  and executables are read-only; only custody and a separate empty holdout
  output root are writable. The exclusive `OPENED_ONCE` token is fully written
  and fsynced before the first Harvard content read. Post-open failure remains
  non-rerunnable.
- The previously omitted synthetic-recovery failure is now incident 005.
  CAL-04B truthfully holds on a science-design defect before population, and
  Harvard remains sealed.

No CAL command, population command, freeze, verifier, holdout, Harvard read, or
model execution was performed by Order 2.
