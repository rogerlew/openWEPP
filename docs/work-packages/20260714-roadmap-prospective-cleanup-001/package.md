# Prospective-Only Roadmap Cleanup

Status: `complete`

Date opened: `2026-07-15 UTC`

Date closed: `2026-07-15 UTC`

Execution mode: `package-end-to-end`

Package type: documentation-only governance maintenance

## Objective

Rewrite `docs/ROADMAP.md` as a compact, prospective-only planning surface.
After this package, a reader should be able to identify what openWEPP intends
to do next, what is queued or deliberately deferred, and what condition moves
each item forward without loading completed program history into context.

## Why This Package Exists

The roadmap already declares that completed work belongs in the work-package
catalog, decisions, contracts, and retained artifacts. Its body contradicts
that rule: it is 1,124 lines and 125,845 bytes and contains completed program
narratives, terminal adjudications, and superseded sequences. That history is
valuable, but it obscures the live queue and makes every roadmap read
unnecessarily expensive for humans and agents.

This package changes information placement, not project authority. It removes
historical execution detail from the roadmap and preserves canonical pointers
to the repositories of record.

## Definitions

A **prospective item** is unfinished work with a current owner or owning queue,
a state, and a trigger or dependency that explains when it may advance.
Allowed roadmap states are `next`, `active`, `queued`, `held`, `recurring`, and
`deferred`. A deferred item remains prospective only when it has a named
promotion trigger and a durable authority or backlog pointer.

A **historical item** is complete, executed, superseded, rejected, terminally
adjudicated, or retained only to explain how the current state was reached.
Historical items belong in the work-package catalog, execution log, decisions,
contracts, or backlog records, not in `docs/ROADMAP.md`.

## Included Scope

- Inventory every current roadmap section and classify its content as
  prospective, historical, standing policy, or authority pointer.
- Preserve every genuinely prospective item in a smaller structure with an
  explicit state and trigger/dependency.
- Remove completed, superseded, and terminal execution narratives from the
  roadmap instead of summarizing them again.
- Keep a short routing section for canonical history, authority, and backlog
  surfaces.
- Reconcile the hydrograph/sediment backlog status narrowly where review proves
  the roadmap would otherwise route readers to a stale pre-implementation
  state.
- Update the work-package catalog and retain package-local evidence for the
  classification, size reduction, reviews, gates, and disposition.

## Excluded Scope

- Changing science contracts, kernel behavior, release criteria, priorities,
  or the scientific-assurance v2 sequence.
- Deleting or broadly rewriting historical work-package evidence, decisions,
  backlog records, or execution logs. The narrow status/remaining-scope
  correction to the hydrograph-resolved sediment backlog and tracker is in
  scope only because accepted traceability finding `B-002` proved the existing
  route stale.
- Cleaning up `docs/work-packages/README.md` beyond adding and closing this
  package entry.
- Creating new implementation packages for retained roadmap items.
- Rust, test, build-system, assurance-source, or generated-usersum changes.

## Intended Write Set

- `docs/ROADMAP.md`
- `docs/backlog/TRACKER.md`
- `docs/backlog/20260704-hydrograph-resolved-sediment-and-routing.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260714-roadmap-prospective-cleanup-001/**`

Any write outside this set requires a package amendment before editing.

## Protected Boundaries

- No prospective item may disappear merely because its current wording is
  embedded in a historical section.
- Removing a completed roadmap narrative does not rescind its decision,
  contract, evidence, or release status.
- The assurance queue remains documentation-first and keeps vendoring as a
  mandatory pre-beta WEPPcloud release gate.
- Backlog records remain the home for ideas that have not been promoted into
  the execution queue.
- The roadmap does not become another execution log or duplicate the detailed
  requirements of a work package.

## Deliverables

1. A prospective inventory that maps retained and removed content to its
   canonical destination.
2. A rewritten `docs/ROADMAP.md` no longer than 250 lines and 35,000 bytes.
3. Context-reduction and link-integrity evidence.
4. Two independent reviews, explicit finding disposition, two independent
   verifications, gate results, worker handoff, and final disposition.

## Phase Plan

### Phase 0: Intake and inventory

Record the instruction chain and frozen base. Mechanically inspect headings,
tables, state words, backlog links, and work-package identifiers. Classify each
section before deleting historical prose.

### Phase 1: Prospective rewrite

Replace the roadmap with the smallest useful planning surface: operating
rules, an immediate queue, the ordered scientific-assurance sequence, recurring
maintenance, deliberately deferred items with promotion triggers, and
canonical routing pointers. Preserve detail only when it changes a future
executor's choice.

### Phase 2: Documentation gates

Measure line and byte reduction, check Markdown and links with canonical
documentation tooling, search for prohibited historical status language, and
inspect the diff for accidental authority changes or lost prospective items.

### Phase 3: Review, remediation, and closure

Obtain two independent reviews. Disposition every finding as `accepted`,
`rejected`, `deferred`, or `follow-up`; fix accepted closure findings and rerun
affected gates. Obtain two independent verifications and close only when all
exit criteria have direct terminal evidence.

## Exit Criteria

| ID | Criterion |
| --- | --- |
| `ROADMAP-PROS-001` | Every retained execution item has an allowed prospective state plus an owner/queue and advancement trigger or dependency. |
| `ROADMAP-PROS-002` | Completed, executed, superseded, rejected, and terminal adjudication narratives are absent except for a short routing statement explaining where history lives. |
| `ROADMAP-PROS-003` | The inventory accounts for every original top-level section and every active-looking, queued, held, recurring, or deferred item before the rewrite. |
| `ROADMAP-PROS-004` | Scientific-assurance ordering, documentation-first gates, scientific pilot constraints, snow flagship position, and mandatory pre-beta vendoring gate are preserved without duplicating subsystem specifications. |
| `ROADMAP-PROS-005` | `docs/ROADMAP.md` is at most 250 lines and 35,000 bytes, with before/after measurements recorded. |
| `ROADMAP-PROS-006` | Markdown lint/validation, repository-relative link checks, and `git diff --check` pass on terminal source. |
| `ROADMAP-PROS-007` | Two independent reviews, complete finding disposition, and two independent verifications confirm prospective completeness, readability, authority preservation, gate legitimacy, and line-count governance. |
| `ROADMAP-PROS-008` | No file outside the intended write set changed and the package catalog identifies this package and its final disposition. |

Every criterion is classified `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`.
Anything other than `PASS` blocks `complete` disposition.

## Required Verification Commands

Run from `/home/workdir/openWEPP` and record concise results:

```bash
wc -l -c docs/ROADMAP.md
rg -n '(complete|completed|executed|superseded|adjudicated|rejected)' docs/ROADMAP.md
wctl doc-lint --path docs/ROADMAP.md
wctl doc-validate --path docs/ROADMAP.md
git diff --check
git diff --name-only
```

If `wctl` is unavailable in this repository, use the exact docs-maintainer
fallback identified by the skill and record the substitution. Inspect every
remaining historical-state match; a passing search is not defined as zero
matches because the short routing policy may name completed work.

## Review Plan

Reviewer A is the planning-information reviewer. It independently checks that
the roadmap is easy to scan, genuinely prospective, materially smaller, and
complete enough to choose the next authorized work without historical context.

Reviewer B is the governance and traceability reviewer. It independently
checks that no live or deferred obligation was lost, authority did not move,
links resolve, package gates have direct evidence, and history is routed to the
correct canonical surfaces.

Both reviewers inspect terminal source independently and do not read the
other's initial review before submitting their own. After accepted findings are
fixed, each independently verifies the relevant remediation and terminal gates.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent reviewer/verifier subagents for the bounded Reviewer A and
Reviewer B scopes above. Expected compact outputs are findings for
`artifacts/review-a.md`, `artifacts/review-b.md`,
`artifacts/verification-a.md`, and `artifacts/verification-b.md`. Their
implementation access is read-only; the parent owns all repository writes and
finding disposition.

Subagent requirement: dual independent review and verification are required.
No comparator or heavy-gate subagent is required for this documentation-only
package.

## Security And Code-Quality Impact

Security impact is `none`: this package changes tracked Markdown only and does
not add executable content, network access, credentials, or publication paths.
Review still checks for accidental secrets and unsafe external links.

The adjudicated CRAP gate and full Rust closure loop are exempt because no Rust
or implementation file is in scope. `.rs` line-count governance is `N/A`; both
reviews must record that no `.rs` file changed and therefore no 2,000-line or
3,000-line disposition is required.

## Hold Boundaries

The package may hold only if a current prospective item cannot be distinguished
from historical content using repository authority, or if required canonical
documentation tooling is unavailable with no safe fallback. Readability work,
inventory reconciliation, review findings, and Markdown failures remain
in-envelope and do not justify a hold.

## Progress

- [x] (2026-07-15) User authorized a work package to make the roadmap
  prospective-only and reduce reading context.
- [x] (2026-07-15) Applicable instructions and docs-maintainer procedure read;
  package scaffolded before roadmap edits.
- [x] (2026-07-15) Classified the pre-rewrite surface and rewrote the roadmap
  from 1,124 lines / 125,845 bytes to 159 lines / 11,702 bytes.
- [x] (2026-07-15) Initial Markdown, link, spelling, scope, and diff gates pass
  using the documented direct `markdown-doc` fallback.
- [x] (2026-07-15) Dual initial review completed: Reviewer A passed without
  findings; Reviewer B held with four findings, all accepted and remediated.
- [x] (2026-07-15) Terminal gates and both independent verifications passed;
  package dispositioned `EXECUTED-COMPLETE`.

## Surprises & Discoveries

- Observation: The roadmap's forward-only policy is already correct, but its
  body retains the history that policy prohibits.
  Evidence: Intake measured 1,124 lines / 125,845 bytes and found completed or
  terminal narratives across the general, winter, performance, watershed, and
  erosion sections.

## Decision Log

- Decision: Treat backlog-only ideas as non-roadmap content unless they have a
  named promotion trigger.
  Rationale: A roadmap cannot be both a compact execution queue and a duplicate
  of the backlog catalog.
  Date/author: 2026-07-15, Codex.
- Decision: Preserve the assurance sequence while collapsing its detailed v2
  requirements behind the canonical strategy and future work packages.
  Rationale: The sequence is prospective; subsystem design detail is not needed
  every time the roadmap is read.
  Date/author: 2026-07-15, Codex.
- Decision: Remove the old forest lateral-flow magnitude deferral rather than
  retaining it as prospective work.
  Rationale: The backlog tracker records its promotion and closure under
  `SC-SUBHYD-001#INV-SUBHYD-033`; the roadmap statement was stale.
  Date/author: 2026-07-15, Codex.
- Decision: Retain only `CQR-NIGHTLY` from the former watershed/CQR table.
  Rationale: The watershed, Lane D, follow-up, and pre-integration campaigns are
  terminal; CQR nightly remains an operator-triggered recurring process.
  Date/author: 2026-07-15, Codex.
- Decision: Expand the write set narrowly to reconcile the hydrograph/sediment
  backlog status after initial review.
  Rationale: Reviewer B proved the old backlog still described completed
  channel-hourly routing as future pre-contract work. Because the roadmap
  delegates unprioritized concepts to the backlog tracker, leaving that route
  stale would defeat this package's traceability goal. The amendment changes
  status and remaining-scope routing only; it does not reopen or alter science
  authority.
  Date/author: 2026-07-15, Codex.

## Outcomes & Retrospective

The roadmap shrank from 1,124 lines / 125,845 bytes to 166 lines / 12,664
bytes while retaining every reviewed prospective obligation. The most useful
review result was not stylistic: tracing apparently deferred work exposed a
promoted canopy program and stale hydrograph/HB-06 routing records. Restoring
the canopy row and reconciling those routes made the smaller roadmap more
truthful, not merely shorter.

The package closes with all eight exit criteria passing, four accepted review
findings fixed, dual verification passing, and no implementation or science-
authority change. The next authorized priority is the documentation-only
`ASSURE-02` scientific-assurance architecture package.
