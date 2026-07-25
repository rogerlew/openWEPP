# Order 4 Pre-Implementation Intent Plan

Evidence class: Static.

Risk: `CRITICAL`.

## Owned Change

Implement a manual forest1 quality-observatory workflow and a deterministic
local controller. The controller owns untrusted queue normalization, typed
TESTGATE-priority deferral, nonblocking lease acquisition, supervised
transition execution, bounded shutdown, compact artifact indexing, and cleanup.

The workflow supplies provider data and exact execution identity. It may invoke
the existing Order-3 `transition` command only after admission. It remains
manual-only, uses exact forest1 labels, and has a stable concurrency group
distinct from TESTGATE.

## Safety Invariants

- Exact lowercase 40-character source SHA and exact checkout are mandatory.
- Only current repository `testgate-shadow.yml` runs whose forest1 execution
  job is queued or in progress count as occupancy.
- Retired Omarchy records, other repositories, other workflows, terminal runs,
  and non-forest1 jobs do not count.
- Malformed or incomplete provider input fails closed as `INVALID_OCCUPANCY`.
- TESTGATE occupancy before lease acquisition starts no child.
- Occupancy after `full`, after `science-manual`, or before report work starts
  no next stage and records `DEFERRED_TESTGATE_PRIORITY`.
- Active children are polled at no more than 30-second intervals, terminated
  on TESTGATE occupancy, and allowed at most 60 seconds for finalization.
- Partial evidence is retained locally but cannot use the complete publication
  artifact name or status.
- The exact 11-file allowlist and 100 MiB ceiling are checked before upload.
- Metric debt does not fail execution integrity; identity, collection,
  occupancy, or publication corruption does.

## Selected Gates

- Python bytecode compilation and controller self-test.
- Focused integration tests for workflow structure, occupancy truth table,
  before-acquisition deferral, all three race boundaries, active-child
  termination, artifact allowlist/size rejection, exact SHA, and observational
  debt semantics.
- Existing TESTGATE workflow/source contracts.
- Rustfmt and warnings-denied Clippy for changed integration tests.
- Canonical Markdown lint, diff hygiene, declared-write-set reconciliation,
  prompt state, and line-count governance.
- Two independent workflow/security reviews and two independent terminal
  verifications.

No live QA, TESTGATE, external workflow, or heavy run is selected.
