# ASSURE-04D — Review Locks, Publication, And Snapshots

Status: EXECUTED-COMPLETE — terminally verified on 2026-07-16

Package ID: `20260716-assure04d-review-lock-publication-snapshot-001`

Frozen base: `ec396c458a5015c504011a75814ff13e274544a1`

This ExecPlan is maintained under `docs/codex_exec_plans.md`. The `Progress`,
`Surprises And Discoveries`, `Decision Log`, and `Outcomes And Retrospective`
sections are living execution records.

## Objective

Implement the smallest fail-closed publication transition demonstrated by the
ASSURE-04C groundwater fixture without claiming that fixture has received
scientific approval. After this package, a maintainer can freeze a deterministic
review-subject root, validate exact-root scientific and reproduction/publication
approval records, promote only matching 04C staging bytes into a separate
confined public fixture, generate its ordinary-language catalog, and create or
confirm an immutable release snapshot.

The tracked openWEPP `usersum` remains the accepted zero-report public surface.
Positive publication evidence uses an explicitly marked synthetic approved
fixture in disposable external roots. A test-only snapshot can prove mechanics
but must be rejected by release verification.

## Rationale

ADR-0038 separates deterministic machinery from scientific judgment. ASSURE-04C
proved that authored reports can be assembled and rendered; it did not create
approval authority. ASSURE-04D must make the transition mechanically auditable
without allowing a passing build, coding-agent review, fake fixture, stale lock,
or incomplete snapshot to masquerade as human scientific publication.

## Authority

Binding authority is ADR-0038; the accepted v2 architecture; lifecycle,
source/build, scientific-report, and `usersum` authoring contracts; the
prospective ASSURE-04 roadmap; and the completed ASSURE-04C handoff. Package
artifacts and synthetic identities are engineering evidence only.

This package changes no kernel process, numerical method, science contract,
scientific conclusion, application-fitness judgment, or empirical evidence. It
does not approve the groundwater report.

## Scope

Included:

- define and implement a versioned deterministic review-subject-root algorithm
  that binds every claim-bearing source and every schema, catalog, principal,
  assembly-tool, and generated-output identity while excluding only explicitly
  classified higher-layer records;
- define layered, domain-separated subject, finding-ledger, approval-lock,
  release-transfer, snapshot, and publication-receipt identities so a later
  record cannot silently rewrite an earlier adjudicated subject;
- admit `DRAFT`, `IN_REVIEW`, and `APPROVED` source records with strict
  state-specific schemas and executable parity;
- require exact-root named scientific, reproduction/publication, and assurance-
  steward approvals, role competence, independence attestations, distinct
  approvers, material-producer/build-maintainer conflicts, and complete finding
  disposition;
- require stable principal IDs, principal kind, trust domain, identity-authority
  reference, declared roles, and deterministic producer/build-maintainer
  conflict checks; synthetic identities are a separate test-only trust domain;
- require an exact release-transfer record with target commit/configuration,
  impact assessment, reproduction disposition, semantic differences, release
  owner, assurance steward, and approved root;
- implement one/all public promotion from exact checked ASSURE-04C staging
  bytes into an explicit external `usersum`-shaped public root;
- generate deterministic public catalog Markdown and a machine catalog that
  contain approved reports only and use domain-reader metadata rather than
  internal lifecycle grades;
- validate the report's canonical link to exact model-narrative bytes and every
  generated public link;
- create or confirm an immutable snapshot containing exact approved source,
  safe evidence, generated public bytes, review/release identities, tool
  identity, and target release identity;
- make an append-only publication receipt the only authority for `PUBLISHED`;
  source records remain `APPROVED`, and public catalogs select only reports with
  a verified receipt for the exact approved lock and release transfer;
- prove snapshot conflicts, root drift, changed dependencies, incomplete/open
  findings, incompatible approvers, draft/in-review states, missing research
  objects, inaccessible output, unsafe roots, symlinks, and special files fail
  before public mutation;
- support explicit synthetic test-fixture mode only in disposable roots, emit a
  durable `TEST_ONLY` marker, and reject that marker at release verification;
- preserve named isolation, all exact-set cleanup, stable ordering,
  idempotence, transactional rollback, and no unrelated mutation;
- execute the synthetic public tree and snapshot in disposable external roots,
  then retain byte-for-byte non-operational evidence copies under package
  artifacts only;
- extend the actual openWEPP release-candidate preflight to verify an optional
  v2 snapshot/receipt pair and reject every `TEST_ONLY` trust-domain artifact
  before release-directory creation, while preserving the existing zero-report
  path;
- add focused integration tests, protected-surface proof, line-count governance,
  fresh CRAP evidence, dual review/disposition, independent heavy closure, and
  dual terminal verification; and
- close queue state and hand off ASSURE-05 without authorizing it.

Excluded:

- actual scientific, reproduction, publication, or human approval of the
  groundwater fixture or any report;
- edits to tracked `usersum`, v1-retirement public files, export manifests,
  releases, vendor trees, or WEPPcloud;
- publication of draft, in-review, withdrawn, superseded, or synthetic fixture
  content to a real release surface;
- selection of methods, datasets, metrics, findings, interpretations,
  limitations, reviewer competence, or application fitness by the builder;
- empirical or scientific reevaluation of groundwater evidence;
- withdrawal/supersession user experience beyond rejecting unauthorized state;
- WEPPcloud vendoring, which remains deferred until the beta release campaign;
- WEPPcloud manifest/navigation/search discovery, deferred to ASSURE-08; 04D
  proves only the current cmarkgfm rendering consumer and openWEPP release
  preflight consumer;
- network, shell, agent, random, hostname, wall-clock-content, file-time, or
  environment-interpolation dependencies during ordinary operations; and
- kernel, comparator, science-contract, or integrated-model changes.

## Declared Write Set

- `assurance/README.md`
- `assurance/v2/README.md`
- `assurance/v2/catalog.yaml`
- `assurance/v2/principals.yaml`
- `assurance/v2/schemas/{catalog.schema.json,principals.schema.json,report.schema.json}`
- `assurance/v2/reports/linear-groundwater-reservoir-recurrence/report.yaml`
- `crates/openwepp-assurance/src/{cli.rs,error.rs,lib.rs,v2.rs}`
- `crates/openwepp-assurance/src/v2/{assembly.rs,confined.rs,lifecycle.rs,publication.rs}`
- `Cargo.toml` for integration-test registration
- `Cargo.lock` and `crates/openwepp-assurance/Cargo.toml` for the indispensable
  parser-derived Markdown-link dependency
- `tests/integration/assurance_v2_{source,planner,assembly,publication}_contract.rs`
- `tests/integration/assurance_dossier_build_contract.rs`
- `tools/release/README.md`
- `tools/release/check_assurance_release_transition.sh`
- `tools/release/materialize_assurance_v2_release.sh`
- `tools/release/run_release_candidate_gates.sh`
- `docs/ROADMAP.md`
- `docs/planning/scientific-assurance-v2-implementation-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260716-assure04d-review-lock-publication-snapshot-001/**`

Manuscript/supplement and result bytes, and `result.schema.json` are excluded
unless the package is amended before an indispensable change. The tracked `assurance/catalog.yaml`,
`assurance/templates/catalog.md`, `assurance/generated/wepppy-usersum.yaml`,
every `usersum/**` file, public export/snapshot/release/vendor surfaces, kernel,
and science-contract files are protected read-only surfaces. Any edit outside
the declared set requires an explicit package amendment before the edit.

## Publication Design Contract

Before production edits, freeze the exact root grammar, state matrices, review
roles, output layout, snapshot manifest, and transaction boundaries in
`artifacts/publication-contract.md`. Contract-derived tests must fail because
the APIs are absent before implementation begins.

The contract uses a strict layered grammar. A domain-separated `SubjectRoot`
binds the complete report after removing only fields explicitly classified as
review-ledger, approval, release-transfer, snapshot, or receipt fields. It also
binds schema, source-catalog, principal-registry, assembly-tool, plan, generated
manuscript, supplement, research-object, and complete staging-tree identities.
Every schema leaf is classified exactly once; an unknown leaf is subject-bound,
never silently excluded. `FindingLedgerRoot` binds the subject and complete
finding ledger. `ApprovalLockRoot` binds the ledger and all approvals.
`ReleaseTransferRoot` binds the approved lock and independently supplied target
release commit/configuration. `SnapshotRoot` binds exact content-addressed
source, evidence, and generated public bytes. `PublicationReceipt` binds all
prior roots and is the sole authority for `PUBLISHED`. The authored source may
reach `APPROVED`; it never self-declares `PUBLISHED`.

Principals use stable IDs resolved through a strict registry. The registry
declares its trust domain; each principal declares kind, identity
authority/reference, roles, and display name. Human scientific and reproduction/publication approvals remain
human judgments that the builder can validate structurally but cannot create or
authenticate. Production accepts only the production trust domain. Synthetic
records use separate fixture types and entry points, and `TEST ONLY` remains
visible in every generated manuscript, supplement, catalog, snapshot, and
receipt surface.

A checked staging result is an opaque held capability, not a collection of
paths and hashes. It owns descriptor-confined handles and captured regular-file
bytes; publication consumes those bytes. Before commit it repeats root/source/
catalog/principal verification and proves named/all exact-set membership. All
mode rejects extra hidden, temporary, stale, symlink, or special entries.

Publication takes exclusive locks on descriptor-opened public and snapshot
roots, validates device/inode ancestry and non-overlap, prepares a complete
owned `usersum/assurance` generation, writes a content-addressed snapshot with
no-replace semantics, writes its no-replace receipt, and atomically exchanges
the complete public generation as the sole public commit. An interruption
before exchange can leave only an unreferenced immutable snapshot/receipt,
which an identical retry confirms and reuses. The old public generation remains
authoritative until exchange. Cleanup is limited to entries owned by the prior
machine catalog; unknown managed report entries fail rather than being deleted.
The implementation does not claim cross-root atomicity.

Public reports remain under
`usersum/assurance/reports/<report-id>/<version>/`. Catalog entries contain only
receipt-backed `PUBLISHED` reports and lead with title, scientific question,
assessed process/quantity, realization, publication date from source data, and
related model narrative. Every public link is verified, and selected publication
requires the external model narrative to equal the subject-bound source bytes.
The model narrative cannot link a report before it exists; ASSURE-08 owns the
later navigation/discovery backlink. 04D proves the actual current cmarkgfm
renderer; WEPPcloud manifest/navigation/search discovery remains ASSURE-08 work
and is not claimed here.

Snapshots live at `<snapshot-root>/<snapshot-id>/`, where the identifier is the
content digest of a normalized manifest plus captured payload and excludes any
self-naming field. Linux no-replace installation and a root lock prevent races;
an existing directory must match the complete expected tree. The receipt is a
separate immutable sibling record. Release verification receives the expected
release commit/configuration independently, reconstructs every root, and rejects
test-domain content. The real release-candidate preflight consumes this verifier
before creating a release directory; the established zero-report path remains
unchanged when no v2 snapshot/receipt is supplied.

## Deliverables

1. Package authority, required-reading map, protected freeze, and frozen
   publication/review/snapshot contract.
2. Versioned schema/source lifecycle, review-lock, release-transfer, catalog,
   and snapshot vocabulary.
3. Typed review-root, publication, catalog, snapshot, and release-verification
   APIs plus real CLI consumers.
4. Synthetic approved fixture helpers and retained disposable public/snapshot
   evidence with `TEST_ONLY` rejection proof.
5. Positive and negative tests for every state, role, root, finding, drift,
   accessibility, confinement, transaction, exact-set, and immutability gate.
6. Focused/quick/full, formatting, strict Clippy, deny, docs, fresh CRAP,
   protected-boundary, line-count, dual review/disposition, terminal
   verification, final-disposition, and ASSURE-05 handoff evidence.

## Phase Plan

### Phase 1 — Intake, Freeze, Contract, And Failing Tests

Freeze base/protected bytes and the terminal 04C API/consumer identities.
Record the canonical subject-root algorithm, lifecycle and role matrices,
public/snapshot layouts, confinement rules, and recovery semantics. Add tests
for review-root inspection, negative draft/in-review publication, synthetic
positive publication, catalog/cross-links, changed-root invalidation, exact
snapshot transfer, test-marker rejection, and failure atomicity. Record the
expected absent-API compile failure before production edits.

### Phase 2 — Source Contract And Publication Implementation

Bump only the necessary source contracts, migrate the real groundwater fixture
without changing its `DRAFT` posture or scientific bytes, and implement root
calculation, approval validation, checked-staging promotion, deterministic
catalogs, immutable snapshots, release verification, repository APIs, and CLI.
Keep publication logic in `v2/publication.rs`; split existing modules before any
file reaches 3,000 lines.

### Phase 3 — Focused Closure And Real Consumer Evidence

Run formatting, assurance unit tests, all four v2 integration suites, quick
workspace, focused strict Clippy, documentation validation, repeated named/all
publication, public link/accessibility checks, actual WEPPcloud renderer,
snapshot verification/rejection, protected hashes, write-set audit, and line
counts. Retain only explicit test-fixture output under package artifacts.

### Phase 4 — Dual Independent Review And Disposition

Dispatch two independent read-only coding-agent reviews. Both audit authority,
root completeness/self-reference avoidance, role independence, finding
disposition, actual checked-staging consumer, catalog/audience fit, research-
object completeness, snapshot/release binding, test-only isolation,
transaction/confinement security, protected surfaces, tests, CRAP, line counts,
and non-deferral. Record and disposition every finding; fix and verify every
accepted finding before heavy closure.

### Phase 5 — Independent Heavy Closure

Dispatch the required heavy-gate runner after review remediation. It runs and
records `cargo fmt --check`, workspace/all-target Clippy with warnings denied,
`cargo nextest run --workspace --profile full`, `cargo deny check`, and
`bash tools/release/run_adjudicated_crap_gate.sh --base-ref
ec396c458a5015c504011a75814ff13e274544a1`. Closure requires zero actionable
workspace rows and every touched production Rust file at adjudicated CRAP 30 or
below. The runner may write only package evidence.

### Phase 6 — Dual Terminal Verification And Closeout

After current heavy evidence, dispatch two independent read-only terminal
verifications. Both check every acceptance row, review disposition, actual
public/catalog/render/snapshot consumers, heavy/CRAP/line evidence, protected
bytes, write-set compliance, test-only rejection, and non-deferral. Resolve any
new blocker, renew invalidated gates, archive the active prompt byte-for-byte,
close package/catalog/roadmap state, and hand off ASSURE-05 without authorizing
it.

## Validation And Acceptance

Closure requires direct current evidence that:

- the canonical review-subject root binds every material scientific/source
  identity, changes on every bound change, is stable across repeated reads, and
  cannot self-reference mutable approval/publication fields;
- draft, in-review, missing/stale root, changed dependency, incomplete/open
  finding, duplicate/incompatible approver, missing competence/independence,
  missing release transfer, withdrawn, superseded, and unauthorized fixture
  publication all fail before destination mutation;
- exactly the required named human roles approve the same root and the builder
  does not invent, infer, backdate, or silently waive approval;
- named and all promotion consume exact current 04C staging bytes, share one
  per-report path, preserve unrelated named output, and produce byte-identical
  report output for the same source;
- the public catalog contains approved reports only, uses reader-facing fields,
  has no status-first headline, resolves every report/narrative/research-object
  link, and all-publish removes stale report/catalog entries;
- every public output renders through the actual WEPPcloud usersum Markdown
  consumer and retains accessible figures/tables and portable links;
- snapshot manifest/source/public hashes independently reconstruct, bind the
  exact release commit/configuration and review root, and repeated creation is
  identical while conflicts fail without mutation;
- synthetic publication is visibly `TEST_ONLY`, confined to disposable roots,
  and rejected by the release-verification consumer;
- a failed promotion or snapshot leaves every prior public/catalog/unrelated
  byte unchanged and does not expose partial output;
- ordinary operations are offline, deterministic, descriptor-confined, and
  reject repository, tracked public, source, staging overlap, snapshot overlap,
  symlink, special-file, path-escape, environment, and wall-clock authority;
- the real groundwater fixture remains `DRAFT`, has no approval/public/snapshot
  identities, and is absent from tracked public catalog/navigation;
- all protected bytes and aggregate `usersum` identity equal intake;
- focused, quick, full, Clippy, formatting, deny, docs, fresh CRAP, dual review,
  disposition, and dual terminal verification gates pass; and
- no finding is undispositioned, no current gate is deferred, no touched Rust
  CRAP exceeds 30, and no nonexempt 3,000-line Rust file remains.

## Delegation Authorization And Roles

Subagent authorization: this package explicitly authorizes spawning/delegating
to one heavy-gate runner and two independent reviewer/verifier subagents for
Phases 4 through 6. Expected outputs are compact review, verification, gate,
and metric reports returned to the parent or written under this package's
`artifacts/`. Reviewer/verifier write access is read-only. The heavy runner has
a bounded write set under this package's `artifacts/` only and may not edit
code, tests, authority, sources, public files, or queue state.

Coding-agent review is internal engineering review. It is never scientific,
reproduction, publication, release-owner, or named human approval and cannot
populate the synthetic or real approval records.

## Amendments

- 2026-07-16, before production edits: expanded the declared write set to add a
  stable principal registry/schema and the existing release-transition
  preflight consumer. Replaced the initial single-lock/cross-root rollback
  design with layered subject, finding-ledger, approval, release-transfer,
  snapshot, and receipt identities; an opaque checked-staging capability; a
  content-addressed no-replace snapshot; a receipt-backed `PUBLISHED` state;
  and one atomic whole-generation public exchange. Clarified that retained
  package artifacts are byte-for-byte evidence copies of operations executed
  only in disposable external roots. This amendment dispositions the two early
  independent design audits recorded in
  `artifacts/early-design-audit-disposition.md` and precedes every production
  source edit.
- 2026-07-16, during focused line-count closure: added
  `crates/openwepp-assurance/src/v2/lifecycle.rs` to the write set and moved
  state-matrix validators there. The migration introduced no behavior change;
  it was required because the expanded strict source contract took `v2.rs`
  above the package's 3,000-line closure block.
- 2026-07-16, during Phase 4 review disposition: corrected the impossible
  prepublication reciprocal-link wording. 04D now requires a real canonical
  report-to-model link and exact subject-bound narrative bytes; ASSURE-08 owns
  later discovery/navigation backlinks. Accepted both independent HOLD reviews
  and strengthened authority reconstruction, exact approval-ledger binding,
  anti-omission classification, held capabilities, durability, public
  ownership, multi-report/fault/special-file proofs, checkout/configuration
  binding, and release-artifact receipt discovery before heavy closure.

## Security And Recovery

Publication remains offline and non-executable. All source/staging/public/
snapshot reads and writes are descriptor-relative, no-follow, confined regular-
file operations. Publication destinations are external roots. An exclusive root
lock and descriptor/inode ancestry checks precede prepared generation; snapshot
and receipt installation are content-addressed and no-replace, and one atomic
exchange commits the complete public generation. Before that exchange the old
generation remains authoritative. An interruption can leave only an orphaned
immutable snapshot/receipt that an identical retry verifies and reuses; it
cannot expose partial public output. Errors are typed; missing inputs or
approvals are never replaced with defaults. Restricted evidence is recorded but
never read, copied, or exposed.

Rollback removes the 04D lifecycle/schema additions, publication module/API/
CLI, publication-specific tests, and disposable retained fixture while
retaining the exact ASSURE-04C staging assembly and zero-public state.

## Progress

- [x] (2026-07-16 08:00Z) Pushed completed ASSURE-04C as `ec396c45`, read core
  04D authority, froze the base, and scaffolded the active package.
- [x] (2026-07-16 09:15Z) Ran two independent early design audits, accepted all
  blocking findings, and amended the package before production edits.
- [x] (2026-07-16 10:00Z) Froze protected identities and the amended layered
  publication contract, then recorded the expected absent-API compile failure.
- [x] (2026-07-16 15:20Z) Implemented the v3 source/principal contract, layered
  roots, checked-staging publication, receipt-backed prior-generation handling,
  public catalog, immutable snapshots/receipts, CLI, and real release preflight.
- [x] (2026-07-16 16:10Z) Passed 54 focused contracts, 1,936 workspace quick
  tests, strict assurance Clippy, formatting, release-script syntax, authored
  documentation checks, retained synthetic publication, actual `cmarkgfm`
  rendering, protected hashes, and line-count closure.
- [x] (2026-07-16 18:10Z) Accepted both second-review HOLD verdicts and
  remediated mount-aware held-root replay, exact catalog-byte binding,
  Markdown-link forgery, receipt crash retry, distinct-generation reads,
  omitted authority/lifecycle/root-mutation negatives, multi-report production
  replay, and executed release materialization. Focused contracts pass 67/67,
  workspace quick passes 1,956/1,956, and strict assurance Clippy passes.
- [x] (2026-07-16 19:00Z) Completed dual review, disposition, all accepted
  remediations, and independent PASS rechecks without scientific approval.
- [x] (2026-07-16 06:20Z) Preserved the first independent heavy HOLD: formatting
  passed, but strict workspace/all-target Clippy found eight test-only
  diagnostics and correctly stopped before full, deny, or CRAP.
- [x] (2026-07-16 06:32Z) Remediated the Clippy findings without suppressions or
  production edits by splitting negative matrices at contract boundaries,
  extracting helpers, and avoiding an allocation. Workspace strict Clippy,
  69/69 focused contracts, 1,958/1,958 workspace quick, and the retained
  byte-identical synthetic publication now pass on the changed test snapshot.
- [x] (2026-07-16 06:39Z) Both independent reviewers renewed PASS on the bounded
  test-only remediation, preserved the first heavy HOLD, and independently
  confirmed strict Clippy and semantic coverage without scientific approval.
- [x] (2026-07-16 07:31Z) Preserved the second independent heavy HOLD after
  format, strict Clippy, 2,043/2,043 full tests, and deny passed: fresh CRAP
  found seven actionable rows in four touched assurance files.
- [x] (2026-07-16 08:00Z) Remediated all seven CRAP rows through bounded
  decomposition without adjudication or suppression. Strict Clippy, 17/17
  library tests, 69/69 focused contracts, 1,961/1,961 quick tests, retained
  byte equality, and a fresh focused zero-violation CRAP estimate pass.
- [x] (2026-07-16 08:13Z) Both independent reviewers returned PASS on the
  production CRAP remediation, independently reproduced zero focused
  touched-file rows above 30, and preserved both earlier heavy HOLDs.
- [x] (2026-07-16 09:05Z) Completed the third independent heavy sequence from
  formatting through fresh CRAP: 2,046/2,046 full tests, dependency policy,
  zero actionable CRAP rows, and every touched-file maximum at or below 30.
- [x] (2026-07-16 09:22Z) Completed dual terminal verification, archived the
  active prompt byte-for-byte, closed roadmap/catalog state, and handed off
  ASSURE-05 as next eligible without authorizing it.

## Surprises And Discoveries

- Observation: the accepted groundwater fixture intentionally cannot enter
  review because human accountability and historical provenance are incomplete.
  Evidence: `report.yaml` remains `DRAFT`, with null approval/publication fields.
  Positive 04D mechanics therefore require an explicit synthetic copy and may
  not mutate the canonical fixture into apparent approval.

- Observation: a snapshot root and public root cannot share one atomic
  filesystem commit, and a path/hash staging check is vulnerable to check/use
  races. Evidence: both independent design audits returned HOLD. Resolution:
  content-addressed immutable snapshot and receipt creation precede one atomic
  whole-generation public exchange, and publication consumes captured bytes
  from an opaque descriptor-held checked-staging capability.

- Observation: the existing WEPPcloud cmarkgfm renderer reads a selected
  Markdown path, while manifest-backed navigation/search belongs to the later
  vendoring integration. Resolution: 04D proves cmarkgfm rendering and the real
  openWEPP release preflight only; ASSURE-08 retains discovery integration.

- Observation: a public catalog cannot embed its own snapshot or receipt
  content address without a cycle, but named publication still must not trust a
  catalog declaration alone. Resolution: named publication discovers immutable
  receipts in the supplied snapshot root, verifies the exact prior public-tree
  digest, release, report set, catalog roots, and complete snapshot, then
  carries forward only those verified bindings and per-report source payloads.
  Unreceipted prior entries fail before mutation.

- Observation: atomic directory exchange does not make multiple independent
  pathname reads transactional. Resolution: each public file lookup/read is
  proved to return exact old or new bytes across distinct realizations;
  coherent multi-file audit reads use the immutable receipt-bound snapshot.

- Observation: lexical path ancestry cannot expose a descendant bind-mounted
  at an unrelated path. Resolution: pairwise checks recursively walk held
  directory descriptors by device/inode and repeat immediately before exchange.

## Decision Log

- Decision: review locks bind a canonical review subject rather than raw
  `report.yaml` bytes.
  Rationale: approval/publication fields reside in the manifest; binding raw
  bytes would make approval self-referential. The subject includes all material
  content and identities while excluding only transition records.
  Date/Author: 2026-07-16, Codex.

- Decision: positive synthetic publication requires explicit test-fixture mode
  and a release-rejected marker.
  Rationale: fake reviewer identities are appropriate for contract tests only;
  they must be mechanically incapable of becoming release evidence.
  Date/Author: 2026-07-16, Codex.

- Decision: source lifecycle terminates at `APPROVED`; an immutable verified
  receipt establishes `PUBLISHED` for a release realization.
  Rationale: authored source cannot truthfully or safely self-declare successful
  mechanics that have not yet occurred.
  Date/Author: 2026-07-16, Codex.

- Decision: root identities are layered rather than excluding an
  undifferentiated review/publication subtree.
  Rationale: accepted findings must be verified and bound before approvals, and
  release transfer, snapshot, and receipt are successively dependent records.
  Date/Author: 2026-07-16, Codex.

- Decision: snapshot source inputs are namespaced by report ID.
  Rationale: named publication can replace one report's approved source payload
  while preserving independently receipt-verified source bytes for unrelated
  public reports without path collisions or false freshness claims.
  Date/Author: 2026-07-16, Codex.

- Decision: add `materialize_assurance_v2_release.sh` to the declared write set
  and make the release runner call it.
  Rationale: the bounded release copy/reverification/discovery operation must be
  the same executable consumer exercised by integration tests, not duplicated
  shell logic or source-text inspection.
  Date/Author: 2026-07-16, Codex.

- Decision: amend the write set for `pulldown-cmark` in
  `openwepp-assurance` and the resulting workspace lockfile update.
  Rationale: independent review demonstrated that a partial hand-written link
  recognizer disagreed with the actual usersum renderer for raw HTML blocks and
  fence-closing rules. Parser-derived link events are indispensable to avoid an
  open-ended Markdown grammar imitation at the publication boundary.
  Date/Author: 2026-07-16, Codex.

## Outcomes And Retrospective

ASSURE-04D implemented a fail-closed publication boundary without publishing a
scientific report. Production and explicitly test-only APIs are separate;
layered subject, finding, approval, release-transfer, snapshot, and receipt
identities bind the complete transition. Publication consumes descriptor-held
checked staging, atomically exchanges a complete public generation, and
retains immutable receipt-bound source/public evidence for release replay.

The real groundwater fixture remains internal `DRAFT`, tracked `usersum`
remains byte-identical and zero-report, and synthetic evidence remains marked
`TEST ONLY` and rejected by the production release path. The actual usersum
renderer, release preflight, materializer, discovery sidecar, and checksum
consumer all executed.

Two independent-review HOLD cycles and two heavy-gate HOLD cycles exposed and
closed authority, parser, confinement, test-quality, and CRAP defects. The
terminal heavy restart passed formatting, strict Clippy, 2,046/2,046 full
tests, dependency policy, and fresh CRAP with zero actionable rows and every
touched-file maximum at or below 30. Both terminal verifiers returned PASS.
ASSURE-05 may now be separately authorized to perform the genuinely human
scientific/reproduction review and first production publication; this package
does not grant that authority.
