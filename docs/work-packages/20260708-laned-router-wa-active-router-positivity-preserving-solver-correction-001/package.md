# WA Active-Router Positivity-Preserving Solver Correction

Status: `EXECUTED-COMPLETE`
Package ID: `20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001`
Owner: Codex
Start date: `2026-07-08`

## Objective

Replace the WA active-router `laned_active_clamp_exceeds_source` fail-closed
outcome with a contract-authorized positivity-preserving solver correction, or
prove that the active explicit TVD-MacCormack path cannot safely support the WA
member without a different solver policy.

This package keeps `SC-OFEROUTE-001` rev-40's clamp-source publication guard
live. It does not promote a target-`dx` mesh policy and does not tune
hydrology, climate, crop, soil, management, disturbed coefficients, or routing
operands.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

Package context:

- `docs/work-packages/20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001/package.md`
- `docs/work-packages/20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001/artifacts/wa-rerun-evidence.md`
- `docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/numerics-adjudication.md`
- `docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/magnitude-attribution.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-fidelity-adjudication.md`

On demand:

- D10B package artifacts under
  `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/artifacts/`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/local-ci-gate-selection.md`

## Scope

### Phase A - Reproduction and Solver Localization

1. Re-run the current WA active `baseline_fixed10` and `dx5` evidence with an
   exact release runner binary and record the fail-closed day/rung.
2. Build package-local diagnostic evidence that localizes the clamp event to
   solver terms: predictor, corrector, final TVD update, boundary flux, CFL,
   or handoff.
3. Preserve the source series, upstream handoff, geometry, friction operands,
   day window, and mesh counts used by the active production path.

### Phase B - Contract-First Correction Authority

1. Amend `SC-OFEROUTE-001` before code if the correction changes normative
   positivity, flux limiting, CFL, boundary, or clamp semantics.
2. Allowed correction classes:
   - bounded conservative flux correction that prevents a substep from
     discharging more water than available local storage plus valid incoming
     flux/source over the same substep;
   - stricter material-negative stage hard failures when no conservation- and
     positivity-preserving correction is authority-backed;
   - instrumentation required to prove the corrected path is the active
     production consumer.
3. Forbidden correction classes:
   - silent fallback to DC01 or shadow routing;
   - clamp-ratio tolerance relaxation or bounded residual ratification for the
     WA material amplification class;
   - route-coefficient, source-producer, climate, crop, soil, or management
     tuning;
   - target-`dx` production promotion;
   - hybrid implicit subsystem revival.

### Phase C - Implementation

Allowed write set:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- focused unit/integration tests needed for the solver and active consumer
- package-local artifacts and prompts
- `docs/work-packages/README.md`

Forbidden write set:

- production mesh-policy promotion or Tier-2 package edits
- hydrology, crop, climate, soil, management, disturbed coefficient, or native
  landuse producer changes
- H2637 re-promotion or hybrid subsystem revival
- rev-27 closure tolerance or rev-40 clamp-source guard relaxation

### Phase D - Verification, Review, and Disposition

Required artifacts:

- `artifacts/required-reading-map.md`
- `artifacts/diagnostic-reproduction.md`
- `artifacts/solver-localization.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/wa-rerun-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

Required gates:

- `git diff --check`
- Markdown/doc lint for touched docs
- contract/profile/BEI checks required by touched `SC-*` contracts
- focused `ofe_routing` solver tests, including D10B oracle/conservation
  surfaces affected by the correction
- focused Lane D active publication/guard tests
- exact release build of the runner used for WA evidence
- WA active fixed `10 cells/OFE` default rerun
- WA active `dx5` rerun
- rev-40 active clamp-source guard proof that the corrected WA runs no longer
  trip `laned_active_clamp_exceeds_source`, or a legitimate hold if they still
  do
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

If any required-case binding, cohort fixture, or external-authority suite
posture is touched, also run:

- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Subagent Authorization

This package explicitly authorizes spawning/delegating to review,
verification, comparator/timing, and solver-localization subagents. Expected
outputs are package-local review, verification, timing/comparator, and
localization artifacts. Subagent write access is bounded to package-local
artifacts unless a subagent is explicitly assigned an implementation fix.

## Completion Criteria

The package may close as `EXECUTED-COMPLETE` only if:

- WA active `baseline_fixed10` and `dx5` no longer fail the rev-40 clamp-source
  publication guard;
- corrected runs satisfy rev-27 seam/cascade/identity closure;
- D10B oracle/conservation surfaces remain acceptable;
- no default/off protected behavior or non-active routing path changes are
  introduced.

If the WA active-router amplification remains after every in-envelope,
authority-backed correction is attempted, close as `EXECUTED-HOLD-*` with the
exact solver blocker, evidence, reason the remaining correction is outside this
package, and the first actionable follow-on package/action.
