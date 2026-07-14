# Scientific Assurance Dossier Lifecycle Foundation

Status: `queued`

Date opened: `2026-07-14`

Execution mode: `package-end-to-end`

Package type: governance and technical contract, bounded Rust tooling,
scientist-facing documentation, and release-integration vertical slice

## Objective

Establish the lifecycle and ownership contract for openWEPP scientific
assurance dossiers, then prove it with one narrow SNOTEL snow-evidence vertical
slice. The package must make the public evidence product easier for
hydrologists, soil scientists, researchers, and practitioners to understand and
audit while making routine rebuilds deterministic and mechanical.

The completed slice will connect four questions without conflating their
owners:

1. **Why does the model behave this way?** A domain-science narrative explains
   the model and its scientific rationale.
2. **How was it evaluated?** A versioned method record explains data, metrics,
   partitions, criteria, and uncertainty treatment.
3. **What does the evidence show?** A generated dossier presents bounded
   verification and empirical evidence, limitations, and reproduction links.
4. **So what for my application?** A decision-owner worksheet helps a user
   compare the evidence envelope with a specific use without openWEPP issuing
   the fitness verdict.

## Why This Package Exists

The active strategy and dossier standard define the right scientific posture:
software verification can close declared obligations, empirical corroboration
is graded and revisable, and application fitness belongs to the responsible
user or institution. They do not yet define who owns each dossier layer, when a
dossier is rebuilt or reviewed, how a release binds an evidence snapshot, or
how public pages remain synchronized with their sources.

The first implementation must remain smaller than a general evidence platform.
One real dossier is enough to expose the lifecycle, cross-reference, template,
and build contracts. It is not enough to justify a database, service,
portfolio query engine, arbitrary task runner, W3C PROV layer, or automated
scientific adjudicator.

## Outcome Boundary

This package closes a **lifecycle-and-publication foundation**, not a claim that
openWEPP snow behavior is generally validated. It may publish an honest
`NOT_EVALUATED` or `INSUFFICIENT_EVIDENCE` baseline if the retained SNOTEL
evidence cannot support a stronger, independently reviewed characterization.
That result is a successful transparency outcome.

The package does not perform an application-fitness assessment, admit a new
observational dataset, rerun a new empirical campaign, or modify another
repository. It emits a bounded handoff contract for wepppy; the corresponding
wepppy vendor, manifest, navigation, rendering, and search changes require a
separate authorized package in that repository.

## Governing Principles

- The human evidence product is primary. Machine records exist to keep it
  traceable and reproducible.
- Narrative, method, evidence, and application decision are separate records
  with explicit owners and cross-references.
- A build can determine mechanical congruence. It cannot determine scientific
  currency, reviewer competence, or application fitness.
- Published evidence is immutable as-of evidence. Material change creates a
  new dossier version and supersession link; it does not rewrite history.
- Ordinary builds are local, deterministic, offline, and agent-free.
- Agent-assisted synthesis, when used during authoring, is a reviewable input,
  never an invisible build step or scientific authority.
- Missing, mixed, contradicted, and unevaluated evidence remains publishable and
  visible.
- Existing science contracts, observation-admission decisions, tests, release
  records, and work packages are linked as authority or evidence; the dossier
  does not duplicate or supersede them.

## Included Scope

### 1. Lifecycle and ownership contract

Author one canonical governance contract that defines:

- stable dossier and method identity;
- the ownership of narrative, method, evidence, review, release snapshot,
  usersum integration, and application decision;
- lifecycle transitions `DRAFT -> CANDIDATE -> PUBLISHED -> SUPERSEDED`, plus a
  terminal `WITHDRAWN` path that preserves the record and rationale;
- which content changes require rebuild, review, a new version, supersession,
  or release-snapshot renewal;
- the distinction between mechanical drift, review invalidation, evidence
  as-of date, and scientific currency;
- immutable release snapshot semantics and rollback/discovery expectations;
- the required cross-references among the public narrative, evaluation method,
  dossier, application worksheet, and generated catalog;
- source-versus-generated ownership and the prohibition on hand-editing
  generated pages; and
- the reproducible procedure for optional agent-assisted analysis.

Reconcile the strategy, dossier standard, and usersum style guide where their
current wording conflicts with the asymmetric assurance vocabulary. In
particular, remove the style guide's `validated / bounded / open` ladder as a
public scientific status scheme.

### 2. Minimal source and build contract

Create a small, tracked source tree under `assurance/` with:

- a catalog;
- typed, versioned schemas;
- strict dossier and method templates;
- one SNOTEL snow-evidence method record;
- one bounded dossier source with separately owned interpretation and
  limitations content;
- an evidence manifest and review lock bound by SHA-256 identities;
- a generated-output contract for `usersum/assurance/`; and
- a machine-readable wepppy handoff/export fragment.

Implement a focused `openwepp-assurance` Rust crate and CLI. It owns only these
operations:

- `validate`: validate schemas, IDs, paths, links, lifecycle fields, review
  locks, and the dependency graph;
- `plan`: report the selected dossier's transitive inputs, content identities,
  generated outputs, and review implications without writing;
- `build`: render one dossier or all dossiers from tracked inputs, and create a
  content-bound release snapshot only when explicitly requested; and
- `check`: rebuild into a temporary location and fail on generated-output,
  catalog, export-fragment, or lock drift.

The exact argument spelling is frozen in the lifecycle/build contract before
production code. The CLI must support `--dossier <stable-id>` and `--all` where
applicable. A release snapshot must require an explicit, path-safe snapshot ID
and must refuse to overwrite an existing snapshot with different content.

The graph is typed data, not executable configuration. Manifests may declare
known node kinds and repository-relative inputs/outputs; they may not contain
shell commands, environment interpolation, network fetches, plugins, or agent
invocations. Node fingerprints include the node specification, ordered input
paths and bytes, relevant tool/schema/template versions, and declared software,
configuration, dataset, and evidence identities. Filesystem modification time
is never an evidence identity.

This first implementation needs deterministic impact reporting and targeted or
all-dossier builds. It does not need a persistent incremental cache or a
general-purpose `affected` query. Add those only after a second real dossier
demonstrates the need.

### 3. One public SNOTEL snow-evidence vertical slice

Use the existing five-climate SNOTEL evidence named by ADR-0028 and the current
snow/frost usersum narrative as the pilot. Bound the dossier to the quantities
and tested domain the retained evidence can actually sustain. Do not silently
extend it to frost, runoff, erosion, watershed behavior, untested climates, or
application fitness.

Before choosing a characterization, inventory the actual retained data,
commands, configurations, outputs, figures, metric definitions, software
identity, calibration/evaluation roles, uncertainty, and review records. The
inventory must distinguish:

- directly reproducible evidence;
- content-identified but externally located evidence;
- historical narrative claims without a complete retained reproduction path;
- verification, comparative, and empirical evidence; and
- forcing-robust versus forcing-limited quantities.

If the evidence is incomplete, render that incompleteness in the dossier and
select `NOT_EVALUATED` or `INSUFFICIENT_EVIDENCE` as required by the standard.
Do not reconstruct favorable provenance from memory or use current prose as its
own evidence.

The vertical slice must produce a self-contained public path under
`usersum/assurance/` containing:

- an assurance catalog/index;
- a readable evaluation-method page;
- a dossier whose first screen answers what was tested, what the evidence says,
  where it applies, and what remains unknown;
- an application-context worksheet;
- links to the existing model-science narrative for the "why" and back from
  that narrative to the method and dossier; and
- reproduction and audit references that do not expose internal-only paths,
  credentials, or restricted data.

Generated Markdown contains a non-rendered source banner and is never edited by
hand. The narrative remains hand-authored under the usersum style guide.

### 4. Release and usersum integration boundary

Add a local drift check modeled on the existing hillslope schedule export
check. Integrate the assurance check into the release-candidate gate and draft
release procedure so a release:

1. validates and checks committed public outputs;
2. creates or consumes an explicitly named immutable dossier snapshot;
3. records the snapshot/catalog digest in the release evidence bundle; and
4. fails if a required generated page, lock, or export fragment is stale.

The openWEPP export fragment must provide stable document IDs, source-relative
paths, titles, minimum role, category, audience tags, status, and navigation
keys compatible with the current wepppy usersum contract. openWEPP owns those
source identities and content. wepppy remains the owner of vendor registration,
copy/sync policy, final manifest merge, navigation placement, role
authorization, rendering, and search indexing.

Do not edit `/home/workdir/wepppy` in this package. Record a precise follow-on
handoff with the files and validation commands a future wepppy package must
own.

### 5. Tests and nextest role

Use nextest to run ordinary Rust tests of the schemas, graph, deterministic
rendering, review locks, snapshots, CLI behavior, and drift checks. Nextest is
the test executor; it is not the dossier dependency graph or build scheduler.

A focused `cargo nextest run -p openwepp-assurance` lane is required. Add a
workspace `assurance` profile only if measured test organization warrants it;
the package does not require a new profile merely for naming symmetry. The
full workspace profile remains the terminal Rust gate.

## Excluded Scope

- A database, server, daemon, dashboard, generalized evidence graph, workflow
  engine, plugin system, W3C PROV export, or RO-Crate export.
- Network access, dataset downloads, credential handling, or command execution
  from dossier manifests.
- Persistent incremental caching or portfolio-scale impact analysis.
- Automatic generation of empirical characterizations, application-fitness
  verdicts, reviewer approval, or scientific-currency claims.
- Agent or language-model invocation during `validate`, `plan`, `build`,
  `check`, release gates, or ordinary dossier rebuilds.
- New observational-data admission, SNOTEL fixture changes, calibration,
  threshold changes, empirical reruns, or favorable reclassification without
  current supporting evidence and review.
- Dossiers for frost, runoff, soil loss, routing, plant growth, channels, or
  watersheds.
- Kernel, science-contract, numerical, simulation, or public result changes.
- Changes in `/home/workdir/wepppy` or any other repository.
- A public release, tag, deployment, or assertion that the wepppy UI already
  publishes the new pages.
- Changes to the adjudicated CRAP exception registry.

## Deliverables

1. Canonical lifecycle/ownership/build contract and aligned strategy,
   standards, release procedure, and navigation.
2. `assurance/` source layout, schemas, templates, catalog, pilot source,
   evidence manifest, review record, and wepppy export contract.
3. `openwepp-assurance` library/CLI with focused and integration tests.
4. Generated `usersum/assurance/` catalog, method, dossier, worksheet, and
   supporting public assets, plus two-way links to the snow/frost narrative.
5. Deterministic drift check and release-candidate integration.
6. Package-local reading map, evidence inventory, lifecycle/ownership matrix,
   dependency/build proof, wepppy handoff, implementation and gate evidence,
   dual reviews, finding disposition, dual verification, worker handoff, and
   final disposition.

## Intended Write Set

- `docs/governance/scientific-assurance-dossier-lifecycle.md`
- `docs/governance/openwepp-verification-validation-strategy.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `docs/governance/README.md`
- `docs/standards/scientific-assurance-dossier.md`
- `docs/standards/usersum-authoring-style-guide.md`
- `docs/standards/README.md`
- `docs/README.md`
- `assurance/**`
- `crates/openwepp-assurance/**`
- `usersum/README.md`
- `usersum/snow-frost-modeling-and-validation.md`
- `usersum/assurance/**`
- `tools/release/check_assurance_dossier_exports.sh`
- `tools/release/run_release_candidate_gates.sh`
- `tools/release/README.md`
- `tests/integration/assurance_dossier_build_contract.rs`
- `.config/nextest.toml` only if a justified focused profile is added
- `Cargo.toml`
- `Cargo.lock`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260714-assurance-dossier-lifecycle-foundation-001/**`

Any required write outside this set must be added to the package before the
edit, with scope and gate impact recorded. No write to another repository is
authorized.

## Protected Boundaries

- Canonical `SC-*` physics authority remains unchanged.
- ADR-0028 continues to control observed-data admission and forcing-robust
  versus forcing-limited interpretation.
- Existing empirical claims are not strengthened by moving them into a new
  format.
- Verification and empirical statuses remain separate; neither becomes an
  application-fitness verdict.
- Generated public pages do not become authority over their tracked sources.
- A matching hash proves content congruence, not scientific adequacy or
  currency.
- A package reviewer or coding agent is not represented as an external
  hydrologist or institutional decision owner.
- Missing evidence is exposed, not replaced by legacy parity, conservation,
  code coverage, test count, or agent judgment.

## Lifecycle And Ownership Minimum

The canonical contract must assign at least these roles. One person may hold
multiple roles when disclosed, but the reviewer may not approve their own
conclusion-bearing work.

| Record or decision | Accountable owner | Required separation |
| --- | --- | --- |
| Model-science narrative (why) | Domain science steward | Does not assign evidence status. |
| Evaluation method (how) | Evaluation method owner | Frozen before new conclusion-bearing execution; retrospective choices are labeled. |
| Evidence manifest and dossier source (what) | Dossier steward | Cannot convert missing provenance into favorable evidence. |
| Empirical characterization | Named scientific assessment owner | Requires independent scientific review at the declared consequence level. |
| Mechanical build and generated pages | Assurance tooling maintainer | May report drift, never scientific validity. |
| Release snapshot inclusion | Release authority | Accepts software gates and snapshot identity, not site fitness. |
| wepppy vendoring and discovery | wepppy documentation owner | Owns downstream merge, role, navigation, render, and search behavior. |
| Application fitness (so what) | Named user or institutional decision owner | Never inferred from openWEPP status. |

The contract must contain a trigger matrix covering at least model code,
configuration, dataset, transformation, metric, tolerance, partition,
template, interpretation, limitation, reviewer, and public-navigation changes.
For each trigger it states whether the effect is mechanical rebuild, evidence
impact review, independent rereview, new dossier version, new release snapshot,
or no scientific impact.

## Optional Agent-Assisted Analysis Procedure

Normal builds contain no semantic agent step. If an agent helps inventory,
summarize, draft, or compare evidence during candidate authoring, retain a
review packet containing:

- the bounded question and procedure version;
- the complete prompt or task instruction;
- repository-relative input paths and SHA-256 digests;
- model/agent identity and tool version where available;
- execution date and declared nondeterministic settings where available;
- the retained output or its content-bound location;
- the accepted edits or extraction decisions;
- an output digest; and
- the named reviewer, findings, dispositions, and approved candidate-root
  digest.

The record supports traceability and procedural repetition; it does not promise
byte-identical model output or preserve private chain-of-thought. Agent output
is a proposal until reviewed and locked. A changed upstream input invalidates
the lock and produces `REVIEW_REQUIRED`; no automatic rewrite or status change
is allowed.

## Dependency And Build Invariants

- Stable IDs are unique and never derived from mutable titles or paths.
- Every generated artifact has one declared producer and a complete transitive
  input set.
- Unknown node kinds, duplicate IDs, missing dependencies, graph cycles,
  undeclared outputs, output collisions, and repository escapes fail closed.
- Inputs and outputs are repository-relative, normalized, and confined to
  approved roots. Absolute paths, `..` traversal, unsafe symlink escape,
  environment expansion, and snapshot-ID traversal are rejected.
- The same frozen inputs and tool version produce byte-identical output.
- `check` compares a clean temporary build with committed generated output and
  writes no tracked source.
- A targeted build may change only the selected dossier's declared outputs and
  shared outputs whose dependencies changed.
- Shared catalog/export output is deterministic and ordered by stable ID.
- Review locks bind the complete conclusion-bearing source root. Any bound
  change requires review before `PUBLISHED` output or a release snapshot.
- An existing snapshot is immutable. An identical rebuild may confirm it; a
  differing rebuild under the same snapshot ID fails.
- External or restricted evidence may be referenced with access posture and
  identity, but the builder never fetches it and never exposes secrets or
  private absolute paths in public output.

## Phase Plan

### Phase 0: Intake and authority freeze

1. Record the scaffold commit as `FROZEN_BASE` before implementation edits.
2. Re-run instruction discovery for the final write set and update the reading
   map if the scope changes.
3. Inventory current usersum, release, dossier, SNOTEL, and wepppy handoff
   contracts without changing scientific status.

### Phase 1: Contract first

1. Author the lifecycle/ownership/build contract and trigger matrix.
2. Align the V&V strategy, dossier standard, usersum style guide, release
   procedure, and indexes.
3. Freeze CLI behavior, source/generated layout, schema versions, lifecycle
   transitions, snapshot rules, and cross-repository ownership before Rust
   implementation.

### Phase 2: Pilot evidence and information architecture

1. Complete the SNOTEL evidence inventory before assigning a pilot status.
2. Define the bounded dossier envelope and public why/how/what/so-what links.
3. Create schemas, templates, sources, manifest, and candidate review packet.

### Phase 3: Minimal builder and tests

1. Implement typed parsing, validation, graph construction, hashing, planning,
   deterministic rendering, drift checking, and immutable snapshot behavior.
2. Add unit and integration tests, including negative security and
   review-invalidation cases.
3. Keep production Rust functions under the adjudicated CRAP threshold and
   keep modules below line-count warning thresholds.

### Phase 4: Public vertical slice and release boundary

1. Build and inspect the usersum catalog, method, dossier, worksheet, and
   export fragment.
2. Reconcile the snow/frost narrative and two-way links without duplicating
   claim-bearing results.
3. Add the drift script and release-candidate gate/snapshot evidence wiring.
4. Write the exact wepppy follow-on handoff without editing wepppy.

### Phase 5: Closure

1. Run focused builder, negative, documentation, security, and deterministic
   rebuild gates.
2. Run the required full Rust and adjudicated CRAP closure gates on terminal
   source.
3. Obtain two independent reviews, disposition every finding, fix every
   accepted closure finding, and rerun affected gates.
4. Obtain dual independent verification of accepted fixes and terminal
   evidence, then record final disposition.

## Exit Criteria

| ID | Criterion |
| --- | --- |
| `ASSURE-LIFE-001` | A canonical contract assigns the why/how/what/so-what records and decisions to named roles, defines separation requirements, and provides two-way public cross-references. |
| `ASSURE-LIFE-002` | The contract defines `DRAFT`, `CANDIDATE`, `PUBLISHED`, `SUPERSEDED`, and `WITHDRAWN` transitions, immutable published history, and a material-change trigger matrix. |
| `ASSURE-LIFE-003` | Mechanical congruence, review validity, evidence as-of date, scientific currency, verification acceptance, empirical characterization, and application fitness remain distinct. |
| `ASSURE-LIFE-004` | Source and generated roots, edit ownership, stable IDs, versioning, supersession, and release snapshot identity are explicit and mechanically checked. |
| `ASSURE-BUILD-001` | The bounded CLI implements contract-frozen `validate`, `plan`, `build`, and `check` operations for one dossier and all dossiers; ordinary operation is offline and agent-free. |
| `ASSURE-BUILD-002` | A typed, content-hashed DAG detects duplicate/unknown/missing/cyclic/escaping dependencies and uses no mtimes, arbitrary commands, environment interpolation, or network fetches. |
| `ASSURE-BUILD-003` | Two clean builds from identical inputs are byte-identical; a targeted plan/build reports the complete transitive input and output set; committed-output drift fails. |
| `ASSURE-BUILD-004` | A bound source change invalidates the review lock and blocks `PUBLISHED` rendering or snapshot creation with `REVIEW_REQUIRED`; no scientific status is changed automatically. |
| `ASSURE-BUILD-005` | Snapshot creation is explicit, content-bound, path-safe, immutable by ID, and records dossier/catalog/tool/source identities suitable for release evidence. |
| `ASSURE-PILOT-001` | The SNOTEL pilot inventory identifies reproducible, external, unavailable, historical, verification, comparative, empirical, forcing-robust, and forcing-limited evidence without strengthening it. |
| `ASSURE-PILOT-002` | The pilot's status follows the dossier standard and current retained evidence; incomplete provenance produces visible `NOT_EVALUATED` or `INSUFFICIENT_EVIDENCE`, not a reconstructed favorable claim. |
| `ASSURE-PILOT-003` | The public catalog, method, dossier, worksheet, narrative links, limitations, and audit pointers let a scientific user navigate why, how, what, and application questions without internal repository knowledge. |
| `ASSURE-PILOT-004` | Generated pages are template-consistent, clearly content-bound, free of broken or repo-internal links, and do not duplicate the pilot's claim-bearing numeric results across narrative, method, and dossier. |
| `ASSURE-XREPO-001` | openWEPP emits a deterministic wepppy-compatible export fragment and a precise handoff; no file in `/home/workdir/wepppy` or another repository is changed. |
| `ASSURE-REL-001` | Release-candidate automation checks dossier drift and records an explicit immutable snapshot/catalog digest; stale or missing required output fails closed. |
| `ASSURE-TEST-001` | Nextest runs builder/contract tests but is not used as the evidence DAG; focused, negative, snapshot, deterministic, and real generated-consumer tests pass. |
| `ASSURE-SEC-001` | Traversal, symlink escape, output collision, unsafe snapshot ID, arbitrary execution, secret/private-path publication, and external-fetch cases fail closed. |
| `ASSURE-GOV-001` | Strategy, dossier standard, usersum style, release procedure, navigation, and public terminology agree; the obsolete `validated / bounded / open` status ladder is removed. |
| `ASSURE-CLOSE-001` | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, and `cargo deny check` pass on terminal source. |
| `ASSURE-CLOSE-002` | The fresh adjudicated CRAP gate passes from `FROZEN_BASE`; every touched production Rust function has raw CRAP at most 30, with no package-local waiver or exception-registry change. |
| `ASSURE-CLOSE-003` | Every touched `.rs` file has a recorded line count; 2000+ lines is dispositioned as `WARN`, and no nonexempt 3000+ line file remains. |
| `ASSURE-CLOSE-004` | Two independent reviews and two accepted-fix verifications complete; all findings are dispositioned as `accepted`, `rejected`, `deferred`, or `follow-up`, and no accepted closure finding remains open. |

Every exit criterion is classified `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`.
Any state other than `PASS` blocks `COMPLETE` disposition.

## Required Verification Commands

Focused commands are frozen to their final CLI spelling during Phase 1 and
recorded in `artifacts/gate-results.md`. At minimum, terminal evidence includes:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run -p openwepp-assurance
cargo nextest run --test assurance_dossier_build_contract
cargo nextest run --workspace --profile full
cargo deny check
bash tools/release/check_assurance_dossier_exports.sh
bash tools/release/run_adjudicated_crap_gate.sh \
  --base-ref <FROZEN_BASE> \
  --output-dir docs/work-packages/20260714-assurance-dossier-lifecycle-foundation-001/artifacts/adjudicated-crap
```

Also record:

- two independent temporary-directory builds and a byte comparison;
- targeted-versus-all plan and output-set comparison;
- stale generated-output detection;
- changed reviewed-input -> `REVIEW_REQUIRED` proof;
- snapshot same-ID/same-content confirmation and same-ID/different-content
  rejection;
- path traversal, symlink escape, output collision, and unsafe snapshot-ID
  rejection;
- generated usersum local-link and source-banner checks;
- a source-level proof that normal build paths cannot invoke shell commands,
  network clients, or agents; and
- `git diff --check`, Markdown lint, spelling preview, intended-write-set, and
  cross-repository status checks.

Do not run the full release-candidate script merely to prove the new hook; its
existing full workspace and cohort lanes are already covered by the terminal
closure loop and package-specific hook tests. If execution changes the release
script in a way that cannot be proven without a full release-candidate run,
amend the package before implementation and record the additional heavy gate.

## Review Plan

Reviewer A is the scientific-user and governance reviewer. It checks whether a
hydrologist or soil scientist can follow why, how, what, limitations, and the
application decision boundary; whether the SNOTEL evidence is classified
without overstatement; whether ownership and rereview triggers are workable;
and whether an agent review is represented honestly.

Reviewer B is the build, release, and security reviewer. It checks schema/DAG
correctness, deterministic and targeted builds, review locks, snapshot
immutability, nextest's bounded role, source/generated ownership, release-gate
consumer proof, path containment, and the wepppy boundary.

Both reviewers inspect terminal source independently and must not read the
other's initial review before submitting their own. After accepted findings are
fixed, each reviewer verifies the relevant remediation and gates.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent reviewer/verifier subagents for the bounded Reviewer A and
Reviewer B scopes above. Expected outputs are
`artifacts/review-a.md`, `artifacts/review-b.md`,
`artifacts/verification-a.md`, and `artifacts/verification-b.md`. Each role has
read-only implementation access and bounded write access only to its assigned
package artifacts.

Subagent authorization: this package explicitly authorizes spawning/delegating
to a `comparator_suite_runner` or equivalent heavy-gate runner subagent for the
full workspace nextest, clippy, deny, and fresh adjudicated CRAP closure runs.
Expected outputs are compact command results and durable log/artifact paths;
write access is limited to package gate artifacts and tool-generated temporary
outputs.

Subagent requirement: dual independent review and verification are required.
A heavy-gate runner is required for the terminal full closure loop when
available. The parent must not run those heavy gates itself unless subagent
tooling is unavailable, in which case it records command-level evidence of the
unavailability before local execution.

## Security Impact

Security impact is moderate because the package introduces a parser, renderer,
filesystem traversal, generated public content, and release-gate integration.
It introduces no network service and must not execute manifest content.

Security review is closure-blocking. It covers path normalization and
containment, symlink behavior, output overwrite, snapshot ID handling, duplicate
and collision detection, YAML/JSON resource bounds where practical, public-link
sanitization, restricted evidence posture, accidental absolute-path disclosure,
and absence of credentials or tokens in source and generated output.

## Hold Boundaries

The package may hold only for a contradiction in controlling governance, an
unavailable required tool/dependency with no safe in-scope implementation, or a
required write outside the authorized repository/write set. Missing or weak
empirical evidence is not a package hold: publish the truthful lower evidence
status. Lack of authority for a favorable characterization is not permission to
invent one. A desired wepppy change is handled by the bounded handoff, not an
unauthorized cross-repository edit.

Implementation effort, review findings, test failures, or the need to revise a
schema/template remain in-envelope work and do not justify early hold.

## Truthfulness

Package artifacts label reasoned or inspected evidence `Static:` and executed
commands `Ran:`. A validator is not reported as a successful build, a build is
not reported as a scientific review, a content hash is not reported as
scientific currency, and a usersum export fragment is not reported as deployed
wepppy documentation.

## Progress

- [x] (2026-07-14) User authorized scaffolding the lifecycle/ownership package.
- [x] (2026-07-14) Applicable instruction chains and current assurance,
  usersum, release, nextest, and wepppy handoff surfaces were inspected.
- [ ] Freeze scaffold commit and final reading map at execution intake.
- [ ] Execute Phases 1 through 5.

## Decision Log

- Decision: Prove the contract with one SNOTEL snow-evidence vertical slice.
  Rationale: Ownership, templates, cross-references, and rebuild semantics are
  difficult to validate in an abstract framework and the existing strategy
  already identifies this evidence as the first suitable pilot.
  Date/author: 2026-07-14, Codex.
- Decision: Use a small Rust builder, not nextest, as the dossier DAG owner.
  Rationale: nextest schedules compiled Rust tests; it does not model arbitrary
  documentation/evidence dependencies. It remains valuable for executable
  invariant and drift tests.
  Date/author: 2026-07-14, Codex.
- Decision: Defer persistent caching, generalized provenance, and portfolio
  query features.
  Rationale: one dossier can establish deterministic build and ownership
  contracts but cannot demonstrate the recurring needs that justify a larger
  platform.
  Date/author: 2026-07-14, Codex.
- Decision: Keep downstream wepppy mutation out of scope.
  Rationale: openWEPP owns source content and an export contract; wepppy owns
  vendor registration, manifest/navigation merge, rendering, and search. The
  repositories require separate authorization and closure evidence.
  Date/author: 2026-07-14, Codex.

## Outcomes And Retrospective

Queued. Populate only during execution; do not infer results from the scaffold.
