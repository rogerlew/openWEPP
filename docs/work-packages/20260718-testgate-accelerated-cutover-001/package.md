# Accelerate TESTGATE Cutover On `omarchy`

Package ID: `20260718-testgate-accelerated-cutover-001`

Queue ID: `TESTGATE-CUTOVER-01`

Status: `READY / ACTIVE`

Authorization date: 2026-07-18

Planning base: `86bce645ae53d5ef9b984666fdb20206f9a62e7e`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. Keep progress, decisions, discoveries, and outcomes
current so execution never depends on an operator remembering a timer or
scorecard.

## Purpose / Big Picture

Make the TESTGATE planner/executor the normal increment gate as soon as concrete
acceptance passes. First move trusted automatic execution onto an isolated
self-hosted runner on the `omarchy` NUC. Then close any post-change actionable
CRAP, conduct adversarial review with a patch loop, run one exact-candidate
acceptance sequence, and cut over immediately.

Success is visible when trusted `main` pushes use the accepted TESTGATE
aggregate on `omarchy`, ordinary increments no longer invoke the conservative
full runner, the old broad path remains manually callable for critical,
campaign, release, and rollback boundaries, and no elapsed-time or increment-
count scorecard remains active.

## Progress

- [x] (2026-07-18) Accepted ADR-0040, retired the timed/count scorecard as
  prospective authority, and scaffolded this autonomous cutover handoff.
- [x] (2026-07-18) Stopped automatic hosted TESTGATE shadow execution while
  provisioning the trusted runner; retained manual recovery access.
- [x] (2026-07-18) Provisioned and registered the isolated `omarchy` runner and
  recorded capacity,
  labels, permissions, service health, and security boundaries.
- [x] (2026-07-18) Implement trusted TESTGATE routing to `omarchy`, locked
  bootstrap, hosted attestation verification, schedule removal, and public-PR
  exclusion while keeping the provider workflow disabled until acceptance.
- [x] (2026-07-18) Run affected CRAP after implementation and patch every
  actionable row; the exact provider candidate owns the one global CRAP run.
- [x] (2026-07-18) Complete two independent reviews including one adversarial security and
  test-selection review; patch every accepted finding and rerun only affected
  focused gates.
- [ ] Pass the event-driven acceptance matrix and one conservative full-suite
  comparison on the same exact candidate without repeating successful heavy
  commands.
- [ ] Activate the normal TESTGATE aggregate, retain the conservative manual
  lane, remove obsolete fallback instructions, and record immediate cutover.
- [ ] Complete dual terminal verification, documentation integrity checks,
  line-count governance, final disposition, and roadmap/catalog closure.

## Authority And Intent

The user's 2026-07-18 direction and
[`ADR-0040`](../../decisions/0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md)
authorize this package. ADR-0039 and the canonical testing/gate strategy retain
authority except where ADR-0040 explicitly supersedes timed, count, 50%, dual-
required, or nonexistent-provider-rule cutover operands.

This is a critical workflow, executor, evidence, and gate-authority change.
Unknown production or authority impact remains global. Safety, receipt,
confinement, coverage, CRAP, conservation, consumer-path, and external-
authority requirements are not reduced.

## Declared Write Set

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `.github/workflows/testgate-shadow.yml`
- `.github/workflows/testgate-conservative.yml`
- `.github/workflows/testgate.yml`
- `.github/workflows/release-gates.yml`
- `.config/nextest.toml`
- `Cargo.toml`
- `Cargo.lock`
- `crates/openwepp-gate-planner/**`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/iwagaki_oracle.rs`
- `gate-policy/v1/**`
- `tools/ci/omarchy-runner/**`
- `tools/local_ci/**`
- `tools/release/**`
- `tests/integration/testgate_ci_*`
- `tests/integration/testgate_align_authority_contract.rs`
- `tests/integration/testgate_assure_campaign_currency_contract.rs`
- `tests/integration/assurance_dossier_build_contract.rs`
- `tests/integration/hphys0298_paired_lineage_partition_contract.rs`
- `docs/work-packages/20260605-hphys0295-cumulative-storage-budget-ownership-closure-001/artifacts/hphys0295_diagnostics.py`
- `docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/hphys0291_diagnostics.py`
- `docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py`
- `tests/python/test_testgate_shadow.py`
- `tests/python/test_testgate.py`
- `docs/ROADMAP.md`
- `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md`
- `docs/decisions/0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/decisions/README.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/standards/rust-scientific-coding-standard.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `docs/dev-guide/01-orientation.md`
- `docs/dev-guide/07-contributing.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/templates/**`
- `docs/work-packages/20260718-testgate-ci-shadow-executor-001/**`
- `docs/work-packages/20260718-testgate-accelerated-cutover-001/**`

Read-only discovery may inspect GitHub Actions state, repository settings,
official GitHub runner documentation, host capacity, adjacent workflows, and
retained TESTGATE evidence. Host writes are limited to the dedicated isolated
runner guest. Writes outside this set require a recorded pre-implementation
amendment.

Write-set amendment (2026-07-19): the first exact-candidate full-suite run on
the pinned Ubuntu runner proved that the Iwagaki oracle's headline metrics are
bitwise identical while its whole-hydrograph fingerprint has two exact,
platform-specific values (the established development host and Ubuntu 24.04).
The exact oracle test file is admitted solely to bind both observed immutable
fingerprints; production numerics, physics, tolerances, and contract acceptance
remain unchanged.
The stale terminal-assurance contract is also admitted explicitly so it can
continue proving campaign-transfer behavior at the supported `INCREMENT`
planner boundary; the production transfer trigger remains terminal-stage
driven.
The exact assurance-dossier special-entry test and historical HPHYS0298 guard
test are admitted prospectively after exact-candidate full-suite evidence
isolated runner-specific failures in those two tests. Remediation is limited to
preserving the same fail-closed assertions with portable temporary paths and
actionable failure diagnostics; assurance publication and snow-physics
authority remain unchanged.
The HPHYS0295 diagnostic imported transitively by the HPHYS0298 guard is also
admitted prospectively after the focused runner reproduction exposed its
hard-coded development-host repository root. Remediation is limited to deriving
the repository root from the artifact's own location, matching the adjacent
HPHYS0296 through HPHYS0298 diagnostics; no process math, inputs, outputs, or
acceptance criteria may change.
The same focused reproduction then exposed identical hard-coded roots in the
transitive HPHYS0291 and HPHYS0265 imports. Those two exact artifacts are
admitted under the same restriction so the full import chain resolves from its
checked-out repository rather than one developer workstation.

## Protected Boundaries

- The public repository's untrusted `pull_request` code never executes on
  `omarchy`. Do not use `pull_request_target` to bypass this boundary.
- Run the GitHub runner in a supported isolated Linux guest, not directly in
  the uninspected desktop host. Use a repository-scoped,
  unprivileged account with no `sudo`, host home mounts, unrelated data roots,
  privileged container socket, or reusable secrets.
- Keep workflow permissions read-only unless one narrowly named publication
  step later proves a stronger permission is required. Never persist a runner
  registration token or expose it in logs or package artifacts.
- Admit automatic jobs only from trusted pushes to `main`. Manual dispatch must
  resolve and verify a commit in this repository before execution.
- Preserve typed plan arguments, fail-closed unknown escalation, exact source
  identity, atomic receipts, source-mutation detection, external output
  confinement, and process-group timeout termination.
- Do not weaken ADR-0021 coverage/CRAP thresholds or exception discipline.
- Do not create a branch-protection/ruleset migration requirement when the
  provider has no current protection or ruleset. Adding one is outside scope
  unless separately directed.
- Do not repeat a successful full Nextest, global CRAP, or exact-candidate
  acceptance run for presentation evidence.

## Required Deliverables

1. An idempotent `omarchy` runner setup guide or script that creates a supported
   isolated Linux guest, dedicated account, repository-scoped registration,
   pinned labels, reviewed image-based runner updates, service management,
   cache/storage limits, health inspection, and clean removal without storing
   registration tokens or repository personal-access tokens.
2. A capacity and security receipt recording CPU, memory, disk, guest OS,
   runner version, labels, outbound connectivity, permissions, forbidden host
   access, and GitHub-visible online/idle state.
3. Workflow routing with exact labels such as
   `[self-hosted, Linux, X64, openwepp, omarchy, trusted]`, no daily schedule,
   no self-hosted public-PR path, concurrency one for the NUC, bounded timeouts,
   and manual conservative fallback.
4. A deterministic bootstrap that installs or verifies pinned tools once,
   fetches the locked dependency graph before offline metadata, reuses that
   same-job cache for execution, and proves a cold writable-surface execution
   without source-tree pollution or cross-job executable state.
5. Current affected and global CRAP evidence after implementation, with every
   actionable row corrected and no broad rerun between accepted evidence and
   cutover.
6. Two independent reviews. One must be adversarial and cover public-repository
   runner compromise, event/ref admission, token/permission scope, cache and
   workspace contamination, plan/receipt fabrication, selector false
   negatives, output escape, rollback, and test-economy regressions. Every
   accepted finding enters an implementation patch loop before acceptance.
7. One event-driven acceptance record on an exact candidate commit proving:
   documentation-only selection; bounded component selection and affected
   quality; integrated/critical selection; unknown-impact global escalation;
   FAIL/BLOCKED receipts; cold-cache bootstrap; source/output confinement;
   untrusted PR non-routing; normal trusted `main` routing; and manual
   conservative rollback.
8. Immediate cutover edits that make TESTGATE authoritative for normal trusted
   increments, preserve broad critical/campaign/release qualification, remove
   obsolete conservative-every-package instructions, retire the old scorecard,
   and close the roadmap item without a monitoring handoff.

## Execution Plan

### Phase 0: Contain Hosted Churn

Disable the active `testgate-shadow` workflow through the provider before host
provisioning. Record the workflow state and the three known cold-cache failures.
Do not enable the already manually disabled `release-gates` workflow. The first
committed workflow revision removes the daily schedule so re-enabling TESTGATE
cannot restart timed observation.

### Phase 1: Provision `omarchy`

Inspect the NUC without changing it, then create the smallest supported
Ubuntu/Debian guest that can use the available CPU, memory, and SSD. Create a
dedicated runner account and repository-scoped runner with reviewed image-based updates
and exact labels. Keep the guest disposable and isolated from host homes,
credentials, Docker sockets, and homelab data. Install the runner as a service,
verify outbound GitHub connectivity, and record online/idle state through the
GitHub API.

If a supported isolated guest cannot be created, stop at a truthful host-
provisioning blocker; do not install a privileged persistent runner directly on
the public-repository desktop host.

### Phase 2: Route And Bootstrap

Change TESTGATE automatic triggers to trusted `main` pushes and explicit manual
dispatch only. Route all substantive jobs and their aggregate to the exact
`omarchy` label set with concurrency one and job timeouts. Remove repeated
`cargo install` work from ordinary jobs. Provision pinned tools once and add an
explicit locked dependency fetch before the planner's offline Cargo metadata.

Exercise one empty-cache job whose build reuses the dependencies fetched by its
bootstrap step. It must produce valid plans and receipts in an external
artifact root, then purge writable job state. A public pull-request event must
be statically and dynamically proven unable to select the self-hosted runner.

### Phase 3: CRAP Cleanup

Run affected CRAP for the implementation diff. Correct every actionable row
through behavior-preserving decomposition or missing focused tests. After the
source stabilizes, run global adjudicated CRAP once on the exact candidate and
require an empty actionable set. Reuse that receipt in terminal acceptance.

### Phase 4: Adversarial Review And Patches

Obtain two independent exact-diff reviews, including the mandatory adversarial
review. Disposition every finding as accepted, rejected, deferred, or follow-up
with evidence. Patch all accepted findings in this package. Rerun only the
focused gates affected by a patch; a patch that changes the exact candidate
invalidates only evidence whose bound inputs changed.

### Phase 5: Acceptance And Immediate Cutover

Freeze one candidate commit and run the acceptance matrix. Run the conservative
full-suite comparison exactly once; reuse the Phase 3 global CRAP receipt if its
bound source is unchanged. The operator's accepted 48.8% benchmark is the
performance decision and is not re-litigated.

When the matrix, reviews, full comparison, and CRAP closure pass, immediately
rename/promote the TESTGATE aggregate from shadow observation to normal gate
authority, re-enable it, and dispatch the accepted candidate. Remove the old
ordinary-increment broad trigger and conservative fallback wording from active
instructions. Keep broad execution available only at critical, campaign,
release, or explicit rollback boundaries. Do not start an observation timer or
create a follow-up scorecard.

## Validation And Acceptance

Focused development selection is driven by the accepted terminal plan. At
minimum, workflow/schema/static checks, planner/executor tests, runner setup
syntax checks, and the TESTGATE integration contract run after their surfaces
change. The package records exact commands after host and implementation
discovery; it does not prescribe redundant workspace runs in advance.

The single exact-candidate conservative comparison is:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check
    bash tools/release/run_adjudicated_crap_gate.sh --base-ref 86bce645ae53d5ef9b984666fdb20206f9a62e7e

Global CRAP may be satisfied by the unchanged Phase 3 receipt. Each other heavy
command runs once on the stable candidate. Accepted review patches invalidate
and rerun only commands whose bound inputs changed.

Cutover acceptance has no minimum elapsed time, increment count, duplicate
environment count, 50% threshold, or dual-required interval. Any concrete
safety failure, nonempty actionable CRAP set, unresolved accepted finding,
failed acceptance case, or unavailable rollback blocks cutover.

## Review, Verification, And Line Counts

Two independent reviews cover implementation and acceptance, with one assigned
the adversarial scope above. Two terminal verifiers inspect the exact accepted
tree, event routing, evidence reuse, Gate Evidence Non-Deferral compliance, and
cutover truthfulness. Every finding is dispositioned and every accepted finding
is patched before closure.

Files at or above 2,000 lines are `WARN` and require decomposition rationale;
non-generated files at or above 3,000 lines block closure without an approved
owner and sunset exception.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewer roles, two terminal-verifier
roles, and one heavy closure-runner role for the scopes above; expected outputs
are compact findings, verdicts, exact commands, counts, timings, and artifact
paths delivered to the parent; write access is read-only except generated
build/coverage output for the closure runner. The parent must not repeat
successful heavy commands.

## Idempotence And Recovery

Runner provisioning must be repeatable without registering duplicates or
retaining expired tokens. Every execution uses a fresh external artifact root;
normal dependency/tool caches are content caches, not accepted evidence. A
failed cutover restores the manual conservative lane and disables the normal
TESTGATE trigger until the concrete failure is patched. Rollback never restores
calendar observation or broad testing on every ordinary increment.

## Surprises & Discoveries

- Live provider inspection on 2026-07-18 found `testgate-shadow` active,
  `release-gates` manually disabled, no registered self-hosted runners, no
  branch protection, and no repository rulesets.
- Runs `29647587353`, `29658898454`, and `29663821156` failed at offline Cargo
  metadata because `alloc-no-stdlib v2.0.4` was absent from the cold cache.
- The repository is public. GitHub's current documentation says standard
  GitHub-hosted Linux execution for public repositories is free, while also
  warning that persistent self-hosted runners should not execute untrusted
  public-repository pull-request code.
- The first adversarial reviews correctly found unauthenticated local receipt
  promotion, post-hoc intent authorization, co-located/weakened rollback,
  documentation false escalation, missing executable authority classes, and a
  job-writable persistent runner control plane. Cutover remained disabled while
  these findings entered the patch loop.
- Exact-candidate runs `29677049559` and `29677779525` proved that full-suite
  repository reconstruction needed both serialized snapshot fixtures and a
  larger dynamically allocated target ceiling. The latter run selected all
  `2,163` tests and reduced failures to four storage reconstructions plus two
  portable-fixture defects; global CRAP remained blocked and did not run.
- Run `29678738923` failed in planner preflight before any gate ran because the
  selected comparison base left no `package.md` change in the increment. The
  frozen acceptance base is therefore `c90d4dab`: that commit prospectively
  authorizes the complete portable-runner write set, while this living-package
  update supplies the required explicit intent delta.
- Run `29678990162` executed the full Nextest command successfully with `2,163`
  observed tests, then failed its evidence adapter because the planner had also
  counted the five explicit `#[ignore]` mismatches returned by `nextest list`.
  The adapter's `expected 2168, observed 2163` error proves the command reached
  PASS artifact validation; the ignored-test accounting patch receives focused
  validation, and the successful full command is not repeated.
- The first fresh global-CRAP invocation stopped before test execution because
  the immutable runner image lacked `llvm-tools-preview`; the read-only runtime
  correctly blocked an on-demand toolchain mutation. The component is now a
  pinned image input and a verified bootstrap prerequisite.
- The legacy global coverage branch bypassed Nextest and exhausted the
  container's small non-executable `/tmp` while concurrently reconstructing
  planner fixtures. Global and affected coverage now share the canonical
  Nextest path, external executable temporary storage, and profile-specific
  serialization; global defaults to the `full` profile.
- Because `cargo llvm-cov report` does not support `--workspace` and an empty
  report selector is incomplete here, the global report expands locked Cargo
  metadata into a sorted, nonempty repeated `--package` list before LCOV
  publication.
- The first canonical Nextest coverage attempt after that repair failed closed
  because the two adjudicated-CRAP gate definitions still bound the adapter's
  pre-repair SHA-256. The definitions now bind the repaired adapter bytes;
  this is a policy-integrity correction, not a test-behavior change.

## Decision Log

- Decision: use an event-driven cutover and prohibit a replacement timer.
  Rationale: the operator explicitly rejected 14-day/20-increment tracking and
  accepted the measured 48.8% savings.
  Date/Author: 2026-07-18 / Roger Lew and Codex.
- Decision: isolate the runner in a supported Linux guest on `omarchy` and
  accept only trusted repository events.
  Rationale: the host OS and isolation are not yet proven compatible, and the
  repository is public.
  Date/Author: 2026-07-18 / Codex.
- Decision: do not create a provider migration gate.
  Rationale: live inspection found no branch protection or ruleset to migrate.
  Date/Author: 2026-07-18 / Codex.
- Decision: amend the isolated guest implementation to an unprivileged Ubuntu
  24.04 container on `omarchy` before implementation edits.
  Rationale: read-only intake proved the host is Arch Linux, Docker 29.2.1 is
  installed and unused, and no VM manager is present. A pinned Ubuntu userspace
  with dedicated volumes, no Docker socket, no host-home mounts, and trusted-
  event-only routing meets the isolation objective with substantially less
  setup than installing a new hypervisor.
  Date/Author: 2026-07-18 / Codex.
- Decision: amend the declared write set before review remediation to include
  the independent conservative workflow and every active guidance surface
  identified by the governance review.
  Rationale: rollback must remain callable when normal TESTGATE is disabled,
  and obsolete conservative-every-increment wording must be removed
  consistently. Publishing this amendment before the implementation candidate
  also provides a base-commit authorization source for intent admission.
  Date/Author: 2026-07-18 / Codex.
- Decision: extend that admission amendment to the prompt-wording and
  code-quality-refactor standards before their obsolete fallback wording is
  committed.
  Rationale: the active-guidance sweep found the same pre-cutover fallback on
  those two canonical surfaces after the first amendment. The provider
  TESTGATE workflow remained disabled during both admission commits.
  Date/Author: 2026-07-18 / Codex.
- Decision: add the TESTGATE authority integration contract to the prospective
  write set before the implementation candidate.
  Rationale: an authorization dry check found that `testgate_ci_*` does not
  match the separately named schema/authority contract. The workflow remained
  disabled while this correction was published.
  Date/Author: 2026-07-18 / Codex.
- Decision: grant `id-token: write` and `attestations: write` only to the normal
  TESTGATE workflow so `actions/attest` can bind the exact unsigned receipt and
  custom predicate to GitHub's repository/workflow identity.
  Rationale: the executor truthfully emits `LOCAL_UNTRUSTED`; normal increment
  authority requires a separately verifiable repository-reviewed envelope.
  Checkout credentials remain disabled and contents remain read-only.
  Date/Author: 2026-07-18 / Codex.
- Decision: make runner registration state and the container root filesystem
  read-only during jobs, use size-bounded tmpfs writable surfaces, purge them
  after every job, and disable in-place runner updates.
  Rationale: trusted-main dependencies must not be able to replace the runner
  control plane or poison later jobs. A reviewed image revision is the runner
  update mechanism; cross-job warm executable caches are intentionally traded
  away for persistence safety.
  Date/Author: 2026-07-18 / Codex.
- Decision: separate substantive execution from authority minting.
  Rationale: the self-hosted job has contents-read permission only. A tokenless
  GitHub-hosted verification job independently reconstructs selection and
  Nextest/A3 inventory, validates the receipt and predicate, and uploads the
  verified immutable evidence. A separate minimal hosted aggregate receives
  OIDC/attestation authority, runs no candidate checkout, build, or Python,
  mints the native attestation, and verifies repository, workflow, source ref,
  source digest, predicate type, and hosted signer identity.
  Date/Author: 2026-07-18 / Codex.
- Decision: admit explicit A0, hard-invariant A1, and inventory-backed A3
  adapters; the broad workspace suite does not claim A1.
  Rationale: science-contract admission, hard-invariant mapping, broad
  regression, and required external authority have distinct meanings. A0
  blocks a changed science surface without an executable hard-fail A1 binding,
  validates every declared applicable A3 mapping, and admits empty A3 mappings
  only when the authority registry has no applicable suite. Generic successful
  processes cannot manufacture scientific conformance.
  Date/Author: 2026-07-18 / Codex.
- Decision: use the pre-edit `package.md` as the prospective intent authority
  for this transition, then materialize and reconcile machine intent/terminal
  plans against its exact base-commit digest at execution.
  Rationale: root `AGENTS.md` explicitly authorizes `package.md` intent during
  transition. The base package existed and authorized the complete write set
  before implementation; the machine record does not retroactively expand it.
  Date/Author: 2026-07-18 / Codex.
- Decision: retain the planner, executor, and verifier files in the 2,000-line
  warning band for this cutover, while extracting execution-environment
  projection from `planner.rs` so every non-generated file remains below the
  3,000-line hard ceiling.
  Rationale: each remaining file is one versioned wire-contract state machine;
  splitting those state transitions during terminal acceptance would increase
  review risk. The new `execution_context.rs` boundary removes a coherent
  responsibility, and the focused `nextest_inventory.rs` boundary owns
  execution-selection parsing. Final counts are `planner.rs` at 2,963 lines,
  `nextest_inventory.rs` at 70, `executor.rs` at 2,514, and `verifier.rs` at
  2,515. Further decomposition belongs to a later authorized refactor, not this
  cutover.
  Date/Author: 2026-07-18 / Codex.

## Outcomes & Retrospective

Not executed. This package is ready and owns the complete remaining TESTGATE
path; there is no separate observation or timed-cutover follow-on.
