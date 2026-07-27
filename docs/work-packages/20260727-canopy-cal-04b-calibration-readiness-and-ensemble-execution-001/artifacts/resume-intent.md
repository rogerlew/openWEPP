# Resume Intent

Status: `PRE-HEAVY HOLD / TOOLING CORRECTION IN PROGRESS`

Evidence class: `Static`

Authenticated restart base:
`dd3b2a59018bd9a39999f9b263b07351afc34290`.

The resumed increment executes the existing frozen CAL-04B command DAG without
changing calibration domains, observations, operators, objectives, acceptance,
stopping behavior, or Harvard custody. Production and predecessor science are
read-only.

The existing `/home/workdir/cal04b-objects/` tree contains retained native
hold-lift evidence. It will be moved byte-preservingly to a uniquely named
pre-resume archive. A new empty `/home/workdir/cal04b-objects/` becomes the sole
attempt-005 execution root. The observed executor must restart at `prepare`.

Pre-execution acceptance:

- native hold-lift, assurance identity, and TESTGATE packages are `COMPLETE`;
- exact full workspace run `7e79049d-0871-4142-a9f7-86ac7ac714be` passed
  2,301/2,301;
- active kickoff is byte-identical to the archived accepted prompt;
- source, binary, command, repository-head, and prerequisite identities are
  rebound before result-bearing work;
- scaffold and executor validators, executor tests, warnings-denied Clippy,
  cargo-deny, documentation lint, and diff hygiene pass;
- two independent read-only reviewers approve the fresh restart;
- Harvard remains `SEALED`.

Heavy execution ownership remains with the authorized
`comparator_suite_runner`. Freeze verification and terminal review/verification
remain independent and read-only.

## Restart Evidence

- Historical object tree: moved byte-preservingly to
  `/home/workdir/cal04b-objects-pre-resume-005` (1.4 GB).
- Fresh attempt root: `/home/workdir/cal04b-objects`; empty before execution.
- Restart commit: `66506b553b30356f0174c51dd7174450d9ab5278`.
- Observed `prepare`: `PASS`, 9,261 candidates, 27,783 saturation rows,
  82,059 forcing rows.
- Prepare receipt:
  `/home/workdir/cal04b-objects/execution-ledger/1-prepare.receipt.csv`,
  SHA-256
  `8d0e0f8f8efe505ab192aebdba4dae453ae4aa67dd4c6dd1b0ebe95fce9aeee5`.
- Freeze manifest and verifier receipts were prospectively restored from the
  historical native-proof HOLD sentinels to uniform `SEALED`/`pending` state.
- Daymet checksum verification: `PASS`, all nine sources plus README.
- Scaffold validator: `PASS`, 14 accepted controls, sealed pre-heavy lifecycle.
- Executor validator: `PASS`, 9,261 candidates, 27,783 saturation rows,
  18 commands.
- Executor tests: `PASS`, 22 tests.
- Warnings-denied executor Clippy: `PASS`.
- Executor cargo-deny: `PASS` with only unmatched-allowance/advisory warnings;
  advisories, bans, licenses, and sources all `ok`.
- Markdown lint and diff hygiene: `PASS`.

Harvard remains `SEALED`. No Hubbard population command has run.

## Resume Review Disposition

The first independent restart review returned `HOLD` before population work.
The package accepts both high-severity findings:

- the observed coordinator lacks the mandatory versioned
  `LIGHT -> READY -> HEAVY` admission barrier required by
  `docs/standards/testing-and-gate-strategy.md`;
- observed receipt schema 01 does not bind the repository, coordinator,
  observer, toolchain, prerequisite-package, and READY-audit identities
  declared by this resume increment.

The prepare-only attempt is not reusable because its receipt predates the
tooling-correction commit and cannot authenticate the corrected observer or
READY audit. It will be retained byte-preservingly outside the active object
root with reuse disposition `INVALIDATED_BY_OBSERVER_AND_ADMISSION_SCHEMA`.
The active root will then be recreated empty and execution will restart at
`prepare`; no successful result-bearing population receipt is being discarded.

Initial review proposed a package-local correction. Further inspection proved
that it cannot remain inside this package's write set. The canonical
`openwepp-pre-heavy-audit-v1` authority and
same-process transition live in `openwepp-gate-planner`; CAL-04B's CSV executor
cannot synthesize or transport equivalent authority. The canonical executor
also rejects repository mutation, while the legacy CAL-04B DAG publishes
result-bearing files directly into this package.

The prerequisite
`20260727-gate-planner-external-dag-transaction-adapter-001` therefore owns the
generic external-output DAG transaction, manifest, receipt, and two-transition
support. CAL-04B remains active on a tooling hold, Harvard remains sealed, and
no population command may resume until that prerequisite is complete and this
package passes a fresh exact-commit dual restart review.
