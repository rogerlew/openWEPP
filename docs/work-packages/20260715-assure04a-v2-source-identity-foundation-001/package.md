# ASSURE-04A V2 Source And Identity Foundation

Package ID: `20260715-assure04a-v2-source-identity-foundation-001`

Status: `EXECUTED-COMPLETE`

Execution date: 2026-07-15

Frozen base: `81770ecb8f9e65702c7401852efa3d7f4682d15a`

This ExecPlan is maintained under `docs/codex_exec_plans.md`. The `Progress`,
`Surprises And Discoveries`, `Decision Log`, and `Outcomes And Retrospective`
sections are living execution records.

## Purpose

Establish the smallest executable v2 scientific-assurance source and identity
contract demonstrated by the accepted groundwater manuscript prototype. After
this package, a maintainer can ask the real `openwepp-assurance` CLI to validate
one named draft source or all v2 sources and receive a deterministic summary
whose identities trace through the manuscript, supplement, claims, methods,
results, figures, references, research objects, review state, and publication
state.

This package does not plan dependencies, render a report, approve science, or
publish anything. The public `usersum` catalog and dormant export remain the
exact ASSURE-03 zero-report outputs.

The user's instruction to scaffold and execute ASSURE-04 authorizes the next
roadmap item, ASSURE-04A. The accepted roadmap decomposes ASSURE-04 into four
independently closable packages and the ASSURE-03 handoff prohibits A from
absorbing B through D. This package records that bounded interpretation rather
than silently widening into planner, renderer, or publication work.

## Context And Orientation

`assurance/catalog.yaml` is the public zero-report transition source consumed
by the existing `Assurance` builder. The new `assurance/v2/` tree is internal
canonical draft source. A source record may be visible to repository readers,
but no draft route, generated page, export entry, release snapshot, or
WEPPcloud vendor record may treat it as published.

The accepted positive fixture is the ASSURE-02 daily linear groundwater-
reservoir prototype. It is architecture evidence, not an approved report. The
fixture demonstrates the source contract and retains that limitation. A later
ASSURE-05 package must revise and scientifically review a production successor;
it may not promote the prototype unchanged.

An identity is a stable logical ID plus a content or immutable external
identity. Paths are confined locators and never sufficient identities alone.
An unused identity is a declared object that no report, claim, method, result,
figure, review, publication, or research-object relation consumes. ASSURE-04A
rejects missing, duplicate, unknown, and unused identities but does not yet
compute rebuild impact; that is ASSURE-04B.

## Authority

Binding authority is ADR-0038, the accepted v2 architecture, lifecycle
contract, source/build contract, scientific report standard, implementation
roadmap, and the completed ASSURE-03 handoff. The report prototype and claim
matrix are the only positive fixture design basis. Package evidence cannot
amend those authorities.

No kernel process, numerical method, science-contract authority, release
qualification, or empirical conclusion is changed by this package.

## Scope

Included:

- add a versioned `assurance/v2/` source catalog and the smallest report-source
  layout demonstrated by the groundwater prototype;
- add executable typed contracts for catalog, report, manuscript, supplement,
  dependency, unit, claim, method, result object, figure, reference,
  research-object, review, and publication records;
- retain authored Markdown as canonical scientific prose and structured
  records only as identities and mechanical support;
- add tracked JSON Schema companions and bind them to executable source
  identities;
- add a nonpublic `DRAFT`, fixture-only groundwater source with content-
  identified manuscript, supplement, and retained result objects;
- validate logical IDs, version fields, units, content hashes, path
  confinement, access/licensing metadata, restrictions, reference closure,
  lifecycle consistency, and unused declarations;
- extend the real CLI with one-report and all-report v2 validation while
  retaining the ASSURE-03 zero-report build/check/snapshot behavior;
- add positive and negative integration tests, deterministic validation
  summaries, field-consumption evidence, line-count governance, fresh CRAP,
  dual review, finding disposition, and dual terminal verification; and
- update prospective queues and the work-package catalog truthfully.

Excluded:

- ASSURE-04B dependency planning, transitive impact, freshness, or incremental
  rebuild decisions;
- ASSURE-04C staging assembly, value substitution, Markdown rendering, table or
  figure generation, or accessibility rendering;
- ASSURE-04D approval locks, promotion, public catalog integration, release
  snapshots, supersession, or withdrawal;
- scientific revision or approval of the groundwater prototype;
- any public `usersum` report, export document, release candidate, or
  WEPPcloud vendoring;
- external network, agent, shell, clock, hostname, or absolute-workspace-path
  dependency in ordinary v2 validation; and
- process-physics, science-contract, comparator, or integrated-model changes.

## Declared Write Set

- `assurance/README.md`
- `assurance/v2/**`
- `crates/openwepp-assurance/Cargo.toml`
- `crates/openwepp-assurance/src/{cli.rs,error.rs,lib.rs,v2.rs}`
- `Cargo.toml` and `Cargo.lock` only when test registration or an explicitly
  justified direct dependency requires them
- `tests/integration/assurance_v2_source_contract.rs`
- `tests/integration/assurance_dossier_build_contract.rs` only for required CLI
  compatibility reconciliation
- `docs/ROADMAP.md`
- `docs/planning/scientific-assurance-v2-implementation-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/**`

The tracked `assurance/catalog.yaml`, `assurance/templates/catalog.md`,
`assurance/generated/wepppy-usersum.yaml`, `usersum/assurance/README.md`, and
all other `usersum/**` paths are protected byte-identity surfaces, not write-set
members. Any edit outside the declared set requires a package amendment before
the edit.

## Interfaces And Dependencies

`crates/openwepp-assurance/src/v2.rs` owns the executable v2 source types and a
`V2Repository` loader. It must expose a report-validation summary suitable for
the CLI and integration tests without exposing mutable publication authority.
The loader reads only repository-local regular files, resolves all paths below
the selected root, rejects symlinks and special entries for claim-bearing
sources, verifies SHA-256 identities, and uses typed `AssuranceError` failures.

The existing `Assurance` type retains ownership of the public zero-report
transition builder. V2 source validation composes beside it rather than
weakening its exact catalog or retired-route guards. `validate --all` validates
both the zero-report public state and all internal v2 sources. `validate
--report <id>` validates the named internal source plus the protected public
boundary. Report-specific `plan`, `build`, and `check` remain rejected with a
message that names ASSURE-04B or ASSURE-04C rather than performing partial work.

Prefer existing `serde`, `serde_json`, `serde_yaml`, and SHA-256 support. A new
direct dependency is allowed only when it materially strengthens executable
version/schema validation and is recorded in the Decision Log.

## Deliverables

1. Package authority, required-reading map, protected-surface freeze, and
   source-contract design record.
2. Versioned v2 catalog/schema/source fixture with content-identified human
   prose and result objects.
3. Typed `V2Repository` validation and deterministic one/all CLI summaries.
4. Positive consumer proof and negative identity, units, version, path,
   restriction, lifecycle, and unused-field tests.
5. Focused, full workspace, dependency-policy, fresh CRAP, documentation,
   line-count, review, verification, final-disposition, and handoff artifacts.

## Phase Plan

### Phase 1 — Intake, Freeze, And Source Contract

Freeze the base commit and exact protected zero-report/public hashes. Record the
accepted prototype fields and map them to the smallest serializations. Scaffold
the package, prompt, artifact set, queue state, and required-reading budget.

### Phase 2 — Contract-Derived Tests

Add a positive real-CLI fixture test and negative vectors before production
implementation. The tests must cover unknown and missing fields, duplicate and
unresolved IDs, unused declarations, unit lookup, content mismatch, schema and
contract versions, unsafe paths, symlinks/special entries, restricted-object
leakage, draft publication contradictions, report selection, deterministic
summary identity, and exact protected public bytes.

### Phase 3 — Typed V2 Admission

Implement the v2 loader and source records, make the fixture pass, and extend
only the validation CLI. Keep the existing zero-report plan/build/check and
snapshot behavior unchanged. Record the real source-to-CLI consumer path and a
field-consumption matrix.

### Phase 4 — Focused And Workspace Closure

Run formatting, focused Nextest, workspace Clippy, full-profile Nextest,
dependency policy, fresh adjudicated CRAP at threshold 30, source-level line
counts, protected-output identity checks, Markdown/link/spelling checks, and
`git diff --check`. Preserve exact source identities across heavy gates.

### Phase 5 — Review, Remediation, And Verification

Dispatch two independent read-only coding-agent reviewers. Reviewer A covers
scientific communication architecture, prototype fidelity, lifecycle/public
boundaries, schema/source usability, and documentation. Reviewer B covers typed
admission, identity/reference closure, confinement, restricted evidence,
versioning, CLI consumers, tests, security, and gate legitimacy. Disposition
every finding, remediate accepted findings, rerun affected gates, then obtain
two independent terminal verifications of the amended tree.

## Gates

- The protected public catalog, template, export, and `usersum` catalog hashes
  equal the frozen ASSURE-03 base before and after implementation.
- The v2 catalog admits exactly one nonpublic `DRAFT`, fixture-only groundwater
  source and no public/export/release route.
- Manuscript and supplement bytes remain the canonical scientific prose;
  structured records do not generate or rewrite scientific interpretation.
- Typed authorship and agent-assistance records disclose the fixture author,
  accountable human roles, procedure limits, input/output provenance, and
  review-entry blockers without claiming external scientific peer review.
- The real CLI validates the named report and all sources, reports deterministic
  counts and source-root identity, and consumes every admitted record type.
- Unknown/missing fields, duplicate/unknown/unused IDs, unknown units,
  mismatched hashes, unsupported schema/contract/report versions, unsafe or
  absolute paths, symlinks/special entries, and restricted-evidence leakage all
  fail closed with typed errors.
- Draft review/publication records cannot claim approval, public paths,
  snapshots, release transfer, or vendoring.
- `plan`, `build`, and `check` cannot perform report-specific v2 work before
  their owning packages; the existing all-report zero-public behavior remains
  covered.
- JSON Schema companions and executable typed contracts agree on version and
  required-field posture.
- `cargo fmt --check`, workspace Clippy with warnings denied, full-profile
  Nextest, and `cargo deny check` pass.
- Fresh adjudicated CRAP passes at threshold 30 with no actionable row, using
  frozen base `81770ecb8f9e65702c7401852efa3d7f4682d15a`.
- Touched `.rs` files satisfy line-count governance; every 2,000+ line warning
  and 3,000+ line exception is explicitly dispositioned.
- Markdown lint/validation, local links, spelling preview, protected path
  hashes, and `git diff --check` pass.
- Dual reviews are dispositioned, all accepted findings are remediated, and
  dual terminal verification passes on the final tree.

An unmet required gate results in `EXECUTED-HOLD` with a named blocker. No
ASSURE-04A gate may be deferred into B through D while A is called complete.

## Security And Publication Boundary

The new loader handles repository paths and records evidence restrictions.
Every declared local source must be a confined regular non-symlink file. It
must reject traversal, absolute paths, special entries, content drift, duplicate
identity ambiguity, and an external/restricted record that exposes a local
protected locator. Error messages may name logical IDs and safe relative paths
but must not leak protected filesystem locations or content.

No ordinary validation operation may invoke the network, shell, agent, clock,
randomness, hostname, or environment-dependent absolute path. No source or test
may create a public report route. `DRAFT` is source state only and never an
export/public label.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes spawning/delegating
to one heavy-gate runner and two independent reviewer/verifier agents for the
scopes in Phases 4 and 5. Expected outputs are compact gate metrics and package
artifact text. Heavy-runner write access is limited to package gate-evidence
artifacts; reviewers and verifiers are read-only. The parent owns production,
test, source-contract, governance, and finding-disposition edits.

Subagent requirement: REQUIRED. Delegate full-workspace Nextest, dependency
policy, fresh CRAP, and any release aggregate to the heavy-gate runner; do not
run those heavy gates on the parent model while that runner is available.
Coding-agent review is internal review, not external scientific peer review.

## Progress

- [x] (2026-07-15) Interpreted the user instruction as ASSURE-04A according to
  the accepted A–D decomposition and ASSURE-03 handoff.
- [x] (2026-07-15) Froze base commit and loaded root, package, crate, test,
  prompt, architecture, lifecycle, source/build, report, prototype, and roadmap
  authority.
- [x] (2026-07-15) Froze protected hashes and finalized the executable
  source-contract and field-consumption maps.
- [x] (2026-07-15) Added the strict positive fixture and contract-derived
  positive/negative integration tests before production admission was closed.
- [x] (2026-07-15) Implemented typed selected/all admission and real-CLI
  validation consumers while preserving zero-public plan/build/check behavior.
- [x] (2026-07-15) Passed focused and terminal workspace gates; full-profile
  Nextest passed 1,985/1,985 selected tests and fresh adjudicated CRAP closed
  with zero actionable rows.
- [x] (2026-07-15) Completed dual review, accepted and remediated every
  finding, and obtained two clean independent re-reviews.
- [x] (2026-07-15) Accepted Verification B's required-nullable presence
  finding and added presence-aware admission plus negative vectors across all
  affected record families.
- [x] (2026-07-15) Reran the complete heavy-gate sequence on the amended
  source: 1,986/1,986 full-profile tests and fresh CRAP with zero actionable
  rows passed on a stable source freeze.
- [x] (2026-07-15) Renewed both independent terminal verifications and recorded
  final disposition and the bounded ASSURE-04B handoff.

## Surprises And Discoveries

- Observation: the user-visible umbrella name `ASSURE-04` is already decomposed
  into four packages with explicit non-absorption boundaries.
  Evidence: `docs/ROADMAP.md`, the implementation roadmap, and the ASSURE-03
  handoff all name ASSURE-04A as the only next item.
- Observation: loading every report before applying `--report` would make one
  unrelated malformed source block named validation as the catalog grows.
  Evidence: a two-report negative fixture now proves the selected report passes
  while `--all` rejects the unselected malformed report.
- Observation: the first fresh adjudicated CRAP run found four actionable
  validators even though focused functional tests and Clippy passed.
  Evidence: the retained HOLD report names CRAP values from 32.94 to 81.42;
  semantic helper decomposition reduced the terminal fresh result to zero
  actionable rows without changing the v2 source or protected public bytes.
- Observation: scientific-source identity needs both bibliographic fidelity
  and accountable authorship/procedure disclosure, not only content hashes.
  Evidence: dual review caught the collapsed claim lineage, incorrect DOI, and
  absent agent-assistance/accountability records before terminal closure.
- Observation: a required-field schema can still diverge from a typed
  deserializer when a required field is nullable.
  Evidence: Verification B removed a nullable dependency field, refreshed the
  manifest hash, and proved plain `Option<T>` admitted the omission. The
  amended type distinguishes missing, explicit null, and present values.

## Decision Log

- Decision: execute only ASSURE-04A under this package.
  Rationale: A has independently measurable source/admission gates; B through D
  own planning, assembly, and publication and are explicitly blocked in order.
  Date/author: 2026-07-15, Codex interpretation of user authorization and
  accepted repository authority.
- Decision: preserve the ASSURE-03 public builder as a separate type and add v2
  source validation beside it.
  Rationale: this prevents draft source admission from weakening or implicitly
  repurposing the exact zero-report publication boundary.
  Date/author: 2026-07-15, Codex.
- Decision: use one strict YAML report manifest with separate Markdown prose
  and JSON result objects, rather than one file per small record.
  Rationale: this is the smallest serialization demonstrated by the accepted
  prototype while preserving canonical prose and content-identified values.
  Date/author: 2026-07-15, Codex.
- Decision: catalog/schema admission is eager; report-source traversal is
  selection-aware.
  Rationale: schema drift must always fail, while a named report operation must
  not inherit an unrelated report's content failure. The all-report route still
  traverses every source.
  Date/author: 2026-07-15, Codex.
- Decision: add `jsonschema` as a test-only direct dependency and exercise all
  three companions with the Draft 2020-12 validator.
  Rationale: companion schemas are an executable interoperability contract,
  not documentation inferred only from the Rust deserializers. Adversarial
  tests additionally bind nested field sets and version constants to the typed
  implementation.
  Date/author: 2026-07-15, Codex after independent review.
- Decision: decompose the four CRAP-heavy validators into small semantic
  checks instead of adjudicating or exempting them.
  Rationale: all four rows were new touched-code debt above the established
  threshold. Decomposition preserved fail-closed behavior and made branch
  obligations independently testable.
  Date/author: 2026-07-15, Codex after the first terminal heavy-gate HOLD.

## Outcomes And Retrospective

ASSURE-04A is complete. The repository has a strict internal v2 source and
identity contract, one disclosed nonpublic groundwater architecture fixture,
and real named/all validation consumers. The public `usersum` and export
surfaces remain byte-identical zero-report outputs; no report was scientifically
approved, rendered, promoted, released, or vendored.

The pre-remediation tree passed focused checks, workspace Clippy, full-profile Nextest
(1,985/1,985 selected tests), dependency policy, fresh adjudicated CRAP with
zero actionable rows, protected-surface checks, dual review with complete
finding remediation, and Verification A. Verification B then found a blocking
required-nullable presence defect. The defect is remediated and 25/25 focused
tests pass. The amended complete sequence passed 1,986/1,986 selected tests,
dependency policy, and fresh CRAP with zero actionable rows on stable source
bytes. Renewed independent Verification A and B both passed. The initial CRAP
HOLD, intermediate PASS, and later verification HOLD remain preserved as audit
evidence.

ASSURE-04B is next eligible but remains unscaffolded and requires explicit user
authorization. It owns dependency planning only; assembly and publication
remain ASSURE-04C/D work.
