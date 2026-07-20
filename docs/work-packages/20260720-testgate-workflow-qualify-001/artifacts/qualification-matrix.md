# Black-Box Qualification Matrix

Status: scaffold contract. Freeze exact scenario schema and expected machine
fields at intake before the executor runs any case.

| Case | Scenario | Evidence path | Required result |
| --- | --- | --- | --- |
| `Q01` | Package absent from base | Real helper in disposable repository | `SCAFFOLD_COMMIT_REQUIRED`; no execution authority or heavy spawn. |
| `Q02` | Malformed heading, undeclared path, or package widening | Real helper in disposable repository | Admission/reconciliation rejects before spawn. |
| `Q03` | Diff, docs/schema, artifact, prompt, or line-count blocker | Real helper plus bounded probe | `LIGHT` failure; heavy spawn count zero. |
| `Q04` | Toolchain, environment, binary, fixture, policy, configuration, runner, or concurrency mismatch | Real helper plus bounded probe | Non-`READY`; heavy spawn count zero. |
| `Q05` | Attempt/output collision, alias, source/index mutation, or unsafe cache key | Real helper plus bounded probe | `INVALID`; prior bytes unchanged; spawn count zero. |
| `Q06` | Planner inventory omission or argument drift | Real verifier with independent enumeration | Audit/receipt rejected; no replacement inventory. |
| `Q07` | Documentation-only edit after executable PASS | Two disposable attempts | Docs node stale; eligible executable receipt imported. |
| `Q08` | Probe heavy A passes; probe heavy B infrastructure-fails | Real staged executor across two attempts | A total spawn count one; B alone resumes when eligible. |
| `Q09` | Same infrastructure cause recurs after allowed retry | Real staged executor | Linked tooling defect blocks another heavy spawn. |
| `Q10` | Terminate before aggregate receipt; destroy worktree/runner directory | Fresh environment plus durable evidence re-ingestion | Hash chain verifies; eligible nodes import; ineligible receipts retain exact reason. |
| `Q11` | Active plus multiple pending events and concurrency timeout | Frozen hermetic workflow/concurrency harness plus retained provider records | One active/newest pending, typed timeout, no duplicate heavy spawn. |
| `Q12` | Combined parity/economy pass | One real current trusted combined run | One full Nextest process supplies functional and LCOV/CRAP lineage. |
| `Q13` | Parity/history/economy non-adoption | Real planner and retained measurement evidence | Separate nodes plus typed `COMBINATION_NOT_ADOPTED`; no deduplication claim. |
| `Q14` | Evidence/docs-only accepted review change | Disposable post-review attempt | Only invalidated light/docs nodes run; heavy count unchanged. |
| `Q15` | Planner/policy/schema/workflow/test accepted review change | Disposable post-review attempt | Audit and affected receipts stale; exact invalidated probe nodes rerun. |

Each row executes once. A matrix case is `PASS` only when its real consumer path,
status, process count, artifact set, and negative proof all match. Reasoning from
source or accepting a producer-only unit result is insufficient.
