# Testing And Gate Strategy

Status: Active

Decision authority: [ADR-0039](../decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md)

Owner: openWEPP maintainers

## 1. Purpose

This standard is the canonical operational authority for deciding which
openWEPP tests and quality checks run, when they run, what evidence they
produce, when that evidence remains current, and which broader obligations may
be carried to a campaign boundary.

The objective is fast, relevant feedback during scientific development without
weakening campaign integration or release qualification. A narrow increment
must directly prove its affected behavior. A campaign must prove that its
increments work together. A release must prove the exact deliverable and its
selected assurance material. Repeating every release-scale check after every
edit does not substitute for this separation.

This standard governs execution lifecycle. It does not redefine correct
science, test obligations, coverage thresholds, CRAP classifications, external
authority, review independence, or public assurance claims.

## 2. Authority And Precedence

The following authorities remain distinct:

| Question | Authority |
| --- | --- |
| What process behavior is correct? | Canonical `SC-*` science contract and accepted decisions |
| Which evidence can support a correctness verdict? | `docs/specifications/correctness-authority-model.md` |
| Which test families and contract obligations must be authored? | `docs/standards/rust-scientific-coding-standard.md` §7 |
| What coverage, function-floor, CRAP, and exception thresholds apply? | ADR-0021 and the module test-enhancement standard |
| Which affected gates run at which lifecycle boundary? | This standard |
| What makes a scientific report reviewed, approved, or publishable? | Scientific-assurance architecture, lifecycle, and report standard |

If a lower-level instruction, package template, profile comment, workflow, or
script disagrees with this standard about gate frequency or lifecycle, this
standard is normative under ADR-0039. The disagreement is implementation debt;
it is not permission to silently change a command during an active package.
Until the alignment package updates executable guidance, record the mismatch
and follow the highest-precedence applicable repository instruction.

`AGENTS.md` files should eventually contain short binding pointers, not copied
gate matrices. Nextest configuration, CI workflows, and release scripts
implement policy; they do not create it.

## 3. Definitions

Terms in this section are normative.

**Test case** is one executable example, property, invariant, reconstruction,
or comparison with a pass/fail or measured outcome.

**Test target** is a Cargo/libtest binary or another named executable harness
that contains one or more test cases.

**Suite** is a stable, declared collection of test cases or targets that share
an authority, domain, dependency, or operational purpose.

**Check** is a static or executable quality operation such as formatting,
Clippy, documentation lint, schema validation, cargo-deny, or CRAP analysis.

**Gate** is a required set of tests, checks, and evidence conditions whose
failure prevents a named transition. A command is not a gate until a policy or
package binds it to a transition.

**Lane** is a scheduling and failure-policy placement such as presubmit,
post-submit, periodic, campaign closure, or release.

**Increment** is one bounded implementation or documentation change with a
declared objective and write set. A work package normally delivers one
increment, although a large package may declare several inseparable steps.

**Campaign** is a declared sequence of related increments that share a base
commit, scientific or engineering objective, affected domains, integration
boundary, gate ledger, and terminal certification.

**Checkpoint** is an intermediate campaign integration event. It provides
broader feedback but does not certify campaign closure.

**Campaign closure** is the exact clean commit at which every campaign-owned
gate has direct current evidence and the campaign receives a terminal
disposition.

**Release qualification** is evidence for an exact release candidate,
configuration, binaries, data and fixture identities, assurance snapshot, and
distribution boundary.

**Affected set** is the mechanically calculated collection of packages, test
targets, explicit consumers, contracts, fixtures, authority suites, assurance
reports, and checks that can be influenced by a change set.

**Change set** is the byte-level difference between a declared base identity
and head identity. An increment may use a dirty tree digest; campaign and
release certification require a clean commit.

**Risk class** is the mechanically explained potential fan-out and consequence
of the change. It selects a minimum gate level; it is not a scientific quality
grade.

**Intent plan** is the deterministic pre-implementation declaration derived
from the authorized write set, declared surfaces, and campaign admission. It
reserves increment and campaign obligations before executable bytes change.

**Terminal gate plan** is the deterministic machine-readable declaration
derived from the exact completed change set. It reconciles the intent plan,
affected set, risk reasons, executable gate DAG, and campaign obligations before
increment closure.

**Pre-heavy closure audit** is the single machine-readable, fail-closed audit
run after the intended closure diff is assembled and before any heavy gate is
launched. It proves that cheap prerequisites, identities, inventories,
artifact locations, reuse decisions, and the exact heavy execution DAG are
ready. It is planning evidence, not closure evidence.

**Tooling defect** is a typed failure in the planner, executor, verifier,
workflow, cache, evidence lifecycle, or operator interface that causes
unnecessary work, prevents a valid governed workflow, loses audit evidence, or
requires a manual workaround. A tooling defect is repository work, not an
ambient inconvenience.

**Heavy gate node** is a gate definition carrying the machine-owned
`execution_cost_class: HEAVY`. Full workspace regression, global
coverage/CRAP, broad Clippy or deny, comparator and parity suites, release
gates, and population or cohort batches must carry that class. Timing history
may propose a policy change but does not dynamically relabel a node.

**Gate receipt** is the immutable machine-readable result of executing one
gate plan against identified inputs.

**Current evidence** is a passing receipt whose complete bound input identity
still matches the target tree and policy.

**Stale evidence** is an earlier result with at least one changed bound input.
Stale does not mean failed; it means the result cannot prove the new target.

**Deferred obligation** is a required gate assigned by an accepted intent plan
to a later named campaign boundary. It has an owner, reason, and trigger. It
remains visible and unresolved. A terminal-plan discovery is not retroactively
called deferred.

**Waiver** is explicit authority to proceed without a normally required gate.
Deferral is not waiver. This standard creates no new waiver authority.

**Escalation** adds gates or moves execution to a broader boundary because of
risk or uncertainty.

**Downgrade** removes a mechanically required gate. Operators and agents may
escalate freely but may not downgrade without an accepted policy change or
existing named exception authority.

**Full workspace regression** means all deterministic workspace test targets
admitted to the canonical full profile. External empirical cohorts, stability
populations, manual authority suites, release assembly, and assurance
publication are separate gates even when a release runner invokes them
together.

## 4. Principles

1. Test the affected behavior immediately; test the integrated system at the
   integration boundary.
2. Match gate cost to risk, dependency fan-out, and consequence of error—not to
   diff line count or operator identity.
3. Select mechanically and explain every selection. Agent analysis may explain
   or challenge a plan but does not silently narrow it.
4. Unknown production impact fails conservatively through escalation and a
   missing-map finding.
5. A passing narrow gate claims only its named affected surface.
6. Campaign and release certification bind exact clean identities. Approximate
   recency, timestamps, or “nothing important changed” are not evidence.
7. Reuse identical evidence. Do not rerun an unchanged gate merely because a
   review artifact, package narrative, or unrelated document changed.
8. Keep correctness authority separate from execution frequency. A periodic
   empirical suite can be scientifically stronger than an every-edit unit test.
9. Preserve failure localization. Deferring every test until campaign closure
   is non-conforming.
10. Preserve auditability. Every deferred, stale, skipped, failed, or escalated
    gate has a machine-readable reason.
11. Optimize the gate system for timely evidence. A discovered tooling flaw is
    corrected at its owning layer; repeatedly paying its cost is not an
    acceptable operating procedure.
12. Close workflow gaps with enforceable tooling. Narrative reminders may
    bridge a transition, but a repeated or mechanically detectable failure
    requires a validator, planner rule, executor guard, or workflow control.

## 5. Test And Check Families

The families describe purpose, not a strength ladder.

| Family | Purpose | Typical examples | Normal earliest boundary |
| --- | --- | --- | --- |
| Source quality | Reject malformed or nonconforming source | Rustfmt, Clippy, Markdown lint, schema validation | Increment |
| Component behavior | Prove local calculations, guards, state transitions, and properties | Unit and property tests | Edit loop and increment |
| Contract obligation | Bind `SC-*` invariants and A–H test-vector obligations | Contract-derived tests, unit/guard vectors | Increment |
| Integration and consumer | Prove orchestration, serialization, restart, downstream reads, and publication values | Integration tests, CLI fixtures, real-consumer tests | Increment when affected; otherwise checkpoint |
| Conservation and reconstruction | Independently reconstruct mass, water, sediment, or energy behavior | Operand reconstruction, closure audit, rejected-formula tests | Increment when affected and campaign closure |
| Comparator and migration | Detect or classify differences against a pinned implementation or independent calculation | Legacy comparator, analytical recurrence, independent solver | Affected increment or campaign |
| Constitutive correctness authority | Exercise every applicable A3 externally governed constitutive suite for a touched process family | Required authority suite and pinned fixture cohort | Increment when applicable; non-deferrable |
| Empirical and independent authority | Evaluate against observations or independent calculations beyond constitutive closure | A4 empirical cohorts, A5 independent solvers, SNOTEL, frost-tube | Domain checkpoint, periodic, campaign, or release as declared |
| System and stability | Exercise large populations, concurrency, binaries, manifests, and failure recovery | Stability cohort, watchlist, multi-worker CLI | Critical increment, campaign, or release |
| Coverage and complexity risk | Measure exercised eligible regions and change risk | LLVM coverage, cargo-crap | Affected increment; global campaign/release |
| Assurance and publication | Plan report impact, reproduce selected evidence, render, approve, transfer, and publish | Assurance plan/build/check/publish/verify-release | Impact planning during campaign; realization and publication at closure/release |

A suite must declare its family, owner, dependencies, expected duration class,
failure policy, and minimum applicable boundary. Suites that mix fast contract
tests and multi-minute external workflows should be split unless the test
semantics require inseparable execution.

### 5.1 Execution and scientific outcome are separate axes

Every authority suite declares `authority_class` (`A0` through `A6`) and an
`outcome_policy`. Execution integrity is one of `PASS`, `PASS_WITH_RETRY`,
`FAIL`, `BLOCKED`, or `INVALID`. A successfully executed scientific or
comparator evaluation separately records `CONFORMS`, `DIVERGES`,
`INCONCLUSIVE`, or `NOT_EVALUATED` plus its quantitative result. Content
integrity and complete expected inventory are prerequisites for any scientific
outcome; an incomplete or crashed suite is an execution failure, not scientific
divergence.

Outcome reduction is exhaustive:

In this table, **accepted execution** means `PASS`, or `PASS_WITH_RETRY` only
when every failed attempt was classified as infrastructure-only by a
prospectively declared closed retry policy. A semantic/scientific failure can
never be retried into accepted execution. Accepted `PASS_WITH_RETRY` retains
all attempts and opens mandatory owned flake/infrastructure debt; boundaries
and reuse may accept it only when the same policy version explicitly permits.

| Authority | Accepted outcome | Other outcome |
| --- | --- | --- |
| A0 | `ADMITTED` from the authority check | Missing, ambiguous, provisional, or stale is `BLOCKED`; no scientific outcome axis applies |
| A1/A3 | Accepted execution and scientific `CONFORMS` | `DIVERGES`, `INCONCLUSIVE`, `NOT_EVALUATED`, semantic retry, or unaccepted execution blocks |
| A2/A6 | Accepted execution plus `CONFORMS`, `DIVERGES`, or `INCONCLUSIVE` | `DIVERGES` or `INCONCLUSIVE` opens investigation; `NOT_EVALUATED` or unaccepted execution does not satisfy a selected suite |
| A4/A5 unpromoted | Accepted execution plus `CONFORMS`, `DIVERGES`, or `INCONCLUSIVE` | `DIVERGES` or `INCONCLUSIVE` opens investigation; `NOT_EVALUATED` or unaccepted execution does not satisfy a selected suite |
| A4/A5 promoted | Accepted execution and the plan-declared accepted scientific predicate | Any other scientific or execution outcome blocks |

`NOT_EVALUATED` is valid only for a plan record proving a suite was not selected
or not applicable; it never satisfies a selected required execution.
A2/A4/A5/A6 investigation disposition is one of `PENDING`,
`ACCEPTED_SIGNAL`, `DEFECT_OPEN`, `SUPERSEDED`, or `RESOLVED`, using the
correctness-authority verdict vocabulary where it applies. These investigations
remain visible but do not block increment, campaign, or release transitions
unless a plan-bound policy version explicitly promotes that authority suite to
blocking. Promotion is declared before execution and cannot be inferred after
seeing a result.

## 6. Lifecycle Levels

### 6.1 Edit loop

The edit loop is optional developer feedback, not closure evidence unless its
result is captured in an increment receipt. Use the smallest test or check that
can falsify the current edit. Typical operations are one unit test, one test
target, formatter, package check, or a deterministic analysis script.

An edit-loop pass does not authorize skipping increment closure.

### 6.2 Increment closure

Every increment must pass a mechanically generated increment plan. For a
bounded Rust production change, the minimum plan contains:

- formatting for the affected language;
- Clippy with warnings denied for affected packages and mechanically selected
  reverse-dependent packages;
- component and contract-obligation tests owned by the changed surface;
- a non-deferrable A0 authority-admission check for every affected kernel,
  process, and public process behavior: each surface must map uniquely to a
  current canonical `SC-*` contract version, index entry, and obligation
  binding; missing, ambiguous, provisional, or stale authority produces
  `BLOCKED`/`HOLD`, and broader testing cannot convert that condition to pass;
- every applicable A1 hard-invariant gate and A3 constitutive-authority suite
  for each affected process family, derived from the correctness-authority
  registry; an incomplete or ambiguous binding blocks the increment;
- affected integration and real-consumer tests from explicit mappings;
- affected negative, error-precedence, chronology, restart, serialization,
  conservation, and publication tests where those behaviors can change;
- targeted coverage/CRAP evidence for the changed eligible production surface;
- affected doctests when public items, examples, feature behavior, or rustdoc
  inputs can change, with planned and executed doctest inventory recorded;
- the repository placeholder/stub-pattern scan when production Rust, tests,
  contracts, or allowed-pattern policy changes;
- required documentation or schema checks; and
- an assurance impact plan when an assurance dependency or semantic watch is
  affected.

`cargo deny check` is an increment gate when a Cargo manifest, lock file,
dependency policy, license policy, source policy, or toolchain dependency
surface changes. It remains a campaign and release gate regardless.

An ordinary bounded increment does not require full workspace regression or
fresh global CRAP. It closes as `PASS-INCREMENT` only after every increment
gate passes and every campaign-owned obligation is already declared in the
campaign ledger.

Documentation-only increments run affected documentation, reference, schema,
catalog, and generated-drift checks. They do not run Rust gates solely because
the repository contains Rust.

### 6.3 Campaign checkpoint

A checkpoint runs after a meaningful integration seam, before a destructive or
hard-to-debug transition, when accumulated increment debt exceeds campaign
policy, or at an operator-selected cadence. Its normal plan contains:

- the fast workspace profile;
- affected domain profiles;
- accumulated cross-increment integration and consumer tests;
- affected comparators or empirical suites whose feedback is useful before
  closure;
- campaign gate-ledger validation; and
- assurance impact status without report rebuild unless explicitly requested.

A checkpoint may run full workspace regression, but a checkpoint pass does not
replace terminal campaign certification unless it binds the eventual exact
closure head and all other closure inputs remain identical.

### 6.4 Campaign closure

Campaign closure requires a clean exact commit and:

- workspace formatting and Clippy with warnings denied;
- full workspace regression;
- full workspace doctests and the repository placeholder/stub-pattern scan;
- cargo-deny;
- current full-workspace coverage and adjudicated CRAP for the exact closure
  execution root;
- every campaign-owned contract, consumer, conservation, comparator,
  empirical, external-authority, and stability gate named by the campaign;
- disposition of all failed, stale, blocked, deferred, and unmapped items;
- affected assurance scientific-impact disposition and required reproduction;
- dual review and verification when required by work-package governance; and
- one campaign certification receipt binding the complete evidence set.

Campaign closure fails if any deferred campaign obligation remains unresolved.
The closure receipt may incorporate still-current increment or checkpoint
receipts instead of rerunning them. Full workspace and global CRAP evidence
must be current for the exact closure execution root.

### 6.5 Release qualification

Release qualification requires an exact candidate and configuration. It
consumes a current campaign certification or reruns every invalidated portion,
then adds:

- release binary construction and provenance;
- sidecar, manifest, schema, packaging, and lint checks;
- required authority lanes and selected periodic/manual lanes;
- release stability and population cohorts;
- exact assurance realization transfer, snapshot, public catalog, and
  publication checks for the mechanically derived release inclusion set; and
- distribution or downstream-vendoring gates in release scope.

A campaign receipt is reusable at release only when its release-accepted trust
and `HERMETIC_CONTENT` reuse contract verifies and the release source,
toolchain, build configuration, fixtures, gate policy, and relevant platform
identity match. Release-only gates always remain release-only obligations.

## 7. Risk Classification And Escalation

The planner assigns the highest applicable class and emits every reason.

### 7.1 Editorial

Editorial changes affect only prose, comments, non-normative examples, or
catalog metadata and cannot change executable, contract, schema, review,
publication, or gate meaning. They require documentation checks only.

Changing a normative contract sentence, command, gate rule, fixture identity,
review authority, or generated digest is not editorial.

### 7.2 Bounded component

A bounded change affects one component with explicit interfaces and limited
reverse dependency fan-out. Examples include a mechanically isolated new
process crate that has no production consumer or a private helper correction
with complete component and contract tests.

An additive workspace member remains bounded only when the planner proves all
of the following and emits reason code `ISOLATED_WORKSPACE_MEMBER_ADDED`:

- the member is new and no existing production package has an incoming edge to
  it;
- existing package dependency resolution and enabled feature sets are
  unchanged;
- workspace defaults, shared dependency declarations, profiles, compiler and
  coverage flags, and toolchain are unchanged;
- test discovery or admission for existing packages is unchanged;
- the new member has explicit impact-map ownership and complete A1/A3 binding;
  and
- `cargo deny check` passes for the resulting graph.

Failure to prove any condition emits
`WORKSPACE_RESOLUTION_OR_BEHAVIOR_CHANGED` or
`WORKSPACE_ISOLATION_UNPROVEN` and escalates to critical.

The increment plan plus targeted coverage/CRAP is normally sufficient.

### 7.3 Integrated domain

An integrated-domain change affects an active production consumer, multiple
packages within one process domain, runner handoff, restart boundary, or domain
publication. It requires the increment plan plus affected domain, integration,
consumer, comparator, and conservation gates. A campaign checkpoint is normally
required before the next integration seam.

### 7.4 Critical

A critical change requires immediate campaign-closure-strength workspace
regression and global CRAP in addition to specialized gates. Critical triggers
include:

- shared numerical primitives, calendars, chronology, state layout, restart,
  or cross-domain orchestration;
- production activation, default-selector change, compatibility deletion,
  cutover, or public output semantic change;
- conservation formula or operand lineage changes;
- serialization, schema, manifest, release-sidecar, or externally consumed API
  changes with broad fan-out;
- `unsafe` code or security-boundary changes;
- Rust toolchain, existing-package dependency resolution, non-isolated
  workspace membership, global feature, compiler flag, coverage configuration,
  or gate-runner changes;
- deletion, disabling, renaming, filtering, reclassification, or weakening of
  tests, fixtures, required cases, anti-evasion checks, or authority lanes;
- changes to the CRAP exception registry or production-surface filter;
- cross-domain changes that cannot be isolated by the dependency map; and
- any unknown or ambiguous executable path whose impact cannot be bounded.

Line count alone never determines risk. A one-line selector change may be
critical; a large isolated test-data addition may not be.

## 8. Mechanical Impact Planning

### 8.1 Required inputs

The future planner must read, without agent interpretation:

1. base and head Git identities, or a base plus dirty-tree digest for an
   increment;
2. changed paths and add/delete/modify/type-change kinds, with a rename
   represented only as delete plus add;
3. `cargo metadata --format-version 1` workspace and dependency graph;
4. a versioned explicit impact map for non-Cargo relationships;
5. science-contract obligation-to-test bindings;
6. external-authority suite registry and fixture provenance;
7. Nextest profiles, filters, overrides, and test inventory;
8. coverage/CRAP production-surface and exception inputs;
9. assurance report dependencies and semantic watches;
10. CI/release configuration relevant to the requested boundary; and
11. this policy's stable identifier and content digest.

The explicit impact map must cover integration tests that read files by path,
CLI binaries, generated schemas, fixtures, contract documents, scripts,
publication surfaces, and semantic process relationships Cargo cannot infer.

### 8.1.1 Canonical Git change set

Committed planning compares exact trees with Git raw, NUL-delimited,
rename-disabled records equivalent to
`git diff-tree --raw -z --no-renames --no-commit-id -r <base> <head>`.
Renames are always delete plus add. Dirty planning separately records
base-to-index (`git diff-index --cached --raw -z --no-renames <base>`),
index-to-worktree (`git diff-files --raw -z --no-renames`), and untracked files
(`git ls-files --others --exclude-standard -z`). The executed source is the
working-tree manifest, not an implicit choice between index and worktree.

File modes and object kinds are part of each record. Unmerged index entries,
intent-to-add, unsupported submodules/sparse state, and non-UTF-8 paths block
planning until normalized or governed by a later schema. Ignored content is not
silently admitted; hermetic execution must make it unreadable or an explicit
manifest input. Git binary/version and configuration are bound, external diff
drivers are disabled, and user configuration cannot alter the change set.

### 8.1.2 Canonical Cargo dependency graph

Base and head graphs are generated in isolated snapshots with the pinned Cargo
and Rust toolchain using `cargo metadata --format-version 1 --locked --offline`
for every versioned release-supported target and feature configuration. The
planner supplies sanitized Cargo configuration and explicitly enumerates
default, optional, and all supported feature sets. Normal, build, proc-macro,
and dev dependency kinds are included wherever their targets/tests can execute;
target conditions are evaluated for the declared matrix. Impact is the union
across that matrix unless a narrower configuration is itself versioned and
bound to a non-release boundary.

The planner normalizes package identity, dependency kind, target condition,
enabled features, and directed edge ordering; it binds that normalized graph,
not raw Cargo JSON serialization. A lockfile or offline source unavailable in
either snapshot blocks planning. Resolver version, workspace/default members,
Cargo config, registry/source replacement, and target matrix are authority-root
inputs. The isolated-workspace-member proof compares these normalized base and
head graphs.

### 8.2 Selection algorithm

Planning has two required stages. Before edits, the planner creates an intent
plan from the work package's authorized write set, declared process and public
surfaces, intended Cargo edges, and campaign admission. Before increment
closure, it creates a terminal plan from the exact diff and reconciles it with
that intent. An absent intent plan blocks implementation admission; it does not
authorize the terminal planner to invent a low-risk history.

At each stage the planner performs these steps in stable order:

1. classify every changed path by owner and semantic surface;
2. map Rust paths to owning packages and calculate reverse transitive workspace
   dependencies;
3. add explicit tests, consumers, contracts, fixtures, authority suites, and
   assurance reports from the impact map;
4. detect test/gate deletions and configuration changes before normal risk
   classification;
5. assign all applicable risk reasons and take the highest risk class;
6. expand the minimum gate set for that risk and lifecycle boundary;
7. add package- or campaign-declared specialized gates;
8. reconcile campaign-owned obligations with the admitted ledger version;
9. reject unmapped executable or normative paths through conservative
   escalation; and
10. emit deterministic human and JSON plans with stable ordering.

Selection is monotonic: adding a changed path or dependency cannot remove a
previously selected gate unless a versioned policy or impact-map change
explicitly explains the removal and itself receives critical treatment.

If the terminal plan discovers an increment-scope obligation, that gate remains
`PENDING` and must pass before increment closure. If it discovers an inherently
campaign-boundary obligation that could not reasonably be identified from the
authorized write set, the campaign owner may accept a ledger amendment before
increment closure. The amendment records discovery identity and rationale and
cannot reclassify a failed, attempted, or increment-required gate. There is no
retroactive `DEFERRED` label.

### 8.3 Required plan and gate-node fields

The JSON gate plan must contain at least:

- schema and policy identifiers and digests, planning stage, predecessor intent
  plan when terminal, deterministic `plan_id`, and pre-execution
  `execution_key`;
- requested boundary and campaign ID when applicable;
- base, head, and dirty-tree identities;
- changed paths with change kind and owner;
- affected packages and reverse dependencies;
- explicit non-Cargo dependency edges used;
- risk class and complete reason codes;
- selected gate nodes and their prerequisite edges in stable topological order;
- each authority class, outcome policy, blocking promotion, and investigation
  ownership rule;
- required environment, fixture, tool, and configuration identities;
- deferred campaign obligations with owner, trigger, and rationale;
- assurance impacts, target identities, request axes, and currency axes;
- unmapped inputs and escalation result; and
- required output locations and artifact identities that are knowable before
  execution.

`plan_id` is the SHA-256 digest of the canonical plan payload excluding the
derived `plan_id`, `execution_key`, receipt fields, and all runtime output.
`execution_key` is then the SHA-256 digest of `plan_id`, all pre-execution bound
roots, and the permitted execution-environment projection. `receipt_id` is
created only after execution as the SHA-256 of the canonical receipt payload
excluding the derived `receipt_id`; it additionally binds results and artifact
digests.

Every gate node contains:

- stable versioned `gate_definition_id`; unique content-derived `node_id`
  binding parameters, target, feature set, matrix coordinates, shard, and retry
  policy; gate family, authority class, outcome policy, owner, boundary, trust
  requirement, and reuse class;
- executor kind and version;
- argument array, repository-relative working directory, and explicit
  environment-variable allowlist;
- prerequisite node IDs;
- expected test or check inventory and cardinality rule;
- machine-evaluable acceptance rule;
- timeout, permitted retry count and reasons, and failure classification;
- required artifact contract and output paths;
- the lifecycle transition blocked by failure; and
- platform and environment fields that break identity for this gate.

Prerequisites reference `node_id`. Node IDs and artifact namespaces are unique;
the graph must be acyclic; matrix and shard expansion is stable and complete;
and every prerequisite must resolve before execution. Duplicate node/output
identity or a cycle is `INVALID`.

`node_id` is the SHA-256 of the complete canonical gate-node payload excluding
the derived `node_id` and runtime attempt/output fields. It therefore binds the
gate definition, all parameters and matrix/shard coordinates, arguments,
environment, prerequisites, acceptance policy, retry policy, artifacts,
boundary, trust/reuse class, and identity-breaking platform fields.

Acceptance rules use a closed, versioned algebra: exit-code equality; exact,
subset, or superset inventory; count equality/range; typed numeric threshold;
required artifact presence and digest; schema validity; and registered typed
comparator predicate. Boolean `all`/`any` may combine those atoms. Arbitrary
code, shell interpolation, dynamic file reads, or implementation-defined
expressions are forbidden. Each executor kind defines one normalized mapping
from process/test output to these predicates.

Executor kinds are closed and versioned. Direct process executors use argument
arrays. A legacy shell workflow may run only through a named adapter whose
script bytes, interpreter, environment projection, and interface schema are
bound into the execution root; arbitrary shell strings are invalid.

Aggregate execution status uses this precedence: `INVALID` when plan, identity,
or inventory integrity fails; otherwise `FAIL` when any required gate fails;
otherwise `BLOCKED` when a prerequisite or declared external requirement
prevents completion; otherwise `PASS_WITH_RETRY` when a policy-permitted retry
was needed; otherwise `PASS`. The aggregate also retains every derivative
blocked node. A deterministic A0/A1/A3 or anti-evasion gate cannot convert an
observed semantic failure into pass through retry; infrastructure retries may
produce `PASS_WITH_RETRY` and open owned flake debt. Configured skips and every
attempt remain visible.

Human output is a rendering of the JSON plan, never a separately authored
decision.

### 8.4 Operator controls

An operator may request additional suites, a broader risk class, or a broader
boundary. The resulting plan records the escalation. There is no general
`--skip`, `--bless`, `--accept-current`, or agent-decided downgrade operation.
Existing explicit exception authorities remain narrow and content-bound.

### 8.5 Pre-heavy closure audit

Before any selected heavy node starts, one canonical pre-heavy closure audit
must evaluate the complete intended closure state. Heavy nodes include full
workspace regression, global coverage/CRAP, broad Clippy or deny, comparator
and parity suites, release gates, and population or cohort batches. The audit
must produce one versioned report consumed unchanged by the executor and
terminal verifier. It must check:

1. the package exists in the authenticated base commit, its write-set schema is
   valid, and its declared plus intended closure paths cover the exact Git
   change set; a scaffold-only validator may prepare this admission but cannot
   authorize execution before the scaffold commit exists;
2. cheap prerequisites, including diff hygiene, documentation and schema lint,
   required artifact presence, prompt state, and line-count governance;
3. one canonical admitted test/check inventory, argument vector, stable
   ordering, and expected cardinality consumed by execution; verification must
   independently enumerate the current inventory and compare it to the admitted
   inventory rather than trusting or replacing it;
4. toolchain, environment, fixture, policy, binary, feature, and configuration
   identities needed by every selected node;
5. a fresh immutable attempt root, collision-free output namespaces, and cache
   keys that cannot expose or reuse mutable source, index, or measurement
   state;
6. execution-, authority-, and documentation-root separation plus every
   evidence-reuse decision and its invalidation reason;
7. whether a proven instrumented execution can satisfy both full-regression
   and global-coverage obligations without duplicate test execution;
8. prerequisite ordering, timeout and retry policy, concurrency ownership, and
   the exact heavy-runner handoff;
9. persistent append-only attempt, timing, cost, and failure records outside an
   ephemeral-only directory; and
10. every open tooling defect that can invalidate, duplicate, or materially
    delay the selected execution.

The report status is `READY`, `BLOCKED`, or `INVALID`. Only `READY` from the
repository-owned command authorizes heavy execution. Until that command is cut
over, no new heavy package may start; the implementation package for the
command must finish the tool under focused checks and then use it for its own
heavy closure.

Execution is a mandatory two-stage state machine. The executor first runs only
`LIGHT` prerequisite nodes, freezes their receipts and the intended closure
state, and obtains the audit decision. It may enter the `HEAVY` stage only with
the exact `READY` audit ID. A monolithic loop that can reach a heavy node before
this transition is invalid. After any late failure, a new attempt imports every
successful per-node receipt that is both current and reusable in the target
attempt under §10.4, then runs only missing, invalidated, or context-ineligible
nodes. Restarting a reusable successful prefix is forbidden. Every receipt not
reusable in the target attempt records the exact trust, reuse-class, or
execution-context reason that requires rerun, including `SAME_EXECUTION` after
a runner, job, or authenticated workflow-attempt change.

A newly discovered tooling defect is recorded with owner, reproducer, impact,
and correction boundary. If correction is inside the active package write set,
fix and verify it before retry. Otherwise stop, retain the failed attempt, and
open or activate a prerequisite tooling package. After one infrastructure-only
workaround, recurrence of the same cause blocks another expensive retry until
the owning tooling defect is corrected or explicit authority accepts a bounded
external outage. Human memory and package prose are not substitutes for this
enforcement.

Checkpoint and recovery records are diagnostic until an independently accepted
aggregate receipt or protected-CI provenance envelope authenticates the exact
node attempt, artifacts, non-documentation roots, execution context, and
claims. A self-hash or predecessor chain proves integrity only; it does not
establish authority. Durable restore must reject symlinks, unindexed bytes,
invalid ledger chains, and workflow or run provenance drift before installing
bytes at their stable paths.

If publication fails before one exact recovery root can receive protected-CI
provenance, that root may be excluded from future resume discovery only by a
later append-only `CLOSED` tooling-defect record. The closure must bind the
exact safe child path inside the durable recovery namespace to an earlier
failed HEAVY record with the same cause, a lowercase 40-character correction
commit that resolves in the current repository ancestry, and nonblank review
evidence. The latest prior state for the same defect and cause must be `OPEN`.
The ledger path and every ancestor must pass no-follow validation
before append. Reopening the same defect revokes the exclusion. Malformed
lifecycle states, deleting the failed record, or broadly ignoring unattested
roots fail closed.

Once the caller-selected durable ledger is admissible, HEAVY records `STARTED`
before audit validation, resume admission, executable checks, or subprocess
preparation, and records exactly one terminal outcome for every such start.
Representable pre-heavy failures still emit the versioned ten-check report:
identity or authority substitution is `INVALID`, while an unavailable external
prerequisite is `BLOCKED`, with the failure assigned to its owning check. Rust
and non-Rust ledger producers must share the same canonical JSON byte contract.

A combined full-regression and LCOV/CRAP node is selectable only by a
repository-reviewed proof record. The executor recomputes its decision from
exactly three compatible protected-CI baselines, exact functional inventory and
result parity, complete JUnit, LCOV, and CRAP lineage, and both economy limits:
combined median time is at most 120 percent of coverage-only median time and at
most 80 percent of the summed full plus coverage medians. Missing, stale,
unpinned, incomplete, or uneconomic proof retains separate nodes with a typed
non-adoption reason.

Nested coverage/CRAP subprocesses are executor children, not independent
launchers: they must consume the executor-injected qualified Nextest
configuration and short process `TMPDIR`. They may not regenerate scheduling,
resource, or temporary-root contracts from repository defaults. Signal or
cleanup termination must be represented as a nonzero failed run status.

The audit is the single independent inventory verifier for the LIGHT-to-HEAVY transition. LIGHT execution may reconstruct the terminal plan once; the audit then independently reconstructs current policy, canonical arguments, and exact inventory once in the same confined attempt workspace. A READY audit binds that result. HEAVY consumes the READY result and must not repeat the same plan or inventory enumeration unless source, policy, execution context, or another identity breaker changed. Executor preflight retains non-inventory safety checks. Repeating enumeration at LIGHT preflight, audit, and HEAVY admission is a tooling defect, not extra assurance.

For documentation, LIGHT runs the exact sorted, deduplicated, non-deleted changed-path `markdown-doc lint --path ...` node. The audit validates that canonical scope, its PASS result, checkpoint, and artifact identity; it does not launch a second lint and may never substitute an unscoped repository-wide lint.

The audit also binds the durable ledger head after orphan reconciliation and before new HEAVY admission. HEAVY appends `STARTED` first, then requires the current ledger to be exactly the audited prefix followed by that one plan-, audit-, artifact-, and claim-bound STARTED record. It validates chain integrity and open-defect posture against the current ledger without rebuilding mutable audit evidence. Any intervening append, wrong predecessor, or claim drift invalidates admission while preserving the balanced STARTED/FAILED lifecycle.

#### 8.5.1 Exact reuse boundaries

The audit reconstruction compares the complete canonical terminal plan digest,
not only node labels, arguments, or inventory. This binds source selection,
policy, execution context, configuration, fixtures, tools, roots, and every node
field to the `READY` decision. Immediately before spawning HEAVY, the executor
recomputes all cheap execution-context identity breakers and rejects drift. It
does not enumerate test inventory again.

After HEAVY, the local executor validates the complete receipt, source roots,
live tool and environment identities, audit, DAG, artifact bytes, executed
inventory, summary, and authority outcomes by consuming the audit-admitted plan
proof. It does not perform another identical terminal-plan reconstruction. An
independent verifier crossing the host or signing boundary reconstructs the
plan once from its own trusted context. Repeating reconstruction inside the
same admitted local transition is a tooling defect; independent reconstruction
at a distinct trust boundary is required assurance.

A production transition authenticates `READY` by keeping LIGHT execution, audit construction, and HEAVY admission inside one trusted binary process while persisting the unchanged audit as evidence. Standalone HEAVY admission rejects a merely self-hashed audit; an external attestation may become an additional authenticated transport only when its trust root and subject contract are explicitly implemented. This construction prevents forged PASS checks without repeating audit inventory enumeration.

Audit reconstruction must use a disposable compilation and inventory workspace
whose cache and target directories are disjoint from every execution-stage
cache. The audit removes that workspace before returning `READY`, including
after reconstruction failure. LIGHT or HEAVY must never consume binaries,
metadata, reports, or fixture paths produced from a disposable source snapshot;
such reuse is cache contamination and a blocking tooling defect.

Package admission reconstructs authority across the exact first-parent
base-to-head commit range. The operator-supplied intent package must be an
active anchor in the authenticated base; only that anchor and packages validly
scaffolded inside the range may authorize implementation and package-tree
transitions. Each transition
consumes package status and write-set bytes from its parent, treats merges
atomically against the first parent, disables rename inference, and allocates
every changed path to one unambiguous prospective authority. A scaffold may
authorize only newly added regular files in its own package directory in that
commit. A top-level `docs/work-packages/*-execplan.md` is separate planning
state: when added as a regular file with a sanctioned prospective lifecycle it
may authorize only its exact path, and later modifications consume its prior
bound status and digest. It never authorizes a sibling or implementation path,
and terminal planning state cannot self-authorize another modification. A
strictly newer prospective package may supersede that exact path. A terminal
package has no ordinary authority and shadows older broad authorities within
its own package tree; its sole closure exception is a
content-preserving move of its one Markdown prompt from `prompts/active/` to
`prompts/archived/`, with no other package-tree change. Same-commit widening,
unrelated historical packages, malformed or non-regular paths, zero authority
outside the planning-only case, and same-sequence ambiguity fail closed. The
canonical chain artifact binds ordered commit, parent, tree, package, lifecycle
status, write-set, path-allocation, planning-state status/digest/introduction,
prompt-owner, and prompt-digest identities. Intent planning retains the exact
Rust-produced artifact and terminal planning binds its chain ID; pre-HEAVY independently
reconstructs the live chain and requires exact identity. Dirty authority chains
remain invalid until a separately specified synthetic-transition contract
exists.

The trusted push controller obtains the operator-supplied anchor from exactly
one case-sensitive `TESTGATE-Intent-Package` trailer on the exact pushed head
commit. Trusted manual dispatch instead requires one explicit
`intent_package` input. Missing, duplicate, malformed, or event-inconsistent
declarations fail before planning; the controller passes the exact resolved
path to `testgate.py --intent-package` and never infers an anchor from multiple
changed package candidates. A push may recover work omitted after an earlier
pre-planning failure through at most one exact-head
`TESTGATE-Comparison-Base` trailer. Its value must be a lowercase full commit
ID that is an ancestor of the push event's `before` commit, so it can only
expand the comparison backward and never exclude event changes. Execution and
hosted verification independently resolve the same base.

Process temporary roots must be isolated per node attempt, use a platform-safe path-length budget for path-sensitive APIs such as Unix-domain sockets, and be removed after both successful and failed execution. Cleanup failure is a typed gate failure. The executor may derive a stricter resource schedule from the plan-bound canonical Nextest configuration when retained timing evidence proves the canonical concurrency cap unsafe on the runner; it must fail if the expected source configuration drifts, retain the test inventory and timeout, and record the qualified cap. A recurrence at two-way fixture concurrency requires a serial qualification before another expensive retry; raising the timeout is not the first correction.

Attempt archives retain plans, audits, receipts, durable ledger snapshots, node logs, checkpoints, reports, and declared output artifacts. Before indexing or upload, the finalizer removes explicitly named disposable compiler targets, reconstruction workspaces, and process-temp trees with no-follow path checks. Disposable cache bytes are neither evidence nor recovery state and must not make finalization scale with build-cache size.
## 9. Execution Architecture

The target architecture is:

```text
change identities
      |
      v
impact planner -----> deterministic gate plan
      |                       |
      |                       v
      |                gate executors
      |                 /    |     \
      |          Nextest  checks  domain tools
      |                 \    |     /
      |                       v
      +-------------- gate receipts
                              |
                  campaign gate ledger
                              |
                 campaign certification
                              |
                  release qualification
```

The planner owns selection. Executors run exactly the admitted plan and cannot
reduce it. Receipt verification owns currency. The campaign ledger aggregates
obligations and results. Release tooling consumes certification and adds
release-specific work.

Nextest executes Rust test targets, filtersets, retries permitted by policy,
test groups, and shards. It does not decide manuscript, fixture, contract,
result, publication, or semantic process dependencies.

## 10. Gate Receipts And Evidence Reuse

### 10.1 Identity layers

Receipts bind separate roots so unrelated documentation does not invalidate
executable evidence. Each root is a canonical versioned manifest over the
plan's complete transitive input closure, not a list copied from the receipt.
The planner calculates that closure; receipt verification independently
recalculates it from the target tree and current authority before comparing
manifests.

Every manifest record contains a normalized repository-relative UTF-8 path,
object kind, executable/mode identity, content SHA-256, semantic role, and
owning gate or authority. Symlinks are recorded without following them and bind
their normalized link target. Paths sort by UTF-8 byte order; duplicate and
non-normalized paths are invalid. Regular-file contents include tracked and
untracked files admitted by the plan. Missing, renamed, and deleted inputs are
represented in the change set, not omitted. Git submodules are unsupported for
an execution closure unless a gate schema explicitly records the gitlink and
recursively certified commit; otherwise they block planning.

Every identity-bearing JSON payload uses UTF-8 JSON with I-JSON constraints and
RFC 8785 canonicalization before SHA-256. This rule covers plans, manifests and
aggregate roots, execution-key payloads, receipts, attestation envelopes,
ledgers, and assurance impact entries. Each derived ID field is excluded from
its own digest payload;
any referenced predecessor or input ID remains included. The aggregate root is
the SHA-256 of the root kind, schema version, and canonical ordered record set.
A dirty-tree identity binds the base commit, index state, working-tree state,
admitted untracked files, and file metadata above, so later commit equivalence
can be proven byte-for-byte.

**Execution root** covers the selected gates' complete transitive production
source, tests, fixtures, scripts, manifests, generated inputs, Cargo resolution,
compiler/toolchain, features, gate configuration, environment projection, and
executor definitions. It is neither the whole repository by default nor only
the initially changed paths.

**Authority root** covers applicable `SC-*` obligations, correctness-authority
registry entries, thresholds, exception records, and impact-map rules.

**Documentation root** covers package narratives, review findings,
dispositions, and explanatory evidence that do not influence execution.

**Assurance root** covers report source, declared dependencies, methods,
results, review events, builder identity, and release-transfer inputs.

A change invalidates only receipts that bind the changed root. A normative
contract or gate-policy change belongs to the authority root, not merely the
documentation root.

Each gate family declares its identity-breaking environment projection.
Compiler and target triples, toolchain components, enabled features, relevant
environment variables, and executor versions always break Rust functional,
doctest, coverage, and CRAP identity. OS, architecture, libc/runtime, locale,
timezone, filesystem behavior, and external-tool versions break identity when
the gate can observe them. Pure schema or documentation checks may declare a
narrower projection, but an undeclared environment dependency invalidates the
receipt and opens an impact-map defect.

### 10.2 Required receipt fields

Every receipt contains:

- schema version and receipt ID;
- plan ID, exact plan digest, and execution key;
- boundary, campaign, base, head, and tree identities;
- execution and authority roots, plus applicable assurance root;
- exact gate-node DAG and executed argument arrays or named legacy-adapter
  identity;
- selected and actually executed test inventory;
- tool, compiler, platform, feature, and configuration identities;
- start, finish, exit status, pass/fail/skip counts, and retry details;
- authority class, execution integrity, scientific outcome, outcome-policy
  generation, and any investigation record ID;
- claimed execution principal/context, repository, source event/ref,
  workflow/job, runner/image, and attempt; these claims are unauthenticated
  until enclosed as described below;
- hashes of JUnit, LCOV, CRAP, comparator, reconstruction, or other evidence;
- skipped or unavailable items with policy reason;
- source mutation checks for gates that require a frozen tree; and
- final execution result: `PASS`, `PASS_WITH_RETRY`, `FAIL`, `BLOCKED`, or
  `INVALID`.

A skipped test is not a pass. Its suite declaration must say whether the skip is
configured and acceptable for the boundary.

### 10.3 Trust classes and authorization

Content addressing proves integrity, not who executed or authorized evidence.
Receipts use these closed trust classes:

- `LOCAL_UNTRUSTED`: local feedback only; it cannot close an increment,
  campaign, or release boundary;
- `REPOSITORY_REVIEWED`: bound to an authenticated repository event and exact
  reviewed source; it may close an increment when repository rules accept its
  issuer and workflow; and
- `PROTECTED_CI`: issued by the protected, pinned CI workflow/runner identity
  with verifiable provenance; required for campaign certification and release.

Receipt authentication is a non-circular two-layer construction:

1. canonicalize the immutable unsigned receipt payload without any attestation
   locator/digest and derive `receipt_id`;
2. issue a signed/attested envelope whose subject is that exact `receipt_id`
   and receipt digest and whose subjects also bind every referenced artifact;
3. canonicalize the envelope excluding its derived `envelope_id`, then derive
   `envelope_id`; and
4. verify that the envelope subject equals an independently recomputed receipt
   before assigning trust class.

The ledger and certificate consume `envelope_id` plus `receipt_id`, never an
unauthenticated receipt alone. A locator is storage metadata outside the
receipt identity. Missing, mismatched, recursive, or multiply inconsistent
envelopes are `INVALID`.

For normal GitHub increment execution, a native GitHub artifact-attestation
bundle is the repository-reviewed envelope when all of these checks pass: its
subject is the exact `receipt.json` bytes; its custom predicate type is
`https://openwepp.org/attestations/testgate/v1`; the predicate repeats the
receipt digest, plan/execution identity, base/head, and pre-edit package
authorization digest; and verification constrains repository, source ref,
source digest, and the pinned TESTGATE workflow identity. The unsigned receipt
truthfully remains `LOCAL_UNTRUSTED`; only the verified bundle plus receipt is
`REPOSITORY_REVIEWED`. Merely uploading either file cannot upgrade trust.
Candidate checkout, builds, intent reconciliation, plan reconstruction, and
independent Nextest/A3 inventory enumeration run in a tokenless hosted
verification job. The OIDC/attestation-enabled aggregate consumes only that
job's immutable artifact, runs no candidate code, fails closed unless execution
and verification both succeeded, and performs the minimal subject recheck,
attestation, and native verification sequence.

The protected TESTGATE architecture separates execution from verification and
attestation:

- `execute-increment`, including every selected HEAVY node, runs on the trusted
  self-hosted forest1 runner. GitHub-hosted compute is not a substitute for
  this workload.
- `verify-increment` runs the bounded independent reconstruction and evidence
  checks on GitHub-hosted infrastructure. It must not execute HEAVY.
- `increment-gates` consumes the verified immutable artifact and performs the
  minimal repository attestation and authority checks. It runs no candidate
  gate code.

Accordingly, "GitHub attestation" identifies the repository-controlled
verification and signing boundary; it does not mean that GitHub-hosted runners
execute TESTGATE's expensive nodes. Runner availability must be evaluated
against the exact runner name, labels, and generation. Defunct records for a
retired runner, such as the pre-pivot Omarchy runner, do not prove that
forest1 is unavailable and do not justify canceling an active forest1 job.
A terminal job state also does not imply a passing gate: cancellation during
`execute-increment` prevents the downstream attestation even when cleanup,
evidence upload, and hosted fail-closed jobs complete.

Work-package engineering closeout and receipt trust assignment are separate
claims. An operator may close implementation, review, and documentation work
from retained exact-head comparator evidence plus the package's required
independent verification when the package explicitly records the exception.
That closeout does not promote a `LOCAL_UNTRUSTED` receipt, satisfy an
`INCREMENT`, campaign-certification, or release trust boundary, or permit a
hosted-attestation claim. The closeout record must name the actual reason the
protected execution or attestation did not complete; it may not substitute an
unrelated retired-runner outage.

Release-eligible envelopes carry an offline-verifiable attestation bundle that
binds repository, source commit/ref, workflow and job revision, runner/image,
attempt, plan/execution key, subjects, and artifact digests. The authority
registry versions accepted issuers, trust roots, repository principals and
roles, workflow identities, rotation history, and revocations. Verification
uses the policy version effective at issuance plus current revocation policy;
wrong repository/ref, replay at a different target, revoked issuer, or attempted
local-to-release promotion is `INVALID`.

Ledger authorization is an immutable signed/attested event, not free-form owner
text. It binds principal and role authority, exact predecessor ledger digest,
proposed transition digest, repository, campaign, and target head. Protected
repository rules and SLSA/GitHub artifact-attestation identity are the initial
online trust mechanism; the stored bundle must remain independently verifiable
offline.

### 10.4 Reuse and hermeticity

Each gate declares one closed reuse class:

- `NON_REUSABLE` (default): execute again at every consuming boundary;
- `SAME_EXECUTION`: reuse only inside one authenticated workflow attempt and
  identical environment; or
- `HERMETIC_CONTENT`: reusable when roots, policy, trust, and target boundary
  all verify.

`HERMETIC_CONTENT` requires enforced readable-filesystem confinement; sanitized
environment, Git and Cargo configuration; explicit executable/container bytes;
network denial or a digesting proxy whose responses enter the root; and
declared clock, randomness, locale, timezone, process, and kernel dependencies.
An observed access outside the declared closure is `INVALID`. If observation or
confinement is unavailable, the gate is `NON_REUSABLE`; assertion of
completeness is not enough.

Cargo registries, Git dependencies, `build.rs`, proc macros, generated files,
external datasets, workflow action revisions, tool installers, compiler
wrappers, `$HOME`, `PATH`, and applicable system libraries enter the execution
manifest when observable. Moving channels, mutable downloads, ignored files,
or network inputs cannot issue reusable protected-CI evidence until pinned and
content-bound.

A receipt is reusable only when:

- the verifier independently recomputes the affected closure, plan, and every
  bound root and they still match;
- the target boundary permits reuse from the original environment;
- the original result is `PASS` or policy-permitted `PASS_WITH_RETRY`;
- its declared reuse and trust class are accepted by the target boundary;
- its attestation envelope subject and artifacts independently verify;
- no policy-defined freshness or nondeterminism rule requires rerun; and
- the receipt and referenced artifacts pass integrity verification.

Campaign and release certification cannot use a dirty-tree receipt unless the
exact dirty tree is first committed without byte or admitted-metadata changes
and the verifier proves manifest equivalence under the clean-commit identity.

Review remediation that changes only documentation-root bytes does not stale an
execution receipt. A review finding that changes code, tests, authority,
fixtures, selectors, or gate configuration does.

## 11. Campaign Gate Ledger

Every implementation increment belongs to a campaign. A standalone repair uses
a mechanically created one-increment campaign with the same rules; critical
urgent work may begin only after that campaign and its initial intent plan
exist. A repository adopting this policy may bootstrap already-active work by
recording its current base/head, known obligations, and all prior receipts as
`LEGACY_UNVERIFIED` or imported content-verified evidence. Bootstrap cannot
backdate deferral or create a pass.

Every campaign declares before its first implementation increment:

- stable campaign ID and owner;
- base commit and intended integration boundary;
- included domains and public/consumer surfaces;
- expected increments or a bounded admission rule;
- checkpoint triggers;
- campaign-closure and release obligations;
- assurance registry/watch generation for complete impact discovery and any
  reports whose campaign transfer is requested;
- the maximum permitted unresolved interval for failed or unmapped checks; and
- a full-regression backstop cadence no greater than 14 elapsed days or 10
  merged increments, whichever occurs first, unless a stricter repository
  default applies.

The ledger is append-only and content-addressed. Each version binds campaign
ID, schema and policy, base, current campaign head, predecessor ledger digest,
admitted increment intent-plan IDs, obligation transitions, amendments,
receipts, investigation records, and authenticated authorization events.
Ledger publication uses compare-and-swap on the exact predecessor digest; a
stale predecessor is rejected and never resolved by last-writer-wins.

Multiple increments may be admitted against one campaign head, but every
admission binds `expected_parent_head` and predecessor ledger digest. Before
head advancement, its terminal plan is regenerated and reconciled against the
then-current campaign head. If another increment advanced first, all affected
sets, risk, receipts, and deferred assumptions are recomputed; newly selected
increment gates run before closure. Overlap/conflict blocks advancement until
resolved. Disjoint work may proceed after replan. Superseded or abandoned
admissions receive explicit terminal events and cannot later publish a head.
Advancing requires source ancestry from the current campaign head, a closed
terminal plan, and successful ledger compare-and-swap. Rebasing creates a new
admission/terminal-plan event and stales receipts whose recomputed roots differ.

Allowed lifecycle transitions are `CREATED -> ACTIVE -> CLOSING -> CERTIFIED`,
with `ACTIVE -> ABORTED`, `ACTIVE -> SUPERSEDED`, `CLOSING -> ACTIVE`,
`CLOSING -> ABORTED`, and `CLOSING -> SUPERSEDED`. Failed, stale, blocked, or
invalid closure evidence returns `CLOSING` to `ACTIVE` through an owner-recorded
remediation transition; it never advances to `CERTIFIED`. No campaign head
advance or new increment admission is permitted while `CLOSING`. A superseding
campaign names the predecessor and imports only independently verified current
receipts. Overlapping campaigns have distinct ledgers and heads; evidence
crosses between them only through ordinary receipt verification. An increment
is admitted only when its intent plan fits the campaign's domain/write-set
admission rule and is appended before edits. Anything outside that rule
requires an owner-authorized ledger amendment or a new campaign.

The machine-readable ledger records each obligation as:

- `PENDING` — required but not yet run;
- `PASS` — current passing receipt exists;
- `FAIL` — current execution failed;
- `BLOCKED` — execution cannot proceed for a recorded external reason;
- `STALE` — earlier evidence exists but its bound inputs changed;
- `DEFERRED` — assigned by the accepted intent plan to a named later boundary;
- `NOT_APPLICABLE` — mechanically or authoritatively proven outside scope;
- `SUPERSEDED` — replaced by a named obligation through an authorized
  amendment; or
- `LEGACY_UNVERIFIED` — bootstrap evidence exists but does not satisfy the new
  receipt contract.

`DEFERRED` must include owner, boundary, reason, and activation trigger. It
automatically becomes `PENDING` at that boundary. It can never become `PASS`
without a passing receipt.

An increment may close with ledger entries still deferred to campaign closure.
A campaign cannot close with `PENDING`, `FAIL`, `BLOCKED`, `STALE`, or
`DEFERRED` closure obligations. `SUPERSEDED` satisfies only proof that the named
replacement obligation exists; `LEGACY_UNVERIFIED` never satisfies a required
obligation.

Obligation state is the deterministic fold of immutable events ordered by
ledger ancestry and stable event ID. Allowed transitions are:

- ordinary creation to `PENDING`, `DEFERRED`, or proven `NOT_APPLICABLE`, and
  bootstrap-only creation to `LEGACY_UNVERIFIED` with imported artifact IDs;
- `DEFERRED -> PENDING` only when its named trigger/boundary activates;
- `PENDING`, `FAIL`, `BLOCKED`, or `STALE -> PASS` only through a current
  accepted receipt;
- `PASS -> STALE` when any bound input or policy changes;
- `FAIL`, `BLOCKED`, or `STALE -> PENDING` through an authorized retry,
  blocker-resolution, or invalidation-replan event;
- `LEGACY_UNVERIFIED -> PENDING` through an adopted replan/rerun event, or to
  `SUPERSEDED` through an atomic replacement obligation; it cannot transition
  directly to `PASS` and its imported trust/reuse class cannot be promoted;
- `PENDING -> FAIL` or `PENDING -> BLOCKED` from the next accepted execution
  event;
- any nonterminal state to `SUPERSEDED` through a versioned amendment
  that names its replacement; and
- idempotent ingestion of the identical receipt/event as no state change.

For competing valid events on one predecessor, `INVALID` evidence is rejected;
otherwise `FAIL` dominates `BLOCKED`, which dominates `STALE`, `PENDING`,
`DEFERRED`, and `PASS`. A later authorized event can advance only through the
transition table. Investigation records fold separately under Section 5.1.

A campaign owner may accept an append-only amendment for a newly discovered
inherently campaign-boundary obligation before increment closure. The amendment
binds discovery plan, discovering path/edge, owner, boundary, and rationale.
It cannot defer an increment-scope A1/A3, correctness, conservation, consumer,
security, or anti-evasion obligation and cannot reclassify a gate after it was
attempted or failed.

Backstop state is anchored to the most recent passing protected-CI full
regression executed on a named ancestor campaign head, even after later source
changes make that receipt non-current for certification. The authenticated CI
completion time and count of successful head-advance events after that ancestor
are authoritative. `CURRENT` means age `< 14 days` and count `< 10`; `DUE`
means age `>= 14 days` or count `>= 10` before the next admission/advance;
`OVERDUE` begins when one additional ordinary increment is proposed while
`DUE`, or when the due backstop fails/blocks. `DUE` permits only the already
admitted increment to close and simultaneously requires the backstop;
`OVERDUE` blocks new admission and head advancement. Clock rollback or missing
authenticated time is `OVERDUE`. Rebase does not reset the anchor; abort ends
it; supersession imports it only when ancestry and receipt trust verify. A broad
gate that detects a missed regression opens both a product defect and a
selector/impact-map defect; repairing only the product does not disposition the
planning failure.

### 11.1 Certification and evidence persistence

The **subject source commit** is the frozen clean commit whose executable and
authority roots are certified; evidence publication never changes that subject.
The authoritative store uses GitHub-enforceable branch/tag namespaces plus the
protected-CI attestation bundle. Each campaign has exactly one mutable compare-
and-swap branch,
`refs/heads/openwepp-evidence/<campaign-id>`, whose head points to the current
evidence commit/ledger version. Each certification creates an immutable tag at
`refs/tags/openwepp-evidence/<campaign-id>/<subject-commit>`. The evidence
commit/tree contains plans, receipts, envelopes, artifacts or durable artifact
locators, ledger events, trust bundles, and certificate. Neither branch nor tag
is a source/release head. Release material embeds the certificate and trust
bundle and resolves through the immutable subject tag.

Two active GitHub rulesets target `openwepp-evidence/**` branches and tags.
Both restrict creations, updates, and deletions; tags additionally reject
recreation. The sole always-bypass actor is a dedicated evidence-publisher GitHub App with
minimum repository-content permission. No user, team, generic write/admin role,
or ordinary workflow token is a bypass actor. The app may run only from the
pinned protected certification workflow. Ruleset IDs/configuration, app ID and
installation/revocation state are authority-root inputs and captured release
evidence. If the provider cannot enforce or export these rules, certification
is `BLOCKED` rather than falling back to an unprotected custom ref.

Finalization is two-phase:

1. freeze subject `C`; plan and execute against `C`; upload immutable
   content-addressed artifacts to protected staging; verify every receipt;
2. fold the ledger, calculate the certificate last without self-inclusion, then
   use `git push --atomic` with force-with-lease on the evidence branch's exact
   predecessor and creation of the absent immutable subject tag. Remote atomic
   capability, both active rulesets, and the dedicated app identity are checked
   before push. If any lease, tag absence, ruleset, attestation, or atomic
   capability check fails, neither ref changes and the candidate replans from
   the winning branch head.

A crash before the ref update leaves unreferenced staging only and no
certificate. Identical retry is idempotent; different bytes for the same
identity or partial upload is `INVALID`. Only the protected evidence-publisher
app may advance the branch/create the tag. Certified and release evidence refs and their referenced
artifacts are retained indefinitely; failed/unreferenced staging is retained at
least 180 days for diagnosis, then may be garbage-collected by recorded policy.
A later in-repository documentation archive names the subject, evidence ref,
certificate ID, and digest but is not part of the certified source. Fresh-clone
verification must fetch the evidence branch/tag and validate the offline trust
bundle.

## 12. Coverage And CRAP

ADR-0021 remains authority for eligible surfaces, thresholds, function floor,
CRAP threshold 30, retained exceptions, and adjudication. This section governs
when those measurements run.

### 12.1 Increment measurement

A Rust production increment first maps changed **source items**: functions,
constants and statics, coefficient/data tables, types and traits, impls, macros,
generated inputs, build scripts, feature-controlled items, and shared error or
configuration definitions. It then expands those items to every eligible
production function whose behavior can depend on them and measures that
affected function surface using selected component, contract, integration, and
reverse-dependency tests. It must:

- include every new or changed eligible function;
- include unchanged functions when their branch behavior, dependencies, build
  inputs, features, or tests were changed;
- preserve the obligation-to-test map;
- expand each affected function to its complete mechanically known
  covering-test closure, including unchanged tests whose retained coverage
  contributes to that function; the behavior-suite selection alone is not the
  CRAP denominator;
- report coverage and CRAP for the affected surface;
- fail when an affected actionable function exceeds CRAP 30 or violates its
  applicable coverage closure requirement; and
- record that global workspace certification is pending for campaign closure.

The follow-up implementation must define a source-item dependency mapper.
When exact expansion is unavailable for a non-function item, the affected
surface becomes all eligible functions in the owning package and selected
reverse-dependent packages. If that conservative surface still cannot be
bounded, global measurement is required. An empty affected selection for a
production constant, type/trait, macro/build input, or feature change is
invalid. Acceptance fixtures must exercise each of those cases.

Coverage-contribution mappings are versioned evidence produced from a current
global or affected instrumented run. If the planner cannot prove that the map
contains every known covering test for an affected function, it expands to the
owning package and reverse-dependent test inventory; if completeness remains
unknown, it runs global measurement rather than reporting artificially narrow
CRAP.

### 12.2 Immediate global measurement

Fresh full-workspace coverage and adjudicated CRAP run immediately after any
change that can reduce coverage or alter classification outside a bounded
source surface, including:

- deleting, disabling, ignoring, filtering, or moving a test or required case
  whose prior coverage contribution is not proven to remain in the complete
  affected closure (`COVERAGE_CONTRIBUTION_REMOVED`);
- changing a test so its prior covered-function set cannot be reconstructed or
  bounded (`COVERAGE_CONTRIBUTION_UNKNOWN`);
- changing workspace test membership, features, coverage flags, profiles, or
  production filters;
- changing the adjudication registry or exception evidence;
- changing shared test helpers with broad fan-out; or
- any critical change for which affected coverage cannot be bounded.

Purely additive tests and bounded test edits remain on affected measurement
when inventory comparison and prior/new contribution maps prove that no
coverage outside the affected closure was removed. A modified test whose prior
coverage remains supplied by other mapped tests does not trigger global
measurement. Unknown mapping fails to the global gate; planner or agent
judgment is not a predicate.

### 12.3 Campaign and release measurement

Campaign closure and release qualification require **current** global evidence
for their exact execution root. Here, current/fresh means produced for that
exact root and reuse contract, not necessarily rerun at both lifecycle labels.
A current campaign full-regression or global CRAP receipt satisfies the release
gate only when it is `HERMETIC_CONTENT`, has release-accepted `PROTECTED_CI`
trust, the release changes none of its bound inputs, and the gate has no
explicit `rerun_on_release` policy. `NON_REUSABLE` and `SAME_EXECUTION` evidence
must rerun at release. A new execution is also required when the release changes
a bound input, the receipt fails verification, or that declared policy applies.
The full run applies the canonical production filter,
deduplication, adjudication registry, source freeze, and zero actionable
workspace condition.

The implementation must provide one instrumented full Nextest run that can
supply both full regression results and LCOV to cargo-crap. Adoption requires
test-inventory parity, acceptable runtime, complete required coverage, and
evidence that coverage instrumentation does not invalidate a gate's semantics.
Until proven, functional and coverage executions remain distinct. After parity
is proven for a gate definition and environment identity, separately rerunning
the same full inventory for regression and coverage is forbidden.

## 13. Assurance Impact And Deferral

Testing and assurance are related but not interchangeable. Ordinary code tests
prove software behavior; assurance reports communicate assessed scientific
evidence for named realizations.

### 13.1 Assurance impact record

Assurance state is a versioned multi-axis record, not one mutually exclusive
enum. Each record binds report ID and source/assessed-realization root, watch
generation, campaign ID and exact head, policy generation, and—when release
transfer is requested—exact release identity. A bare, target-free `CURRENT` is
invalid. Each registered report records:

| Axis | Closed states |
| --- | --- |
| Assessed realization integrity | `CURRENT`, `INVALIDATED_BY_EVIDENCE`, `UNKNOWN` |
| Campaign impact disposition | `NO_IMPACT_DETECTED`, `IMPACT_PENDING`, `NO_MATERIAL_IMPACT_AUTHORIZED`, `REFRESH_REQUIRED`, `REFRESH_COMPLETE` |
| Campaign-head transfer request | `NOT_REQUESTED`, `REQUESTED` |
| Campaign-head transfer currency | `BLOCKED`, `CURRENT` |
| Release transfer request | `NOT_REQUESTED`, `REQUESTED` |
| Release transfer currency | `BLOCKED`, `CURRENT` |

An exact or semantic match changes campaign impact from `NO_IMPACT_DETECTED` to
`IMPACT_PENDING` and, when transfer is requested, makes its currency `BLOCKED`.
An authorized scientific
disposition changes it to `NO_MATERIAL_IMPACT_AUTHORIZED` or
`REFRESH_REQUIRED`. Required reproduction/revision/review changes the latter to
`REFRESH_COMPLETE`; only then may the campaign-head axis become `CURRENT`.
Release transfer becomes `CURRENT` only through the assurance lifecycle's
exact release-transfer procedure. New contrary evidence, not ordinary source
movement, may change assessed-realization integrity. Therefore historical
integrity can remain `CURRENT` while campaign impact is `IMPACT_PENDING`.

### 13.2 Mechanical campaign behavior

During an active campaign, the planner must discover every report in the
versioned assurance registry and mechanically add an impact entry for every
exact or semantic match; operator preselection cannot narrow discovery. Every
registered report must declare exact
dependencies plus process/domain and contract watches; incomplete, unknown, or
unmapped watch coverage blocks campaign-head and release currency.

Semantic watches are versioned records with stable watch ID, report ID, owner,
kind, match value, and governing lifecycle boundary. Closed kinds are:

- `exact_path`;
- `path_prefix` using component-boundary prefix matching;
- `path_glob` using repository-rooted, slash-separated Git wildmatch semantics;
- `contract_id`;
- `cargo_package`;
- `process_domain_tag`;
- `result_procedure`; and
- `builder_schema`.

Each impact entry has one closed state:

- `OPEN_UNKNOWN` for incomplete mapping or ownership;
- `OPEN_ASSESSMENT` for a known match awaiting scientific disposition;
- `NO_MATERIAL_IMPACT_AUTHORIZED`;
- `REFRESH_REQUIRED`;
- `REFRESH_COMPLETE`;
- `SUPERSEDED` with a named replacement entry; or
- `WITHDRAWN` through an authorized report-withdrawal event.

Allowed transitions are `OPEN_UNKNOWN -> OPEN_ASSESSMENT` after the missing map
is governed; `OPEN_ASSESSMENT -> NO_MATERIAL_IMPACT_AUTHORIZED` or
`REFRESH_REQUIRED`; `REFRESH_REQUIRED -> REFRESH_COMPLETE`; and any open state
to authorized `SUPERSEDED` or `WITHDRAWN`. `SUPERSEDED` is valid only when one
atomic event creates or proves an existing replacement with the same report,
campaign/target head, assessed realization, and watch/policy generation and
includes that replacement in the same fold. A dangling, cross-report,
cross-target, withdrawn, or recursively invalid replacement rejects the
transition and leaves the original entry open. A refresh-complete event binds the
exact set of impact-entry IDs it resolves, reproduction/result receipts, report
root, review/approval events, and target head. It cannot close an unlisted or
later impact.

Rename is evaluated as deletion plus addition; a deleted watched input and a
new matching input both create entries. A newly added production, contract, or
result path with no complete watch/ownership classification creates an
`OPEN_UNKNOWN` entry for every registered report in the
mapped process/domain and blocks transfer. If even that process/domain cannot
be mapped, every registered report is conservatively impacted and an impact-map
defect is opened.

Impact-entry identity is the SHA-256 of the canonical entry payload excluding
the derived impact-entry ID. The payload binds schema, report, the predecessor
or admission ledger digest supplied to the planner, terminal plan, changed
object, change kind, and matching watch IDs. Multiple watches for one changed
object coalesce into one entry while retaining all matches. Its resolution
owner is the exact principal and role record selected by the report lifecycle;
only lifecycle-named review/approval authority can advance transfer currency. A
`NO_MATERIAL_IMPACT_AUTHORIZED` disposition records rationale and old/new
identities and advances campaign-head currency without manuscript rewrite or
claimed reproduction when the lifecycle permits it.

Impact entries are immutable lifecycle events. A later matching change creates
a new entry and resets any prior aggregate `NO_MATERIAL_IMPACT_AUTHORIZED` or
`REFRESH_COMPLETE` result to `IMPACT_PENDING` for the new exact head. For an
exact target, the deterministic per-report fold covers every nonwithdrawn and
not-validly-superseded entry since the preceding current transfer whose changed
object can influence that target. A superseded chain is excluded only when its
valid terminal replacement is included in this fold; otherwise it yields
`IMPACT_PENDING`. Any `OPEN_UNKNOWN` or `OPEN_ASSESSMENT` yields `IMPACT_PENDING`;
otherwise any `REFRESH_REQUIRED` yields `REFRESH_REQUIRED`; otherwise any
`REFRESH_COMPLETE` yields aggregate `REFRESH_COMPLETE`; otherwise resolved
entries yield `NO_MATERIAL_IMPACT_AUTHORIZED`, and an empty set yields
`NO_IMPACT_DETECTED`. Ordering cannot change the result. Transfer becomes
`CURRENT` only for the exact bound target after every blocking entry is resolved.
Withdrawal, supersession, principal-role revocation, and target-head change are
new events that block currency until folded under current authority. Resolution
events bind the exact principal ID and role record authorized by the assurance
lifecycle, not an unverified prose role label.

Mechanical impact creation must not:

- rewrite manuscript prose;
- rebind declared hashes;
- create or revoke human approval;
- regenerate results;
- invoke an agent or network service;
- rebuild every report; or
- expose an internal stale/pending label as a public scientific headline.

Draft or held reports may accumulate impact entries until their named campaign
closure or review-entry trigger. Approved or published reports remain valid for
their assessed version; they are excluded from a new release until transfer
currency is resolved.

At release, the inclusion set is derived mechanically from the exact public
catalog, snapshot, export, vendoring, package, and distribution inventories.
That shipped identity set must equal the set of reports with exact current
release-transfer records. Any public or generated assurance object without one
current owning report blocks release. Historical exclusion is valid only when
all public release inventories omit that identity; it does not rewrite or
invalidate the retained historical report.

### 13.3 Human boundary

The planner determines dependency impact, not scientific materiality. At the
declared boundary, the report lead, process owner, independent reviewer, or
assurance steward required by the lifecycle decides whether the impact demands
new analysis, reproduction, report revision, review, supersession, or no
scientific change. That disposition binds old and new identities.

## 14. CI And Operational Lanes

The target CI mapping is:

| Lane | Trigger | Normal work | Blocking scope |
| --- | --- | --- | --- |
| Presubmit/increment | Pull request or requested local closure | Mechanical increment plan | Increment merge/closure |
| Post-submit/checkpoint | Push to integration branch or campaign checkpoint | Fast workspace, affected domains, accumulated integration | Opens visible campaign debt; critical failures require prompt action |
| Periodic/backstop | Scheduled | Full regression or expensive authority families according to cadence | Campaign/release closure until disposition |
| Campaign closure | Explicit dispatch on candidate head | Complete campaign gate set and certification | Campaign disposition |
| Release | Explicit release dispatch | Current campaign certification plus release-only gates | Release publication/distribution |

Workflows use separate stable contexts:

- `openwepp/gate-plan` reports only `PLAN_VALID` or planning failure and is not
  a merge-sufficient required check;
- `openwepp/increment-gates` is the always-reporting presubmit aggregate and
  succeeds only after the complete admitted DAG executes, every required job is
  present, receipts verify, and aggregate execution is accepted;
- `openwepp/backstop`, `openwepp/campaign-certification`, and
  `openwepp/release-qualification` report their corresponding boundaries.

A zero-work plan can make the aggregate succeed only after receipt verification
proves the empty DAG is correct. Missing/canceled dynamic jobs,
planner/executor inventory disagreement, or an unfinished matrix fails the
aggregate closed. Branch protection and merge queues require the aggregate
execution context, never planner generation alone.

Failures are not hidden because they occurred post-submit. They enter the
campaign ledger with owner and affected head. A critical correctness,
conservation, security, or default-production failure pauses dependent campaign
work until contained; unrelated diagnostic failures may remain investigation
debt only when their governing authority permits it.

### 14.1 Queue and runner governance

Normal TESTGATE uses one permanent repository concurrency identity with the
provider's single-pending queue and `cancel-in-progress: false`. The current run
may finish rather than repeatedly restarting cold work; only the newest pending
run is retained. Never version or rename that identity without first draining
its exact live group.

Every self-hosted job uses exact capability and site labels. Generic
`self-hosted` routing is forbidden because it can make an unrelated workflow
consume the trusted increment runner. Historical manual dispatch is rejected:
normal execution, independent verification, and authority minting each require
the run SHA to equal current `main`. A second current-head check immediately
before gate execution bounds work when a push is superseded after admission.
The aggregate checks current `main` both before attestation and as its final
success condition after native verification and authenticated evidence upload;
a head superseded during authority work cannot finish successful.

Agents make package-required scaffold and intermediate commits locally and push
once when the increment is stable. Manual dispatch first proves no TESTGATE run
is queued or active. Provider queue records bound to retired labels or obsolete
concurrency identities are canceled before later dispatch; they are never
revived by re-registering an obsolete runner.

## 15. Performance And Friction Budgets

Budgets are diagnostic objectives, not permission to omit affected gates:

| Activity | Target wall time on the reference development host |
| --- | --- |
| Edit loop | Preferably under 60 seconds |
| Ordinary bounded increment | Preferably under 5 minutes |
| Integrated-domain increment | Preferably under 10 minutes |
| Campaign checkpoint | Preferably under 15 minutes |
| Campaign closure and release | No fixed ceiling; record and optimize dominant families |

Every executor records timing. A gate that repeatedly exceeds its budget must
be profiled and considered for suite splitting, caching, sharding, fixture
reuse, or boundary reassignment. The response must not weaken its authority or
hide its execution.

The following are tracked separately:

- compile/setup time;
- test execution wall time and aggregate test time;
- coverage instrumentation overhead;
- fixture acquisition/preparation;
- assurance planning/build/reproduction; and
- agent review or manual scientific review time.

This separation prevents a slow build, duplicated coverage pass, or manual
governance step from being mislabeled as test execution.

An infrastructure-invalid heavy attempt may be repeated once only under its
declared retry policy. If the same cause recurs, execution stops before another
heavy attempt and opens or updates the owning tooling defect. Restarting in a
new directory, clearing a cache, changing an unbound environment variable, or
manually restating an argument is a workaround, not a correction.

## 16. Failure, Flakiness, And Nondeterminism

- Failures retain the failing receipt and artifacts; a retry never erases the
  first result.
- Automatic retries are allowed only for suites with an explicit retry policy.
  A pass after retry is reported as flaky, not indistinguishable from a clean
  pass.
- A newly flaky deterministic contract, conservation, or publication test is a
  defect, not a reason to move the test out of the increment lane.
- External or nondeterministic suites declare environmental dependencies,
  tolerance, retry, and failure class.
- Infrastructure failure is `INVALID` or `BLOCKED`, not scientific `FAIL`, but
  it cannot satisfy the gate.
- Quarantine, ignore, skip, filter, tolerance expansion, or lane demotion is a
  governed test change and receives critical-risk treatment.

## 17. Anti-evasion And Security

The planner and verifier must reject:

- unknown executable paths classified as editorial;
- changed tests or fixtures omitted from the impact plan;
- a selected test target that executes zero expected tests;
- differences between planned and executed inventories without escalation;
- wildcard CRAP or coverage exceptions;
- stale receipts relabeled current;
- receipt reuse after toolchain, feature, fixture, policy, or source change;
- mutation of source or the Git index during frozen measurement;
- a deferred obligation without owner and boundary;
- `NOT_APPLICABLE` without a mechanical or authoritative reason;
- assurance hash rebinding used to avoid impact review; and
- public presentation of internal gate state as a scientific merit grade.

Changes to the planner, impact map, receipt verifier, gate policy, test filters,
coverage acquisition, or anti-evasion checks are themselves critical.

## 18. Review And Audit Expectations

Reviewers evaluate both adequacy and economy:

- Does the plan cover every changed authority and consumer?
- Are contract obligations and negative cases present?
- Did the risk classifier escalate every applicable trigger?
- Are deferred items genuinely campaign-owned and declared before work?
- Are receipts current for their claimed roots?
- Did the package overclaim from a focused pass?
- Did unnecessary full or assurance work reveal a missing impact mapping?
- Did a broad gate discover a regression the planner missed? If so, that is a
  planner defect and must update the impact map or risk rule.
- Did the pre-heavy closure audit run once against the final intended closure
  state, and did execution consume that exact report?
- Did any workaround or repeated failure reveal a tooling defect, and is the
  defect fixed or blocking further expensive execution?

Avoidable friction is evidence. The timing history, selected inventory, missed
regressions, escalations, and false-positive broad runs should inform later
policy revisions without silently weakening the current policy. Narrative
capture alone does not close a mechanically preventable recurrence.

## 19. Transition Requirements

ADR-0039 activates this standard before implementation alignment. The follow-up
package must, at minimum:

1. replace duplicated gate-frequency rules in root and nested instructions with
   binding pointers;
2. amend ADR-0021 Decision 8 and the Gate Evidence Non-Deferral language while
   preserving thresholds and non-waivable current-scope gates;
3. define campaign metadata and the machine-readable ledger;
4. implement the gate planner, explicit impact map, receipt schema, verifier,
   and status renderer;
5. implement targeted affected-surface coverage/CRAP and critical escalation;
6. split presubmit, post-submit, periodic, campaign, and release workflows;
7. prevent duplicate full regression and coverage runs where a proven combined
   path is safe;
8. extend assurance planning with semantic watches and campaign-head currency;
9. update package templates, prompt standards, test guidance, local-CI docs,
   release docs, and catalogs; and
10. prove the selector against the event-driven acceptance matrix and one exact
    cutover candidate before enforcing it as the normal path. Retained campaign
    replay remains campaign/release evidence, not an ordinary-cutover delay.

Blocking cutover is event-driven under ADR-0040. It does not wait for elapsed
days, an increment count, duplicate clean environments, or a dual-required
provider interval. Acceptance on one exact candidate requires:

- zero missed non-deferrable, critical, consumer, conservation, authority, or
  anti-evasion obligations in the documentation, bounded, integrated/critical,
  unknown-impact, and failure-injection acceptance matrix;
- zero unsafe receipt reuse, missing executor jobs, unexplained inventory
  mismatches, or unresolved production/authority mappings;
- one cold writable-surface bootstrap followed by same-job cached build and
  deterministic plan/root/receipt execution on the trusted runner; writable
  work, target, and dependency surfaces are destroyed after every job;
- current affected and global CRAP with zero actionable rows after accepted
  patches;
- every adversarial and independent review finding dispositioned, with accepted
  findings patched and verified;
- one exact-candidate conservative full-suite comparison;
- untrusted public pull-request code proven unable to route to the trusted
  self-hosted runner; and
- a callable conservative manual rollback lane.

The operator accepted the measured 48.8% projected reduction on 2026-07-18;
there is no separate 50% threshold. Performance evidence cannot waive safety.
Rollback triggers are any false-negative required obligation, unsafe reuse,
certificate trust failure, ledger lost update, untrusted-event admission, or
required aggregate disappearance. Rollback disables the normal TESTGATE
trigger and invokes the conservative manual lane until the concrete defect is
patched. Campaigns admitted under the new schema retain their ledgers but
cannot claim certification through legacy evidence.

When repository rulesets, branch protection, or merge queues actually require
an old context, migration replaces it with the accepted aggregate in one
authenticated owner change after acceptance; no dual-required interval is
required. If no provider rule exists, provider migration is not a cutover
operand. After acceptance, the accepted TESTGATE aggregate becomes normal
increment authority immediately and the broad runner remains available only
for critical, campaign, release, or explicit rollback boundaries.

Cutover is recorded only after the exact-candidate acceptance and terminal
verification artifacts pass. No elapsed-time or increment-count gate applies.

## 20. References

### Internal authority

- [Correctness Authority Model](../specifications/correctness-authority-model.md)
- [ADR-0039: Campaign-scoped, risk-based testing and assurance gates](../decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md)
- [ADR-0040: Accelerated TESTGATE cutover](../decisions/0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md)
- [ADR-0021: Module coverage and complexity-risk closure thresholds](../decisions/0021-module-coverage-closure-thresholds.md)
- [Rust Scientific Coding Standard](rust-scientific-coding-standard.md)
- [Module Test-Enhancement Work-Package Authoring Guide](module-test-enhancement-authoring-guide.md)
- [Local CI Gate Selection](local-ci-gate-selection.md)
- [Scientific Assurance V2 Architecture](../governance/scientific-assurance-v2-architecture.md)
- [Scientific Assurance Dossier Lifecycle](../governance/scientific-assurance-dossier-lifecycle.md)

### External primary and authoritative sources

- [Rust compiler testing with CI](https://rustc-dev-guide.rust-lang.org/tests/ci.html)
- [Rust compiler running tests](https://rustc-dev-guide.rust-lang.org/tests/running.html)
- [Chromium commit queue](https://chromium.googlesource.com/chromium/src/+/master/docs/infra/cq.md)
- [Firefox CI and Taskgraph](https://firefox-source-docs.mozilla.org/taskcluster/index.html)
- [Kubernetes Prow jobs](https://docs.prow.k8s.io/docs/jobs/)
- [Cargo metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)
- [Cargo dependency resolution](https://doc.rust-lang.org/cargo/reference/resolver.html)
- [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Cargo configuration](https://doc.rust-lang.org/cargo/reference/config.html)
- [RFC 7493: The I-JSON Message Format](https://www.rfc-editor.org/rfc/rfc7493)
- [RFC 8785: JSON Canonicalization Scheme](https://www.rfc-editor.org/rfc/rfc8785)
- [NIST Secure Hash Standard, FIPS 180-4](https://csrc.nist.gov/pubs/fips/180-4/upd1/final)
- [Git status porcelain format](https://git-scm.com/docs/git-status)
- [Git diff](https://git-scm.com/docs/git-diff)
- [Git index format](https://git-scm.com/docs/index-format)
- [Git atomic push](https://git-scm.com/docs/git-push#Documentation/git-push.txt---atomic)
- [Git reference transactions](https://git-scm.com/docs/git-update-ref)
- [Git path-pattern format](https://git-scm.com/docs/gitignore#_pattern_format)
- [SLSA v1.2 provenance](https://slsa.dev/spec/v1.2/provenance)
- [GitHub artifact attestations](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations)
- [GitHub repository rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository)
- [GitHub ruleset update/deletion restrictions](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)
- [Nextest filtersets](https://nexte.st/docs/filtersets/)
- [Nextest partitioning](https://nexte.st/docs/ci-features/partitioning/)
- [Nextest test-coverage integration](https://nexte.st/docs/integrations/test-coverage/)
- [Snakemake command-line rerun triggers](https://snakemake.readthedocs.io/en/stable/executing/cli.html)
- [Predictive Test Selection](https://arxiv.org/abs/1810.05286)
- [Assessing Transition-based Test Selection Algorithms at Google](https://research.google/pubs/assessing-transition-based-test-selection-algorithms-at-google/)
