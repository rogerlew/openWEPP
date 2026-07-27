# Finding Disposition

Status: `PASS / ALL FINDINGS CLOSED`

Evidence class: `Static + Ran`

| Finding | Disposition | Evidence |
|---|---|---|
| `TESTGATE-ASSURANCE-HISTORICAL-ROOT-001`: mutable current DRAFT roots were incorrectly required to equal immutable historical registry roots | Accepted and corrected | Production integration loads the divergent current state and emitted impacts retain registry `source_root` and `assessed_realization_root` |
| Review A/B high: four-field subset parsing was fail-open against malformed or unbound current locks | Accepted and corrected | Canonical identity/review schemas, exact identity-lock byte binding, and report association are enforced; missing-field, extra-field, and digest-mismatch tests pass |
| Misleading assessed-root helper/error naming | Accepted and corrected | Loader is now `verify_current_review_locks`; association and identity failures have specific error codes |

No finding was deferred. Coverage/CRAP remains
`DEFERRED_TO_QUALITY_CI` by canonical policy, not as a finding disposition.
