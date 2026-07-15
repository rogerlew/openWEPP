# ASSURE-03 Retire V1 And Establish Zero Reports

Package ID: `20260714-assure03-v1-retirement-zero-report-001`

Status: `EXECUTED-COMPLETE`

Execution date: 2026-07-14

Frozen base: `3352388465f8b288aed4636e8f9752ca6c1cceb9`

This ExecPlan is maintained under `docs/codex_exec_plans.md`. The `Progress`,
`Surprises And Discoveries`, `Decision Log`, and `Outcomes And Retrospective`
sections are living execution records.

## Purpose

Remove the failed v1 status-first assurance publication from every active,
public, export, and release path without diminishing or discarding the
snow/frost science behind it. After this package, readers encounter a neutral
catalog stating that no report has completed the manuscript-first review and
publication process; ordinary CI cannot assemble or label a release candidate;
and an explicitly invoked release route can snapshot only the zero-report
state.

The user instruction to scaffold and execute ASSURE-03 is the explicit
ASSURE-02 acceptance required by ADR-0038. It accepts the manuscript-first
direction, activates the v2 report authority, and finalizes v1 retirement. It
does not approve a scientific report, reassess snow/frost evidence, authorize
WEPPcloud vendoring, or declare openWEPP beta-ready.

## Authority

The binding migration contract is
`docs/planning/scientific-assurance-v2-migration-plan.md`; sequencing is in
`docs/planning/scientific-assurance-v2-implementation-roadmap.md`. ADR-0038 and
the v2 architecture, lifecycle, source/build contract, and report standard
govern the target state. Package evidence may prove execution but cannot alter
those authorities.

## Scope

Included:

- record the atomic ASSURE-02 acceptance transition;
- freeze exact pre-removal v1 paths, byte sizes, SHA-256 identities, mappings,
  review provenance, build command, and Git recovery procedure;
- split ordinary validation from explicit release assembly and close
  `ASSURE03-REL-001` with executable negative tests;
- reduce the retained v1 compiler to a fail-closed zero-report transition tool;
- remove active v1 dossier/method sources, schemas/templates no longer needed,
  public pages, and export routes;
- generate and check a deterministic neutral catalog and dormant empty export;
- repair `usersum` and snow/frost narrative links without weakening the model's
  scientific explanation or evidence history;
- prove a zero-report release snapshot, absence of candidate routes, exact
  historical recovery, and real release/workflow consumer behavior; and
- run implementation closure, CRAP, dual review, finding disposition, and dual
  terminal verification.

Excluded:

- v2 schemas, dependency planning, manuscript rendering, report publication,
  or scientific interpretation;
- snow/frost scientific reassessment, metric recomputation, or claim downgrade;
- changes to process physics or `SC-*` authority;
- WEPPcloud vendoring or edits outside this repository; and
- restoration of v1 as current public content. Git recovery is audit-only.

## Declared Write Set

- `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md`
- `docs/decisions/README.md`
- `docs/governance/{README.md,openwepp-release-procedure-draft.md,openwepp-verification-validation-strategy.md,scientific-assurance-dossier-lifecycle.md,scientific-assurance-v2-architecture.md,scientific-assurance-v2-source-build-contract.md`
- `docs/planning/{scientific-assurance-v2-migration-plan.md,scientific-assurance-v2-implementation-roadmap.md}`
- `docs/standards/{README.md,scientific-assurance-dossier.md,scientific-model-evaluation-report.md}`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/{package.md,artifacts/final-disposition.md,artifacts/worker-handoff.md}`
- `docs/work-packages/20260714-assure03-v1-retirement-zero-report-001/**`
- `assurance/**`
- `usersum/{README.md,snow-frost-modeling-and-validation.md,assurance/**}`
- `crates/openwepp-assurance/**`
- `Cargo.toml` and `Cargo.lock` only if retained compiler dependencies require it
- `tests/integration/assurance_dossier_build_contract.rs`
- `tools/release/{README.md,check_assurance_dossier_exports.sh,check_assurance_release_transition.sh,run_release_candidate_gates.sh}`
- `.github/workflows/release-gates.yml`

Any edit outside this set requires a package amendment before the edit.

## Deliverables

1. Exact v1 content/recovery manifest and search-hit classification.
2. Accepted v2 authority and closed ASSURE-02 acceptance record.
3. Minimal zero-report compiler, neutral source/template, generated catalog,
   and dormant empty export.
4. Validation/release mode split, transition preflight, workflow split, and
   negative route tests.
5. Repaired public documentation with no premature v1 routes and no diminution
   of snow/frost science.
6. Gate, CRAP, line-count, consumer-path, review, verification, disposition,
   and handoff artifacts.

## Phase Plan

### Phase 1 — Acceptance, Intake, And Preservation Freeze

Record the user acceptance, applicable instructions, frozen base, expanded v1
inventory, exact hashes, mappings, search classification, and recovery commands
before deleting active content. Directly verify every manifest row from Git.

### Phase 2 — Close `ASSURE03-REL-001`

Introduce explicit validation and release modes. Ordinary PR, push, schedule,
and nonrelease dispatch run validation only and upload validation evidence, not
release-candidate artifacts. The explicit release route executes a transition
preflight before assembly. Negative tests prove validation cannot snapshot and
release rejects a transition marker, a nonempty catalog, and retired routes.

### Phase 3 — Zero-Report Migration

Remove the active v1 source/public surfaces, retain only the smallest typed
offline tool required to validate, build, check, and snapshot the neutral state,
and repair public navigation and snow/frost prose. The tool must reject every
nonempty legacy dossier catalog rather than retain a hidden publication path.

### Phase 4 — Consumer And Closure Evidence

Exercise the real CLI, export check, release preflight, explicit zero-report
release snapshot path, workflow contract, exact-history recovery, deterministic
outputs, links, searches, focused tests, full workspace gates, and adjudicated
CRAP gate. Record source line-count governance.

### Phase 5 — Review, Remediation, And Verification

Dispatch two independent read-only coding-agent reviews. Reviewer A covers
scientific communication, snow/frost preservation, public wording, history,
and reader routes. Reviewer B covers fail-closed implementation, release and CI
consumers, security, tests, deterministic build/snapshot behavior, recovery,
and gate legitimacy. Disposition every finding, remediate accepted findings,
then run two independent terminal verifications on the amended tree.

## Gates

- Pre-removal manifest reconstructs every removed/changed v1 byte from the
  frozen base and records source-to-generated mappings and review provenance.
- ADR-0038, the v2 contracts/standard, and the ASSURE-02 record agree that the
  direction is accepted and v1 retirement is final.
- Ordinary CI invokes validation mode and never creates, names, or uploads a
  release-candidate snapshot or artifact.
- Explicit release mode fails before assembly on a transition marker, nonempty
  catalog, or retired v1 route; final zero-report release preflight passes.
- The real CLI validates, plans, builds, and checks an empty catalog; it rejects
  a nonempty dossier catalog and produces deterministic bytes.
- Tracked `usersum/assurance/` contains only the neutral catalog; the dormant
  export enumerates zero documents/reports; no v1 route is current.
- Snow/frost narrative remains substantive and no edited language implies that
  the model lacks scientific or empirical evidence.
- Zero-report snapshot manifest enumerates zero reports and contains only the
  approved neutral transition outputs.
- Link and classified search checks pass; historical evidence remains clearly
  historical and current authority points to v2.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full`, and `cargo deny check` pass.
- `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 3352388465f8b288aed4636e8f9752ca6c1cceb9`
  passes with fresh terminal evidence and no actionable function above 30.
- Markdown lint/validate, local links, spelling preview, and
  `git diff --check` pass.
- Dual reviews are dispositioned; all accepted findings are fixed; dual
  terminal verification passes on the final tree.

An unmet required gate results in `EXECUTED-HOLD` with a named blocker. No gate
may be deferred to ASSURE-04 while this package is called complete.

## Security And Publication Boundary

The migration deletes a public publication route and changes release controls.
All filesystem outputs must remain repository/output-root confined, snapshot
IDs must be safe path components, existing immutable snapshots must not mutate,
and explicit release assembly must fail before writing release artifacts when
assurance preflight fails. No network, shell, agent, clock-derived content, or
absolute workspace path may enter an ordinary assurance build. No protected or
copyrighted evidence is copied into package artifacts.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes spawning/delegation
to two independent read-only coding-agent reviewers and two independent
read-only coding-agent verifiers for the scopes and artifacts described in
Phase 5. It also explicitly authorizes and requires delegation of heavy full
workspace, release, and CRAP closure runs to the heavy-gate runner; expected
outputs are compact metrics and log/artifact paths; write access is limited to
package gate-evidence artifacts when assigned. Coding-agent review is internal
review, not external scientific peer review. The parent owns production edits
and finding disposition.

## Progress

- [x] (2026-07-14) User explicitly accepted the ASSURE-02 direction by
  instructing execution of ASSURE-03.
- [x] (2026-07-14) Froze base commit and loaded governing instructions,
  migration authority, architecture, implementation, tests, and release paths.
- [x] (2026-07-15) Froze and directly verified the exact 51-row v1 recovery
  manifest; corrected eight removal dispositions found during review.
- [x] (2026-07-15) Closed the release/validation conflict and added negative
  tests for marker, catalog, retired-route, workflow, and symlink evasions.
- [x] (2026-07-15) Completed zero-report migration and documentation repair.
- [x] (2026-07-15) Passed focused remediation gates and both renewed r4 full
  transition-route aggregates, including fresh adjudicated CRAP at threshold
  30 with zero actionable rows.
- [x] (2026-07-15) Completed dual review, dispositioned every finding, and
  passed dual terminal verification after all accepted remediation.
- [x] (2026-07-15) Recorded the complete disposition, archived execution
  prompt, and bounded ASSURE-04A handoff.

## Surprises And Discoveries

- Observation: the current ordinary PR/push job invokes the aggregate release
  script and uploads `openwepp-release-candidate-*` even though the v1 candidate
  is expressly prohibited.
  Evidence: `.github/workflows/release-gates.yml` and
  `tools/release/run_release_candidate_gates.sh` at the frozen base.
- Observation: initial independent review found three symlink/failure-artifact
  evasions not covered by the pre-review test set, and found that the initial
  transition-assembly run skipped stability.
  Evidence: `artifacts/review-agent-b.md`; the accepted remediation rejects
  those evasions, prevents candidate-named failure uploads, binds candidate
  publication to successful stability, and labels skip-stability evidence as
  transition-route verification rather than release qualification.
- Observation: terminal verification found that content-presence checks alone
  admitted a duplicate YAML key and special filesystem entries at retired
  roots.
  Evidence: `artifacts/review-disposition.md`; the accepted remediation binds
  preflight to the exact typed zero-report catalog and rejects every retired
  root except an actual, non-symlink, completely empty directory.

## Decision Log

- Decision: treat the user's imperative to execute ASSURE-03 as the explicit
  ASSURE-02 acceptance requested by its handoff.
  Rationale: ASSURE-03 was deliberately blocked on that acceptance; directing
  its execution unambiguously authorizes the named next transition.
  Date/author: 2026-07-14, user/Codex record.
- Decision: retain a minimal zero-report transition tool instead of preserving
  the status-first compiler as an active hidden publication path.
  Rationale: candidate rejection must be architectural and testable, while
  ASSURE-04 owns all v2 report machinery.
  Date/author: 2026-07-14, Codex.
- Decision: do not treat the package's `--skip-stability` assembly exercise as
  a conformant release candidate or release qualification.
  Rationale: ASSURE-03 must prove the zero-report transition path, while the
  release runbook requires stability and a retained complete evidence bundle.
  The workflow now gates candidate-named upload on successful separately bound
  stability; package evidence retains only transition-route verification.
  Date/author: 2026-07-15, Codex disposition of Review B `B-004`.
- Decision: make the zero-report transition state exact and intentionally
  narrow rather than retain general v1 parsing or publication machinery.
  Rationale: the accepted v2 implementation has not started, so exact catalog
  bytes, two generated outputs, and fail-closed rejection provide a smaller
  and more auditable bridge than a dormant status-first compiler.
  Date/author: 2026-07-15, Codex disposition of `VB-001`.

## Outcomes And Retrospective

ASSURE-03 retired the failed v1 status-first publication architecture from
active source, public navigation, exports, and ordinary/release automation
while preserving exact historical recovery and the substantive snow/frost
science narrative. The resulting transition tool admits only the typed
zero-report state, deterministically builds and checks two outputs, and creates
an immutable confined snapshot only on the explicit release route.

Both renewed r4 aggregates passed 1,974 tests, dependency policy, authority
checks, and fresh adjudicated CRAP at threshold 30 with zero actionable rows.
All review and verification findings were accepted and closed. Stability was
not run, so this result is not release qualification and authorizes neither a
candidate nor WEPPcloud vendoring.

The principal lesson is that retirement gates must validate the whole admitted
filesystem and source state, not merely search for expected text or ordinary
files. ASSURE-04A may now be scaffolded only by a new operator instruction; it
must retain the zero-report public state while introducing the smallest v2
source and identity contracts.
