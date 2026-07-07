# LANED-HYB-GAP001 — Shock-Quiet Hold-Lift Design Increment

Status: **ACTIVE** (2026-07-07, operator-directed: "scaffold and execute the
GAP-OFEHYB-001 hold-lift design increment").

## Objective

Close, or prove still-held, `SC-OFEROUTE-002#GAP-OFEHYB-001`: the hybrid
switching predicate currently routes source-quiet but shock-carrying bins
implicitly, and the Case-4 hybrid ladder fails the parent `5%` peak tolerance.

This package is a contract-first design increment. It may amend
`SC-OFEROUTE-002`, add contract-derived tests, and update the Lane-D hybrid
switching implementation and retained D-val harness. It must not change the
explicit TVD-MacCormack scheme, friction physics, source authority, default
behavior, or production activation policy.

## Scope

Included:
- Required package scaffold, catalog entry, active prompt, artifacts, and
  final disposition.
- Contract-first amendment of `SC-OFEROUTE-002` for the selected
  shock-quiet/cool-down predicate.
- Exploratory design evidence comparing:
  - the recorded explicit cool-down fallback,
  - a state-aware spatial wave-quiet predicate if it can be justified from
    contract evidence,
  - and the current source-free predicate baseline.
- Implementation only if backed by `SC-OFEROUTE-002`.
- Contract-derived focused tests for the switching predicate and retained
  Case-4 hybrid ladder.
- H2637-class active hybrid timing/profile evidence if the Case-4 ladder
  passes, or hold evidence if it does not.
- Package-local review, verification, gate results, disposition, and worker
  handoff.

Excluded:
- Default/D16 promotion.
- Tier-1 solve-cost optimization (`GAP-OFEHYB-002`) except for measuring the
  timing impact of any predicate change.
- Tier-2 mesh-resolution policy changes.
- Surrogate physics, empirical compatibility wrappers, or silent fallbacks.
- Weakening the parent Case-4 acceptance tolerance.

## Required Reading

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/package.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/ratification-evidence.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/hold-legitimacy-audit.md`
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i0-scheme-design.md`
- Package-local `artifacts/required-reading-map.md`

Conditional:
- `docs/specifications/science-contract-authoring-procedure.md` if the
  contract schema/profile requires repairs beyond a normal amendment.
- `docs/standards/local-ci-gate-selection.md` when selecting narrowed
  iteration gates before final closure.

On-demand:
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- Runner active-lane files only if H2637 timing/profile output requires
  manifest/profile changes.

Required-reading budget:
- local_required_bytes_total: 215000 (estimated from core files)
- threshold_outcome: OK (`<=400000` bytes)
- map: `artifacts/required-reading-map.md`

## Write Set

Primary:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` only if
  parent pointer/version synchronization is required.
- `docs/specifications/science-contracts/index.md` only if lifecycle metadata
  changes.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/`
- `docs/work-packages/README.md`

## Phase Plan

### Phase A - Scaffold And Baseline

Create the package scaffold, active prompt, required-reading map, initial
gate table, and README/catalog entry. Record clean starting tree and current
baseline hold metrics.

### Phase B - Design Exploration

Run small, local design probes to determine whether the recorded explicit
cool-down fallback or a state-aware wave-quiet predicate can lift Case-4
without weakening the parent acceptance surface. Record commands, metrics,
and rejected candidates in `artifacts/design-evidence.md`.

### Phase C - Contract-First Amendment

Amend `SC-OFEROUTE-002` before code edits. The amendment must:
- define the selected predicate using deterministic, typed inputs,
- preserve exact ledgers and rev-30 deficit-carry behavior,
- preserve default/off and all-explicit bit identity,
- define test-vector obligations and guard-map evidence,
- update `GAP-OFEHYB-001` disposition if and only if the acceptance evidence
  supports it.

### Phase D - Implementation

Implement only the contract-authorized predicate in the hybrid composition and
the retained Case-4 hybrid harness. Do not add compatibility wrappers. Do not
weaken the implicit solve determinism or closure guards.

### Phase E - Ratification And Timing Gates

Run and record:
- Case-4 hybrid oracle ladder.
- Focused `ofe_routing` tests for switching/composition.
- H2637 active hybrid timing/profile if the Case-4 ladder passes.
- Protected off/default identity only if a default/off surface is touched.

If Case-4 still fails, stop at `EXECUTED-HOLD-GAP-OFEHYB-001-*` and record
the exact blocker, evidence, rejected in-envelope routes, and first follow-on.

### Phase F - Review, Verification, And Disposition

Complete dual review, finding disposition, verification, gate results,
line-count governance, final disposition, and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `explorer` subagents for comparator/timing execution,
code correctness review, QA/maintainability review, and bounded codebase
questions. Expected outputs are package-local `artifacts/review-*.md`,
`artifacts/verification-*.md`, compact timing/comparator metrics, and command
log paths. Write access is read-only for review/verification/comparator
subagents unless a worker is explicitly assigned a disjoint implementation
write set.

Subagent requirement: REQUIRED for heavy batch/closure/comparator runs,
including full workspace `nextest`, release timing runs, comparator ladders,
and `cargo deny check`, unless the subagent tool is unavailable. If
unavailable, record the tool-policy block before running locally.

## Required Gates

- `git diff --check`
- Markdown/doc lint for touched docs
- Contract/profile/BEI checks required by touched `SC-*` contracts
- SC unit-compliance checks for touched contracts
- Focused `ofe_routing` tests
- Case-4 hybrid oracle ladder
- H2637-class active hybrid timing/profile with
  `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1` if Case-4 passes
- Protected-output byte identity with subsystem off if any default/off surface
  is touched
- `.rs` line-count governance
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

Authority anti-evasion guard is required if any required-case binding, cohort
fixture, or external-authority suite posture is touched:
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Exit Criteria

Complete only if:
- `SC-OFEROUTE-002` authorizes the selected predicate.
- The retained Case-4 hybrid ladder passes the parent acceptance criteria at
  every rung.
- Current H2637 active hybrid evidence is recorded and any timing regression is
  honestly dispositioned.
- Focused tests, docs/contract gates, Rust closure gates, review, and
  verification pass.

Hold if:
- No in-envelope predicate passes Case-4 without weakening authority.
- The selected predicate requires new physics or empirical tolerance
  ratification outside this package.
- Heavy required gates cannot be run or reviewed and no equivalent evidence is
  available.
