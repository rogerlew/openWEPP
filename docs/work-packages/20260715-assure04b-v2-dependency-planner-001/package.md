# ASSURE-04B V2 Dependency Planner

Package ID: `20260715-assure04b-v2-dependency-planner-001`

Status: `EXECUTED-COMPLETE`

Execution date: 2026-07-15

Frozen base: `22fb7dfbafdb9e82a42afe0a5356b4c923a45232`

This ExecPlan is maintained under `docs/codex_exec_plans.md`. The `Progress`,
`Surprises And Discoveries`, `Decision Log`, and `Outcomes And Retrospective`
sections are living execution records.

## Purpose

Implement the smallest deterministic v2 dependency planner needed to explain
which scientific-assurance sources are current, which have changed, which
cannot be consumed, and which transitive report targets therefore need work.
After this package, a maintainer can run the real `openwepp-assurance plan`
command for one named report or all admitted reports and inspect the same
stable plan as ordinary text or JSON.

The planner is mechanical support for authors and reviewers. It does not render
the manuscript, interpret evidence, decide scientific impact, approve a report,
or publish anything. ASSURE-04C remains the owner of staging assembly, and
ASSURE-04D remains the owner of locks, promotion, public catalogs, snapshots,
release transfer, and vendoring.

The user's instruction to scaffold and execute ASSURE-04B is explicit authority
for this bounded package and for the package-required delegated heavy runner,
dual independent reviews, and dual independent terminal verifications.

## Context And Orientation

`assurance/v2/catalog.yaml` admits one internal `DRAFT`, `fixture_only`
groundwater report. `crates/openwepp-assurance/src/v2.rs` owns strict source
admission and stable source roots. `crates/openwepp-assurance/src/cli.rs`
currently allows v2 validation for one/all selections but rejects report-
specific planning. The separate `Assurance` engine retains the protected
ASSURE-03 zero-public transition state.

A planner node is one declared source or logical record. A directed edge means
that the node consumes the identified prerequisite. A target's transitive
dependency set includes direct prerequisites, their prerequisites, and so on.
The planner orders prerequisites before consumers and uses lexical stable IDs
to break ties.

The four planner states have deliberately narrow meanings:

- `current`: the node's declared content or immutable identity is available and
  unchanged, and every prerequisite is current;
- `stale`: locally observable bytes differ from the declared SHA-256 identity;
- `blocked`: required bytes are unavailable/unreadable or a prerequisite is
  blocked, so the target cannot be rebuilt safely; and
- `selected`: the node itself remains structurally usable but a direct or
  transitive prerequisite is stale, so a later build must revisit it.

These are build-impact states, not scientific grades. A `selected` review or
claim node means its bound input changed; it does not decide whether scientific
rereview is required. Cycles, missing logical targets, and unused declarations
are invalid graph definitions and fail rather than being reduced to a status.

The planner compares bytes and declared immutable identities. Modification
time, wall clock, hostname, absolute workspace path, environment interpolation,
network access, shell execution, and agent output are not planner inputs.

## Authority

Binding authority is ADR-0038, the accepted v2 architecture, lifecycle
contract, source/build contract, report standard, prospective ASSURE-04
roadmap, and the completed ASSURE-04A handoff. The 04A schema and executable
source admission define the graph vocabulary; this package may consume and
factor that implementation but may not reinterpret the scientific source.

Package artifacts are implementation evidence, not authority replacements. No
kernel process, numerical method, science contract, scientific conclusion, or
public report is changed.

## Scope

Included:

- add a typed planner module consumed through `V2Repository`;
- build the report graph from admitted manuscript, supplement, authorship,
  agent-assistance, dependency, unit, claim, method, result, figure, reference,
  research-object, review, publication, schema, and planner-tool identities;
- classify local content by observed versus declared SHA-256 and immutable
  external/restricted content by its declared stable identity;
- propagate stale and blocked states transitively without using file times;
- reject cycles, references to missing nodes, and unreachable/unused declared
  records;
- expose stable one-report and all-report plans through the real CLI;
- provide equivalent deterministic human and JSON representations;
- preserve selection isolation so a named plan never traverses an unselected
  report and one changed report does not select unrelated report targets;
- retain the zero-public ASSURE-03 boundary in plan output and checks;
- add focused unit/integration tests, protected-surface proof, line-count
  governance, fresh touched-code CRAP, dual review/disposition, independent
  heavy closure, and dual terminal verification; and
- update prospective queue and work-package catalog state truthfully.

Excluded:

- persistent caches, file watchers, automatic source-hash rewriting, or writes
  during planning;
- manuscript/supplement rendering, value substitution, table/figure assembly,
  staging output, or drift comparison against generated output (ASSURE-04C);
- choosing an impact disposition, rerun, metric, method, interpretation,
  conclusion, review decision, or application-fitness verdict;
- review locks, approval, public promotion, catalog/search integration,
  snapshots, release transfer, withdrawal/supersession, or vendoring
  (ASSURE-04D or later);
- source-schema expansion unless a proven planner requirement cannot be
  represented by the completed 04A vocabulary and the package is amended
  before that edit;
- edits to public `usersum`, public assurance catalogs, or protected export
  surfaces; and
- kernel, science-contract, comparator, or integrated-model changes.

## Declared Write Set

- `assurance/v2/README.md`
- `crates/openwepp-assurance/Cargo.toml`
- `crates/openwepp-assurance/src/{cli.rs,engine.rs,lib.rs,v2.rs}`
- `crates/openwepp-assurance/src/v2/confined.rs`
- `crates/openwepp-assurance/src/v2/planner.rs`
- `Cargo.toml` for integration-test registration
- `Cargo.lock` only for the accepted direct confinement dependency
- `tests/integration/assurance_v2_source_contract.rs` only for compatibility
  expectations owned by the prior boundary
- `tests/integration/assurance_v2_planner_contract.rs`
- `docs/ROADMAP.md`
- `docs/planning/scientific-assurance-v2-implementation-roadmap.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260715-assure04b-v2-dependency-planner-001/**`

`assurance/v2/catalog.yaml`, its schemas and report source are positive inputs,
not write-set members. The tracked `assurance/catalog.yaml`,
`assurance/templates/catalog.md`, `assurance/generated/wepppy-usersum.yaml`,
every `usersum/**` file, public export/snapshot surfaces, and all kernel/science
files are protected read-only surfaces. Any edit outside the declared set
requires a package amendment before the edit.

## Interfaces And Design Contract

`V2Repository::plan_report(id)` and `V2Repository::plan_all()` return one
immutable typed plan model. The named and all paths call the same per-report
planner. All traversal begins from the selected report target; unrelated report
graphs are not merged into that target.

The graph uses the stable logical IDs already admitted by 04A. Source and tool
nodes receive reserved, namespaced IDs that cannot collide with report-local
IDs. Every node records its kind, final state, observed or immutable identity,
declared identity where applicable, explanation, and sorted direct
dependencies. The plan records selected/total/public report counts and the
protected zero-public publication state.

Locally stored content is inspected as a confined regular non-symlink file.
Matching observed and declared SHA-256 means current. Mismatch means stale.
Missing/unreadable/special content means blocked. Embedded logical records are
bound to the observed report-manifest identity, so a method, figure, review,
software declaration, or other manifest edit selects its report even before
the catalog digest is reconciled. Result and narrative files retain their own
identities, and local science-contract or implementation evidence is represented
as an ordinary dependency node.

An invalid manifest that cannot be parsed produces a blocked report plan only
when its catalog entry and confined locator are still available. A parseable
manifest with an invalid identity/reference/unused-edge contract fails with a
typed error. This distinction lets the planner explain unavailable work without
normalizing malformed graph authority.

The CLI syntax becomes:

    openwepp-assurance plan (--all | --report <id>) [--format human|json]

Human is the default. `--format` is plan-only. `validate`, `build`, and `check`
retain their existing contracts, and report-specific build/check remain
explicitly owned by ASSURE-04C.

## Deliverables

1. Package authority, required-reading map, protected-surface freeze, and
   planner-state/graph design record.
2. Typed deterministic graph planner and public one/all repository API.
3. Real CLI human/JSON plan consumers with zero-public boundary context.
4. Positive and negative tests for graph closure, states, transitive impact,
   stable ordering, selection isolation, and one/all equivalence.
5. Focused/full workspace, dependency-policy, documentation, fresh CRAP,
   line-count, review, disposition, verification, final-disposition, and
   ASSURE-04C handoff artifacts.

## Phase Plan

### Phase 1 — Intake, Freeze, And Contract-Derived Tests

Freeze the base and protected bytes; record the graph/state semantics; scaffold
the active prompt and artifacts. Add failing tests for the real current plan,
human/JSON equivalence, stale narrative/result/contract and manifest-contained
method/figure/review/software changes, blocked missing content, transitive
selection, stable topological order, one/all equivalence, named isolation,
unrelated-report nonselection, cycles, missing edges, unused nodes, and
report-specific build/check deferral.

### Phase 2 — Planner Implementation

Factor structural admission from content reads where necessary, add the
planner module, expose repository plan APIs, and connect the CLI. Prefer one
small graph core over record-family special cases. No ordinary plan operation
may write or execute external work.

### Phase 3 — Focused Closure And Evidence

Run formatting, the assurance crate tests, both assurance integration suites,
quick workspace tests, focused clippy, documentation validation, CLI
demonstrations, protected hashes, and explicit path/write-set audits. Record
line counts and decompose new planner code before review if it reaches the
2,000-line warning threshold.

### Phase 4 — Dual Independent Review And Disposition

Dispatch two independent read-only coding-agent reviews. Each checks the full
diff against package authority, graph semantics, actual downstream consumers,
fail-closed behavior, state explanations, stable output, no-publication
boundary, test adequacy, CRAP gate requirement, line-count governance, and the
Gate Evidence Non-Deferral Rule. Record every finding and disposition it as
`accepted`, `rejected`, `deferred`, or `follow-up`; fix and verify every
accepted finding before heavy closure.

### Phase 5 — Independent Heavy Closure

Dispatch the required heavy-gate runner after review remediation. It runs and
records `cargo fmt --check`, workspace/all-target clippy with warnings denied,
`cargo nextest run --workspace --profile full`, `cargo deny check`, and
`bash tools/release/run_adjudicated_crap_gate.sh --base-ref
22fb7dfbafdb9e82a42afe0a5356b4c923a45232`. Closure requires zero actionable
CRAP rows and every touched Rust file at adjudicated CRAP at or below 30. The
runner may write only package evidence and must not change production or test
sources.

### Phase 6 — Dual Terminal Verification And Closeout

After heavy evidence is current, dispatch two independent read-only terminal
verifications. Both must verify each acceptance row, review disposition,
consumer path, heavy evidence, CRAP closure, line-count disposition, protected
bytes, write-set compliance, and non-deferral. Resolve any new blocking finding,
renew invalidated gates, archive the active prompt without changing its bytes,
close the package/catalog, remove completed 04B work from the prospective
roadmap, and hand off only ASSURE-04C.

## Validation And Acceptance

Package closure requires direct current evidence for all of the following:

- `plan --report <id>` and `plan --all` use the same per-report graph and the
  named report plan equals its entry in the all-report plan;
- human output and JSON expose the same ordered nodes, states, identities,
  dependencies, and reasons, and repeated runs are byte-identical;
- a current fixture is entirely current; no rebuild is invented from selection
  alone;
- changed narrative, result, local contract/evidence, and manifest-contained
  method, figure, review, and software identities select the report through the
  expected transitive path;
- unavailable local content blocks its consumers with a relative-path reason;
- graph cycles, missing logical destinations, and unused declarations fail;
- a named plan does not traverse an unselected malformed report, and a changed
  report does not select an unrelated report in an all plan;
- modification-time-only changes do not alter plan bytes;
- planning performs no writes and includes no absolute workspace path, clock,
  hostname, network, shell, or agent dependency;
- report-specific build/check remain fail-closed for ASSURE-04C and no v2
  source reaches `usersum`, export, snapshot, or vendoring surfaces;
- all protected bytes and the aggregate `usersum` identity equal the intake
  freeze;
- focused, quick, full, clippy, formatting, deny, docs, fresh CRAP, dual review,
  disposition, and dual terminal verification gates pass; and
- the package has no undispositioned finding, no unmet current gate, no touched
  Rust CRAP above 30, and no unresolved 3,000-line file.

## Delegation Authorization And Roles

This package explicitly authorizes subagent spawning/delegation for one
heavy-gate runner and two independent reviewer/verifier agents.

The heavy runner executes only the Phase 5 commands against the frozen
implementation tree and may write compact logs/reports only under this
package's `artifacts/`. It may not edit code, tests, authority, source records,
public files, or queue state.

Reviewer A and Reviewer B independently receive the package, authority list,
frozen base, and diff. During review and terminal verification they are
read-only and return compact evidence/findings to the parent; the parent writes
their artifacts and dispositions. They do not share findings before returning
independent reports. Coding-agent review is internal engineering review, not
external scientific peer review.

## Security And Recovery

Planning is offline and read-only. All file locators remain repository-relative,
confined, regular, and non-symlink. Errors are typed; missing dependencies are
not replaced with fallbacks. JSON contains no absolute root. No secret,
protected evidence content, environment variable, or external command is
admitted into the graph.

Rollback removes the planner module and its API/CLI integration while leaving
04A validation and the exact zero-public state intact. No generated or public
output exists to clean up.

## Progress

- [x] (2026-07-15 19:31Z) Read package, assurance, prompt, crate, test, and CI
  authority; froze base and protected surfaces.
- [x] (2026-07-15 19:31Z) Scaffolded this package, prompt, artifact controls,
  and active queue entries.
- [x] (2026-07-15 20:07Z) Added contract-derived planner tests with an expected
  pre-implementation compile failure and 35 passing assurance integration gates
  after review remediation.
- [x] (2026-07-15 20:07Z) Implemented graph planning, state propagation,
  repository APIs, and human/JSON CLI formats.
- [x] (2026-07-15 20:07Z) Completed focused clippy/tests, 1,916-test quick
  workspace closure, documentation checks, no-write proof, and protected hashes;
  focused gates were renewed after review remediation.
- [x] (2026-07-15 21:04Z) Completed dual independent review, dispositioned and
  fixed every finding, and obtained two independent remediation PASS verdicts.
- [x] (2026-07-15 22:41Z) Completed the independently restarted five-gate
  terminal sequence: full Nextest 2,001/2,001, dependency policy PASS, fresh
  CRAP 2 raw / 2 adjudicated / 0 actionable, and all touched maxima at or below
  26.
- [x] (2026-07-15 22:56Z) Completed dual terminal verification; accepted and
  corrected the sole artifact-truth finding; both independent verifiers PASS.
- [x] (2026-07-15 22:58Z) Recorded final disposition, archived the execution
  prompt byte-for-byte, closed queue/catalog state, and handed off ASSURE-04C
  as next eligible but not authorized.

## Surprises And Discoveries

- Observation: 04A deliberately admits strict content identities but has no
  generated-output or cache identity; treating a plan as proof of a completed
  build would cross the 04C boundary.
  Evidence: the 04A handoff assigns planning only to 04B and assembly/checks to
  04C; the source/build contract says no persistent incremental cache is needed
  for the first implementation.
- Observation: `v2.rs` is 2,042 lines before 04B.
  Evidence: `wc -l crates/openwepp-assurance/src/v2.rs` at intake. New graph
  code therefore belongs in `v2/planner.rs`; the existing file carries a WARN
  and a later split intent rather than absorbing another subsystem.

## Decision Log

- Decision: classify declared-vs-observed content directly and propagate its
  effect instead of adding a persisted planner cache or pre-04C build lock.
  Rationale: this exposes current/stale/blocked/selected semantics now without
  claiming that absent rendered output is current or expanding the schema.
  Date/Author: 2026-07-15, Codex.
- Decision: logical graph defects fail, while unavailable bytes produce a
  blocked plan.
  Rationale: malformed authority is not a usable plan state, but a maintainer
  needs a deterministic explanation when an otherwise declared local input is
  absent.
  Date/Author: 2026-07-15, Codex.
- Decision: all embedded records depend on the observed manifest identity,
  while separately stored narrative/result/dependency content keeps its own
  identity.
  Rationale: the 04A serialization keeps these records in one strict manifest;
  the planner must not invent false sub-file baselines.
  Date/Author: 2026-07-15, Codex.
- Decision: amend the write set for a descriptor-relative confinement module
  and direct `libc` dependency in response to accepted Review A finding A02.
  Rationale: the inherited metadata-check-then-path-read sequence has a TOCTOU
  escape window. A no-follow `openat` chain validates and reads the same opened
  descriptors; the lock already contains `libc` 0.2.186 transitively.
  Date/Author: 2026-07-15, Codex.
- Decision: route the ASSURE-03 compatibility engine's source/check reads
  through the same descriptor-confined reader.
  Rationale: the real 04B CLI validates the protected zero-public engine before
  planning; leaving its canonicalize-then-read helper pathname-based would
  preserve the same A02 replacement window in the actual consumer.
  Date/Author: 2026-07-15, Codex.
- Decision: accept and remediate the first heavy run's test-only
  `clippy::format_push_string` finding with `write!` into the existing string,
  then restart the complete heavy sequence from a new freeze.
  Rationale: workspace/all-target Clippy is a current closure gate; the held
  attempt cannot be combined with later evidence. The mechanical test-helper
  correction preserves the generated fixture bytes and passes its 10-test
  integration suite.
  Date/Author: 2026-07-15, Codex.
- Decision: accept the second heavy run's actionable CRAP row and decompose the
  CLI dispatcher into command-specific helpers without adding an exception.
  Rationale: `execute` at complexity 27 and CRAP 37.7074 violates the package's
  touched-code closure rule. The dispatcher is now complexity 6 in a
  zero-coverage structural scan, and real CLI build/check paths supplement the
  existing plan/validate/negative consumers.
  Date/Author: 2026-07-15, Codex.

## Outcomes And Retrospective

ASSURE-04B delivered one immutable typed dependency plan for named and all
report selections plus real human and JSON CLI consumers. The plan explains
content-identity current/stale/blocked/selected impact, orders prerequisites
before consumers, rejects invalid graphs, isolates named selection, ignores
modification time, and writes nothing.

Review materially improved the result: blocked prerequisites now outrank stale
consumers, and both source-admission paths use descriptor-relative no-follow
reads that validate and read the same descriptor. Heavy closure then exposed a
test-only Clippy defect and an actionable CLI dispatcher CRAP row; both were
fixed rather than waived, independently reviewed, and followed by a complete
fresh PASS.

Terminal evidence is full Nextest 2,001/2,001 with three skipped, dependency
policy PASS, and fresh CRAP 2 raw / 2 adjudicated / 0 actionable with every
touched-production-file maximum at or below 26. Protected public bytes and the
aggregate `usersum/**` identity equal intake. No report was rendered, approved,
published, snapshotted, released, or vendored.
