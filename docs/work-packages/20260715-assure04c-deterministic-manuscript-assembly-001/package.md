# ASSURE-04C — Deterministic Manuscript Assembly

Status: `EXECUTED-COMPLETE`

Execution date: 2026-07-15 to 2026-07-16 UTC

Package ID: `20260715-assure04c-deterministic-manuscript-assembly-001`

Frozen base: `e704f0202278ebb86c6a8c667caf73d599be04ab`

## Objective

Implement the smallest deterministic, offline, staging-only assembler proven by
the accepted groundwater manuscript. The real `openwepp-assurance build` and
`check` commands must assemble a named report or all admitted reports through
the ASSURE-04B typed dependency plan, preserve human-authored interpretation,
and mechanically resolve retained values, tables, result-bearing figures,
citations, accessible alternatives, and portable Markdown links.

The package does not approve or publish scientific work. Its positive consumer
is a disposable `usersum`-shaped staging tree containing the internal
groundwater fixture. The tracked public `usersum`, zero-report catalog, export,
snapshot, release, and vendor surfaces remain byte-identical.

## Rationale

ADR-0038 makes the manuscript the scientific argument and restricts tooling to
mechanical assembly. ASSURE-04A admitted typed source identities; ASSURE-04B
implemented deterministic dependency planning. ASSURE-04C must now prove that
the plan feeds an actual reader-compatible artifact without reviving the v1
status-first architecture or allowing generated interpretation.

## Authority

Binding authority is ADR-0038; the accepted v2 architecture; the lifecycle,
source/build, report, and `usersum` authoring contracts; the prospective
ASSURE-04 roadmap; and the completed ASSURE-04B handoff. Package artifacts are
implementation evidence, not scientific or publication authority.

No kernel process, numerical method, science contract, scientific conclusion,
review decision, or application-fitness judgment is changed.

## Scope

Included:

- define a small versioned assembly vocabulary demonstrated by the groundwater
  fixture, with stable logical identities and strict schema/executable parity;
- migrate claim-bearing manuscript/supplement repetitions to mechanical
  references without rewriting the surrounding authored scientific prose;
- resolve retained result values under declared unit and precision contracts;
- assemble manifest-declared tables and deterministic result-bearing figures
  directly from retained result objects;
- emit visible figure alternatives and accessible SVG title/description text;
- resolve references and report/supplement/figure/research-object links to
  portable `usersum`-compatible relative routes;
- consume `V2Repository::{plan_report, plan_all}` and reject every report whose
  target or required dependency is not current;
- implement named/all staging build and byte-for-byte staging check through the
  real CLI without changing the protected zero-report build/check contract;
- preserve named-build isolation, all/named byte equivalence, stable ordering,
  exact output-set checking, and no unrelated report mutation;
- stage safely redistributable fixture research objects only inside the
  selected staging root so rendered links are inspectable;
- add focused unit/integration tests, retained staging evidence, protected-
  surface proof, line-count governance, fresh CRAP evidence, dual review and
  disposition, independent heavy closure, and dual terminal verification; and
- close queue/catalog state truthfully and hand off only ASSURE-04D.

Excluded:

- generated scientific prose, interpretation, method choice, result
  adjudication, limitation selection, or fitness verdicts;
- review locks, named approval, public promotion, public catalog/search
  integration, snapshots, release transfer, withdrawal, supersession, export,
  or vendoring (ASSURE-04D or later);
- writes to tracked `usersum`, protected v1-retirement source/output, release,
  snapshot, or vendor paths;
- network, shell, agent, random, hostname, wall-clock-content, file-time, or
  environment-interpolation dependencies during ordinary assembly;
- a general-purpose templating language, arbitrary Markdown/HTML execution,
  arbitrary transforms, or runtime plugins;
- empirical/scientific reevaluation of the groundwater fixture; and
- kernel, science-contract, comparator, or integrated-model changes.

## Declared Write Set

- `assurance/README.md`
- `assurance/v2/README.md`
- `assurance/v2/catalog.yaml`
- `assurance/v2/schemas/{catalog.schema.json,report.schema.json,result.schema.json}`
- `assurance/v2/reports/linear-groundwater-reservoir-recurrence/**`
- `crates/openwepp-assurance/src/{cli.rs,error.rs,lib.rs,v2.rs}`
- `crates/openwepp-assurance/src/v2/{assembly.rs,confined.rs,planner.rs}`
- `Cargo.toml` for integration-test registration
- `tests/integration/assurance_v2_{source,planner,assembly}_contract.rs`
- `docs/ROADMAP.md`
- `docs/planning/scientific-assurance-v2-implementation-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260715-assure04c-deterministic-manuscript-assembly-001/**`

`Cargo.lock` and `crates/openwepp-assurance/Cargo.toml` are excluded unless the
package is amended before adding an indispensable direct dependency. The
tracked `assurance/catalog.yaml`, `assurance/templates/catalog.md`,
`assurance/generated/wepppy-usersum.yaml`, every `usersum/**` file, public
export/snapshot/release/vendor surfaces, and all kernel/science files are
protected read-only surfaces. Any edit outside the declared set requires an
explicit package amendment before the edit.

## Amendments

- 2026-07-16: Added `crates/openwepp-assurance/src/error.rs` to the declared
  write set so the accepted typed transaction-recovery finding could be closed.
  The enum edit occurred during review remediation before this write-set
  omission was detected; both independent reviewers caught the governance
  exception. The package owner accepts and records that exception here before
  any further implementation edit. No other write-set expansion is authorized.

## Assembly Design Contract

The assembler consumes one immutable typed `V2Plan`. It must reject a report
unless every selected node and the report target are `current`; it cannot
silently rebuild from stale, blocked, or merely selected authority. Named and
all builds call the same per-report assembly function in stable report-ID order.

The canonical manuscript and supplement remain UTF-8 Markdown. ASSURE-04C may
add a deliberately small directive vocabulary for stable value, table, figure,
reference, research-object, and report-relative link identities. Directives are
data references only: no conditionals, loops, includes, expressions, shell,
environment access, or prose generation. Before implementation, the exact
grammar, allowed transforms, unit/precision rules, output layout, and portable-
link rules must be frozen in `artifacts/assembly-contract.md` and covered by
contract-derived tests.

Every numeric rendering must bind a result ID, value ID, declared unit, and
explicit display precision. Unit mismatch, unknown/duplicate directive,
unsupported precision/transform, or an unreferenced retained result fails. A
changed unit/precision or source identity must either make the plan noncurrent
or make `check` report deterministic drift; no stale displayed value may pass.

Tables and result-bearing figures are manifest-declared, consume retained value
bindings, and emit source/caption/alternative information without retyping
numeric claims. Figures must be deterministic SVG or an equally inspectable
non-raster format supported by the `usersum` consumer, use no color-only
meaning, and carry a visible data alternative plus embedded accessible text.

Staging output is rooted below a caller-selected disposable directory and uses
the future public shape beneath `usersum/assurance/reports/`. The writer rejects
repository root, tracked public/source/export/snapshot/vendor roots, path
escape, symlink traversal, special files, and output collisions. A named build
may replace only its own confined report subtree. A check recomputes expected
bytes in memory and verifies the exact selected output set without writing.

The retained zero-public `build --all` / `check --all` behavior remains
available when no v2 staging selection is requested. V2 staging requires an
explicit staging-root option and cannot accept snapshot/release options.

## Deliverables

1. Package authority, required-reading map, protected-surface freeze, and frozen
   assembly grammar/output contract.
2. Versioned typed source/schema additions and mechanically referenced
   groundwater manuscript/supplement source.
3. Typed deterministic assembler, confined staging writer/checker, and public
   named/all API.
4. Real CLI build/check consumers and a retained `usersum`-shaped groundwater
   staging artifact with link/accessibility proof.
5. Positive and negative tests for semantic preservation, units, precision,
   missing/orphaned content, figures, links, sandboxing, isolation, exact output
   sets, and deterministic one/all equivalence.
6. Focused/full workspace, dependency-policy, documentation, fresh CRAP,
   line-count, review, disposition, verification, final-disposition, and
   ASSURE-04D handoff evidence.

## Phase Plan

### Phase 1 — Intake, Freeze, And Contract-Derived Tests

Freeze the base, protected bytes, inherited APIs, and aggregate `usersum`
identity. Record the exact assembly grammar, typed bindings, output layout,
semantic-preservation rule, and write sandbox. Add initially failing tests for
the real current named/all build/check path and every required negative gate.

### Phase 2 — Source Contract And Assembly Implementation

Add only the typed/schema vocabulary required by the accepted fixture, migrate
the fixture's duplicated claim-bearing values/tables/figures/references to that
vocabulary, and update identities explicitly. Implement the pure per-report
assembler, accessible deterministic figures, portable links, staging writer,
exact checker, repository API, and CLI. Split source-admission structure before
any file approaches 3,000 lines; new assembly code belongs in `v2/assembly.rs`.

### Phase 3 — Focused Closure And Consumer Evidence

Run formatting, assurance crate tests, all three assurance integration suites,
quick workspace tests, focused clippy, documentation validation, deterministic
repeat/one-all checks, staged link/accessibility checks, protected hashes, and
path/write-set audits. Retain one real staged groundwater build under package
artifacts only after proving the builder cannot target tracked public paths.

### Phase 4 — Dual Independent Review And Disposition

Dispatch two independent read-only coding-agent reviews. Each checks authority,
semantic preservation, result/unit/precision lineage, actual downstream
consumer, accessibility, sandbox confinement, fail-closed behavior, stable
bytes, zero-public boundary, tests, CRAP, line counts, and non-deferral. Record
every finding as `accepted`, `rejected`, `deferred`, or `follow-up`; fix and
verify every accepted finding before heavy closure.

### Phase 5 — Independent Heavy Closure

Dispatch the required heavy-gate runner after review remediation. It runs and
records `cargo fmt --check`, workspace/all-target Clippy with warnings denied,
`cargo nextest run --workspace --profile full`, `cargo deny check`, and
`bash tools/release/run_adjudicated_crap_gate.sh --base-ref
e704f0202278ebb86c6a8c667caf73d599be04ab`. Closure requires zero actionable
workspace CRAP rows and every touched production Rust file at adjudicated CRAP
at or below 30. The runner may write only package evidence.

### Phase 6 — Dual Terminal Verification And Closeout

After heavy evidence is current, dispatch two independent read-only terminal
verifications. Both verify every acceptance row, review disposition, real
rendered consumer, heavy/CRAP/line-count evidence, protected bytes, write-set
compliance, and non-deferral. Resolve new blocking findings, renew invalidated
gates, archive the active prompt byte-for-byte, close package/catalog/roadmap
state, and hand off ASSURE-04D without authorizing it.

## Validation And Acceptance

Closure requires direct current evidence that:

- named and all assembly consume the 04B typed plan and share one per-report
  implementation; the named bytes exactly equal the same report in all output;
- repeated builds and checks are byte-identical and contain no timestamp,
  hostname, random value, absolute workspace path, or modification-time input;
- authored non-directive prose is preserved byte-for-byte modulo documented
  newline normalization, and the assembler never chooses interpretation;
- every rendered numeric claim, table cell, and result-bearing figure resolves
  from a declared retained result value with matching unit and precision;
- stale/missing content, unit mismatch, unsupported or changed precision,
  unknown/duplicate references, orphaned results, inaccessible figures, unsafe
  links, and extra/missing staged files fail with typed errors;
- figure output includes caption, embedded title/description, and a visible
  non-color-dependent table/text alternative;
- a disposable `usersum`-shaped staging tree parses and every generated local
  link resolves; no generated link enters `docs/`, `crates/`, work packages, or
  another unavailable contributor-only route;
- a named build changes only its report subtree and a failed build leaves the
  prior selected subtree and all unrelated bytes unchanged;
- ordinary assembly is confined below the explicit staging root and rejects
  tracked source/public/export/snapshot/release/vendor targets and symlink/path
  escapes;
- report lifecycle remains internal `DRAFT`, zero public reports remain, and no
  v2 source reaches tracked `usersum`, export, snapshot, release, or vendoring;
- all protected bytes and aggregate `usersum` identity equal intake;
- focused, quick, full, Clippy, formatting, deny, docs, fresh CRAP, dual review,
  disposition, and dual terminal verification gates pass; and
- no finding is undispositioned, no current gate is deferred, no touched Rust
  CRAP exceeds 30, and no nonexempt 3,000-line Rust file remains.

## Delegation Authorization And Roles

This package explicitly authorizes and requires subagent spawning/delegation
for one heavy-gate runner and two independent reviewer/verifier agents.

The heavy runner executes only Phase 5 against the frozen implementation tree
and may write compact logs/reports only under this package's `artifacts/`. It
may not edit code, tests, authority, source records, public files, or queue
state. Reviewer A and Reviewer B are read-only, work independently, and return
compact findings/evidence to the parent. Coding-agent review is internal
engineering review, never scientific peer review or publication approval.

## Security And Recovery

Assembly is offline and non-executable. All source reads remain descriptor-
relative, no-follow, confined regular-file reads. Staging writes are confined
to a caller-selected disposable root and use replacement semantics that do not
expose partial selected-report output. Errors are typed; no missing input is
replaced with a fallback. Restricted evidence is never read or staged.

Rollback removes the assembly module, v2 staging CLI/API, assembly-specific
source vocabulary, and disposable staging output while retaining 04A validation,
04B planning, and the exact zero-public ASSURE-03 state.

## Progress

- [x] (2026-07-15 23:13Z) Recovered ASSURE-04C authority, completed required
  governance/architecture reading, froze base and protected identities, and
  scaffolded the active package.
- [x] (2026-07-15 23:28Z) Froze the assembly grammar/output contract and added
  initially failing contract-derived integration tests.
- [x] (2026-07-15 23:51Z) Implemented source migration, deterministic assembly, staging, checking,
  and real CLI consumers.
- [x] (2026-07-16 00:33Z) Completed focused closure and retained
  rendered-consumer evidence; accepted and remediated three fresh CRAP HOLD
  rows pending the required independent heavy rerun.
- [x] (2026-07-16) Completed dual independent review, two technical renewal
  rounds, governance-only confirmation, and remediation of every finding.
- [x] (2026-07-16) Completed independent heavy closure: formatting, strict
  workspace/all-target Clippy, 2,011 full-profile tests, dependency policy,
  and fresh adjudicated CRAP all pass; zero actionable rows and all seven
  touched production Rust files are at CRAP 30 or below.
- [x] (2026-07-16) Completed dual independent terminal verification, archived
  the execution prompt byte-for-byte, closed package/catalog/roadmap state,
  and recorded the unapproved ASSURE-04D handoff.

## Surprises And Discoveries

- Observation: the first Phase 5 sequence stopped at strict workspace Clippy
  because the assembly test used adjacent local names `stale` and `stage`.
  Evidence: `artifacts/heavy-gate-runner.md`. The runner correctly skipped
  later gates; the local was renamed `stale_stage`, strict workspace Clippy and
  31/31 focused tests pass, and the complete heavy sequence must restart.

- Observation: the inherited `v2.rs` source-admission module is already 2,064
  lines at intake.
  Evidence: ASSURE-04B handoff and intake line count. New assembly logic must
  remain in a separate module; any source-admission split must be behavior-
  preserving and reviewable.
- Observation: existing `build --all` and `check --all` are the protected
  ASSURE-03 zero-public operations.
  Evidence: `engine.rs` and the 04B CLI. The v2 staging interface must be
  explicit and cannot reinterpret a release/public command silently.

## Decision Log

- Decision: require an explicit v2 staging-root selector and preserve the
  no-selector zero-public build/check path.
  Rationale: staging mechanics and public transition behavior are different
  authority lanes; an ambiguous `--all` must not expose draft reports.
  Date/Author: 2026-07-15, Codex.

## Outcomes And Retrospective

ASSURE-04C delivered deterministic staging-only assembly for one named v2
report and all admitted reports through the ASSURE-04B typed plan. The real CLI
resolves typed quantities, tables, accessible figures, citations, research
objects, and portable links into an exact `usersum`-shaped staging tree. It
checks existing staged bytes without writing and preserves the protected
zero-public path when no staging root is selected.

Review materially strengthened the result. Staging now uses descriptor-relative
no-follow operations and transactional restoration; source drift is checked
through accepted completion; local links resolve through the held staging
capability; authored quantitative text cannot bypass typed quantity bindings;
and the actual WEPPcloud `cmarkgfm` consumer is retained as evidence. Every
review finding was accepted and remediated, with none deferred.

Terminal evidence is strict workspace Clippy, full Nextest 2,011/2,011,
dependency-policy PASS, and fresh CRAP 2 raw / 2 adjudicated / 0 actionable.
Every touched production Rust file is at CRAP 30 or below and below 3,000 lines.
Both independent terminal verifiers passed. The fixture remains internal
`DRAFT`; no tracked `usersum`, catalog, export, snapshot, release, or vendor
surface changed. ASSURE-04D is next eligible but requires separate user
authorization.
