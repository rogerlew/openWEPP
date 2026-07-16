# ASSURE-04D Early Design Audit And Disposition

Status: all findings accepted before production edits

Evidence class: Static design review

Two independent coding-agent audits reviewed the initial package and relevant
04C/release consumers. Both returned `HOLD`. These reviews are engineering
threat analysis only; they are not scientific or publication approval.

## Audit A — Root And Approval Integrity

| Finding | Disposition | Contract correction |
| --- | --- | --- |
| Subject root omitted schema, catalog, assembly-tool, and generated-output identities. | Accepted | Bind schema/catalog/principal/tool/plan and complete staging identities. |
| Excluding an entire review/publication subtree permits findings and approvals to escape the lock. | Accepted | Use layered subject, finding-ledger, approval, transfer, snapshot, and receipt roots. Accepted findings require `resolved_and_verified`. |
| No authoritative representation of successful publication existed. | Accepted | Authored source stops at `APPROVED`; an append-only verified receipt establishes `PUBLISHED`. |
| Free-text approver names and attestations are not identities or competence evidence. | Accepted | Resolve stable IDs through a strict principal registry with kind, trust domain, authority reference, and roles. The tool validates declarations but does not authenticate real-world judgment. |
| `TEST_ONLY` was a marker, not a trust-domain boundary. | Accepted | Separate types/entry points; bind trust domain into every root and expose `TEST ONLY` on every synthetic surface. Production verification rejects test-domain bytes. |
| Snapshot and transfer fields were circular or caller-authoritative. | Accepted | Content-address normalized snapshot without self-naming; verifier receives expected release commit/configuration independently. |
| Future schema fields could evade root coverage. | Accepted | Strict schema and deserialization plus exact leaf classification: unknown fields fail admission and any unclassified admitted leaf defaults to subject-bound. Add mutation coverage. |

## Audit B — Transaction And Consumer Integrity

| Finding | Disposition | Contract correction |
| --- | --- | --- |
| Atomic mutation across public and snapshot roots is impossible. | Accepted | Snapshot and receipt are immutable pre-commit artifacts; atomic exchange of the complete `usersum/assurance` generation is the sole public commit. Orphans are harmless and retry-verifiable. |
| A staging path/hash check leaves a check/use race. | Accepted | Return an opaque checked-staging capability holding descriptor-confined captured bytes; publish those bytes and reverify identities before commit. |
| All-mode could omit or retain stale/hidden material. | Accepted | Require exact report-ID membership and reject hidden, temporary, symlink, special, and unknown managed entries. |
| Snapshot existence checks race concurrent writers. | Accepted | Hold root locks, install content-addressed snapshots and receipts with Linux no-replace semantics, and verify the complete existing tree. |
| Lexical root checks do not prove separation. | Accepted | Open roots descriptor-relatively, compare device/inode ancestry, hold descriptors/locks, and reject overlap. |
| Cleanup authority was too broad. | Accepted | Build and exchange the whole owned assurance generation; remove only prior machine-catalog-owned entries and fail on unknown managed report content. |
| The actual release gate remained zero-report-only. | Accepted | Amend the existing release-transition script and release-candidate driver to invoke production snapshot/receipt verification before release-directory creation when v2 inputs are supplied. Preserve zero-report behavior otherwise. |
| Package retention language conflicted with external-root confinement. | Accepted | Execute only in disposable external roots; retain byte-for-byte non-operational evidence copies under package artifacts. |
| “Actual WEPPcloud consumer” overclaimed discovery. | Accepted | Prove the current cmarkgfm renderer only. Manifest/navigation/search and vendoring remain ASSURE-08. |
| Publication date and receipt/manifest identity needed deterministic authority. | Accepted | Publication date is authored source data; normalized manifests have no self-hash; immutable receipt names and binds the snapshot. |

No finding was rejected or deferred. The package amendment and frozen
publication contract implement these corrections before contract-derived tests
or production source changes.
