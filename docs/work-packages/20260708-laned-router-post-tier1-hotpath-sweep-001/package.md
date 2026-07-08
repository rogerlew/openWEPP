# Lane D Post-Tier1 Explicit Router Hotpath Sweep

Status: `EXECUTED-COMPLETE-POST-TIER1-HOTPATH-SWEEP`
Package ID: `20260708-laned-router-post-tier1-hotpath-sweep-001`
Owner: Codex
Scaffold date: `2026-07-08`
Evidence mode: `Static + Ran; implementation landed and gates passed`

## Objective

Execute a bounded post-Tier1 hotpath sweep over the Lane D active explicit
overland-flow router after `20260708-laned-router-tier1-local-numerics-001`.
The package targets behavior-preserving local work that reduces repeated work
inside the existing TVD-MacCormack explicit scheme without reopening mesh,
fidelity, tolerance, or hybrid implicit adjudication.

Current implementation targets:

1. Reuse the celerity maximum already found during `prepare_step_alpha()` for
   step-level CFL evidence instead of rescanning all wet cells.
2. Avoid additive-friction-only prework on pure-skin cells by moving the
   additive-path slope square root after the pure-skin branch selection.

Classification targets:

- Keep the Tier1 `Re^0.45` approximation envelope on hold unless a separately
  ratified bounded-error envelope exists.
- Classify static per-cell precomputation, source-free/homogeneous step
  specialization, and loop-fusion opportunities for future work only if they
  remain inside the current explicit scheme and do not require contract or
  fidelity readjudication.

## Rationale

Tier1 lowered H2637 active-router user time from `37.48 s` to `11.90 s`
median, but its profile still shows hot local loops:
`solver_steps=10016170`, `alpha_evaluations=100161700`,
`solver_cfl_ns=2488591327`, and `solver_step_ns=6853399353`. The remaining
low-risk work is not a new numerical method; it is eliminating duplicate scans
and unnecessary local prework in the explicit path.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/package.md`
- `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/artifacts/timing-evidence.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`

Implementation-local:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs`
- `tests/integration/laned_shadow_h2637.rs`

## Scope

### Included

- Package-local scaffold, prompt, artifacts, and work-package catalog update.
- Contract-authority audit proving whether an `SC-OFEROUTE-001` amendment is
  required before code.
- Behavior-preserving implementation in
  `ofe_routing::kinematic_wave` only:
  - retain first-maximum celerity cell index during alpha preparation;
  - compute step max Courant from that retained maximum after final `dt`
    clipping;
  - preserve fail-closed non-finite and `Cr > 1` checks;
  - move additive-friction slope square root below pure-skin branch selection.
- Focused tests that pin the retained celerity maximum against an explicit
  scratch scan and preserve CFL boundedness.
- Timing/profiling evidence if release binary execution is available in the
  current environment.
- Review, verification, line-count governance, gate disposition, and final
  worker handoff.

### Excluded

- Hybrid implicit stepping, implicit solve warm seeding, composed residual
  Newton, or any revival of abandoned hybrid code paths.
- Mesh-policy changes, target-`dx` promotion, coupled space-time
  readjudication, or fidelity tolerance changes.
- The unratified Hirsch `Re^0.45` approximation envelope unless separately
  authorized by contract and bounded-error evidence.
- Source-free/homogeneous step specialization if it changes the explicit
  numerical scheme, limiter behavior, trace semantics, or mass ledger.
- Watershed/channel routing, HBP hourly consumption, baseflow export,
  sediment process-physics, crop/climate/soil/management source tuning, and
  wepppy orchestration.
- Silent fallback wrappers, fast-math substitutions, `f32` substitutions, or
  tolerance relaxation.

## Dependencies

- Tier1 package final state:
  `20260708-laned-router-tier1-local-numerics-001` at
  `EXECUTED-HOLD-APPROXIMATION-ENVELOPE`.
- Current Lane D active authority in `SC-OFEROUTE-001` rev 47.
- Existing explicit-router unit/integration tests and H2637 active fixture.

## Intended Write Set

Package and catalog:

- `docs/work-packages/20260708-laned-router-post-tier1-hotpath-sweep-001/**`
- `docs/work-packages/README.md`

Primary implementation:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`

Read-only or conditional:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs`
- `tests/integration/laned_shadow_h2637.rs`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`

Protected:

- No package may edit hybrid-implicit deletion posture, mesh-policy defaults,
  watershed/channel routing, sediment physics, or fixture required-case
  bindings in this scope.

## Phase Plan

### Phase A - Scaffold And Authority Audit

- Record worktree state and package scope.
- Create package-local scaffold and catalog entry.
- Record contract-authority disposition: behavior-preserving loop/prework
  removal needs no `SC-OFEROUTE-001` amendment; any mathematical formula,
  tolerance, or scheme change requires hold or a new contract-first package.

### Phase B - Implementation

- Add a private step-celerity summary that carries `max_celerity` and
  first-occurrence `max_cell_index`.
- Return that summary from `prepare_step_alpha()`.
- Replace the post-dt wet-cell CFL evidence scan with a direct calculation
  from the retained maximum and final `dt`.
- Move the additive-path `slope.sqrt()` after pure-skin branch selection.

### Phase C - Tests And Gates

- Add focused unit coverage for retained maximum celerity/index.
- Run focused router tests.
- Run required Rust closure gates for implementation packages.
- Run package doc/diff/line-count gates.
- Run release timing evidence if the release runner/fixture are available.

### Phase D - Review, Verification, Disposition

- Record implementation, gates, line-count governance, review, verification,
  final disposition, and worker handoff.
- Do not claim performance wins beyond measured evidence.

## Subagent Authorization

No subagent authorization is granted by this package. Review and verification
are local unless the operator separately authorizes subagent spawning.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/contract-disposition.md`
- `artifacts/implementation.md`
- `artifacts/timing-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review-codex.md`
- `artifacts/verification-codex.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required before completion:

- Contract-authority audit completed before code.
- Focused unit coverage for retained celerity maximum.
- Existing CFL boundedness tests remain green.
- `git diff --check`.
- Markdown/doc lint for touched docs.
- `.rs` line-count disposition.
- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo nextest run --workspace --profile full`.
- `cargo deny check`.

Conditional:

- H2637 active timing evidence if release-runner execution remains available
  and does not require fixture or required-case binding edits.
- Authority anti-evasion guard only if required-case bindings, cohort fixtures,
  or external-authority suite posture are touched.

## Security Impact Gate

No security-sensitive code, dependency, credential, network, or external input
surface is in scope. The package still fails closed on non-finite celerity and
`Cr > 1` CFL violations.

## Exit Criteria

`EXECUTED-COMPLETE-POST-TIER1-HOTPATH-SWEEP`:

- The explicit-router hotpath edits land without changing the numerical scheme,
  mesh policy, tolerances, or authority posture.
- Focused and full Rust gates pass.
- Review and verification artifacts find no closure-blocking issue.
- Final artifacts classify any deferred hotpath ideas without creating new
  active package obligations ahead of WSHED-W7R.

`EXECUTED-HOLD-*`:

- Any required gate fails.
- A candidate optimization requires contract, mesh, fidelity, tolerance, or
  hybrid implicit readjudication.
- Timing/release evidence is unavailable for an acceptance claim the package
  needs to make.
