# Independent Terminal Verification A

Evidence class: `Ran + Static exact-current-diff verification`

Verdict: `PASS`

Verifier independence: I did not read the other assurance terminal verifier's
artifact before reaching this verdict. I did not rerun the full workspace.

## Findings

No open finding remains.

### `VERIFY-A-MEDIUM-001` — accepted and closed

The lifted native-forest package initially retained present-tense hold wording
in its disposition, terminal-diff reconciliation, and worker handoff after its
status had changed to complete. Those artifacts now distinguish the historical
hold from the satisfied lift condition, state `RECONCILED / COMPLETE`, and say
that future production work may begin only through the independently held
coupled successor. The package no longer instructs a worker to replay gates
that have already passed.

### `VERIFY-A-LOW-002` — accepted and closed

The assurance review disposition initially conditioned closure on a future QA
confirmation even though `rust_qa_review.md` already recorded unconditional
`PASS`. The disposition now says that no review finding remains open and that
QA reverified the terminal identity.

## Exact Diff And Root-Cause Verification

- Current Git HEAD is `4237552aa8dbc84a8baeff800014b23c7e75be9f`.
- Current assurance integration-test blob is
  `07e65f289049cfa6a96617a9922f70a06d8f5165`, exactly the reviewed and
  full-workspace-bound blob.
- `git diff --name-only -- crates/openwepp-assurance` is empty. The complete
  current status contains no production Rust, Cargo manifest, lockfile,
  selector, output, publication, or deployment edit.
- Static control-flow inspection confirms `publish` calls `validate_roots`
  before context loading and `validate_publishable`. The latter returns the
  report-specific lifecycle error before `finalize_publication`, where snapshot,
  receipt, and public writes begin. Production ordering is correct and remains
  unchanged.
- `/home/workdir/openWEPP` and `/home/workdir/openwepp-task-tmp` resolve to
  distinct sibling directories with distinct inode identities. The external
  scratch root is not a repository descendant. In contrast,
  `/home/workdir/openWEPP/target/task-tmp` is a repository descendant and must
  fail the unrelated-root invariant.

Independent replay from `/home/workdir/openWEPP`:

```text
TMPDIR=/home/workdir/openwepp-task-tmp cargo nextest run \
  --test assurance_v2_publication_contract \
  draft_subject_root_is_stable_but_cannot_publish --profile quick
PASS: 1 test run, 1 passed, 36 skipped, 3.182 s

TMPDIR=/home/workdir/openWEPP/target/task-tmp cargo nextest run \
  --test assurance_v2_publication_contract \
  draft_subject_root_is_stable_but_cannot_publish --profile quick
EXPECTED FAIL: 1 test run, 0 passed, exit 100
actual typed message: staging and repository roots must be unrelated
expected external-topology message: report
'linear-groundwater-reservoir-recurrence' is DRAFT; publication requires
APPROVED
```

This reproduces the diagnosed invocation-topology defect: an in-repository
`TMPDIR` correctly trips confinement before lifecycle validation, while the
external scratch topology reaches the intended DRAFT rejection.

## Exact Assertion And Side-Effect Proof

The final test no longer accepts an arbitrary invalid message containing
`DRAFT`. It requires exact equality with the selected report's governed error:

```text
report 'linear-groundwater-reservoir-recurrence' is DRAFT; publication requires APPROVED
```

Before the publication call, the test captures every seeded public-tree file
path and byte vector. After rejection it compares the complete capture to the
original and separately requires the snapshot root to contain zero entries,
which excludes snapshot and receipt creation. Static production inspection
confirms lifecycle validation precedes finalization and therefore precedes all
snapshot, receipt, and public writes. The passing external-topology replay
executes all three assertions.

## Reviews And Full-Workspace Evidence

- `rust_code_review.md` is unconditionally `APPROVED` with its lifecycle-identity
  and public-side-effect finding accepted and fixed.
- `rust_qa_review.md` is unconditional `PASS`; its exact-current-diff,
  lifecycle-error, public non-mutation, formatting, lint, and stale-evidence
  findings are accepted and fixed. The review disposition now agrees. No Rust
  finding is open.
- The retained terminal command is
  `TMPDIR=/home/workdir/openwepp-task-tmp cargo nextest run --workspace
  --profile full` from `/home/workdir/openWEPP` at the HEAD and reviewed blob
  above. Its pre-run working-tree fingerprint is
  `4243658ad03b52e20ff621b8c957664abd549b65cbb0b6feb20aa3957546360d`.
- The raw terminal log independently hashes to
  `d125d5ff4c5050e5068ac07bd698aa07ae2118ce46a204e1dfc679e159d9730d`;
  metadata records exit `0` and a 3,302-second run. The log starts 2,325 tests
  across 209 binaries and terminates with 2,325/2,325 passed, 55 slow, and 33
  declared full-profile skips in 3,300.706 seconds. It contains no failure,
  timeout, abort, signal, or test-error marker.
- The assurance test, vegetation contract test, vegetation science contract,
  and contract index current filesystem modification times all precede the
  retained run start. The explicitly recorded assurance blob still matches;
  subsequent changes are package evidence and lifecycle documentation only.
  The full-workspace evidence therefore remains reusable for the authorized
  vegetation hold lift.

## Hold Lift, Catalogs, And Lifecycle

The native-forest authority-reframe package consistently reports
`complete / exact-head full-workspace pass`. Its exact former blocker is
satisfied through the authorized assurance closure. The lift is validation and
lifecycle bookkeeping only: no production vegetation implementation, default,
calibration, validation, cutover, or release is authorized.

`docs/ROADMAP.md`, the vegetation backlog note, `docs/backlog/TRACKER.md`, and
the work-package catalog agree that the authority reframe is complete while
the coupled vegetation implementation successor remains held on complete
schema/constitutive authority and contract-first gates. They also preserve the
earlier authority-closure package as historical executed-hold evidence without
reopening universal site-value selection.

The assurance package, roadmap, and catalog consistently remain
`complete / terminal verification pending` while this dual-verifier step is in
progress. The kickoff exists only under `prompts/completed/`; its reproduced
SHA-256 is
`a68817d4fc21bad2ad2f55fd9532109172646374da6de123831e19bf328c9a5b`.

## Lightweight Terminal Checks

Ran from `/home/workdir/openWEPP` after the lifecycle wording corrections:

```text
git diff --check
PASS

cargo fmt --all -- --check
PASS

markdown-doc lint --path <assurance package> --format plain
PASS: 15 files, 0 errors, 0 warnings before this verifier artifact

markdown-doc lint --path <native reframe package> --format plain
PASS: 35 files, 0 errors, 0 warnings

markdown-doc lint --path <held coupled successor> --format plain
PASS: 34 files, 0 errors, 0 warnings

markdown-doc lint --path <SC-VEGETATION-001, ROADMAP, tracker, vegetation
backlog, and work-package catalog individually> --format plain
PASS: each path, 0 errors, 0 warnings
```

The superseded pre-review full-workspace attempt and historical invalid-scratch
failures remain labeled as non-passing evidence. The terminal external-scratch
run alone carries the 2,325/2,325 correctness claim. Lifecycle, review, prompt,
catalog, and evidence language now match the observed state.

## Disposition

`PASS` — root cause, unchanged production assurance behavior, exact regression
proof, review closure, terminal full-workspace identity/result, external scratch
topology, vegetation hold lift boundaries, catalog consistency, prompt digest,
and Markdown/diff hygiene are all supported. No residual severity finding
blocks final package reconciliation.
