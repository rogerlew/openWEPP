# Scientific Assurance V2 Source, Build, And Dependency Contract

Status: active v2 contract — implementation staged through ASSURE-04

## Design Principle

The manuscript is the canonical scientific argument. Structured records supply
stable identities and mechanically resolvable results; they do not generate the
argument. Build behavior is deterministic, offline, and auditable.

## Future Source Layout

ASSURE-04 may implement the following conceptual layout under `assurance/v2/`
after ASSURE-03 has retired the v1 public candidate:

```text
assurance/v2/
  catalog
  reports/<report-id>/
    manuscript.md
    supplement.md
    dependencies
    claims
    methods/
    results/
    figures/
    research-objects/
    references
    review
    publication
```

The names without extensions denote contracts, not a prematurely frozen file
format. ASSURE-04 chooses the smallest serializations demonstrated by the
accepted prototype and records them in versioned schemas.

`manuscript.md` and `supplement.md` are human-authored sources. Claim, result,
figure, reference, review, and publication records carry identifiers and
metadata needed for mechanical checks. Generated prose is not canonical source.
The `research-objects` name denotes the manifest for the mandatory public-safe
reproduction surface; it does not authorize copying protected evidence.

## Stable Identities

Each report, version, claim, method, dataset, software realization, result,
table, figure, review, and snapshot has a stable identifier within its scope.
Every identity resolves to:

- owner and descriptive title;
- content or immutable external identity;
- units and quantity semantics where numeric;
- provenance and creation procedure;
- dependency links;
- review and currency state where applicable; and
- access and licensing information for public research objects.

Paths are locators, not sufficient identities. Git commits, SHA-256 digests,
dataset/software DOIs, and versioned repository releases are accepted identity
components. Restricted evidence records its existence, restriction, and review
path without leaking protected locations or content.

Cross-references are logical identities with output-specific resolution. A
source report may identify a contributor document, science contract, public
narrative, supplement, or retained asset; the builder resolves it to a portable
public route, release-snapshot object, or durable external identifier. It fails
rather than emitting a relative `docs/` or workspace link that will break after
`usersum` is vendored.

## Dependency Classes

The builder understands at least these edges:

| Dependency | Example impact |
| --- | --- |
| Manuscript to claim/result/table/figure | Rebuild and scientific impact review when referenced content changes |
| Report to supplement | Version and publish as one reviewed unit |
| Report to model narrative/science contract | Cross-reference check; science-owner review when authority changes materially |
| Method to dataset/software/configuration | Reproduction and currency review |
| Table/figure to result data and generation procedure | Regenerate/check; preserve source values |
| Claim to review decision | Block approval if the reviewed root no longer matches |
| Report to public research objects | Block publication when a required safe object is absent, stale, inaccessible, or not review-bound |
| Published report to release realization | New transfer check and immutable release snapshot |
| Public catalog to approved report | Rebuild navigation only after publication authorization |

Transitive impact is explicit. A changed dependency never silently leaves a
publication “current.” The role matrix under Review Locks determines who may
choose among rebuild, rereview, new report version, supersession, or withdrawal
and records the reason; there is no generic or self-appointed “impact owner.”

## Source Of Claim-Bearing Values

Numbers in conclusions, results, captions, tables, and figures resolve from
retained result objects rather than being independently retyped in multiple
places. Every value carries units, precision policy, method/result identity, and
software realization. A manuscript may state a value in prose, but the builder
must prove it matches the referenced result identity.

Tables have machine-readable source rows. Figures have source data, an
identified generation procedure, accessible text or tabular alternatives, and
a caption that explains the scientific point. Hand-drawn conceptual diagrams
declare that they are explanatory rather than result-bearing.

Every safely redistributable project-owned claim-bearing value, table/figure
source, analysis procedure, configuration identity, and reproduction
instruction is copied or durably linked through the version-bound public
research-object manifest. Restricted objects stay internal with a public
restriction record. Publication fails when a required safe object is absent.

## Build Operations

The minimal v2 tool supports equivalent operations for one report or all
reports:

1. **Validate** source shape, identities, references, units, and lifecycle
   permissions without rendering.
2. **Plan** the transitive dependency graph and report exactly why each target
   is current, stale, blocked, or selected for rebuild.
3. **Build** deterministic staging outputs from canonical sources and retained
   dependencies.
4. **Check** that rebuilt outputs match tracked outputs and that no undeclared,
   missing, stale, or unused dependency remains.
5. **Publish** only an approved, review-locked source root to the public tree.
6. **Snapshot** the exact approved source, safe dependencies, generated outputs,
   reviews, tool version, and target openWEPP release identity.

An ordinary operation has no network, shell, wall-clock-content, random,
hostname, absolute-workspace-path, or agent dependency. Reproduction procedures
that run scientific software are separate explicit operations with retained
logs and outputs; they do not run as a side effect of rendering.

American-English normalization is an explicit author-maintenance operation,
not an ordinary build operation. For a pre-review `DRAFT`, the maintenance
tool may invoke the canonical `uk2us` executable without a shell, accept only
its exact idempotent output for the manuscript and supplement, and
mechanically propagate the resulting content identities through the disclosed
agent packet, report descriptor, and catalog. It must check the current source
before mutation, replace the complete v2 source generation transactionally,
reopen and validate the new generation, restore the old generation on any
pre-commit or validation failure, and emit a deterministic receipt outside the
content graph. The transaction preserves source permission modes and syncs
directory entries around generation exchange and cleanup. Once the new
generation has validated, a cleanup failure does not roll back to a possibly
partially removed old generation: it returns the committed receipt and retains
any recoverable old-generation directory for explicit disposition. Retained
recovery state blocks later normalization, including no-op checks, until that
disposition occurs.
It cannot normalize
an `IN_REVIEW` or `APPROVED` source, authorize review, classify arbitrary prose
edits, waive a new root, or decide that scientific meaning is unchanged.

The implemented replacement for manual hash propagation and the former
normalization-only maintenance surface is the
[assurance amendment and generated-identity specification](../specifications/assurance-amendment-and-identity-workflow.md).
Authored sources now contain logical paths and semantic inputs without derived
digests or calculated roots. Generated identity and review locks, immutable
authority events, and transaction receipts preserve fail-closed review and
publication behavior. `normalize` remains a compatibility alias for the typed
amendment transaction during one transition cycle.

After reviewed assurance implementation code changes, the typed
`amend rebind-implementation --all` operation recalculates generated review
locks without changing authored sources or immutable authority events. A
repeat is a mechanical no-op. Existing approval bindings remain authoritative
and cause the operation to fail closed when they do not admit the current
implementation identity. Only the enumerated v2 README and schema files are
admissible implementation-contract drift; report, evidence, principal,
catalog, and event drift remains rejected.

## Incremental Rebuilds

One-report builds traverse the selected report's complete transitive dependency
set and may update only that report, its supplement, and affected shared
navigation in staging. All-report builds traverse the full catalog. Both use the
same validation and rendering logic.

The planner compares content identities, schema/tool versions, and declared
edges. File modification time is not authority. A source change that leaves the
rendered bytes unchanged can still require scientific impact review.

## Review Locks

Generated review locks separate science, communication, attribution, review
governance, findings, approval, realization, and release transfer. Immutable
approval events bind the exact applicable roots and predecessor event IDs.
Scientific and reproduction/publication approvals bind:

- report and supplement source roots;
- claim, method, result, table, figure, and reference identities;
- material model narrative and science-contract versions;
- disclosed agent-assistance packet, if any; and
- the software realization assessed.

Any bound change invalidates the dependent event or lock mechanically. Renewal
follows this matrix:

| Change | Minimum decision and approval |
| --- | --- |
| Bibliographic attribution only | Attribution changes without changing the content-review subject; scientific and reproduction approvals survive only when their complete bound input set remains current; steward and later authority are recalculated |
| Deterministic DRAFT normalization | Communication and content-review roots change before review; the receipt proves the protected transformation, not scientific equivalence |
| Claim, method, dataset, result, table, figure, software realization, or science authority | New root; affected independent scientific and reproduction reviewers plus assurance steward approve a bounded impact disposition or repeat full review |
| Builder, schema, or template | New root; independent publication/reproduction reviewer plus assurance steward approve semantic equivalence; any scientific meaning change follows the material row above |
| Unclear or mixed impact | Full scientific, reproduction, and publication rereview |

The report lead or any material dataset, method, result, table, or figure
producer cannot be the sole scientific approver. The report lead, a material
dataset/method/result producer, or the build maintainer cannot be the sole
reproduction approver. None of those producers or maintainers may be the sole
material-change waiver authority.

Every renewal record binds the old and new content roots, changed identities,
classification and rationale, named approvers, role-independence attestations,
and resulting review scope. The builder fails closed on a missing, stale,
self-incompatible, or incomplete record; it verifies the decision and never
makes it.

## Agent-Assisted Work

Agents may assist research synthesis, drafting, code inspection, result
reconstruction, or review only through a versioned procedure. The retained
packet contains:

- procedure version and objective;
- content identities of every supplied input;
- tool/model identity and material configuration;
- exact output used by the human author;
- human edits or disposition where material;
- known nondeterminism and limits; and
- independent review.

Ordinary builds never call an agent. Human authors remain accountable for the
manuscript, and agent review is not external scientific peer review.

## Testing Boundary

ASSURE-04 should use ordinary Rust tests and `cargo nextest` profiles to test
schema admission, dependency planning, deterministic rendering, staging/public
separation, review locks, drift, snapshots, and real public-catalog consumers.
Report-specific reproduction tests may run through nextest when they are
executable and appropriately isolated. Nextest does not decide which scientific
assets are dependencies and does not replace a reproduction record or
scientific review.
