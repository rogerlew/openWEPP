# TESTGATE-PLAN-01 Shadow Planner And Receipt Verifier

Package ID: `20260717-testgate-plan-shadow-planner-001`

Queue ID: `TESTGATE-PLAN-01`

Status: `COMPLETE`

Execution date: 2026-07-17

Frozen base: `0873bdae960f7f8c76401845acb476750fdd020e`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. Its `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` sections must remain current.

## Purpose / Big Picture

Implement the first repository-owned, nonblocking planner and verifier for the
ADR-0039 gate contracts. A developer will be able to derive deterministic
intent and terminal plans from declared or observed changes, inspect Cargo and
non-Cargo impact expansion, verify unsigned receipts and evidence reuse without
trust promotion, and replay retained packages to expose selector misses before
any CI enforcement changes. The CLI and library must fail closed on unknown
inputs, incomplete inventories, identity mismatch, invalid DAGs, and unsafe
reuse.

This is shadow implementation only. It does not execute planned commands,
publish evidence, alter CI, reduce current gates, certify campaigns, or mutate
assurance records.

## Progress

- [x] (2026-07-17) Froze the clean base, authority, write set, and package scope.
- [x] (2026-07-17) Added machine-readable gate definitions and representative replay fixtures.
- [x] (2026-07-17) Implemented canonical identities, Git/Cargo/non-Cargo impact expansion, and
  deterministic plan construction.
- [x] (2026-07-17) Implemented receipt, envelope, ledger, and reuse verification.
- [x] (2026-07-17) Ran focused behavior/replay validation and recorded the shadow scorecard.
- [x] (2026-07-17) Completed dual static review/remediation with two exact-tree PASS verdicts.
- [x] (2026-07-17) Ran the single terminal conservative closure set once; format,
  Clippy, full Nextest, and cargo-deny passed, then adjudicated CRAP blocked closure.
- [x] (2026-07-17) Recorded `EXECUTED-BLOCKED-CRAP`; no successful broad gate was repeated.
- [x] (2026-07-18) Bounded successor `20260718-testgate-plan-crap-cleanup-001`
  closed all 12 actionable rows and renewed the full terminal sequence.

## Authority And Rationale

The `TESTGATE-PLAN-01` row in `docs/ROADMAP.md`, ADR-0039,
`docs/standards/testing-and-gate-strategy.md`, and the completed
TESTGATE-ALIGN-01 implementation handoff authorize this package. The v1 schemas
remain the wire-contract authority. The planner must consume them rather than
inventing a parallel representation.

The package is critical gate-policy implementation. Until a later cutover
package proves the adoption scorecard, existing conservative gates remain
authoritative. Test economy is nevertheless binding: development uses only
focused checks, and unchanged broad evidence is not rerun repeatedly. One
terminal conservative closure set is recorded after the implementation tree is
stable.

## Declared Write Set

- `Cargo.toml`
- `Cargo.lock`
- `crates/openwepp-gate-planner/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3_r4.rs`
- `gate-policy/v1/**`
- `tests/integration/testgate_align_authority_contract.rs`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260717-testgate-plan-shadow-planner-001/**`

Read-only discovery may inspect Git history, Cargo manifests and metadata,
retained work packages, gate schemas, tests, and policy authority. Writes
outside this set require a recorded pre-implementation amendment.

## Protected Boundaries

- Keep every generated plan and verdict nonblocking and labeled `SHADOW`.
- Do not add an executor, shell-evaluation path, CI workflow, protected-ref
  publisher, campaign certificate writer, or assurance mutation path.
- Do not reduce or defer a current gate based on shadow output.
- Do not accept unknown executable/normative inputs, empty required inventories,
  malformed Git states, unavailable locked/offline Cargo metadata, identity
  drift, incomplete roots, unverifiable attestation, or unsafe reuse.
- Preserve typed argument vectors and versioned adapters; never evaluate a
  command through a shell.
- Preserve the v1 schema identities and fail-closed state vocabularies unless a
  schema defect is demonstrated and dispositioned in this package.
- Do not claim cryptographic trust from an unsigned receipt or a digest-only
  envelope. Protected trust requires an independently verified attestation
  bundle and current issuer/role/revocation authority.

## Required Deliverables

1. A workspace crate exposing a typed planner/verifier library and a thin
   `openwepp-gate-plan` CLI that writes JSON to a caller-selected path.
2. Canonical JSON SHA-256 identities, stable byte ordering, transitive
   execution/authority/documentation/assurance manifests, and exact root
   reconstruction.
3. Canonical committed and dirty Git change discovery with explicit rejection
   of unmerged, intent-to-add, submodule, sparse, non-UTF-8, or otherwise
   ambiguous states.
4. Locked/offline Cargo metadata parsing, base/head union graph construction,
   reverse-dependency and feature/build-input expansion, plus versioned
   non-Cargo impact edges and critical unknown fallback.
5. Deterministic intent/terminal reconciliation, stable topological DAGs,
   exact expected inventories, and governed zero-work proof.
6. Receipt verification for schema/identity/source/root/DAG/inventory/attempt/
   count/outcome/artifact/mutation consistency, with typed aggregate verdicts.
7. Envelope subject, artifact, issuer, role, revocation, and attestation-bundle
   verification; reuse limited by trust and enforced hermeticity.
8. Deterministic append-only campaign and assurance folds with predecessor/CAS
   ancestry, backstop, certification, and exact-target currency checks.
9. Retained replay fixtures and a scorecard covering at least governance,
   ordinary Rust, Cargo/build input, unknown path, empty inventory, identity
   drift, and unsafe reuse cases.
10. Focused evidence, one terminal conservative closure set, dual independent
    review and verification, line-count governance, handoff, and disposition.

## Plan Of Work

Milestone 1 adds the missing gate-definition registry and retained replay
corpus. Each gate definition owns typed executor arguments, inventory mode,
boundary, trust, reuse, prerequisites, and acceptance. The impact map references
only registered definitions.

Milestone 2 adds `crates/openwepp-gate-planner`. Separate modules own canonical
identity, repository observation, Cargo graph expansion, impact selection, plan
construction, receipt verification, trust/reuse, and ledger folds. The CLI is a
serialization boundary over those library APIs, not an executor.

Milestone 3 replays representative retained changes twice and compares
canonical output bytes. Negative replays mutate one property at a time and must
produce a typed fail-closed verdict. The scorecard records selector misses,
empty inventories, drift, unsafe reuse, and nondeterminism as blockers.

Milestone 4 stabilizes the tree, runs focused checks, then runs the single
terminal conservative closure set. Two independent reviewers inspect the exact
diff, every finding is dispositioned, accepted findings are remediated, and two
independent verifiers inspect the remediated terminal tree.

## Validation And Acceptance

Focused development commands are:

    cargo fmt --check
    cargo nextest run -p openwepp-gate-planner
    cargo nextest run --test testgate_align_authority_contract
    cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings
    git diff --check

Acceptance requires deterministic byte-identical repeat plans; correct
base/head Cargo reverse dependencies; explicit non-Cargo edges; critical
fallback for every unknown; stable topological node order; no empty required
inventory; valid RFC-8785-derived IDs; exact receipt/root/DAG reconstruction;
rejection of subject, signature, issuer, revocation, inventory, outcome, source,
or reuse contradictions; and a retained replay scorecard with no unexplained
selection miss.

After the tree and evidence are stable, run exactly once unless remediation
invalidates the affected result:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check
    bash tools/release/run_adjudicated_crap_gate.sh --base-ref 0873bdae960f7f8c76401845acb476750fdd020e

Do not repeat a passing broad command merely to refresh presentation evidence.
Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` current gate blocks completion.

## Review, Verification, And Line Counts

Two independent reviews must cover architecture, schema conformance, Git/Cargo
correctness, determinism, security/trust, replay adequacy, non-cutover scope,
test economy, and `.rs` line counts. Every finding is `accepted`, `rejected`,
`deferred`, or `follow-up` with rationale. Accepted findings are fixed and
verified. Two independent terminal verifications then inspect the exact
remediated tree and the Gate Evidence Non-Deferral Rule.

Files at or above 2000 lines are `WARN` with decomposition rationale and split
intent. Non-generated files at or above 3000 lines block closure without an
approved owner and sunset exception.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewer roles and two independent
terminal-verifier roles for read-only inspection of the package diff and exact
terminal tree; expected outputs are compact findings/verdicts delivered to the
parent for package artifacts; write access is read-only.

Subagent requirement: REQUIRED for the single terminal full-workspace Nextest,
cargo-deny, and adjudicated-CRAP closure set. This package explicitly authorizes
subagent spawning/delegation to one closure-runner role for those commands;
expected outputs are exact commands, verdicts, counts, timing, and artifact
paths; write access is limited to generated build/coverage output. The parent
must not repeat successful heavy commands. If delegation is unavailable, record
the command-level failure before local execution.

## Idempotence And Recovery

Planning and verification are read-only. Output files use caller-selected paths
and atomic replacement. Repeated planning against identical inputs must produce
identical canonical bytes. Temporary Git/Cargo snapshots live outside the
repository and are removed on success or error. A failed replay or validator
does not modify campaign, assurance, Git, or source state.

## Surprises & Discoveries

- Observation: The v1 impact map references gate-definition IDs but the
  predecessor intentionally did not define their executable registry.
  Evidence: `gate-policy/v1/impact-map.json` names
  `gate-policy-schema-consistency-v1`; no corresponding policy object existed at
  package intake.
- Observation: The repository-wide line-count census found a pre-existing
  3,103-line Rust test module, which blocks closure under current work-package
  governance even though TESTGATE-PLAN-01 did not create it.
  Evidence: the module was mechanically split at a test boundary into 1,685-
  and 1,422-line modules; compile validation preserved the test surface.

## Decision Log

- Decision: Implement a new workspace crate rather than extending the root
  integration guard.
  Rationale: Planning and verification need reusable production APIs, typed
  errors, and a CLI boundary; the existing test remains a schema guard.
  Date/Author: 2026-07-17 / Codex.
- Decision: Keep attestation verification behind a typed external verifier
  interface and fail closed when no verifier or trust authority is supplied.
  Rationale: A receipt/envelope digest is not proof of a signature.
  Date/Author: 2026-07-17 / Codex.
- Decision: Run broad closure once at the stable terminal tree.
  Rationale: This preserves pre-cutover governance while honoring the campaign's
  explicit goal of eliminating redundant test execution.
  Date/Author: 2026-07-17 / Codex.
- Decision: Amend the write set to include the two paths used for the mechanical
  hillslope test-module split.
  Rationale: The split is the minimum behavior-preserving correction for the
  package-wide 3,000-line closure blocker. The amendment was recorded after the
  split, not before it; that ordering defect is retained here rather than
  represented as prior authorization.
  Date/Author: 2026-07-17 / Codex.

## Outcomes & Retrospective

The shadow planner/verifier, retained replay, focused validation, terminal
reconciliation, and dual static review are implemented. The bounded
`20260718-testgate-plan-crap-cleanup-001` successor added three essential
security tests, strengthened the existing reuse test, decomposed the 12
eligible production rows without semantic change, and renewed the conservative
terminal sequence. Format,
workspace Clippy, full Nextest (2,118/2,118), cargo-deny, and fresh adjudicated
CRAP all pass; CRAP closed at 2 raw / 2 existing adjudications / 0 actionable.

TESTGATE-PLAN-01 is complete and TESTGATE-CI-01 may begin. This does not
authorize gate reduction, evidence publication, campaign certification,
assurance mutation, or cutover; those remain governed CI/adoption work.

Revision note (2026-07-17): initial end-to-end scaffold created from the
TESTGATE-PLAN-01 roadmap row and TESTGATE-ALIGN-01 handoff.
