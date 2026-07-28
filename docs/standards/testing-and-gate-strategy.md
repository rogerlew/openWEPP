# Testing And Validation Strategy

Status: canonical

Authority: ADR-0043, as preserving the independent correctness and quality
substance of ADR-0039 through ADR-0041.

## 1. Purpose

This standard governs validation selection, lifecycle timing, campaign
deferral, evidence reuse, risk escalation, and failure disposition. It is
written for direct use by agents and maintainers.

This document, applicable science contracts, repository instructions, and
authorized work packages establish requirements. Agents apply them directly.

## 2. Core Terms

A **validation requirement** is a command or evidence obligation established
by repository governance, a contract, or an authorized package. The historical
word **gate** means the same thing; it does not imply a planner-owned
transition.

**Evidence** is the retained result of work that actually ran. A suggested
command is not evidence.

**Direct execution** means the agent invokes the documented command without a
planner, TESTGATE transition, receipt authority, or CI admission layer.

**Campaign deferral** means a named campaign-owned obligation will run at a
later declared boundary. Deferred is not passed, skipped, or waived.

## 3. Authority And Responsibility

Agents:

- declare implementation intent and the intended write set before edits;
- inspect the exact change and its dependency/authority surfaces;
- select and run applicable commands directly;
- document uncertainty and conservatively escalate unknown production impact;
- retain exact command, working directory, source identity, result, and
  required outputs;
- reconcile the terminal diff before disposition; and
- report unmet requirements truthfully.

Agents may revise an initial validation plan as exact-diff and authority
inspection reveals new facts. They may not silently narrow, waive, or
misrepresent an independently applicable requirement. Manual validation
planning is the only prospective route. A known unmet underlying requirement
can still prevent truthful closure.

## 4. Principles

1. Test affected behavior immediately and the integrated system at its
   integration boundary.
2. Match validation cost to risk, dependency fan-out, and consequence of
   error—not diff size or operator identity.
3. Map requirements from the exact change and document the selection. Agent
   analysis may challenge advice but may not silently narrow a governing
   requirement.
4. Escalate unknown production impact conservatively and record the ambiguity.
5. A passing focused check claims only its named affected surface.
6. Campaign and release evidence binds exact clean identities; approximate
   recency and “nothing important changed” are not evidence.
7. Reuse identical current evidence; do not rerun unchanged work because only
   unrelated narrative changed.
8. Keep correctness authority separate from execution frequency.
9. Preserve failure localization; deferring all feedback to campaign closure is
   nonconforming.
10. Record a reason for every deferred, stale, skipped, failed, or escalated
    requirement.
11. Treat avoidable tool friction as debt. Tool repair is not a prerequisite
    to unrelated modeling.
12. Correct repeated workflow gaps at the simplest maintainable owning layer
    without recreating a planner authority system.

## 5. Test And Check Families

Families describe purpose, not a strength ladder.

| Family | Purpose | Typical examples | Normal earliest boundary |
| --- | --- | --- | --- |
| Source quality | Reject malformed or nonconforming source | Rustfmt, Clippy, Markdown lint, schema validation | Increment |
| Component behavior | Prove local calculations, guards, state transitions, and properties | Unit and property tests | Edit loop and increment |
| Contract obligation | Bind `SC-*` invariants and A–H test-vector obligations | Contract-derived tests, unit/guard vectors | Increment |
| Integration and consumer | Prove orchestration, serialization, restart, downstream reads, and publication values | Integration tests, CLI fixtures, real-consumer tests | Increment when affected; otherwise checkpoint |
| Conservation and reconstruction | Independently reconstruct mass, water, sediment, or energy behavior | Operand reconstruction, closure audit, rejected-formula tests | Increment when affected and campaign closure |
| Comparator and migration | Detect or classify differences against a pinned implementation or independent calculation | Legacy comparator, analytical recurrence, independent solver | Affected increment or campaign |
| Constitutive correctness authority | Exercise applicable A3 externally governed constitutive suites | Required authority suite and pinned fixture cohort | Increment when applicable; non-deferrable |
| Empirical and independent authority | Evaluate against observations or independent calculations beyond constitutive closure | A4 empirical cohorts, A5 independent solvers, SNOTEL, frost-tube | Domain checkpoint, periodic, campaign, or release as declared |
| System and stability | Exercise large populations, concurrency, binaries, manifests, and failure recovery | Stability cohort, watchlist, multi-worker CLI | Critical increment, campaign, or release |
| Coverage and complexity risk | Measure exercised eligible regions and change risk | LLVM coverage, cargo-crap | Optional QA; package-local when explicitly owned |
| Assurance and publication | Assess report impact, reproduce evidence, render, approve, transfer, and publish | Assurance plan/build/check/publish/verify-release | Impact analysis during campaign; realization and publication at closure/release |

Every suite declares its family, owner, dependencies, expected duration class,
failure policy, minimum applicable boundary, authority class when applicable,
and scientific outcome policy when applicable. Split mixed fast-contract and
multi-minute external workflows unless their semantics require inseparable
execution.

Execution integrity and scientific outcome are separate:

- execution is `PASS`, `PASS_WITH_RETRY`, `FAIL`, `BLOCKED`, or `INVALID`;
- a completed scientific/comparator evaluation separately records `CONFORMS`,
  `DIVERGES`, `INCONCLUSIVE`, or `NOT_EVALUATED`, plus quantitative results;
- complete expected inventory and content integrity are prerequisites to a
  scientific outcome; a crash or incomplete suite is execution failure, not
  scientific divergence; and
- `PASS_WITH_RETRY` is accepted only when every failed attempt is
  infrastructure-only under a prospectively declared closed retry policy. A
  semantic or scientific failure can never be retried into accepted execution.

Outcome reduction is exhaustive:

| Authority | Accepted outcome | Other outcome |
| --- | --- | --- |
| A0 | `ADMITTED` from the authority check | Missing, ambiguous, provisional, or stale is `BLOCKED`; no scientific outcome axis applies |
| A1/A3 | Accepted execution and scientific `CONFORMS` | `DIVERGES`, `INCONCLUSIVE`, `NOT_EVALUATED`, semantic retry, or unaccepted execution blocks |
| A2/A6 | Accepted execution plus `CONFORMS`, `DIVERGES`, or `INCONCLUSIVE` | Divergence/inconclusive opens investigation; `NOT_EVALUATED` or unaccepted execution does not satisfy a selected suite |
| A4/A5 unpromoted | Accepted execution plus `CONFORMS`, `DIVERGES`, or `INCONCLUSIVE` | Divergence/inconclusive opens investigation; `NOT_EVALUATED` or unaccepted execution does not satisfy a selected suite |
| A4/A5 promoted | Accepted execution and the prospectively declared scientific predicate | Any other scientific or execution outcome blocks |

`NOT_EVALUATED` is valid only when the package record shows the suite was not
selected or not applicable; it never satisfies selected required execution.
A2/A4/A5/A6 investigation disposition is `PENDING`, `ACCEPTED_SIGNAL`,
`DEFECT_OPEN`, `SUPERSEDED`, or `RESOLVED`. Promotion is declared before
execution and cannot be inferred after seeing a result.

## 6. Validation Moments

### 6.1 Edit Loop

The edit loop is non-authoritative feedback. Prefer the smallest focused tests,
formatters, schema checks, and documentation checks that shorten correction
time. A focused pass claims only its affected surface.

### 6.2 Increment Closure

Before an implementation increment closes:

1. reconcile the exact terminal diff with declared intent and write set;
2. run affected-language formatting and warnings-denied lint for affected
   packages and applicable reverse dependents;
3. run component and contract-obligation tests owned by the changed surface;
4. prove current A0 authority admission and run every applicable A1 hard
   invariant and A3 constitutive-authority suite;
5. run affected integration, real-consumer, negative, error-precedence,
   chronology, restart, serialization, conservation, reconstruction,
   publication, and anti-evasion checks;
6. run affected doctests, placeholder/stub scanning, documentation/schema
   checks, and assurance-impact analysis;
7. run `cargo deny check` when manifests, lockfiles, dependency/license/source
   policy, toolchain dependency, or workspace resolution can change;
8. disposition uncertainty and every discovered obligation; and
9. retain exact commands and results in the owning package.

Newly discovered current-scope obligations run before closure. They cannot be
retroactively deferred.

Documentation-only increments run affected documentation, reference, schema,
catalog, and generated-drift checks. They do not run Rust requirements solely
because the repository contains Rust.

### 6.3 Campaign Checkpoint

A campaign checkpoint integrates multiple increments when its package names a
checkpoint trigger. It normally runs the fast workspace profile, affected
domain profiles, accumulated cross-increment integration/consumer checks,
useful affected comparators or empirical suites, campaign-record validation,
and assurance-impact status. It does not require a planner ledger or receipt.

### 6.4 Campaign Closure

Campaign closure requires an exact clean commit and runs workspace formatting,
warnings-denied Clippy, full-workspace correctness regression, full doctests,
placeholder/stub scanning, cargo-deny, and every campaign-owned contract,
consumer, conservation, comparator, empirical, external-authority, stability,
provenance, assurance, and distribution obligation. It includes required dual
review/verification and resolves every failed, stale, blocked, deferred, and
unmapped item. Optional coverage/CRAP remains observational under ADR-0041.

### 6.5 Release Qualification

Release qualification runs exact-source correctness, packaging, distribution,
security, selected authority/manual lanes, stability/population cohorts,
publication, assurance realization/transfer, and downstream vendoring checks
required by the release process. Release evidence identifies the exact source,
toolchain, configuration, fixtures, policy, platform when relevant, and other
bound inputs. Release-only obligations remain release-only. The advisory linter
and historical TESTGATE do not certify releases.

## 7. Correctness Authority

Use `docs/specifications/correctness-authority-model.md`.

- **A0 — conservation/closure:** binding when applicable.
- **A1 — hard invariants:** binding when applicable.
- **A2 — external benchmarks:** investigation evidence unless prospectively
  promoted.
- **A3 — constitutive relationships:** binding for touched constitutive
  behavior and required external-authority suites.
- **A4 — observed cohorts:** investigation/calibration/validation evidence
  according to their prospectively assigned role.
- **A5 — legacy comparator:** discrepancy flag, not correctness target.
- **A6 — stability/numerics:** investigation evidence unless prospectively
  promoted.

Execution integrity and scientific outcome are separate axes. A valid
A2/A4/A5/A6 divergence remains visible and may require investigation without
being relabeled a pass or automatically converted into an execution failure.
A0/A1/A3 remain non-deferrable where their authority applies.

Kernel work retains typed guards, touched contract invariants, direct-consumer
proof, conservation/reconstruction evidence, and anti-tautology protection.
No validation mechanism authorizes surrogate or provisional production
physics.

## 8. Risk And Escalation

The agent assigns and records risk from the changed invariant, not merely from
the containing filename.

### Editorial

Use documentation/path/link/schema checks when a change cannot alter executable
behavior, authority, package lifecycle, protected data, or generated runtime
inputs.

### Bounded Component

Use focused owning-crate/module tests for a narrow implementation with explicit
consumers and no cross-domain or authority impact.

### Integrated Domain

Use domain profiles and real-consumer checks when a change spans crates,
orchestration, serialization, runtime publication, or a science domain.

### Critical

Run immediate campaign-strength full correctness regression when a change
semantically alters:

- shared numerical primitives, calendars, chronology, state layout, restart,
  production kernel authority, or cross-domain orchestration;
- production activation, default selectors, compatibility deletion, cutover,
  or public-output semantics;
- conservation formulas or operand lineage;
- serialization, schema, manifest, release sidecar, or broadly consumed APIs;
- `unsafe` code or security boundaries;
- toolchain, existing-package dependency resolution, non-isolated workspace
  membership, global features, compiler flags, coverage configuration, or
  validation-runner behavior;
- deletion, disabling, renaming, filtering, reclassification, or weakening of
  tests, fixtures, required cases, anti-evasion checks, or authority lanes;
- CRAP exception registries or production-surface filters;
- trust roots, protected-data custody, source-mutation protection, or retry
  acceptance;
- external-authority suite admission or anti-evasion;
- workspace-wide test execution, release correctness, or another unbounded
  cross-domain change;
- any unknown or ambiguous executable path whose impact cannot be bounded; or
- another explicitly named critical boundary.

A diagnostic, documentation, test-local provider, or retired gate-planner
change is not critical merely because it concerns validation tooling. Classify
its actual invariant.

Unknown or ambiguous production impact receives documented conservative
escalation or authority clarification. Never silently select narrower
coverage.

## 9. Impact Analysis And Command Selection

Inspect:

- the canonical base-to-head diff plus index, worktree, and untracked paths;
- Cargo manifests/lockfile and relevant reverse dependencies;
- declared non-Cargo dependencies and generated assets;
- applicable contracts, test bindings, authority registries, and assurance
  dependencies;
- the package's included/excluded scope and write set; and
- current consumer, conservation, publication, and protected-data boundaries.

Map each affected surface to a canonical command and explain the reason. Use
`docs/standards/local-ci-gate-selection.md` for direct command profiles. Prefer
deterministic and explainable selection. Predictive selection cannot replace
canonical conservative requirements without a separately accepted decision
and measured miss-rate evidence.

## 10. Evidence And Reuse

Retain evidence proportional to the claim. At minimum record:

- exact argv and working directory;
- source commit plus dirty-state identity when applicable;
- relevant input/fixture/config identities;
- start/end or duration for expensive work;
- exit status and concise output/log location; and
- the requirement the result supports.

Evidence may be reused only when source, execution and documentation roots, and
all inputs relevant to the claim are identical or demonstrably excluded.
Review prose and unrelated documentation do not force rerunning unchanged
executable evidence. Never rewrite historical failures or incompatible
receipts as passes.

Local hashes do not provide remote trust by themselves. When a release,
publication, or protected-data boundary explicitly requires stronger
provenance, use that boundary's separately owned identity mechanism. Do not
generalize it into routine increment admission.

## 11. Campaign Deferral

A package may assign a campaign-owned requirement to a later boundary before
implementation when it records:

- the exact obligation;
- why it is not increment scope;
- the owner;
- the trigger/boundary; and
- the evidence required at that boundary.

The package records this directly. No planner ledger is required. Deferred
obligations remain visible and must resolve at their declared boundary.
Failed or current-scope requirements cannot be deferred after execution.

Long campaigns choose documented broad-regression checkpoints based on domain
risk and integration cadence. A planner defect or arbitrary elapsed-time
counter does not create a modeling prerequisite.

## 12. Observational Coverage And CRAP

ADR-0041 governs coverage and CRAP:

- quality observation is optional and operator-directed;
- absence, staleness, debt verdict, or actionable-row count does not block
  increment, campaign, or release transition;
- valid reports carry `closure_eligible=false`;
- ADR-0021's percentages, region/line definitions, function floor, CRAP
  formula/threshold, eligibility taxonomy, adjudication registry, and exception
  discipline remain the quality model; and
- an explicitly authorized CQR, coverage-closure, CRAP-reduction, or module
  test-enhancement package must meet its own declared metric objective.

When operator-directed quality collection runs, measure the complete intended
profile set and retain its exact source/input identity. Quality debt is not
scientific inaccuracy.

## 13. Assurance

Static dependency analysis may suggest reports affected by a change. It cannot
create assurance lifecycle state or replace scientific review.

Preserve distinct:

- validity for the assessed realization;
- campaign-impact disposition;
- campaign-head transfer;
- release transfer; and
- human materiality/approval.

Applicable validity, impact, approval, publication, campaign-transfer, and
release-transfer dispositions remain direct assurance-governance duties and
must resolve at their governing boundary.

## 14. CI And Operational Posture

Validation planning has no executable, CI workflow, trusted runner,
concurrency, attestation, or promotion role. TESTGATE, its forest1 workflow,
planner transitions, receipts, ledgers, recovery, and publication machinery
are frozen historical interfaces under ADR-0043.

Do not dispatch, repair, or extend TESTGATE for prospective work. Existing
bytes and verdicts remain historical evidence. Defunct Omarchy records do not
occupy a live queue.

Optional quality observation and separately authorized release workflows
remain independent of TESTGATE.

## 15. Tool Friction And Stop-Loss

Tool friction is diagnostic evidence, not an excuse to waive correctness.
Roadmap Order 5 applied the advisory-linter stop-loss and deleted the
implementation after it missed critical-obligation, noise, planning-time, and
interaction thresholds. Manual planning remains the only prospective route.
Reintroducing an automated planning tool requires explicit user authorization
and a new package; tool repair never opens a prerequisite or stops originating
work.

## 16. Failure, Flakiness, And Nondeterminism

- A deterministic failure is a failure; do not rerun it into a pass.
- A test classified flaky must retain its failure frequency, owner, and repair
  path. Flakiness does not erase a current failure.
- Infrastructure failure may be retried only under the owning package's
  explicit bounded policy.
- A scientific or model-semantic failure exposed by tooling belongs to the
  originating workflow and must be recorded before cleanup or closeout.
- Cleanup/finalization failure cannot overwrite a safely retained primary
  scientific result.

## 17. Anti-Evasion And Security

Preserve source-mutation protection, explicit argv/working directories,
dependency availability, typed guards, output confinement where required, and
external-authority fixture/required-case bindings.

When touching external-authority suite posture, cohort fixtures, or required
case bindings, run:

```text
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
```

Do not weaken or delete a failing assertion merely to make a migration pass.
Move the assertion to its surviving canonical authority or explicitly retire
it when the protected capability is retired.

## 18. Review And Closure

Review checks:

- declared intent versus exact terminal diff;
- every applicable current-scope requirement and direct result;
- legitimate campaign deferral;
- evidence identity/reuse;
- underlying correctness/science/security/protected-data obligations;
- source-coupled guard migration without semantic weakening;
- line-count governance; and
- truthful package/prompt/catalog status.

Package-required independent review and verification remain binding. Review
economy may reuse unchanged evidence but cannot reduce the required independent
perspective.

## 19. Transition

ADR-0043 and
`docs/work-packages/gate-planner-advisory-linter-roadmap.md` record the
completed removal.
Historical evidence keeps its original bytes and meaning. Historical policy
verification uses the exact frozen policy object named by its registry, never
the current live standard.

The governance-alignment package lands directly under ADR-0043. It does not
require a final planner plan, pre-heavy audit, receipt, TESTGATE transition,
trusted-runner admission, or planner-certified closeout.

## 20. References

- `docs/decisions/0043-gate-planner-is-a-non-authoritative-advisory-linter.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/decisions/0041-separate-testgate-from-observational-quality-ci.md`
- `docs/decisions/0042-science-implementation-and-calibration-readiness.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/work-packages/AGENTS.md`
