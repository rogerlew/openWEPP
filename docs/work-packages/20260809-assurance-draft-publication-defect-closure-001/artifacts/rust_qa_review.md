# Secondary Rust QA Review

Evidence class: `Static + Ran`

Date: `2026-08-09`

## Findings

No open findings.

### Resolved — Exact-Current-Diff Full-Workspace Evidence

Severity before correction: `High`.

Paths:
`tests/integration/assurance_v2_publication_contract.rs:531`,
`docs/work-packages/20260809-assurance-draft-publication-defect-closure-001/artifacts/full-workspace-gate-pre.txt`,
`docs/work-packages/20260809-assurance-draft-publication-defect-closure-001/artifacts/full-workspace-gate-run.log`,
and
`docs/work-packages/20260809-assurance-draft-publication-defect-closure-001/artifacts/full-workspace-gate.md`.

The stale pre-review run was interrupted and retained only as superseded
diagnostic evidence. The replacement full-workspace run is bound to:

- Git HEAD `4237552aa8dbc84a8baeff800014b23c7e75be9f`;
- reviewed assurance test blob
  `07e65f289049cfa6a96617a9922f70a06d8f5165` at pre-run, post-run, and this
  terminal QA inspection; and
- pre-run working-tree fingerprint
  `4243658ad03b52e20ff621b8c957664abd549b65cbb0b6feb20aa3957546360d`.

The raw terminal log hash is
`d125d5ff4c5050e5068ac07bd698aa07ae2118ce46a204e1dfc679e159d9730d`,
matching the evidence record. Its metadata reports exit `0` and 3,302 seconds.
The log selects 2,325 tests and terminates with
`2325 tests run: 2325 passed (55 slow), 33 skipped` in 3,300.706 seconds. The
33 skips are declared at startup by the full profile; the raw log contains no
failure, timeout, abort, signal, or test-error marker.

Disposition: `accepted-and-fixed`; the exact-current-diff Critical gate is
closed.

### Resolved During Review — Exact Lifecycle Error

Severity before correction: `High`.

Path: `tests/integration/assurance_v2_publication_contract.rs:542`.

The initial diff retained the old semantic predicate: any
`AssuranceError::Invalid` message containing `DRAFT` passed. The corrected test
now requires exact equality with
`report '<REPORT_ID>' is DRAFT; publication requires APPROVED`, binding the
selected report, lifecycle, and publication precondition.

Disposition: `accepted-and-fixed`; no open code finding.

### Resolved During Review — Public Non-Mutation Proof

Severity before correction: `High`.

Path: `tests/integration/assurance_v2_publication_contract.rs:531`.

The initial diff checked only absence of `assurance/catalog.json`. The corrected
test captures the seeded public file tree before publication and compares it
after rejection, while retaining the empty snapshot-root assertion. This
detects public file creation or mutation and excludes snapshot and receipt
creation.

Disposition: `accepted-and-fixed`; no open code finding.

## Non-Blocking Debt And Follow-Ups

- `capture_tree` records paths and bytes for files, not empty directories. That
  is adequate for the current byte-publication claim because static inspection
  confirms lifecycle rejection occurs before finalization opens or writes the
  public and snapshot roots. If future publication code creates directories
  before validation, extend the manifest to include entry kinds.
- `Scratch` uses process ID plus an atomic serial for unique paths and removes
  each path through `Drop`; neither invocation mutates process-wide environment
  state. This is suitable for concurrent Nextest isolation. No scratch
  directory from either independent topology invocation remained after process
  exit, including the assertion-failure path.
- No production Rust file changed. The correct
  `validate_roots`-before-`validate_publishable` ordering remains intact, so an
  in-repository temporary root must continue to reject before lifecycle loading.
- The integration-test file is 1,960 lines after correction. It remains below
  the repository's 2,000-line warning threshold, but has only 40 lines of
  headroom; the next material expansion should split publication-contract
  scenarios into cohesive targets.
- The package validation plan now includes affected-language formatting and
  warnings-denied lint, and both passed independently. `cargo deny check` is
  not applicable because no manifest, lockfile, dependency policy, toolchain,
  or workspace-resolution input changed.

## Independent Validation

- Ran after the final assertion corrections:
  `TMPDIR=/home/workdir/openwepp-task-tmp cargo nextest run --test assurance_v2_publication_contract draft_subject_root_is_stable_but_cannot_publish --profile quick`
  — `PASS`, exit `0`, 1 passed, 36 skipped.
- Ran on the pre-final diagnostic diff:
  `TMPDIR=/home/workdir/openWEPP/target/task-tmp cargo nextest run --test assurance_v2_publication_contract draft_subject_root_is_stable_but_cannot_publish --profile quick`
  — expected `FAIL`, exit `100`; the rejection was
  `staging and repository roots must be unrelated`.
- Ran after the final assertion corrections: `cargo fmt --all -- --check` —
  `PASS`, exit `0`.
- Ran after the final assertion corrections:
  `cargo clippy --test assurance_v2_publication_contract -- -D warnings` —
  `PASS`, exit `0`.
- Ran after the final review update:
  `markdown-doc lint --path docs/work-packages/20260809-assurance-draft-publication-defect-closure-001 --format plain`
  — `PASS`, 11 files, 0 errors, 0 warnings.
- Ran: `git diff --check -- tests/integration/assurance_v2_publication_contract.rs docs/work-packages/20260809-assurance-draft-publication-defect-closure-001`
  — `PASS`, exit `0` after the final review update.
- Static verification of retained `Ran` evidence:
  `TMPDIR=/home/workdir/openwepp-task-tmp cargo nextest run --workspace --profile full`
  — `PASS`, exit `0`, 2,325/2,325 selected tests passed, 55 slow, 33 declared
  full-profile skips, 3,300.706 seconds.

## QA Disposition

`PASS` — unconditional. The corrected Rust test is readable, deterministic,
isolated, fail-closed, and adequately validates the exact lifecycle error plus
absence of public, snapshot, and receipt side effects. All QA findings are
resolved, and the exact-current-diff full-workspace evidence passes. No
production correction is warranted.
