# Finding Disposition

Evidence class: `Static + Ran`

| Finding | Severity | Disposition | Correction |
| --- | --- | --- | --- |
| Writable sandbox roots could overlap protected paths. | High | `ACCEPTED / FIXED` | Resolved-path ancestor/descendant checks reject repository, Harvard, attempt, execution, and mutual overlap; adversarial tests pass. |
| Freeze and terminal validation used the retired execution-root token. | High | `ACCEPTED / FIXED` | Custody owns the token everywhere; freeze regeneration and terminal authority check the same path. |
| Holdout receipt/layout and terminal validation differed. | High | `ACCEPTED / FIXED` | Holdout emits the complete hashed receipt and terminal arithmetic reads the separate output object root. |
| Opening token omitted the exact resolved invocation. | High | `ACCEPTED / FIXED` | The full inner sandbox argv is written and fsynced before Harvard access and terminally compared. |
| `validate_executor.py` depended on the retired CSV schema. | High | `ACCEPTED / FIXED` | Validator now loads the canonical direct JSON plan and scans prospective tools. |
| CAL disposition artifacts still named incident 004 as current. | High | `ACCEPTED / FIXED` | Matrix, disposition, handoff, identifiability, package, and stage ledger now identify incident 005. |
| Direct-plan validation did not enforce exact structure/topology. | Medium | `ACCEPTED / FIXED` | Exact phases, IDs, order, predecessor chain, required fields, executables, and Harvard policy are fail-closed. |
| Two verifier labels did not evidence separate invocations. | Medium | `ACCEPTED / FIXED` | Each receipt has a generated invocation ID and the barrier requires two distinct IDs. |
| Preflight-only consumed its initially empty output root. | Low | `ACCEPTED / FIXED` | Preflight creates no output directory or file. |
| Order 2 progress ledger was stale. | Low | `ACCEPTED / FIXED` | Completed implementation and validation phases are checked. |

No accepted finding remains open or deferred.
