# Finding Disposition

Evidence class: Static + Ran

All independent-review findings were accepted and corrected in the current
package. No finding is rejected, deferred, or assigned to follow-up work.

| Finding | Disposition | Closure evidence |
| --- | --- | --- |
| `EDIT-A-001` canonical-converter bypass | Accepted; resolved | Production options expose only language/mode; converter and faults are module-private; receipt truthfully identifies `uk2us`. |
| `EDIT-A-002` incomplete packet binding | Accepted; resolved | Exact top-level `draft_outputs`, global uniqueness, scoped replacement, no-op validation, and malformed/duplicate/extra negative contracts pass. |
| `EDIT-A-003` unbound and concurrent reads | Accepted; resolved | Every candidate read binds to the held full-tree snapshot; active and staged tree digests/modes are rechecked; deterministic drift contracts pass. |
| `EDIT-A-004` partial-old cleanup rollback | Accepted; resolved | Commit point is explicit; rollback-capable failures restore, post-commit cleanup never restores partial old state, and its error carries the receipt. |
| `EDIT-A-005` metadata loss | Accepted; resolved | Full mode map is preserved and verified; descriptor-relative `fchmod` is followed by inode synchronization. |
| `EDIT-A-006` missing negative/determinism tests | Accepted; resolved | Lifecycle, review, CLI, converter, packet, receipt, drift, rollback, cleanup, mode, build/check, and idempotence contracts pass. |
| `EDIT-A-007` 2,000-line warning record | Accepted; resolved | `line-count-governance.md` records the exact touched manifest, rationale, owner, split intent, and 3,000-line sunset. |
| Review A supplemental cleanup-state bypass | Accepted; resolved | Recovery-state detection occurs immediately after lock; a cleanup-fault/retry contract proves no-op checks fail closed. |
| Review A supplemental mode durability | Accepted; resolved | Mode changes are synced on the opened inode before generation exchange. |
| `B-01` scheduling-sensitive converter communication | Accepted; resolved | Concurrent pipe draining, unconditional wait/reap, nonzero-error precedence, 32 immediate-exit trials, and a 2 MiB streaming round trip pass. |
| First heavy closure: `normalize_report_with_controls` CRAP `31.75489881112413` | Accepted; resolved | Preparation and application orchestration were separated; canonical remediation CRAP is 8 with 100% coverage. |
| First heavy closure: `clone_v2_tree` CRAP `30.000000000000018` | Accepted; resolved | Regular-file, directory-mode, and root-mode cloning were separated; canonical remediation CRAP is 6 with 100% coverage. |
| `RR-A-001` non-UTF-8 fixture race | Accepted; resolved | The successful fake converter now consumes stdin before emitting invalid bytes; the exact regression passed 20/20 stress runs and both re-reviews. |

Review A and Review B independently rechecked the terminal implementation and
both recommend `PASS`. Their artifacts preserve the initial failures, required
fixes, exact candidate identities, and final executed evidence.

The fresh post-remediation census passes at 2 raw / 2 adjudicated / 0
actionable. Maximum touched production CRAP is exactly 30.0; maximum
normalization-module CRAP is `prepare_normalization` at
`15.101256515775034`.
