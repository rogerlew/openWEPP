# Pre-Heavy Acceptance Matrix

Status: implementation control map. Focused producer/consumer seam evidence is
owned here; the independent black-box executions are `Q01` through `Q15` in
`TESTGATE-WORKFLOW-QUALIFY-01`, which freezes this package's landed bytes and
cannot repair them.

| Case | Injection | Required observation |
| --- | --- | --- |
| `A01` | Package absent from base commit | Scaffold validation reports `SCAFFOLD_COMMIT_REQUIRED`; execution is not authorized. |
| `A02` | Malformed write-set heading, undeclared path, or current-package widening | Package/schema or terminal reconciliation fails before process spawn. |
| `A03` | Diff whitespace, Markdown/schema error, missing artifact, active prompt mismatch, or 3000+ nonexempt Rust file | `LIGHT` prerequisite fails; heavy-spawn counter remains zero. |
| `A04` | Toolchain, environment, binary, feature, fixture, policy, configuration, runner, or concurrency identity mismatch | Audit is non-`READY`; heavy-spawn counter remains zero. |
| `A05` | Attempt-root/output collision, path alias, source/index mutation, or incomplete cache key | Audit is `INVALID`; prior attempt and cache remain unchanged; spawn counter remains zero. |
| `A06` | Planner omits or changes one inventory item or argument | Independent verifier enumeration/comparison rejects the audit or receipt. |
| `A07` | Documentation-only edit after executable PASS | Documentation node becomes stale; unchanged executable per-node receipt remains current and is imported. |
| `A08` | Heavy A passes, heavy B infrastructure-fails | Attempt retains both node results; next attempt imports target-reusable A by verified receipt ID and spawns B only. |
| `A09` | Same infrastructure cause recurs after the one allowed retry | Linked tooling defect blocks another heavy spawn. |
| `A10` | Runner/job resets before aggregate receipt | Uploaded digest-bound attempt and per-node records are indexed and re-ingested; every `SAME_EXECUTION` or otherwise ineligible receipt records its exact §10.4 rejection reason before rerun. |
| `A11` | Active run plus multiple pending requests and a concurrency timeout | One active and only newest pending request remain; timeout is typed and does not trigger parallel or duplicate heavy work. |
| `A12` | Full/coverage parity and economy thresholds pass | Planner emits one combined instrumented node; duplicate full inventory is rejected. |
| `A13` | Parity, compatible history, or economy threshold fails | Separate nodes remain with typed `COMBINATION_NOT_ADOPTED`; no efficiency claim is made. |
| `A14` | Accepted review changes only evidence/docs after heavy PASS | Only invalidated light/docs nodes rerun; heavy spawn counts remain unchanged. |
| `A15` | Accepted review changes planner, policy, schema, workflow, tests, or another heavy input | Audit and affected receipts become stale; only mechanically invalidated nodes rerun. |

Every non-`READY` case uses an injected spawn counter at the actual executor or
workflow boundary. Unit-level producer output alone cannot close `A03` through
`A15`; the real local helper or trusted-workflow contract path must consume the
artifact. Trusted-workflow cases may use a hermetic workflow harness rather than
live GitHub dispatch, which remains outside package scope.

Implementation evidence before landing includes schema rejection of a forged
check set, binary-image binding across LIGHT/audit/HEAVY, exact audit
reconstruction from the durable ledger, automatic recurrence defect creation,
checkpoint import without an aggregate receipt or matching plan ID, symlink-
confined output import, durable-history snapshot/indexing, and workflow-source
contracts for restore and persistent paths. The follow-up qualification package
owns the once-only real-helper and provider-path observations; those results are
not claimed by this implementation package.
