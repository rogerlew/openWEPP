# ASSURE-MAINT-01 — Mechanical Amendment And Identity Workflow

Status: queued; user-authorized scaffold

Package ID: `20260716-assure-maint01-mechanical-amendment-workflow-001`

Frozen base: `15763d7f6d5d4125333d9b7583424c714f5f5ea4`

This ExecPlan is maintained under `docs/codex_exec_plans.md`. The `Progress`,
`Surprises And Discoveries`, `Decision Log`, and `Outcomes And Retrospective`
sections are living execution records.

## Purpose And Big Picture

After this package, a maintainer can make a bounded attribution, role, or
normalization change through one typed transaction and one receipt-gate runner.
The tooling calculates and installs all affected identities, proves the exact
impact, and finishes in minutes without an agent, manual hash propagation,
duplicate staging, or unrelated scientific evidence.

The package also migrates derived hashes out of authored files, introduces
layered report identities, and identifies other deterministic assurance work
that should be owned by commands rather than agent analysis. It makes no model-
science, empirical-result, public-report, release, export, vendoring, or
WEPPcloud change.

## Progress

- [x] (2026-07-16) User identified repeated 30–42 minute latency for spelling
  and attribution revisions and directed a specification, implementation
  package, and independent review.
- [x] (2026-07-16) Authored the canonical amendment and generated-identity
  specification and scaffolded this package.
- [x] (2026-07-16) Completed three independent specification reviews, accepted
  every finding, and revised identity acyclicity, lifecycle authority,
  proportional gate authority, operability, and scope.
- [x] Obtain final independent re-verification of the revised specification and
  package after closing the role-binding and release-transfer findings.
- [ ] Execute the migration and implementation milestones end to end.
- [ ] Run focused, performance, full closure, and adjudicated CRAP gates.
- [ ] Complete dual implementation review, finding disposition, heavy-gate
  delegation, and dual terminal verification.

## Surprises And Discoveries

- Observation: the existing `normalize --apply` already contains the confined,
  locked, atomic generation-exchange primitive needed by a generalized
  amendment engine.
  Evidence: `crates/openwepp-assurance/src/v2/normalization.rs` snapshots the v2
  tree, rebinds dependent identities, validates a candidate, and uses the
  rollback/committed-cleanup transaction contract.
- Observation: production CLI support is narrow, while integration fixtures
  contain repeated `refresh_local_hash`, `refresh_report_hash`, and
  `refresh_catalog_identity` helpers.
  Evidence: source, assembly, planner, normalization, and publication contract
  tests reproduce the same manual identity cascade in separate helper code.
- Observation: the single review-subject root binds staged output, catalog,
  principals, report content, and inputs, so attribution and scientific content
  cannot be invalidated proportionally.
  Evidence: `calculate_roots` in
  `crates/openwepp-assurance/src/v2/publication.rs` constructs one subject from
  normalized report, catalog, stable inputs, and all staged outputs.

## Decision Log

- Decision: make generated locks the only storage for derived file hashes.
  Rationale: derived values must be calculated once by production code rather
  than maintained in authored YAML, JSON packets, and catalogs.
  Date/Author: 2026-07-16 / Roger Lew and Codex.
- Decision: keep typed operations and reject generic adoption of dirty files.
  Rationale: automation should remove bookkeeping without becoming a mechanism
  for silently blessing arbitrary scientific changes.
  Date/Author: 2026-07-16 / Roger Lew and Codex.
- Decision: split bibliographic attribution from content-review governance,
  then derive an acyclic content-subject, ledger, preapproval realization,
  ordered approval-event, final realization, and release-transfer chain.
  Rationale: a name change must not invalidate unchanged scientific evidence,
  while the exact rendered and release identities must remain auditable.
  Date/Author: 2026-07-16 / Roger Lew and Codex.
- Decision: retain full implementation closure for this one-time architecture
  change, then enforce fast-path timing for routine amendments.
  Rationale: building the safety mechanism is high risk; using a proven typed
  mechanism for a bounded change is not.
  Date/Author: 2026-07-16 / Codex.
- Decision: remove arbitrary copyedit and scientific-patch commands from this
  package.
  Rationale: structural token checks cannot prove that human prose preserved
  scientific meaning, and those commands do not close the immediate latency
  defect.
  Date/Author: 2026-07-16 / independent review/Codex.
- Decision: make valid focused receipts explicit no-package/no-agent authority.
  Rationale: without a binding execution rule, ordinary CI policy would route
  an `IN_REVIEW` attribution change back to the 30–56 minute full-gate path.
  Date/Author: 2026-07-16 / independent review/Codex.

## Context And Orientation

`crates/openwepp-assurance/src/v2.rs` loads the v2 catalog, schemas, principal
registry, and report descriptors. `v2/planner.rs` builds a content-identity
dependency graph. `v2/assembly.rs` renders deterministic staging outputs.
`v2/publication.rs` calculates review and release roots and enforces publication
locks. `v2/normalization.rs` is currently the only transactional source writer.
`cli.rs` exposes these operations.

Current report descriptors contain SHA-256 values for manuscripts,
supplements, dependencies, procedures, inputs, and results. Agent-assistance
packets repeat some identities, and `catalog.yaml` hashes each report manifest
and the principal registry. Small edits therefore cause a manually maintained
cascade. The target contract is
`docs/specifications/assurance-amendment-and-identity-workflow.md`.

“Generated lock” means a canonical, tracked JSON file written only by the
assurance tool. “Layer identity” means a digest over one declared concern, such
as science or attribution, rather than one digest over every report byte.
“Fast lane” means a gate set mechanically authorized by an amendment receipt;
it is not a subjective assertion that a change is harmless.

## Authority And Constraints

The canonical target is
`docs/specifications/assurance-amendment-and-identity-workflow.md`. Existing
ADR-0038, the v2 source/build contract, lifecycle contract, scientific report
standard, and zero-public-report boundary remain binding. Where the current
source/build contract describes the old embedded-hash implementation, this
package must update it to the new generated-lock model without weakening review
or publication gates.

The package explicitly authorizes and requires subagent spawning/delegation to:

- two read-only implementation reviewers, one focused on identity/lifecycle
  integrity and one on proportional workflow and agent-to-mechanical transfer;
- one delegated heavy-gate runner for full workspace closure and fresh
  adjudicated CRAP; and
- two read-only terminal verifiers after accepted findings are fixed.

Expected outputs are compact package artifacts with evidence class, findings,
commands, timing, and disposition. Reviewers and verifiers may write only their
assigned artifact. The heavy runner may write package evidence and ignored
build outputs but must not edit production source.

## Included Scope

- Implement report inspection; attribution correction; global principal
  versioning; report-local role assignment; normalization; one-time identity
  migration; and typed recovery with `--check` and `--apply` semantics.
- Extract the reusable transaction engine from normalization without weakening
  confinement, permission, sync, rollback, or committed-cleanup behavior.
- Introduce canonical generated identity and review locks; remove derived file
  hashes and calculated roots from authored v2 files.
- Implement the specification's acyclic root graph, immutable review events,
  exhaustive field/file projection, and role-specific approval binding.
- Implement minimum typed lifecycle writers for review entry, findings,
  dispositions, approvals, withdrawal, supersession, and release transfer.
- Generate every duplicated attribution and lifecycle reader block from one
  structured source, including the current-governance part of the assistance
  packet.
- Migrate both current production-domain reports without changing their
  scientific values, claims, methods, results, figures, evidence, conclusions,
  or public status. Retain the snow/frost monolithic root only as historical
  migration evidence and reenter its still-pending review under the new root.
- Retain `normalize` as a one-cycle compatibility alias for `amend normalize`.
- Emit deterministic receipts containing complete consumers, affected paths,
  identity changes, invalidations, stable gate IDs, and structured argv.
- Add `tools/local_ci/run_assurance_amendment.py` as the one-shot focused receipt
  validator/runner and evidence recorder.
- Add a focused `assurance-amendment` nextest profile and measured fast-lane
  workflow gates with a pinned test manifest.
- Replace duplicated integration-test digest refresh helpers with production
  fixture mutation APIs.
- Record report scaffolding, object ingestion, reproduction receipts, catalogs,
  and other deterministic agent work in an ordered follow-up queue; do not
  implement those commands here.

## Excluded Scope And Protected Boundaries

- No scientific claim, method, dataset, result, numerical value, table, figure,
  conclusion, limitation, or model realization change.
- No human or agent may be inferred as an independent reviewer, approver,
  assurance steward, or release owner.
- No public report, approval, release transfer, snapshot, export, vendoring, or
  WEPPcloud action.
- No kernel, science-contract, model runtime, comparator, or usersum scientific
  narrative changes.
- No generic `sync`, `refresh`, `adopt`, or “trust current bytes” command.
- No arbitrary prose-copyedit or scientific-patch fast path.
- No permanent dual parser that accepts old and new identity formats.
- No agent invocation from an ordinary assurance command.

## Declared Write Set

- `crates/openwepp-assurance/src/{cli,error,lib,v2}.rs`
- `crates/openwepp-assurance/src/v2/{amendment,identity,normalization,planner,assembly,confined,lifecycle,publication}.rs`
- `tests/integration/assurance_v2_{amendment,source,planner,assembly,normalization,publication}_contract.rs`
- `tools/local_ci/run_assurance_amendment.py`
- `tools/local_ci/README.md`
- `Cargo.toml`
- `.config/nextest.toml`
- `assurance/v2/catalog.yaml`
- `assurance/v2/principals.yaml`
- `assurance/v2/identity.lock.json`
- `assurance/v2/schemas/{catalog,principals,report,identity-lock,review-lock,review-event,transaction-receipt}.schema.json`
- `assurance/v2/reports/linear-groundwater-reservoir-recurrence/**`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/**`
- `assurance/v2/README.md`
- `docs/specifications/assurance-amendment-and-identity-workflow.md`
- `docs/governance/scientific-assurance-v2-source-build-contract.md`
- `docs/governance/scientific-assurance-dossier-lifecycle.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260716-assure-maint01-mechanical-amendment-workflow-001/**`

Everything else is read-only. Temporary tests may mutate copies only under
their test-owned temporary directories.

## Plan Of Work

### Milestone 1 — Generated Identity Foundation

Extract canonical identity calculation into `v2/identity.rs`. Define typed lock
schemas and deterministic serializers. Migrate the catalog and report schemas
so logical paths remain authored while derived hashes live only in the
generated lock. Define the migration genesis, previous-generation chain,
generated-file inclusion/exclusion, and independent Git/base anchor. Rework
repository opening to load authored declarations plus the lock and verify every
admitted byte.

Acceptance is a one-time migration command that produces the same generation
twice, admits both reports, leaves scientific source bytes unchanged, verifies
the anchored transition chain, and makes a lock change without a valid receipt
fail CI. Internal consistency must not be mislabeled as proof of authorship.

### Milestone 2 — Layered Review Identities

Replace the monolithic subject calculation with the specified acyclic graph.
Move decisions into immutable lifecycle events and calculated roots into
generated review locks. Use exhaustive typed projection and normalized
root-bearing staged fields. Generate every attribution/lifecycle reader region
from structured metadata and prove attribution corrections preserve science and
communication roots.

Acceptance includes negative fixtures for circular/self inclusion, unclassified
fields, stale roots, wrong-layer approval, role conflicts, privilege change via
metadata, approval carry-forward, shared principals, predecessor-event
recombination, release transfer without its exact authority event, and
publication with any invalidated layer. Tests must prove that bibliographic-only
attribution preserves scientific/reproduction approval while normalization
invalidates every approval bound to communication.

### Milestone 3 — Typed Amendment Transaction

Extract the normalization transaction into a reusable module and implement the
specified amendment, lifecycle, migration, and recovery operations. Parse and
serialize typed structures; do not edit YAML/JSON by substring. Capture and
reverify external identity inputs. Candidate calculation, complete-consumer
planning, ephemeral build/check, repeated-build determinism, exchange,
rollback, cleanup, and receipt generation share one implementation.

Acceptance includes no-op idempotence, optional compare-and-swap rejection,
external drift, symlink/traversal, permission preservation, deterministic fault
injection, typed recovery, committed-cleanup, request validation, and proof that
ordinary requests contain no derived digest.

### Milestone 4 — Proportional Gates And Agent Reduction

Add the `assurance-amendment` profile, pinned selected-test manifest, stable gate
IDs, and receipt runner. Exercise attribution correction, shared-principal
consumer closure, role assignment, normalization/protected-region rejection,
lifecycle events, unknown-prose rejection, and non-escalation fixtures.

Benchmark the release binary for at least ten isolated trials on the current and
100-report/32-MiB scaled corpus. Produce `artifacts/mechanical-work-queue.md`
with command interface, inputs, outputs, human boundary, dependencies, and
priority for deterministic work outside this package.

### Milestone 5 — Migration And Closure

Migrate both current reports, update governance and operator documentation, and
prove protected scientific and public boundaries. Run focused iteration gates,
then delegate full closure and fresh CRAP. Complete dual review, disposition
every finding, fix accepted findings, and run dual terminal verification.

## Concrete Steps

From `/home/workdir/openWEPP`, the executing agent must begin with:

```text
tools/agents/find-agents --for <each declared write path>
cargo run --quiet -p openwepp-assurance -- validate --all
cargo run --quiet -p openwepp-assurance -- plan --all --format json
```

During implementation, run focused contracts after each milestone. The final
observable workflows must include:

```text
target/release/openwepp-assurance inspect \
  --report snow-and-frozen-soil-process-evaluation --format json
target/release/openwepp-assurance amend role \
  --report <temporary-fixture-report> --request <request.yaml> --check
target/release/openwepp-assurance amend role \
  --report <temporary-fixture-report> --request <request.yaml> --apply
.venv/bin/python tools/local_ci/run_assurance_amendment.py \
  --receipt <printed-receipt-path>
cargo nextest run --workspace --profile assurance-amendment
```

Production reports must not be used as mutation targets for demonstrations.
Integration tests copy them to confined temporary roots.

## Validation And Acceptance

Focused acceptance requires:

- amendment, source, planner, assembly, normalization, lifecycle, and
  publication integration contracts;
- named validation, planning, build, and check for both migrated reports;
- exact reproduction of all current retained report values;
- before/after semantic inventories proving claims, methods, results, values,
  tables, figures, evidence objects, limitations, and conclusions unchanged;
- zero public reports and unchanged protected v1/usersum hashes;
- timing evidence meeting the specification's fast-lane thresholds; and
- anchored generation-chain, acyclic-root, exhaustive-projection, immutable-
  event, shared-principal, state-matrix, and focused-receipt non-escalation
  contracts;
- Markdown lint, schema validation, American-English comparison, and
  `git diff --check` for documentation.

Implementation closure requires:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
bash tools/release/run_adjudicated_crap_gate.sh --base-ref 15763d7f6d5d4125333d9b7583424c714f5f5ea4
```

The heavy commands and fresh CRAP run belong to the delegated heavy-gate
runner. Record exact commands, run IDs, timing, raw/adjudicated/actionable CRAP,
and touched production files. Closure requires zero actionable CRAP.

Record every touched `.rs` line count. Files at or above 2,000 lines are WARN
and require decomposition rationale plus split intent. A touched 3,000-line
nonexempt file blocks closure. `v2/publication.rs` begins near that boundary;
new identity behavior must be extracted rather than growing it past 3,000.

Dual reviewers must examine identity-layer completeness, transaction safety,
approval invalidation, fast-lane non-escalation, timing, agent-work reduction,
manual-hash elimination, and Gate Evidence Non-Deferral. Every finding receives
an accepted, rejected, deferred, or follow-up disposition with rationale;
accepted findings are fixed and reverified. Deferred current-scope acceptance
is prohibited.

## Performance And Proportionality Gate

The package fails even if correctness tests pass when an accepted focused
fixture requires a work package, agent analysis, manual hash editing, full or
quick workspace tests, CRAP, scientific reproduction, duplicate staging, or
more than the specified time budget. The receipt and runner are the complete
local proof.

Measure cold compilation separately. Use the prebuilt release binary and at
least ten isolated trials. Record p50, p95, maximum, host, binary identity,
report count, corpus bytes, focused test manifest, and end-to-end apply-through-
evidence timing. Require p95 at most 60 seconds, maximum at most 120 seconds,
and hard failure above 300 seconds on both current and scaled fixtures.

## Idempotence And Recovery

Every `--check` is read-only. Equivalent requests against equivalent source
generations produce byte-identical candidate trees and receipts. Reapplying a
completed request returns a no-op without adding another receipt. A stale
expected generation fails without writes.

The existing confined transaction and fault-injection contracts remain
authoritative. Failure before validated exchange restores or leaves the old
  generation. Validated exchange followed by cleanup failure leaves the new
  generation active, returns the committed receipt, and blocks later mutation
  until typed recovery verifies and selects a generation. Tests cover external
  read-set drift and use deterministic fault points, never sleeps, polling,
  padding, or wall-clock races.

## Required Artifacts

- `artifacts/spec-review-identity.md`
- `artifacts/spec-review-operability.md`
- `artifacts/spec-review-mechanization.md`
- `artifacts/spec-review-disposition.md`
- `artifacts/required-reading-map.md`
- `artifacts/baseline-and-migration-inventory.md`
- `artifacts/identity-layer-migration.md`
- `artifacts/mechanical-work-queue.md`
- `artifacts/performance-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/heavy-gate-runner.md`
- `artifacts/review-a.md`
- `artifacts/review-b.md`
- `artifacts/finding-disposition.md`
- `artifacts/terminal-verification-a.md`
- `artifacts/terminal-verification-b.md`
- `artifacts/final-disposition.md`

## Outcomes And Retrospective

Pending execution. Closure must compare measured amendment latency and manual
touch count with the 30-minute spelling and 42-minute attribution incidents,
state which agent-maintained steps were eliminated, and name any deterministic
work still awaiting a mechanical owner.

Revision note (2026-07-16): initial user-authorized scaffold created to replace
manual assurance identity propagation and disproportionate small-edit gates.

Revision note (2026-07-16): independent specification review removed unsafe
copyedit/scientific commands, replaced circular roots with an acyclic event
graph, required lifecycle/recovery writers and anchored generation history,
made focused receipts no-package/no-agent authority, and narrowed current scope
to the latency defect plus prerequisites.
