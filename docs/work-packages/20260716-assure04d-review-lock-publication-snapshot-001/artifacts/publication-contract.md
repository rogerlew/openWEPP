# ASSURE-04D Publication Contract

Status: frozen before contract-derived tests and production edits

Evidence class: Static design contract

Frozen base: `ec396c458a5015c504011a75814ff13e274544a1`

## Purpose And Boundary

This contract governs the deterministic transition from an assembled v2 report
to a release-bound public assurance generation. It verifies declared review and
publication records; it does not perform scientific judgment, infer competence,
authenticate a person, or decide application fitness.

The canonical groundwater source remains `DRAFT` and nonpublic. Positive 04D
evidence uses a synthetic `TEST_ONLY` duplicate in disposable external roots.
The repository's tracked `usersum` and retired-v1 surfaces remain unchanged.

## Source Contract Version

ASSURE-04D advances the v2 source catalog and report schema together to contract
version 3. Result schema version 1 remains unchanged. The catalog binds exactly
four schemas: catalog v3, report v3, result v1, and principal-registry v1.

The catalog and report add `trust_domain`, whose only values are `production`
and `test_only`. Catalog/report values must agree. The canonical architecture
fixture is `test_only`; a future ASSURE-05 source must be explicitly converted
to `production` as part of accountable redrafting.

The report adds reader metadata:

- `scientific_question`;
- `assessed_process`;
- `assessed_quantity`;
- `realization`;
- `related_model_narrative`; and
- `manuscript_date` in `YYYY-MM-DD` form.

These values are subject-bound and mechanically populate public discovery.
They are authored scientific communication, not generated conclusions.

## Principal Registry

`assurance/v2/principals.yaml` is a strict version-1 registry. It contains a
top-level trust domain and stable, unique principal records with:

- `id`;
- `display_name`;
- `kind`: `human`, `organization`, `agent`, or `software`;
- `identity_authority`;
- `identity_reference`; and
- one or more declared `roles`.

Approval and release roles reference principal IDs, never display names.
Production approvals require `kind: human`, `trust_domain: production`, and the
corresponding declared role. The three approvers, release owner, build
maintainer, material producers, and assurance steward are resolved before any
root is accepted. The registry is declaration and provenance, not identity
authentication; real approval remains an accountable human procedure.

Synthetic tests construct a separate `test_only` registry. Production APIs
cannot deserialize test-domain approval records into production approval types.

## Lifecycle And Record Matrix

The authored lifecycle has three states. `PUBLISHED` is deliberately absent.

| Source state | Review record | Release-transfer record | Publication permitted |
| --- | --- | --- | --- |
| `DRAFT` | `not_started`; roots null; no findings or approvals | `not_started`; roots and release fields null | No |
| `IN_REVIEW` | Exact subject root, immutable charge, declared maintainers/producers, complete current findings, no approvals | `not_started` | No |
| `APPROVED` | Exact subject and ledger roots, terminal findings, exactly three exact-ledger approvals, exact approval-lock root | Exact approval-lock root, release identity, realization comparison, impact/reproduction disposition, public path/date, owner/steward, exact transfer root | Yes, subject to all mechanical checks |

Withdrawal and supersession are later lifecycle operations. A source with
`withdrawn: true`, a non-null unsupported supersession, or a publication state
other than the matrix above fails 04D publication.

### Findings

Each finding has a stable ID, summary, severity, disposition, rationale,
resolution, verification, and verifier principal ID. `open` findings have no
terminal resolution and block approval. `accepted` findings require disposition
`resolved_and_verified`, nonempty resolution and verification records, and a
qualified verifier distinct from the material producer responsible for the
change. `rejected` findings require an adjudication rationale and verifier.
`deferred`, `duplicate`, `follow_up`, missing, or unknown dispositions block.

### Approvals

An approved record contains exactly one each of:

- `scientific` approval by a declared human scientific reviewer;
- `reproduction_publication` approval by a declared human reproduction and
  publication reviewer; and
- `assurance_steward` approval by a declared human assurance steward.

Every approval binds the same finding-ledger root, records an `approved`
decision, competence basis, independence attestation, and authored date. The
principals are distinct. Scientific and reproduction/publication approvers may
not be the report lead or any material producer. The reproduction/publication
approver may not be the build maintainer. The builder checks these declarations
and conflicts; it does not judge whether prose assertions are true.

### Release transfer

The transfer binds the approval-lock root and contains target Git commit,
release-configuration identity, prior and candidate realization identities,
impact assessment, reproduction disposition, semantic-difference statements,
release-owner ID, assurance-steward ID, publication date, public path, and
transfer root. Commit/configuration are also supplied independently to publish
and verify calls; caller/source disagreement fails.

## Layered Root Grammar

All hashes are lowercase SHA-256 over canonical UTF-8 JSON with recursively
lexicographically sorted object keys, preserved array order, no insignificant
whitespace, and a trailing newline. Each payload contains `algorithm` and a
unique `domain` string. Raw YAML presentation, filesystem times, host identity,
and environment are not authority.

1. `SubjectRoot`, domain `openwepp-assurance-subject-v1`, binds the admitted
   report with transition leaves normalized out, plus exact catalog, schema,
   principal-registry, planner, assembly-tool, source-input, generated staging,
   and related-model-narrative identities. The ASSURE-04C `build-manifest.json`
   is bound after normalizing only its transition-sensitive
   `source_root_sha256` to null; its complete file inventory and every listed
   output digest remain bound. Exact catalog source bytes are hashed after
   replacing only report-manifest digest values with a fixed token; this binds
   byte drift without introducing cross-report root cycles. The final raw manifest is later bound by the
   snapshot and receipt. This explicit normalization avoids a transition-root
   cycle without excluding generated scientific bytes.
2. `FindingLedgerRoot`, domain `openwepp-assurance-findings-v1`, binds the
   subject root, immutable review charge, build maintainer, material producers,
   and every complete finding in stable ID order.
3. `ApprovalLockRoot`, domain `openwepp-assurance-approvals-v1`, binds the
   finding-ledger root, all three approvals in role order, and the complete
   independence assessment.
4. `ReleaseTransferRoot`, domain `openwepp-assurance-transfer-v1`, binds the
   approval-lock root and complete transfer record except its declared root.
5. `SnapshotRoot`, domain `openwepp-assurance-snapshot-v1`, binds the normalized
   snapshot manifest and every payload path/hash/size/kind. It has no self-name
   or snapshot-root field.
6. `PublicationReceipt`, domain `openwepp-assurance-receipt-v1`, binds all five
   prior roots, report/version, trust domain, release identity, public tree
   digest, publication date/path, builder identity, and receipt format.

The receipt hash is the receipt ID. A verified receipt is the only authority
for the derived state `PUBLISHED`.

### Exact leaf classification

The top-level `lifecycle` value and explicitly enumerated review/publication
transition leaves are excluded only from `SubjectRoot` and enter a named later
layer. Review/publication identity fields (`id`, `title`, `owner`) remain in the
subject. The executable normalizer removes those named leaves individually; it
does not replace or discard a whole subtree. Every other admitted report leaf
is subject-bound. Strict schema, `deny_unknown_fields`, and executable
classification tests require every current transition leaf to appear exactly
once in the layer grammar. Declared subject/finding/approval/transfer roots are
equality constraints against calculated roots rather than self-hashed leaves.
Executable mutation tests prove every other transition leaf changes its named
finding, approval, or transfer layer. A newly admitted leaf not explicitly classified as a transition remains
subject-bound by construction. Finding IDs are sorted before hashing;
scientifically ordered report arrays retain source order. Every approval's
ledger identity must equal the calculated finding-ledger root.

## Checked Staging Capability

Publication never consumes unchecked paths. It performs an ASSURE-04C check,
reassembles expected bytes in memory, opens the staging root and descendants
without following links, and captures every regular file into an opaque
`CheckedStaging` capability. The capability binds:

- repository and staging directory device/inode identities;
- exact selected report IDs and versions;
- plan/source/subject roots;
- every expected path, bytes, digest, and size; and
- the complete staging-tree digest.

No public constructor exposes unchecked values. Publishing consumes the
captured bytes, not a reopened pathname. Immediately before commit it reopens
the source/catalog/schema/principal inputs and reconstructs all roots. Drift
fails before the public exchange.

Named mode requires exactly the selected report directory within the captured
selection and preserves catalog-owned unselected public reports. All mode
requires the staging report-ID set to equal the catalog-selected approved set.
Hidden files, temporary files, stale report directories, symlinks, sockets,
FIFOs, devices, hard-link ambiguity, and nested unknown entries fail.

## Public Generation

The caller supplies an absolute external `usersum` root. The owned generation
is its `assurance` child. Public layout is:

```text
assurance/
  README.md
  catalog.json
  reports/<report-id>/<version>/
    index.md
    supplement.md
    ... generated assets and public-safe research objects
```

`catalog.json` is strict deterministic JSON. Each entry is backed by a verified
receipt and contains report/version, title, scientific question, assessed
process/quantity, realization, publication date, public report path, supplement
path, related model narrative, subject root, approval-lock root, transfer root,
and derived `PUBLISHED` state. It deliberately omits snapshot and receipt IDs:
the snapshot binds the exact public catalog, and the later receipt binds that
snapshot, so embedding either content address would be circular. Release
sidecars provide receipt discovery. The Markdown catalog presents the reader
fields first and never headlines an internal status grade.

Named publication retains only entries owned by the prior machine catalog after
discovering and fully verifying an immutable receipt and snapshot for the exact
prior public-tree digest, report set, release identity, and catalog roots. Its
prior per-report source payload and root bindings are carried into the new
snapshot; the selected report's payload and bindings are replaced. All
publication makes the catalog and report directory exact. Unknown content under
`assurance/reports`, malformed prior catalogs, unreceipted entries, or
catalog/file disagreement fails instead of being deleted.
Every generated local link is resolved before commit. The report must contain a
real canonical Markdown link to the exact subject-bound model narrative bytes
present in the selected external usersum root. The recognizer accepts only
parser-derived Markdown link events, not matching target text. Renderer-backed
negatives cover fenced/indented/inline code, malformed fence closers, multiline
raw HTML, comments, escapes, and images. The model narrative cannot
reciprocally link a report before that report exists; public discovery and the
later navigation backlink belong to ASSURE-08. This corrects the earlier
reciprocal-link requirement without weakening the report's required “why/how”
cross-reference. 04D proves `cmarkgfm` parsing; WEPPcloud manifest, navigation,
search, and vendoring remain ASSURE-08.

Synthetic output adds a prominent `TEST ONLY — NOT SCIENTIFICALLY APPROVED`
banner to README, report, supplement, catalog, manifest, and receipt. This
banner is part of all roots.

## Snapshot And Receipt

Snapshots live at `<snapshot-root>/<snapshot-id>/`; receipts live at
`<snapshot-root>/receipts/<receipt-id>.json`. A snapshot contains:

```text
manifest.json
source/<report-id>/<repository-relative input paths>
public/assurance/<complete generated assurance tree>
```

All local source dependencies, public-safe evidence, schemas, catalog,
principal registry, authored report inputs, results, related model narrative,
and captured generated outputs are copied. Restricted objects are represented
only by their non-sensitive declaration and are never opened or copied.
Manifest entries state normalized path, SHA-256, byte length, and kind.

Snapshot installation uses an exclusive root lock and Linux no-replace rename.
Every prepared file is data-synced, prepared directory trees are synced
bottom-up, and parent directories are synced before final rename. An existing
snapshot/receipt is accepted only after byte-for-byte complete-tree
verification. Any difference returns `SnapshotConflict`. The manifest excludes
snapshot ID and receipt ID, preventing self-reference.

## Confinement, Locking, And Commit Protocol

Repository, staging, usersum, and snapshot roots must be absolute, explicitly
supplied where applicable, descriptor-opened without symlink traversal, and
pairwise unrelated by device/inode ancestry. Relationship checks recursively
walk opened directory descriptors with visited device/inode identities, so a
bind-mounted descendant cannot hide behind an unrelated lexical path. All four
held identities and pairwise relationships are checked before preparation,
after final source replay, and again immediately before exchange.
Publication rejects roots inside the repository, tracked `usersum`, source,
staging, one another, or aliases of those locations.

The transaction is:

1. acquire exclusive advisory locks on the public usersum and snapshot roots in
   deterministic identity order;
2. validate/capture staging and revalidate every source/root/approval/transfer;
3. read and validate the prior machine-owned public generation;
4. prepare a complete replacement assurance generation in the public parent;
5. prepare and no-replace install or confirm the immutable snapshot;
6. prepare and no-replace install or confirm the immutable receipt;
7. revalidate all source roots and all four held identities/relationships;
8. atomically exchange the prepared and current `assurance` directories; and
9. remove the old exchanged generation.

Step 8 is the sole public commit. Failure before it leaves public bytes
unchanged. Once exchange succeeds, publication is committed: cleanup or final
parent-sync diagnostics are best effort and cannot return a false precommit
error. A crash may leave an unreferenced immutable snapshot/receipt or a
prepared directory; retry verifies/reuses immutable artifacts and removes only
its own transaction-named preparation. An exact receipt preparation is reused;
a same-name preparation with different bytes is a conflict. Cross-root atomicity
is not claimed. Concurrent writers serialize through locks. Each public file
lookup and read observes a complete old or complete new byte stream; multiple
independent pathname reads are not a transaction and consumers needing a
coherent multi-file generation use the immutable receipt-bound snapshot. Typed test-only fault
points prove all three precommit install/revalidation boundaries preserve the
prior generation and permit identical retry.

## Typed API And CLI

The public Rust surface provides:

- `V2TrustDomain`;
- `V2ReleaseIdentity`;
- `V2PublicationOptions`;
- `V2PublicationResult`;
- `V2ReleaseVerification`;
- `V2Repository::publish_report` and `publish_all` for production only;
- `V2Repository::publish_test_fixture_report` and
  `publish_all_test_fixtures` for the test domain; and
- `verify_v2_release_snapshot`, which receives expected release identity
  independently and accepts production receipts only.

Production verification does not stop at content-address syntax. It requires a
nonempty production public catalog and exact known builder identity, opens each
per-report snapshotted source as a strict v2 repository, revalidates schemas,
catalog, principal roles, lifecycle, findings, approvals, release transfer, and
all input identities, reconstructs every layered root from the captured public
staging bytes, and compares the complete expected public and source payloads to
the manifest and receipt. Empty containers, arbitrary root maps, missing or
relabeled source, malformed catalogs, unknown builders, and test-domain bytes
therefore fail even when their outer JSON is self-hashed.

The ordinary CLI exposes `publish`, `publish-test-fixture`, and `verify-release`.
Production and test publication are different subcommands. Publish requires
explicit staging, usersum, snapshot, release-commit, and release-configuration
arguments plus exactly one of `--report` or `--all`. Verify requires explicit
snapshot directory, receipt file, expected commit, and expected configuration.

No API accepts a boolean “test mode,” implicit current directory, wall-clock
date, environment-derived release identity, or caller-supplied root digest.

## Release Consumer

`check_assurance_release_transition.sh` retains the exact zero-report behavior
when no v2 arguments are supplied. Optional v2 release mode requires a snapshot,
receipt, expected commit, and expected configuration as one complete set and
invokes the production `verify-release` CLI. Any omitted peer argument,
`TEST_ONLY` marker, mismatch, malformed artifact, or verifier failure blocks.

`run_release_candidate_gates.sh` accepts and forwards the same complete set to
the transition preflight before it creates `RELEASE_DIR`. The preflight requires
the supplied commit to equal the selected checkout's `HEAD` and the
configuration to equal the driver's actual build configuration. After
verification, release mode invokes `materialize_assurance_v2_release.sh`, which
copies the exact content-addressed snapshot and receipt into
`RELEASE_DIR/assurance-v2/`, verifies the copies again, emits
`assurance-v2-publication.json` for receipt discovery, and records checksums.
Validation mode may not accept or verify publication artifacts. The default
workflow remains on the zero-report path until a production report exists.
The executed contract test runs the real preflight and the same materializer
called by the release runner, then replays the copied authority, inspects the
discovery sidecar, compares payload bytes, and verifies checksums. Source-text
inspection alone does not close this requirement.

## Required Negative Proofs

Tests must show no public mutation for draft/in-review sources, stale or wrong
layer roots, open or incompletely resolved findings, missing/duplicate/conflict
approvers, wrong principal kind/domain/role, changed schema/catalog/principal/
dependency/staging/output/narrative bytes, mismatched release identity,
incomplete research objects, stale/hidden/all-mode entries, unknown prior public
content, inaccessible links, root overlap, symlinks, special files, lock races,
snapshot/receipt conflicts, injected failure before exchange, and production
verification of test-domain artifacts.

Crash-boundary tests must demonstrate exact retry. Concurrent-reader tests use
distinct approved realizations and require every observed report read to equal
the complete old or complete new bytes; coherent multi-file audit reads use the
immutable snapshot. Concurrency tests must demonstrate no-replace snapshot
identity and serialized catalog updates.
