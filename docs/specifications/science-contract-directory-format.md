# Directory-Based Science Contract Format

Status: specified; adoption gated (no contracts migrated by this specification).
Format identifier: `directory-v1`. Specification revision: 1, 2026-09-07.
Owner: openWEPP science-contract maintainers.
Parent schema: [Science Contract Artifact Specification](science-contract-spec.md).

## Authority and adoption

A canonical contract consists of its retained `SC-<DOMAIN>-<NNN>.md` entry
point and its explicitly listed normative modules. Each binding definition has
one canonical location within that document set. The entire contract remains
binding; the entire contract need not be loaded into every agent's initial context.
Reading selection routes access to authority, never exemptions from authority.

This specification defines a prospective format, not scientific amendments or
permission to migrate an existing contract. Until a separately authorized
migration meets [Adoption gates](#adoption-gates), existing single-file schema,
whole-contract reading, checker behavior and frozen package bindings continue.
The separately authorized [checker implementation and LSE pilot](../work-packages/20260907-directory-contract-checker-lse-adoption-001/artifacts/worker-handoff.md)
implements structural checker support and retains its historical HOLD. The separate
[LSE correction package](../work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md)
adopts the v32 LSE directory set; its canonical entry governs selective reading and
recursive dependencies. The entire normative set remains binding.
A legacy lint PASS cannot qualify a directory contract. Small contracts may remain
single files indefinitely.

No equation, constant, tolerance, selector, failure precedence, guard, state
ownership, restart, serialization, scientific claim or activation status changes
by relocation. Contract-first sequencing, applicable A0/A1/A3, real-consumer
proof, independent conservation reconstruction and anti-evasion remain binding.
An unresolved scientific interpretation blocks the affected migration; do not
choose the convenient amendment or declare old text historical by age.

## Layout and membership

Retain the registry path `contracts/SC-<DOMAIN>-<NNN>.md`. Chapters live in its
same-named sibling directory, for example `contracts/SC-LANDSURFACEENERGY-001/`.
Names reflect scientific mechanisms, not generic equations/constants/guards bins.
An illustrative inventory is interface, surface-energy-balance, soil-thermal-coupling,
nonlinear-solve, state-custody, binding-index, provenance and history Markdown files.
These names are not prescribed LSE boundaries; the pilot must establish them.

The entry has the parent schema's full target front matter plus:

| Field | Value / constraint |
| --- | --- |
| `contract_format` | Exactly `directory-v1`; absent means legacy single file. |
| `binding_index` | Entry-relative path plus explicit anchor to its sole BEI. |

The entry's `## Document inventory` table has exactly these columns:

| Path | Kind | Purpose | Applicability |
| --- | --- | --- | --- |
| entry-relative chapter path | `normative` or `historical` | Concern owned | Task/mechanism trigger or `all` |

This table declares exact membership; no globs, directory scans or implicit
normative inclusion. The entry itself is implicitly normative and not listed.
Every file beneath the chapter directory must be listed once. Normative content
in undeclared documents is a hard error, not optional context. Historical files
are retained non-authoritative material governed by the
[provenance lifecycle](science-contract-provenance-spec.md); normative chapters
are not provenance sidecars. A provenance chapter containing required assumptions
or derivations is normative, even if it is not always read at bootstrap.

Paths are case-sensitive UTF-8 repository-relative POSIX paths resolved relative
to the containing entry (inventory) or referencing chapter (links). Membership
paths must remain within the same-named chapter directory: no absolute paths,
`..`, backslashes, empty components, symlinks, duplicate paths or non-Markdown files.
Dependency links may reach other repository documents with relative `..` only
when normalized resolution stays inside the repository and encounters no symlink.
Missing/unreadable files, malformed UTF-8, unsupported format or ambiguous inventory
fail closed. External citations have explicit source identity and limitations;
the checker must not fetch network resources or imply it validated their contents.

## One revision and identity

The entry alone owns lifecycle metadata and one monotonic `contract_version`
covering the coherent set. Chapters name the parent entry, not independent versions.
Any membership or chapter change updates that revision under the authoring cycle;
the initial migration increments it without changing scientific meaning. Approval
applies to the reviewed set, never a mix of individually approved chapters.

Package authority bindings record repository commit plus exact inventory paths
and content hashes (including entry, normative and retained historical files).
Bind external dependencies by exact path, relevant explicit anchor and commit or
content hash. A dirty review cut requires per-file hashes; final committed identity
must reconcile them. Detached evidence manifests exclude themselves; no document
contains its own final hash or predicts a future commit. Changed bindings trigger
impact review under [evidence reuse](../standards/testing-and-gate-strategy.md#identity-membership-and-change-impact),
not automatic scientific equivalence. No new planner or admission service.

## Entry interface

Keep the entry a few KiB where practical, with no repeated detailed invariant catalog.
It contains identity/status/version, overall scientific scope, protected
cross-cutting obligations, document inventory, reading routes, and links to the
binding index, gap/qualification register and history. Explicitly distinguish:

- current scientific implementation authority;
- production activation/selector/cutover authorization;
- calibration, empirical qualification and release evidence status.

Contract approval does not activate a model or override a retained FAIL/HOLD.
Cross-cutting detail lives once in a shared interface module; the entry states
when it is mandatory and links exact boundary sections, not a second specification.

The entry's `## Reading routes` table names Task, Role/check, Initial material,
and Expansion trigger. Every route includes the entry and its cross-cutting
requirements. Unknown tasks, missing dependency edges or uncertain applicability
require expansion and recorded reconciliation before affected action, never
scope-based omission. Reading the whole set remains an available conservative route.

## Mechanism chapters and dependencies

Each chapter contains its current effective rule, applicability/regimes,
authority IDs/source anchors and specifically linked history where useful.
Keep each mechanism's equations, variable units, domains, constants, guards,
ordering, invariants, ownership interfaces, citations, assumptions and test
obligations together. Shared definitions have one home with precise references.
Detailed derivations can be separate; the citations and assumptions needed to
justify a mechanism's equations/constants cannot become optional history.

Each normative chapter starts with a parent-entry link and a `## Dependencies`
table: Target (path plus explicit anchor), Required when, Boundary/obligation,
and Reading extent. State `none beyond entry` explicitly if there are none.
The current package reading map binds those targets to revisions/hashes.
References labelled dependencies are binding reading when their condition applies;
bibliographic or historical links alone do not create whole-file reading rules.
Conditions must name concrete task effects/regimes, not agent discretion.

Resolve applicable dependencies recursively to the named section boundaries.
A selected section includes its directly referenced definition/assumption
dependencies. Include the destination entry's scope and routing constraints when
crossing contracts, but not automatically all its implementation chapters.
Cyclic interface dependencies are read as the finite union of required sections,
not repeated infinitely or used to omit either owner. Missing edges discovered
from the actual diff cause map expansion and review of affected work.

Examples of required routing (pilot fills exact paths/anchors):

| Task / role | Required concerns | Expansion trigger |
| --- | --- | --- |
| Surface-energy implementation | Entry, shared signs/units/owners, full surface-energy mechanism and dependencies | Soil/vapor/vegetation evaluator changes add those owners' applicable authority. |
| Soil transfer implementation | Entry, shared interface, coupling, receiver acceptance boundaries | Changed receiver internals add receiver mechanisms. |
| Solver correctness review | Entry, solver, every affected evaluator/boundary and error-order dependency | Equivalence, reuse or domain changes require full affected physics, not solver-only review. |
| Restart/owner-state work | Entry, shared state, custody, serialization authority | Changed transaction/rollback/publication expands to all affected owners. |
| Executable-identity verifier | Entry scope/qualification, frozen identity and capture protocol | Scientific result/closure claims add physical and operand authority. |
| Closure-reconstruction verifier | Entry, energy/operand definitions, shared units and receiver obligations | Changed lineage or interpretation expands relevant mechanisms. |

Role alone cannot exclude an obligation. Freeze selected modules/sections and
dependencies prospectively in the package; independent reviewers compare selection
with actual diff. Frozen historical kickoffs remain binding until explicitly
replaced through their authorized acceptance-change process, never by this router.

## Schema coverage and binding definitions

The parent schema and kernel profile's logical content requirements all remain.
For directory-v1, physical section order is replaced by a `## Schema coverage`
table in the binding-index chapter: Requirement, Canonical target(s), Applicability.
Use exactly the 32 coverage keys below, each once; unknown or duplicate keys fail.
A distributed requirement lists every owning section in its one row.

| Key range | Source list, in listed order |
| --- | --- |
| `artifact.section.01` through `artifact.section.18` | [Artifact Required Section Order](science-contract-spec.md#required-section-order), items 1-18. |
| `kernel.section.01` through `kernel.section.14` | [Profile Required Section Schema](science-contracts/kernel-process-contract-profile.md#required-section-schema-normative), items 1-14. |

For example, `artifact.section.08` maps invariants/guard maps;
`kernel.section.07` maps the profile's corresponding requirement. These are
coverage keys, not new scientific obligation IDs. Their meanings bind this
format revision and the listed schema/profile lists; an upstream list change
requires explicit format/checker reconciliation, not silent renumbering.
All subordinate requirements remain binding through their containing section:
draft readiness, metadata, table fields, algorithm detail, calibration/readiness,
typed failures, enforcement and compliance duties are not reduced to headings.
Metadata is checked separately; other rule lists are not independent coverage
key namespaces. Review must verify this subordinate content, not just 32 rows.
For non-kernel contracts retain kernel rows with explicit profile-inapplicability
rationale; otherwise all applicable rows resolve to normative section targets.
Not-applicable needs the original schema's allowed rationale, not a layout
exemption. Mechanism chapters retain
complete algorithm pre/postconditions, guards, aliases, units, calibration posture
and test-vector duties. The coverage table is a locator, not copied authority.

The binding-index chapter contains one `## Binding definitions` table with
columns ID and Definition (chapter-relative path plus explicit anchor).
Every current `INV-*` / `OBL-*` definition occurs exactly once in normative material.
Retain existing IDs, including producer/consumer-qualified OBL IDs. Definitions
use one encoding: a Markdown table row with the first cell exactly an HTML anchor
followed by one space and the same backtick-delimited ID. Leading/trailing cell
whitespace is ignored; anchor syntax is exactly `<a id="ID"></a>` with double
quotes and no extra attributes. The anchor id equals that row's binding ID.
The table header is `Invariant ID` or `Obligation ID`; invariant tables retain
all parent-schema columns, and obligation tables require `Statement`,
`Applicability`, `Authority`, `Enforcement/failure`, and `Test bindings` columns.
Required cells are nonempty. Normative semantic adequacy remains a review duty.
Heading-only and standalone-anchor declarations are not supported by directory-v1.
Fenced code blocks are examples, never definitions; the scanner must exclude
CommonMark backtick and tilde fences (including longer fences and info strings).
Literal unescaped table pipes inside a cell are unsupported; use `&#124;`.

Illustrative accepted row form (artificial ID; this fenced example is not authority):

```md
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
| --- | --- | --- | --- | --- | --- |
| <a id="INV-EXAMPLE-001"></a> `INV-EXAMPLE-001` | Example requirement. | Example source. | [DIRECT][Static] | Example guard. | Typed failure. |
```

Rejected declaration forms: a first cell containing only `INV-EXAMPLE-001`;
an anchor before/outside the row; a heading containing the ID; an anchor whose
ID differs from the cell ID. Mentions in prose and other reference tables are
permitted but cannot satisfy a binding registry target. Any definition-shaped
anchor outside this grammar is an error, except explicitly declared entry aliases.
The table locates actual definitions; mere ID mentions, aliases, registry rows,
BEI rows or copied lists are not definitions and cannot satisfy existence.

One `## Binding Exposure Index` retains the parent schema's seven columns and
vocabularies. It maps every former addendum/sidecar and migrated source span to
surviving IDs. Source references identify original commit/path/anchor (line spans
where necessary), not line numbers against a rewritten file. Detailed preservation
evidence stays in the migration package; the canonical index remains resolvable.
Current definitions may be in entry or normative chapters, never historical files.
Unpromoted/undecidable residue retains binding text until adjudicated; default
PASS-DEFERRED is not migration completion. Uncertain supersession blocks adoption.

## Resolving amendments and preserving references

Before relocating a source span, inventory every binding clause, including those
without an ID: equations, qualifiers, exceptions, guards, ownership, source and
test anchors. Map it to a current definition, or document exact superseding
authority and applicability with independent review. Adding IDs to unindexed
binding residue follows existing flagged-binding-addition review; do not invent
science to close a mapping. Keep surviving restrictions from earlier versions.

Current chapters state effective rules directly. History preserves prior wording,
rejected candidates, failures and rationale without requiring a reader to solve
an amendment puzzle. If an amendment's applicability cannot be resolved without
a scientific decision, retain current authority and HOLD the affected migration.
Unrelated authorized architecture work is not blocked by that historical HOLD.

Stable logical identifiers remain `SC-...#INV-...` / `SC-...#OBL-...`; physical
references resolve via the binding-definitions table to the canonical chapter.
Migration inventories incoming current links and tests. Preserve old entry anchors
as explicit non-defining compatibility aliases with a `## Compatibility anchors`
table (Alias, Target), or update every unpinned live consumer before adoption.
Alias targets are actual definitions/sections; no alias chains or second definitions.
Checker excludes declared entry aliases from definition counting and verifies each
target and alias uniquely. No unlisted definition-shaped alias is permitted.
Commit-pinned references and historical work-package bytes are never rewritten;
their original Git revision remains the reference, not current chapter text.

## Checker extension contract

Extend `tools/check_sc_binding_exposure.py` in a separately authorized implementation.
Do not build a dependency engine or scientific-equivalence classifier. For
directory-v1 it must parse declared inventories and check:

1. Supported format, required metadata, one entry/revision, exact safe membership.
2. Local inventory, dependency, coverage, BEI and compatibility paths/anchors exist;
   explicit anchors are unique per file, normative dependencies cannot resolve
   to historical-only authority, and malformed rows/empty required tables fail.
3. Every binding registry target is an actual marked substantive definition in
   normative material; every such definition has one registry row. Duplicate
   definitions, absent IDs, history-only IDs, and ID mention padding fail.
4. BEI mapped IDs resolve through that registry; retain sidecar provenance fields,
   statuses, supersession pointers and coverage duties from the parent specs.
5. Schema coverage includes all required logical requirements; mechanical presence
   does not certify scientific sufficiency or dependency completeness.
6. Unsupported/missing/malformed inputs return nonzero with path/anchor diagnostics;
   never retry directory data as legacy or fetch external content.

Preserve legacy invocation and PASS/FAIL/PASS-DEFERRED semantics. Hard violations
exit 1, usage errors exit 2; PASS exits 0, PASS-DEFERRED exits 0 by default and 1
with `--strict`. Directory adoption requires strict PASS plus independent semantic
review, never checker PASS alone. Legacy-only mode remains legacy-only evidence.

Required implementation conformance cases (not executed by this specification):

| Fixture or mutation | Expected result |
| --- | --- |
| Complete two-mechanism set, section dependencies, qualified OBL and preserved aliases | PASS; resolve actual definitions and all declared anchors. |
| Existing valid/invalid legacy fixtures | Preserve their legacy outcomes. |
| Duplicate definition across chapters; registry or BEI mention without a definition | FAIL. |
| Missing module/anchor, duplicate membership, unlisted chapter, historical-only definition | FAIL. |
| Absolute/escaping/member-symlink path, malformed UTF-8 or unsupported format | FAIL, no external read or fallback. |
| Missing sidecar provenance, false supersession target, invalid status | FAIL. |
| Unresolved owned science-review-follow-on with binding text retained | PASS-DEFERRED; strict fails and adoption blocked. |
| Compatibility alias missing/looping/duplicated or aimed at non-authority | FAIL. |
| Missing schema coverage or conditional dependency target | FAIL. |
| Qualifier removed but all IDs preserved | Structural checks may pass; preservation review must reject. |
| Authority wrongly classified historical; narrower role route hides closure operands | Semantic review rejects even if structural checks pass. |

## Adoption gates

The first migration should pilot LSE, not bulk-migrate snow or all contracts.
Its package must freeze exact chapter boundaries/write set and preservation
acceptance before edits, then complete all of these gates:

1. Source-commit-bound clause inventory and bidirectional coverage (old to current
   and current to original authority); no active qualifier/source/test anchor lost.
2. Effective-rule consolidation and independently adjudicated supersession; retain
   historical FAIL/HOLD, production activation limits and frozen references.
3. Implement/test the bounded checker extension above before adopting the format;
   reconcile affected live links, templates, guides and source-coupled Rust tests
   without deleting or weakening the obligation each test protects.
4. Strict checker PASS, complete schema/profile coverage, all affected checks,
   dual independent review, finding resolution and dual independent verification.
5. Fresh non-forked agents perform bounded surface/coupling, solver-review and
   identity-versus-closure verification reading exercises. They identify exact
   applicable rules from primary modules without reconstructing historical
   amendments. Incorrect or missing authority selection fails usability acceptance.
6. Measure same tasks/roles before and after: full/range bytes, unique versus
   repeated exposure, automatically required instructions, recursively applicable
   dependencies, bootstrap versus later expansion. Record workflow-total only if
   observed. An unconditional pointer is no saving; do not report bytes as tokens
   or quota. Demonstrate a smaller sufficient reading set, not just a smaller entry.
7. Reconcile exact final diff and publish the coherent entry/set revision with
   reviewed source identities. Only then enable its selective-reading route.

The migration package owns exact numeric byte results and reopening triggers.
This specification provides neither an LSE scientific adjudication nor measured
savings, and authorizes no solver experiment or rejected-candidate resurrection.
