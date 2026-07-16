# ASSURE-04D Review Disposition

Status: PASS — every review and heavy-HOLD finding dispositioned; accepted
remediations and independent renewals complete

Evidence class: Static + Ran

| Finding | Disposition | Resolution |
| --- | --- | --- |
| A1/B1 release verifier did not reconstruct authority | Accepted | Production verification now requires a nonempty strict public catalog and exact builder, opens every namespaced snapshotted source as a production v2 repository, revalidates schema/catalog/principals/lifecycle/findings/approvals/transfer, recalculates all layered roots, and compares complete source/public payloads. Positive reconstructed production, forged-empty, forged-root, unknown-builder, missing artifact, and relabeled test-domain cases execute. |
| A2 anti-omission root grammar | Accepted | Subject normalization removes only explicit transition leaves. Exact catalog bytes are bound after normalizing report-manifest identity values that would create cycles. Executable classification and per-leaf mutation tests prove current layers, while a future unclassified subtree leaf remains subject-bound. Finding IDs are sorted; review state/decision and all publication authorization/state leaves enter later roots. |
| A3 approval ledger mismatch | Accepted | Every approval ledger must equal the declared finding-ledger root before approval-root calculation. A stale-ledger publication negative executes. |
| A4/B3 held roots, ancestry, hard links | Accepted | Report contexts retain repository/staging directory capabilities through commit; public/snapshot capabilities remain locked. Pairwise ancestry recursively walks held descriptor subtrees with a visited device/inode set, including mounted descendants. All four ambient identities and pairwise relationships are checked before preparation, after final replay, and immediately before exchange. Every regular read rejects multiply linked files. Symlink alias, staging symlink, FIFO, hard-linked staging, and hard-linked receipt negatives execute. |
| A5 reciprocal cross-link | Accepted with contract correction | A model narrative cannot link a report before it exists. 04D now enforces the report's real canonical Markdown link and byte equality between the external model narrative and approved subject input. Missing link, wrong public path, and narrative drift fail. ASSURE-08 retains later discovery/navigation backlinks. |
| A6/B6 proof matrix | Accepted | Publication tests expanded from 10 to 25 plus layer unit tests. Added principal kind/domain/role, exact-message competence/independence/withdrawn/superseded/missing-transfer rejection, research-object, schema/catalog/dependency/generated-byte, distinct old/new reader, exact receipt retry/conflict, multi-report production replay, public/immutable special-file, real draft/in-review, reconstructed-forgery, two-report, fault, bootstrap, link, confinement, ledger, and release-identity cases. Deterministic fault points avoid environment authority. The final two-test increase is only the strict-Clippy-driven split of existing negative matrices. |
| A7/B5 release transfer/discovery | Accepted | Preflight binds commit to checkout `HEAD` and configuration to `openwepp-release-default-v1`. Tests derive the selected checkout commit. The release runner calls a bounded materializer that is executed by contract test; copied snapshot/receipt authority is replayed, payload bytes and discovery sidecar are inspected, and checksums execute. The default workflow correctly remains zero-report until production assurance exists. |
| A8/B7 line disposition | Accepted | `publication.rs` is WARN with decomposition intent; lifecycle/identity logic remains split out and `v2.rs` is 2,984 lines, below the hard block. |
| B2 false postcommit error and durability | Accepted | Files sync on creation; prepared trees sync bottom-up and parents sync before rename. Snapshot/receipt installs have no-replace semantics. Atomic exchange is the irreversible success point; cleanup/final sync cannot return a false precommit error. Three injected precommit boundaries prove byte-identical old generation and successful retry. |
| B4 public ownership/link weakness | Accepted | README-only bootstrap requires the frozen zero-report digest; directory enumeration rejects empty unknown structure; prior reports require verified receipt/snapshot; canonical public paths, exact narrative bytes, and real Markdown targets are enforced. |
| B8 Markdown target forgery | Accepted | Canonical-link recognition now accepts only `pulldown-cmark` link events. Renderer-backed negatives cover ordinary and malformed fence closers, multiline raw HTML, inline/indented code, comments, escapes, and images; a typed directive rendered only inside raw HTML fails publication. |
| B9 receipt crash retry | Accepted | An exact transaction-named receipt preparation is synced, reused, and renamed on retry; different bytes return `SnapshotConflict`. Both cases execute without public mutation. |
| B10 reader proof | Accepted with contract correction | Atomic directory exchange guarantees each individual file lookup/read returns complete old or new bytes, not a transaction across separate pathname reads. The test transitions between distinct approved report realizations and accepts only exact old/new byte streams. Coherent multi-file audit reads use the immutable receipt-bound snapshot. |

No disposition constitutes scientific, reproduction, publication, release-
owner, or human approval. Both independent reviewers returned final Phase 4
PASS after the accepted remediations. The first heavy sequence later exposed
strict-Clippy findings only in the publication integration test. Both
independent reviewers returned bounded PASS on its no-suppression,
semantics-preserving restructuring. Current focused and quick evidence is
recorded in `gate-results.md`; the restarted heavy closure remains separate.

## Second Heavy-HOLD Disposition

The restarted heavy sequence passed format, strict Clippy, full Nextest, and
deny, then returned HOLD on seven actionable CRAP rows in touched assurance
code. The findings were accepted. No adjudication was added. Each oversized
decision path was decomposed into bounded contract-named helpers, with focused
tests for CLI option binding, ambient-root creation/confinement, and every
finding disposition.

Both independent reviewers returned PASS on the production remediation. They
confirmed fail-closed CLI/lifecycle, confinement, receipt, snapshot,
trust-domain, and atomic-publication semantics; no suppression/evasion; exact
retained output; protected/write-set compliance; and zero focused touched-file
CRAP rows greater than 30. This review evidence does not replace the required
fresh full-workspace heavy restart.
