# Assurance Amendment And Generated-Identity Specification

Status: implemented and closed by `ASSURE-MAINT-01`

Owner: openWEPP scientific assurance maintainers

Applies to: `openwepp-assurance` v2 internal report sources

## Purpose

Small report revisions must be safe, auditable, and fast. Correcting a spelling,
an accountable lead, an affiliation, or another bounded administrative field
must not require an agent to calculate hashes, copy digests between files,
reconstruct unchanged scientific results, or run unrelated workspace gates.

This specification defines typed amendment and lifecycle transactions,
generated identity locks, acyclic review identities, mechanical impact
classification, and receipt-driven proportional gates. The tooling performs
bookkeeping and proves the limits of enumerated operations. It does not judge
arbitrary prose, create a human decision, approve science, or replace qualified
review.

## Design Principles

1. Hashes are generated consequences, not human-authored inputs.
2. One typed operation owns the complete affected-file transaction.
3. The tool calculates dependency impact and gate obligations; an agent does
   not infer which hashes or tests should change.
4. Scientific content, reader communication, attribution, review decisions,
   realization, and release transfer remain distinguishable.
5. Only enumerated deterministic transformations qualify for an editorial fast
   lane. Unknown, mixed, and human-authored prose changes fail closed.
6. Existing approvals survive only when every identity they bind is unchanged.
   A new approval or equivalence decision is an immutable human-authority event,
   never a regenerated property.
7. Validation, planning, assembly, amendment, and lifecycle recording remain
   deterministic, offline, and agent-free.
8. A receipt-qualified report-data revision uses its focused gate contract. It
   does not require a work package, coding-agent review, CRAP, or a workspace
   test suite.

## Trust Model

SHA-256 and canonical serialization prove internal consistency, not who wrote a
file. A `machine_owned` field cannot cryptographically distinguish tool output
from a coordinated manual edit. Git history, protected-branch review, and an
independently supplied trusted generation are the provenance anchors.

The initial generated identity lock is a migration genesis bound to the frozen
Git base and migration receipt. Each later lock contains the previous
generation ID. `verify-generation` accepts an expected generation directly or
derives it from a local Git base ref, then verifies the complete receipt chain.
Ordinary repository validation checks current internal consistency; it must not
claim that consistency proves tool authorship.

CI and package closure reject a generated-lock change without a valid
old-generation-to-new-generation receipt. Runtime commands reject stale or
noncanonical locks and never offer a generic operation to adopt current bytes.

## Source, Decision, And Generated Data

### Authored scientific and administrative sources

- `assurance/v2/catalog.yaml` declares schemas, reports, and logical paths
  without file-content digests.
- `assurance/v2/principals.yaml` declares immutable principal identity records
  and global role eligibility. Display-name or authority changes create a new
  principal-record version rather than rewriting an identity used by another
  report.
- Each report's `report.yaml` declares scientific metadata, logical dependency
  IDs and paths, report-local role assignments, review charge, and publication
  intent without calculated digests or roots.
- Manuscripts, supplements, procedures, inputs, and result objects remain
  canonical authored or retained sources.

### Immutable human-authority events

Typed lifecycle commands record review entry, findings, dispositions,
approvals, withdrawal, supersession, and release-transfer decisions under
`assurance/v2/reports/<report-id>/review-events/`. The human supplies the
decision, identity, competence, independence, rationale, and date. The command
binds the current applicable roots and writes an immutable canonical event.

An event ID is the domain-separated SHA-256 of its canonical body with its ID
field omitted. Generated review locks may index events, but may not supply,
rewrite, or carry their authority to a new root.

### Generated active locks

`assurance/v2/identity.lock.json` is the active generation manifest. Its
canonical payload contains:

- `machine_owned: true`, format version, identity algorithm version, and tool
  identity;
- migration genesis or `previous_generation_id`;
- every admitted authored source path and SHA-256;
- every immutable review-event path and SHA-256; and
- every final generated review-lock path and SHA-256.

The identity lock never hashes itself. `generation_id` is a domain-separated
digest of the canonical lock payload with `generation_id` omitted.

Each `review.lock.json` stores only calculated current roots, indexed event IDs,
and invalidation state. It contains no human decision. Review locks are
calculated first and identified by the final identity-lock payload.

Amendment and lifecycle receipts live under
`assurance/v2/transactions/<receipt-id>.json`. Receipt archives are outside the
active generation identity. A receipt body records the operation, old and new
generation IDs, affected reports and paths, old and new roots, invalidated
events or locks, stable gate IDs, and resolved executable/argument arrays. Its
ID and filename are calculated after the body is final; the body contains
neither its own ID nor path.

Generated files use canonical JSON, stable lexical ordering, and no hostname,
absolute path, random value, or implicit wall-clock content. Human-supplied
decision dates remain explicit event inputs. Repeating an identical request on
an already-updated generation returns a no-op and does not append a receipt.

## Acyclic Identity Graph

The monolithic subject root is replaced by this directed acyclic graph:

```text
science_root ────────────────┐
communication_root ──────────┼─> content_review_subject_root
review_governance_root ──────┘              │
                                            ├─> finding_ledger_root
complete projected inputs + embedded implementation digest
    ─> preapproval_realization_root
                                            │
finding ledger + applicable roots + preapproval realization
    ─> scientific/reproduction approval events
    ─> pre_steward_approval_root

attribution_root + pre_steward_approval_root + exact predecessor event IDs
    ─> steward approval event
    ─> approval_lock_root

preapproval_realization_root + approval_lock_root ─> realization_root

approval_lock_root + realization_root + target release + transfer event
    ─> release_transfer_root
```

- `science_root` binds claims, methods, datasets, result objects, table and
  figure sources, procedures, software realization, science authority, and
  claim-bearing directives.
- `communication_root` binds exact manuscript and supplement bytes after only
  explicitly delimited generated attribution/lifecycle regions are replaced by
  canonical sentinels.
- `attribution_root` binds only reader-facing bibliographic identity: the
  bibliographic projections of selected immutable principal-record versions,
  display names, affiliations, role presentation, and attribution
  attestations. It does not independently confer or bind role eligibility,
  report-lead, material-producer, build-maintainer, reviewer, approver, steward,
  or release-owner authority.
- `review_governance_root` binds the review charge, report lead, material
  producers, build maintainer, role assignments that affect competence or
  independence, applicable principal authority/eligibility versions, identity
  algorithm, and applicable tool identities.
- `content_review_subject_root` binds `science_root`, `communication_root`, and
  `review_governance_root`. It contains no bibliographic-only attribution,
  findings, or approvals.
- `finding_ledger_root` binds the content review subject and ordered
  finding/disposition events. It contains no approvals.
- `preapproval_realization_root` binds the complete deterministic inputs to the
  review projection, the exact embedded implementation digest for identity,
  assembly, lifecycle, planning, and publication code, builder identity,
  `science_root`, `communication_root`, `review_governance_root`, and the
  current content review subject. This derivation-complete identity is
  available before staging and contains no attribution, approval, or release
  event.
- Each scientific or reproduction/publication approval event binds the exact
  current roots and predecessor events required by the normative role matrix
  below. It never binds a root containing itself.
- `pre_steward_approval_root` binds the exact complete set of required valid
  scientific and reproduction/publication approval event IDs. A steward event
  binds that root, the same event IDs, `attribution_root`, and every applicable
  subject and realization root.
- `approval_lock_root` binds `pre_steward_approval_root`, the exact steward
  event ID, and the complete ordered set of predecessor approval event IDs.
- `realization_root` binds `preapproval_realization_root`, the same embedded
  implementation digest, `attribution_root`, and `approval_lock_root`. It is an
  approval realization, not a substitute for observing built files.
- A release-transfer event binds `approval_lock_root`, `realization_root`, the
  exact target release identity, release owner, steward, decision, and date.
  `release_transfer_root` binds that immutable event ID plus those same roots
  and release inputs. A mechanically identical realization cannot acquire
  transfer authority without this event.

Staged files that display a calculated root are canonicalized by replacing that
field with null before hashing the realization. The identity algorithm version
defines the complete normalization. No root may directly or indirectly contain
itself.

The build/check and publication contracts independently hash and compare the
exact observed staged files. Publication and release receipts bind those exact
observed digests. A change in deterministic implementation invalidates the
preapproval identity; a nondeterministic or externally altered output fails the
observed-stage comparison. This two-part rule avoids a circular identity while
still requiring both derivation completeness and exact-byte observation.

### Approval-role binding matrix

This matrix is executable authority. Every listed input is mandatory, and an
event with a missing, stale, substituted, or recombined input is invalid.

| Approval role | Required bound roots and predecessor events |
| --- | --- |
| scientific | `finding_ledger_root`, `content_review_subject_root`, `science_root`, `communication_root`, `review_governance_root` |
| reproduction/publication | all scientific inputs; `preapproval_realization_root`; every applicable exact scientific approval event ID |
| assurance steward | all scientific and reproduction/publication inputs; `attribution_root`; `pre_steward_approval_root`; every applicable exact scientific and reproduction/publication approval event ID |
| release transfer | `approval_lock_root`; `realization_root`; exact target release identity; exact steward approval event ID; release owner; steward; decision; date |

The schema and projection code define which roles are required for a report,
but may not weaken the bindings in this matrix. Steward approval is ordered
after the required scientific and reproduction/publication approvals; events
from different predecessor sets cannot be recombined. Lifecycle invalidation
is derived mechanically from this matrix and the root dependency graph, never
from a separately maintained list.

## Exhaustive Layer Projection

Layer membership is executable authority, not prose guidance. Production
projection functions must destructure every typed schema object without a
catch-all, and report dependencies must carry a typed identity class. A new
schema field, file class, or dependency type fails compilation or validation
until deliberately projected.

Exhaustiveness tests must prove that every admitted field and file participates
in at least one applicable root or has an explicit tested exclusion. Intentional
multi-binding is allowed. The algorithm version, Markdown parser version when
used, raw-HTML treatment, generated-region sentinel, path ordering, and
canonical JSON rules are part of the identity contract.

Reader-facing attribution and lifecycle text may be excluded from
`communication_root` only inside exact directives such as
`{{ assurance_attribution }}` and `{{ assurance_lifecycle }}`. The builder
renders every such region from one structured source. Equivalent facts in
manuscript front matter, About-this-report, supplement, or the current agent
packet must be migrated or remain communication-bound; they may not be silently
ignored.

The agent-assistance packet is split into an immutable original generation
record and a generated current-governance summary. The latter is never a second
authored source for attribution or lifecycle facts.

## Typed Amendment Classes

`amend` accepts only versioned typed operations. It never accepts hashes, a
generic source patch, or a “bless,” “sync,” “refresh,” or “adopt” request.

### Attribution correction

An attribution correction changes display name, affiliation, or another
non-capability bibliographic field. A report-scoped correction selects an
existing immutable principal-record version. A global principal correction
creates a new principal-record version, calculates the complete consumer set,
and atomically updates every selected consumer or refuses the operation.

This class may qualify for `metadata-fast`. It cannot add role eligibility,
change identity authority, assign report lead, or change material-producer
status.

### Role assignment

A role assignment records report lead, contributor, material producer, or
build maintainer against an existing eligible principal and required
attestation. It is `governance-focused`, not metadata-fast. It recalculates
independence and the content review subject, invalidates downstream ledgers and
approvals, and records the resulting lifecycle disposition.

Global role eligibility or identity-authority changes use a separate governance
operation that calculates all consumers. They never qualify for a fast metadata
receipt.

### Normalization

Normalization invokes the identified `uk2us` executable without a shell and
accepts only its exact idempotent whole diff. Protected regions include
directives, code spans and blocks, identifiers, paths, link targets, reference
definitions, numbers, equations, table cells, and mixed-case scientific
abbreviations. Any converter change to a protected region fails closed.

`normalization` is the only editorial-fast operation in the initial
implementation. Future editorial transformers require their own finite
transformation contract and tests. A human-authored copyedit, including a
qualitative word change, remains an ordinary scientific/communication change
requiring the existing impact-review process.

The old `normalize` command remains a one-cycle compatibility alias for
`amend normalize`.

### Implementation Rebinding

`amend rebind-implementation --all` recalculates generated review locks after
the assurance identity, assembly, lifecycle, planner, or publication
implementation changes. It cannot alter report sources or authority events.
It is a no-op when every generated lock already binds the current embedded
implementation identity, and it fails closed when an existing approval cannot
bind the recalculated realization. This operation replaces one-off migration
parsers as the ordinary post-implementation-change path. The operation may
adopt drift only for its finite implementation-contract surface: the v2 README
and the enumerated v2 schemas. Report descriptors, manuscripts, supplements,
evidence, principal records, catalog data, and events remain strict and require
their own typed operation. A changed rebind receipt is `scientific-full`, names
the implementation-package gate, and carries no focused gate argv; it therefore
cannot authorize the proportional receipt runner. Rebinding generated identity
does not reduce the full closure required for the implementation change that
made the locks stale.

## Lifecycle State Matrix

The initial implementation is deliberately narrow:

| Operation | `DRAFT` | `IN_REVIEW` | `APPROVED` | `PUBLISHED`, `SUPERSEDED`, or `WITHDRAWN` |
| --- | --- | --- | --- | --- |
| inspect or verify | allow, read-only | allow, read-only | allow, read-only | allow, read-only |
| attribution correction | allow | allow; preserve the content review subject, ledger, and scientific/reproduction approvals; invalidate steward approval and every downstream lock, realization, or transfer that binds `attribution_root` | reject; require typed reentry or new version | reject mutation; require new patch version and supersession workflow |
| role assignment | allow | allow only through governance event; create a new content review subject and invalidate downstream review authority | reject; require typed reentry | reject mutation; require new version |
| normalization | allow | allow; preserve `science_root`, create a new communication/content-review subject and ledger, and invalidate every approval that binds `communication_root`, including scientific, reproduction/publication, and steward approval | reject; require typed reentry or new version | reject mutation; require new patch version |
| implementation rebind | allow when generated locks are stale | allow when current events remain valid; recalculate generated locks only | allow only when every existing approval remains valid against the current implementation identity; otherwise reject | reject mutation; require new version or explicit lifecycle workflow |
| enter review | allow transition to `IN_REVIEW` | no-op only when subject is identical | reject | reject |
| finding or disposition | reject | allow | reject | reject |
| approval | reject | allow only for current ledger and eligible independent principal | no-op only for identical event | reject |
| withdraw or supersede | typed lifecycle rules only | typed lifecycle rules only | typed lifecycle rules only | allow only as a new immutable governance event |
| release transfer | reject | reject | allow only through an immutable event satisfying the approval-role matrix | immutable historical event only |

An `IN_REVIEW` role assignment or normalization with existing findings does not
rewrite them. It retains old events as history and requires an explicit
carry-forward, reverification, or reopen disposition for each finding. Existing
approvals are never carried forward. A bibliographic-only attribution
correction does not change the content review subject or finding ledger and
therefore needs no finding carry-forward.

Every invalidation in this table is calculated by traversing the normative root
and event dependencies. Tests must prove the table and the calculated closure
agree. In particular, a bibliographic-only attribution correction must not
invalidate scientific or reproduction approval, while normalization must
invalidate scientific approval because that event binds `communication_root`.

## CLI Contract

The required command surface is:

```text
openwepp-assurance inspect --report <id> [--format human|json]
openwepp-assurance amend attribution --report <id> <typed flags> (--check|--apply)
openwepp-assurance amend principal --request <yaml> (--check|--apply)
openwepp-assurance amend role --report <id> --request <yaml> (--check|--apply)
openwepp-assurance amend normalize --report <id> --language en-US (--check|--apply)
openwepp-assurance amend rebind-implementation --all (--check|--apply)
openwepp-assurance amend recover (--inspect|--finish-cleanup|--restore-old)
openwepp-assurance lifecycle --report <id> --request <yaml> (--check|--apply)
openwepp-assurance verify-generation --base-ref <commit>
```

The lifecycle request schema provides typed variants for review entry, finding,
disposition, approval, withdrawal, supersession, and release transfer. The
command records a supplied human decision; it never invents one.

Ordinary attribution flags cover existing-principal display/affiliation
corrections. YAML is reserved for new principal versions, multi-consumer
changes, and governance events. Requests contain semantic inputs and
attestation references only. An optional `--if-generation <opaque-id>` supports
automation compare-and-swap; humans never copy a generation hash into a request.

`--check` calculates and validates the complete candidate without writing.
`--apply` is required for mutation. Global principal operations report and
update the complete induced report set. Implementation rebinding is the only
ordinary all-report generated-lock recalculation. All other amendments select
exactly one report. Interactive prompting and implicit lifecycle transitions
are prohibited.

Successful human output is short: operation, changed/no-op, impact class,
affected reports and paths, old/new root abbreviations, invalidated authority,
receipt path, and gate IDs. JSON output contains the deterministic receipt.

## Fast Workflow

Build the implementation once outside routine amendment timing:

```text
cargo build --release -p openwepp-assurance
```

A routine role assignment then has two mechanical steps:

```yaml
schema_version: 1
operation: role_assignment
principal_id: roger-lew
assignments:
  report_lead: true
  material_producer: true
attestation:
  authority: direct_operator_statement
  statement: I lead development of the snow and frost campaign in openWEPP.
```

The request contains no digest and does not require a work-package artifact.
Save it to a temporary file, then run:

```text
target/release/openwepp-assurance amend role \
  --report snow-and-frozen-soil-process-evaluation \
  --request /tmp/report-lead.yaml --apply
.venv/bin/python tools/local_ci/run_assurance_amendment.py \
  --receipt assurance/v2/transactions/<receipt-id>.json
```

The first command prints the exact receipt path. The runner validates the
receipt against the current generation, resolves versioned gate IDs to
structured executable/argument arrays, runs the complete focused contract once,
and writes one JSON evidence record. It never invokes a shell.

A valid `metadata-fast`, `editorial-fast`, or `governance-focused` receipt for a
report-data-only change is the complete local gate authority. It explicitly
forbids a work package, coding-agent review, terminal verification, full/quick
workspace profile, adjudicated CRAP, scientific reproduction, and duplicate
staging builds. Escalation is permitted only when the command refuses the
focused class, assurance implementation/schema/builder code changed, or a
publication/release operation is requested.

## Transaction And Recovery

Every check or apply operation must:

1. acquire the existing confined exclusive transaction lock;
2. reject unresolved recovery state;
3. capture the complete mutable v2 generation plus a read-set manifest for
   every identity-bearing dependency outside `assurance/v2`;
4. validate current source, lock consistency, and optional compare-and-swap;
5. calculate the complete affected-report closure;
6. apply the typed mutation in an isolated candidate generation;
7. regenerate structured reader blocks, locks, affected ephemeral staging, and
   the candidate receipt;
8. calculate invalidation and gate IDs from the dependency graph;
9. reopen and validate the candidate with production parsers;
10. prove a repeated candidate calculation is byte-identical;
11. reverify the complete internal and external read set immediately before
    exchange;
12. atomically exchange the mutable generation for `--apply`; and
13. reverify, sync, and clean up using the existing rollback versus
    committed-cleanup distinction.

Candidate validation renders and checks one ephemeral realization internally;
it does not require a caller-maintained staging root. Immutable evidence bytes
are copied only when they are in the affected closure. A scaled fixture must
prove that the strategy remains bounded at 100 reports and 32 MiB of assurance
content; if full copying misses the performance gate, implementation must use a
confined closure snapshot or a proven copy-on-write mechanism.

No production implementation may edit YAML or JSON by string replacement.
Parse typed structures and serialize canonical machine-owned files. Preserve
authored Markdown except for an enumerated deterministic transformation.

Failure before validated exchange leaves the old generation active. Cleanup
failure after validated exchange leaves the new generation active, returns the
committed receipt, and blocks later mutation. `amend recover --inspect` reports
both generations; `--finish-cleanup` or `--restore-old` verifies the selected
generation before acting. Manual recovery-directory deletion is not valid.

## Mechanical Impact And Gate Selection

Receipts contain stable versioned gate IDs. Permanent receipts do not treat
copied shell strings as authority. Human display may show commands, while JSON
uses executable/argument arrays resolved by the current gate runner.

| Class | Qualifying operation | Complete local proof |
| --- | --- | --- |
| `metadata-fast` | report-local or complete-consumer attribution correction with unchanged science and communication roots | amendment contract, named validation and ephemeral build/check, generated-reader lint |
| `editorial-fast` | exact approved deterministic normalization | amendment/normalization contract, named validation and ephemeral build/check, protected-region proof |
| `governance-focused` | typed role or lifecycle event with unchanged scientific source | lifecycle/publication contract, root/role/invalidation checks, named build/check |
| `scientific-full` | arbitrary prose, scientific, mixed, unknown, schema, builder, or authority change | owning scientific or implementation package and its full gates |
| `release-full` | publication, snapshot, release transfer, or vendoring | release and real-consumer gates |

Contract tests must prove that the three focused receipt classes cannot emit
package creation, agent, workspace `quick`/`full`, CRAP, comparator, scientific
reproduction, publication, or release gate IDs.

## Performance Acceptance

Measure the prebuilt release binary, not `cargo run`. Separate compilation from
routine workflow timing. On the current two-report corpus and the scaled
100-report/32-MiB fixture, run at least ten isolated trials and record p50, p95,
maximum, host, corpus size, report count, binary identity, and selected focused
test manifest.

- inspect and no-op check p95: at most 2 seconds;
- amendment transaction p95: at most 5 seconds on the current corpus and at
  most 10 seconds on the scaled fixture;
- named ephemeral build/check p95: at most 10 seconds;
- complete focused receipt runner p95: at most 60 seconds;
- end-to-end apply through evidence receipt p95: at most 60 seconds and maximum
  at most 120 seconds; and
- any focused run exceeding 300 seconds is a hard workflow regression.

Profile selection is pinned by a tested gate manifest so publication, full-
workspace, snow/frost cohort, comparator, and CRAP tests cannot enter silently.
These are workflow regression limits, not scientific acceptance criteria.

## Other Mechanical Work

`ASSURE-MAINT-01` produces an ordered follow-up queue; it does not implement
these additional commands unless one is strictly necessary for the amendment
transaction:

| Candidate command | Mechanical work | Human boundary |
| --- | --- | --- |
| `report scaffold` | create schema-valid directories, templates, directives, and logical IDs | select question, scope, and accountable lead |
| `object ingest` | hash, register, license-tag, and inventory a research object | decide relevance, redistribution, and restriction |
| `reproduce` | run a confined declared procedure, capture inputs/results/timing, and emit a receipt | judge method and result adequacy |
| gate evidence recorder | capture gate IDs, argv, status, timing, protected hashes, and line counts | disposition failures and findings |
| generated catalogs and revision histories | render current report lists and transaction history | decide publication and supersession |

Current-package mechanization does include production fixture mutation APIs to
replace repeated test-only `refresh_local_hash`, `refresh_report_hash`, and
`refresh_catalog_identity` helpers, plus deterministic protected-surface,
public-count, and changed-path evidence.

Scientific question selection, method choice, data suitability, interpretation,
limitations, arbitrary prose impact, competence, independence, finding
substance, approval, and release decisions remain human judgments.

## Security And Audit Requirements

- All paths are repository-relative, confined regular files; symlinks and
  traversal are rejected.
- No network access, shell interpolation, hidden fallback, or automatic
  dependency installation is permitted.
- Requests and receipts contain no secrets or absolute workstation paths.
- Every internal and external identity input is captured and reverified across
  the transaction.
- The genesis, old generation, request, receipt, tool identity, changed-path
  set, and trusted Git/base anchor reproduce and audit the new generation.
- A generated-lock change without a valid transition receipt fails CI; internal
  consistency alone is not represented as provenance proof.
- Concurrent drift, duplicate identity, privilege change in a metadata request,
  role conflict, unsupported lifecycle, or unclassified diff fails closed.

## Compatibility And Migration

`ASSURE-MAINT-01` migrates both current v2 reports while public report count is
zero. It preserves scientific values, claims, methods, results, tables, figures,
evidence, conclusions, and public state. Generated attribution directives may
replace inventoried duplicate attribution prose; every replacement is recorded
in the migration receipt.

The groundwater report remains `DRAFT`. The snow/frost report's current
monolithic `IN_REVIEW` root is retained as historical migration evidence, then
invalidated. Because it has no independent findings or approvals, the migration
records a new review-entry event under the layered root and leaves the report
`IN_REVIEW` with independent human review still pending. It does not claim that
calculated equivalence preserved human review authority.

`migrate-identities` was the one-time all-report initialization operation and
refused an already migrated tree. It and the old embedded-hash parser were
deleted after the atomic migration; there is no dual parser in the completed
implementation. The existing `normalize` alias remains for one deprecation
cycle.

## Acceptance Requirements

Implementation is complete only when:

1. no ordinary human or agent request contains a derived hash, and no authored
   report/catalog/assistance source stores a calculated file digest or root;
2. the identity graph is acyclic and exhaustive, generated-file inclusion and
   exclusion are tested, and an independently anchored generation chain
   verifies;
3. an `IN_REVIEW` attribution correction and role assignment each complete
   through a typed transaction and focused receipt runner without changing
   science or communication roots;
4. arbitrary prose cannot receive a fast class, while normalization proves its
   complete protected deterministic diff;
5. lifecycle events bind immutable current roots, old authority is never
   rewritten, and invalidation follows the state matrix;
6. principal changes calculate the complete consumer set and capability changes
   cannot enter metadata-fast;
7. current report science, exact retained values, and zero-public boundary
   remain unchanged through migration, with the prior snow/frost review root
   retained only as history;
8. external-input drift, stale generations, partial transactions, and recovery
   defects fail closed through deterministic fault tests;
9. a valid focused receipt is sufficient to finish a routine amendment without
   a work package, agents, CRAP, duplicate staging, or workspace-wide tests; and
10. current and scaled-corpus measurements satisfy the end-to-end performance
    limits.
