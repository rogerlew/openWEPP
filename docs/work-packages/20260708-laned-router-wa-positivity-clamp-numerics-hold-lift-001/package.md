# WA Positivity-Clamp Numerics Hold-Lift

Status: `EXECUTED-HOLD-SOLVER-CORRECTION-REQUIRED`
Package ID: `20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001`
Owner: Codex
Start date: `2026-07-08`

## Objective

Resolve the active-router positivity-clamp numerics hold exposed by the WA
selected-cohort mesh ladder. The package must either:

- close the defect with contract-authorized implementation and evidence, or
- stop at an explicit executed hold that names the remaining solver blocker,
  evidence, and first follow-on action.

This package does not promote a target-`dx` mesh policy and does not tune
hydrology, climate, soil, crop, management, or disturbed-route coefficients.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/package.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-summary.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-fidelity-adjudication.md`
- `docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/package.md`
- `docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/timing-and-closure-refresh.md`
- `docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/clamp-numerics-investigation.md`
- `docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/worker-handoff.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

## Scope

### Phase A - Evidence Reproduction and Localization

1. Reuse the prior WA run evidence to identify the active lane/day/rung defect
   surface.
2. Re-run at least the retained fixed `10 cells/OFE` active default and one
   affected fine or candidate WA rung on the current tree with release-binary
   provenance.
3. Record whether the first failure is closure residual, material clamp
   magnitude, material stage negativity, or another typed router error.

### Phase B - Contract-First Guard or Solver Fix

1. Amend `SC-OFEROUTE-001` before implementation if the package changes the
   normative meaning of positivity clamps, stage validity, or fail-closed
   behavior.
2. Prefer the narrowest authority-backed correction:
   - sub-dry negative-depth excursions may be clamped and booked;
   - material negative depths in predictor, corrector, or final stages must
     fail closed;
   - no surrogate physics, coefficient tuning, or silent fallback.
3. If a true solver-stability correction is required beyond this package's
   safe write set, stop with an executed hold and first follow-on package.

### Phase C - Implementation

Allowed write set:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- focused runner/orchestrator tests needed to prove the active consumer sees
  the corrected fail-closed behavior
- package-local artifacts and prompts
- `docs/work-packages/README.md`

Forbidden write set:

- production mesh-policy promotion
- hydrology, crop, climate, soil, management, or disturbed coefficient changes
- H2637 re-promotion or hybrid subsystem revival
- Tier-2 mesh re-scope edits except for package-local cross-reference

### Phase D - Verification, Review, and Disposition

Required evidence:

- `artifacts/diagnostic-reproduction.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/wa-rerun-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- worker handoff for any remaining solver-correction or target-`dx` reopening

Required gates:

- `git diff --check`
- Markdown/doc lint for touched docs
- `cargo fmt --check`
- focused `ofe_routing`/Lane D active tests
- release build of the runner used for WA evidence
- WA active fixed `10 cells/OFE` default rerun
- at least one WA affected rung rerun (`dx5`, `dx2p5`, or narrower if runtime
  requires)
- contract/profile/BEI checks required by touched `SC-*` contracts
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

If any required-case binding, cohort fixture, or external-authority suite
posture is touched, also run:

- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Subagent Authorization

This package explicitly authorizes spawning/delegating to review,
verification, and comparator/timing subagents. Subagent write access is bounded
to package-local review, verification, timing, and comparator artifacts unless a
subagent is explicitly assigned an implementation fix.

## Completion Criteria

The package may close as `EXECUTED-COMPLETE` only if the WA clamp numerics
defect is either fixed or fail-closed by contract-authorized behavior, with
active consumer evidence and regression gates recorded.

If WA active routing still requires a deeper solver correction, close as
`EXECUTED-HOLD-*` with the exact blocker, evidence proving it, why it is outside
this package's safe closure, and the first follow-on action.
