# TESTGATE-ALIGN-01 Governance Alignment And Schemas

Package ID: `20260717-testgate-align-governance-schemas-001`

Queue ID: `TESTGATE-ALIGN-01`

Status: `COMPLETE-BOUNDED-USER-DIRECTED`

Execution date: 2026-07-17

Frozen base: `371988a787281416226658b5e6ef6ebf56f98e0a`

## Objective

Make ADR-0039's campaign-scoped, risk-based testing authority internally
consistent and mechanically consumable before any selector can block work.
Replace duplicated every-package full-gate wording with short lifecycle
pointers, amend ADR-0021's cadence without changing its quality thresholds,
define strict versioned impact-map, plan/DAG, receipt/envelope,
campaign-ledger, and assurance-impact JSON schemas, and enforce those contracts
with fixtures and a source-level consistency test.

## Authority And Rationale

The package is authorized by the `TESTGATE-ALIGN-01` `next` row in
`docs/ROADMAP.md`, ADR-0039, and the completed authority package's
`artifacts/implementation-handoff.md`. Existing conservative execution remains
in force for this transition. No planner, executor, receipt verifier,
CI-lane cutover, affected-coverage runner, evidence publisher, or assurance
mutation path is implemented here.

This is a critical gate-policy increment under
`docs/standards/testing-and-gate-strategy.md` section 7.4. Its terminal tree
therefore receives campaign-strength full regression and global CRAP even
though its executable source addition is a contract guard rather than
production simulation code.

## Declared Write Set

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `Cargo.toml`
- `gate-policy/v1/**`
- `tests/integration/testgate_align_authority_contract.rs`
- `docs/codex_exec_plans.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/standards/rust-scientific-coding-standard.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/work-packages/templates/cqr-nightly-package.md`
- `docs/work-packages/templates/cqr-nightly-kickoff-prompt.md`
- `docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `docs/dev-guide/01-orientation.md`
- `docs/dev-guide/07-contributing.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/work-packages/20260613-refactor022-mofe-scheduler-runner-watershed-line-count-split-001/package.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260717-testgate-align-governance-schemas-001/**`

Read-only discovery may cover every transition-inventory surface named by the
predecessor handoff. Writes outside this set require a pre-implementation
package amendment or, after implementation begins, a recorded finding and a
separately authorized follow-up.

Finding-driven amendment (2026-07-17): independent governance review found
three prospective guides with contradictory universal-frequency language that
the initial transition inventory missed. The user's direct instruction to
scaffold and execute TESTGATE-ALIGN-01 authorizes completing the package's
repository-alignment objective, so the three exact paths above are added as a
bounded follow-on write set before remediation. No historical package evidence,
kernel behavior, CI implementation, or release tooling enters scope.

## Protected Boundaries

- Do not change ADR-0021's coverage percentages, per-function floor, CRAP
  threshold 30, eligibility taxonomy, exception discipline, or global
  empty-actionable-set objective.
- Do not weaken A0/A1/A3 admission, authority-suite, contract-derived,
  fail-closed, consumer-path, conservation, reconstruction, anti-tautology,
  review, or release obligations.
- Do not make test selection agent-dependent or add a generic skip, bless,
  accept-current, or downgrade operation.
- Do not change `.config/nextest.toml`, release/CRAP scripts, CI workflows,
  assurance implementation/data, or public/generated assurance surfaces.
- Do not claim that schemas alone constitute a planner, executor, trusted
  receipt, campaign certificate, or enforcement cutover.

## Required Deliverables

1. Aligned instructions and standards that point gate frequency and lifecycle
   to ADR-0039's canonical strategy while retaining specialized obligations.
2. An ADR-0021 cadence amendment and correctness-authority lane wording that
   distinguish affected increment gates, campaign closure, and release.
3. `gate-policy/v1/` with strict JSON Schema Draft 2020-12 contracts for the
   impact map, plan with typed DAG nodes, unsigned receipt, attestation
   envelope, campaign ledger, and assurance impact record.
4. A minimal versioned non-Cargo impact map plus valid and invalid fixtures for
   every schema. Schemas default closed through `additionalProperties: false`
   or `unevaluatedProperties: false` and use closed enums for governed states.
5. A Rust integration contract that compiles every schema, validates every
   positive fixture, rejects every negative fixture, checks schema identity,
   and guards the primary governance pointers and protected thresholds.
6. Package intake, implementation evidence, gate results, dual reviews,
   finding disposition, dual terminal verification, line-count governance,
   transition inventory, handoff, and final disposition artifacts.

## Execution Plan

### Phase 1 — Scaffold And Intake

Freeze the base and write set, resolve applicable instructions for every path,
inventory duplicated cadence language, and create the active kickoff prompt.

### Phase 2 — Governance Alignment

Amend instruction and standard surfaces so the testing/gate strategy owns
lifecycle selection. Preserve document-specific obligations and use explicit
critical, campaign, and release language where a full gate remains required.

### Phase 3 — Schema Contracts And Fixtures

Add the versioned gate-policy directory, schemas, seed impact map, and compact
positive/negative fixtures. Add the source-level Rust contract and register it
as an integration-test target.

### Phase 4 — Focused Validation And Remediation

Run the schema/authority contract, documentation/reference checks,
anti-evasion scans applicable to gate-policy changes, formatting, Clippy, and
diff hygiene. Fix all in-scope defects before broad validation.

### Phase 5 — Critical Closure, Review, And Verification

Run full workspace Nextest, cargo-deny, full global adjudicated CRAP against
the frozen base, and line-count governance. Obtain two independent reviews,
disposition every finding, remediate accepted findings, rerun invalidated
gates, then obtain two independent terminal verifications of the exact amended
tree.

### Phase 6 — Disposition

Record gate truth, remaining transition debt, the first actionable
`TESTGATE-PLAN-01` handoff, catalog completion, and roadmap advancement.

## Gates And Exit Criteria

- Applicable instruction chains are recorded for every write-set class.
- Every required governance surface points to the canonical strategy and no
  changed surface retains a contradictory every-package frequency rule.
- ADR-0021 thresholds/taxonomy are byte-for-byte semantically preserved while
  its cadence follows ADR-0039.
- Every schema and fixture passes the focused source guard; every negative
  fixture is rejected for its intended invariant.
- `git diff --check` and scoped Markdown/reference checks pass.
- `cargo fmt --check` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `cargo nextest run --workspace --profile full` passes.
- `cargo deny check` passes.
- `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 371988a787281416226658b5e6ef6ebf56f98e0a`
  produces fresh full-workspace evidence with zero actionable rows.
- Gate-policy anti-evasion/source consistency checks pass.
- Two independent reviews are fully dispositioned; accepted findings are
  fixed and verified.
- Two independent terminal verifications pass the exact amended tree.
- `.rs` files at or above 2000 lines are `WARN`; nonexempt files at or above
  3000 lines block closure until refactored or explicitly governed.
- Security impact is reviewed: schemas and guards are fail closed; no
  credential, executable confinement, issuer, CI permission, or publication
  behavior changes in this package.

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` gate prevents completion. A
campaign-owned future implementation obligation is not a current gate unless
listed above; current gates may not be relabeled as later work.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent review subagents for read-only review of
the complete changed tree and to two independent terminal-verification
subagents for read-only verification after finding remediation; expected
outputs are compact review and verification reports delivered to the parent
for package artifacts; write access is read-only. The parent owns all source,
governance, schema, fixture, evidence, and finding-disposition edits.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one closure-runner subagent for the full Nextest,
cargo-deny, and adjudicated-CRAP commands when a concurrency slot and suitable
runner are available; expected outputs are exact command results and artifact
paths; write access is limited to generated untracked build/coverage output and
the parent records durable evidence. Local parent execution is an allowed
equivalent when delegation is unavailable.
