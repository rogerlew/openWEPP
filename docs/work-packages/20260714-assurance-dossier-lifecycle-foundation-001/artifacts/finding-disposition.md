# Finding Disposition

Status: `complete`; all initial-review findings are accepted, remediated, and
independently verified on the terminal candidate.

The parent read each sealed review only after both reviewers completed their
independent assignments. Reviewer A did not read Review B; Reviewer B did not
read Review A. No finding is deferred merely to close this package.

## Disposition Summary

| Finding | Severity | Disposition | Closure treatment | Affected criteria |
| --- | --- | --- | --- | --- |
| `ASSURE-A-001` | high | `accepted` | Replace the single free-text review lock with typed authors, role/expertise and independence disclosures, separate scientific/publication approvals, structured finding dispositions, residual disagreement, and retained approval history. | `ASSURE-LIFE-001..004`, `ASSURE-BUILD-004`, `ASSURE-GOV-001` |
| `ASSURE-A-002` | high | `accepted` | Replace the broad verification `PASS` with an aggregate nonpass state and typed per-obligation records carrying realization, date, evidence, criterion/tolerance, result, mandatory flag, and status. | `ASSURE-LIFE-003`, `ASSURE-PILOT-002..003`, `ASSURE-GOV-001` |
| `ASSURE-A-003` | high | `accepted` | Remove “improved” downstream-language from the narrative; describe only changed snow-process defaults absent separate runoff/erosion/watershed evidence. | `ASSURE-PILOT-002..004`, `ASSURE-GOV-001` |
| `ASSURE-A-004` | medium | `accepted` | Add a complete content-identified authoring-analysis record under `assurance/`, validate its retained inputs/accepted outputs, add it to the typed DAG and scientific review root, and finalize the package-facing review record. | `ASSURE-LIFE-004`, `ASSURE-BUILD-004`, `ASSURE-GOV-001`, `ASSURE-CLOSE-004` |
| `B-001` | high | `accepted` | Reject symlinks at snapshot root/ID/manifest/files and descendants using no-follow metadata checks; cover the reproduced `files/` symlink case. | `ASSURE-BUILD-005`, `ASSURE-REL-001`, `ASSURE-SEC-001`, `ASSURE-TEST-001` |
| `B-002` | high | `accepted` | Model the hand-authored narrative as a public DAG input, include it in plan/review/snapshot identities, and test drift and same-ID conflict. | `ASSURE-LIFE-002/004`, `ASSURE-BUILD-002..005`, `ASSURE-REL-001`, `ASSURE-TEST-001` |
| `B-003` | high | `accepted` | Closed by the same typed, split-scope, historical review model as `ASSURE-A-001`; add self-review, unresolved finding, payload drift, and terminal-state tests. | `ASSURE-LIFE-001/002/004`, `ASSURE-BUILD-004`, `ASSURE-TEST-001` |
| `B-004` | high | `accepted` | Validate all Markdown and export bytes for private paths/common token families; reject unsafe/nonlocal links and Markdown-active scalar injection; add malicious field/list/table/URI/export cases. | `ASSURE-PILOT-004`, `ASSURE-SEC-001`, `ASSURE-TEST-001` |
| `B-005` | medium | `accepted` | Apply the source-size bound to schemas and bind every version-1 schema to its canonical path, ID, structure/version, and reviewed digest; add version/ID/size drift cases. | `ASSURE-LIFE-004`, `ASSURE-BUILD-001/002`, `ASSURE-SEC-001`, `ASSURE-TEST-001` |
| `B-006` | medium | `accepted` | Enforce lifecycle snapshot eligibility before output writes; prohibit `DRAFT` and test the declared treatment of every state. | `ASSURE-LIFE-002`, `ASSURE-BUILD-005`, `ASSURE-REL-001`, `ASSURE-TEST-001` |
| `B-007` | high | `accepted` | Map assurance lifecycle to wepppy's accepted document status while retaining the exact assurance lifecycle in a separate export field; update the handoff and validate a transformed fragment with the real read-only downstream parser. | `ASSURE-XREPO-001`, `ASSURE-REL-001`, `ASSURE-TEST-001` |

## Pre-Freeze Accepted-Fix Audit

Reviewer B continued its independent audit after the initial disposition and
identified additional closure gaps within the already accepted findings. The
parent accepted every item; none was reclassified or deferred.

| Finding | Additional accepted closure treatment |
| --- | --- |
| `B-001` | Validate existing snapshot layout before reading it; create missing root components without following symlinks; use exclusively created retry staging; preserve unknown collision directories; bound existing-file comparisons; and test that a symlink ancestor receives no outside write. |
| `B-002` / `ASSURE-BUILD-002..003` | Represent every tracked authoring input and accepted output as its own graph node and planned input; freeze the complete input identity and compiler-source path set across open and every operation; reject undeclared files anywhere in either generated root; bind the review path and every publication-root template into the dossier output identity. |
| `B-003` | Replace per-entry payloads with prefix-bound ordered-history payloads, require the publication approval to terminate locked history, reject edit/removal/reorder of prior approvals, and validate participant fields in pending and rejected records. |
| `B-004` | Add PEM, PGP, GitHub, OpenAI, Stripe, Google, Slack, and generic long-token families; reject contextual POSIX, Windows, and UNC absolute paths; prohibit Markdown fragments in version 1; restrict manifest paths to canonical portable ASCII segments; reject multiline reference destinations and recursive template tokens; and run full public rendering/link checks in `validate` and `plan`. |
| `B-005` / `ASSURE-BUILD-002` | Read bounded sources through one limited file handle; stream evidence and named-file hashing; bind graph fingerprints to explicit contract/schema versions, stable node/dependency identities, repository-relative path, byte length, and raw bytes. |
| `B-007` | Apply the lifecycle-to-downstream-status mapping to the narrative record as well as the generated method and dossier; exercise all three records for every lifecycle state. |

The expanded focused contract now contains 10 crate tests and 18 integration
tests. The implementation re-freeze identity is recorded in
`owned-file-manifest.md`; the complete heavy evidence and both accepted-fix
verifications pass.

## Code-Quality Closure Remediation

Ran: the first complete heavy sequence passed formatting, workspace clippy,
full workspace nextest, and deny, but correctly failed the fresh CRAP gate on
five touched functions. No waiver was added. The parent split CLI dispatch,
plan rendering, evidence validation, verification-obligation aggregation, and
error formatting into single-purpose helpers without changing their decisions
or output. Focused clippy, 10 crate tests, and all 18 integration tests passed
after the refactor and public outputs were rebuilt.

Static and independently reviewed: each resulting function has a worst-case
zero-coverage CRAP value of at most 20, below the closure ceiling of 30.
Reviewer B traced the refactored control flow against the prior behavior and
reported no semantic regression or new closure gap. Reviewer A independently
traced dispatch, plan formatting, evidence rules, verification precedence, and
error/exit behavior. The reviewer also reconstructed the prior dossier bytes
by replacing only the publication-root digest, confirming that the public
scientific interpretation did not change. A renewed canonical coverage-backed
CRAP run and the complete heavy sequence remain the terminal authority.

## Remediation Closure Evidence

These parent implementation claims were independently verified by the two
named reviewers.

| Finding | Implemented evidence |
| --- | --- |
| `ASSURE-A-001` / `B-003` | `review.yaml`, `review.schema.json`, `review.rs`, and `authoring.yaml` now record conclusion authors, named reviewer roles/expertise/independence, separate scientific and publication approvals, structured findings/dispositions, residual disagreement, history, and an independently approved agent-output root. Self-review, unresolved closure blockers, stale roots, changed review payloads, and all terminal lifecycle states fail closed in integration tests. |
| `ASSURE-A-002` | `dossier.yaml` records six exact verification obligations with realization/date/criterion/result/evidence/status; aggregate status is mechanically derived as `BLOCKED`, while three narrow historical obligations remain individually `PASS`. The generated dossier exposes the full table. |
| `ASSURE-A-003` | The snow/frost narrative now describes changed snow-process states and timing without calling downstream runoff, erosion, or watershed accuracy improved; the dossier explicitly excludes those claims. |
| `ASSURE-A-004` | `authoring.yaml` content-identifies 17 inputs, six retained outputs, six accepted decisions, and root `01aa0936...fae9ed`; it is schema-checked, represented by record/input/output DAG nodes and `plan` paths, bound into both roots and snapshots, and independently procedurally approved. A unique authoring-only input test proves the generic dependency path. |
| `B-001` | `snapshot.rs` uses no-follow creation and checks for the snapshot-root chain, ID directory, manifest, `files/`, and recursive descendants; layout is checked before and after bounded reads. Exclusive retry staging never deletes unknown collisions. Integration cases reproduce symlinks at every named location, prove no outside ancestor write, and preserve a collision sentinel. |
| `B-002` | Typed narrative metadata is required by the catalog/dossier schemas; narrative bytes participate in plan, scientific/publication roots, generated links, snapshots, and same-ID conflict tests. The compiler freezes the complete input/path set across open and every operation, binds the review path and full template set into the rendered dossier identity, and makes `check` reject missing, orphaned, symlinked, or special generated-root entries. |
| `B-004` | `publication.rs` validates every public scalar, rendered Markdown page, and YAML export for contextual absolute paths, expanded credential/token/PEM families, raw HTML, active Markdown injection, unsafe/nonlocal links, fragments, URI schemes, and reference links. Trim-stable scalar grammar, nonrecursive template replacement, canonical portable path grammar, and all-command rendered validation close manifest and read-only-command bypasses. |
| `B-005` | The compiler requires exactly six known version-1 schema paths/IDs/dialects/versions/digests, uses a single bounded handle before parsing, and streams evidence, named-source, and graph identities. Fingerprints bind explicit versions, ordered identities, paths, byte lengths, and raw bytes; unit tests independently reconstruct the contract and stream a sparse 8 MiB asset. |
| `B-006` | Lifecycle methods declare review-lock and snapshot eligibility semantics for all five states; `DRAFT` snapshot attempts fail before writes, while terminal historical states require current approvals. Unit/integration tests enumerate every state. |
| `B-007` | Export rendering maps `DRAFT` to `draft`, `CANDIDATE`/`PUBLISHED` to `active`, and `SUPERSEDED`/`WITHDRAWN` to `deprecated`, retaining `assurance_lifecycle`. Integration tests exercise method, dossier, and narrative records in all five states; the current five-record fragment passed the real read-only wepppy parser at its recorded HEAD. |

## Consolidated Remediation Contract

The overlapping A/B review-lock findings will receive one implementation, not
parallel ad hoc fields. A scientific approval binds conclusion-bearing science,
narrative, evidence, and authoring-analysis inputs. A separate publication
approval binds the complete scientific root plus schemas, templates, and
compiler identity. Historical approval entries remain in the record. A
nonsemantic renderer change can therefore invalidate publication approval
without impersonating or erasing scientific review; a science/narrative change
invalidates both.

The current SNOTEL record remains a `CANDIDATE`. Package reviewers are not
represented as external hydrologists, and remediation will not manufacture a
favorable scientific approval. Pending scientific/publication entries and
unresolved evidence gaps remain visible.

Both named reviewers verified their assigned remediation, the terminal source
identities, and the complete heavy evidence. No accepted closure finding
remains open, waived, deferred, or undispositioned.
