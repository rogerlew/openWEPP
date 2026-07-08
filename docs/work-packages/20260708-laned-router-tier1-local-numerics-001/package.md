# Laned Router Tier 1 Local Numerics

Status: `QUEUED`
Package ID: `20260708-laned-router-tier1-local-numerics-001`
Owner: Codex
Scaffold date: `2026-07-08`
Evidence mode: `Static scaffold; no implementation executed`
Backlog source:
`docs/backlog/20260706-laned-router-numerics-performance-tiers.md`

## Objective

Execute the backlog's `Tier 1 - local numerics` optimizations as one
contract-first package for the Lane D active overland-flow router:

1. Replace perturbed-depth numerical celerity with contract-authorized analytic
   celerity for Manning, laminar skin, Hirsch turbulent skin, and additive
   friction-menu cases.
2. Replace the capped alpha fixed-point loop with a bounded Newton solve for
   `alpha`/`q` where the local friction law is implicit in discharge.
3. Replace hot-path `h.powf(1.5)` with `h * h.sqrt()` and replace or bound the
   hot `Re^0.45` computation with a proven minimax/vector-ready approximation
   only if the contract and tests authorize its fidelity envelope.

The package target is the backlog estimate of roughly `2.5-4x` combined active
router speedup on H2637, measured after D15A/D14-style timing evidence. The
package must not claim bit identity; it intentionally changes numerical method
and must close through authority, oracle agreement, conservation, fidelity
deltas, and timing.

## Rationale

D15A exhausted the bit-identity optimization headroom in the active router. The
remaining dominant cost is local per-cell numeric work in
`ofe_routing::kinematic_wave` and `ofe_routing::friction`: alpha fixed-point
iterations, the second celerity alpha evaluation at perturbed depth, and libm
power calls in the hot path. The backlog classifies the Tier 1 changes as low
risk because they are local algebraic/numerical substitutions, but they are not
behavior-preserving at the bit level. Contract authority must come first.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- this package's `package.md`

Implementation-local:

- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/standards/local-ci-gate-selection.md`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/iwagaki_oracle.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/laned_active.rs`
- `tests/integration/laned_shadow_h2637.rs`

Contract-authoring conditional reading before the first `SC-*` edit:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`

Timing/profiling context:

- `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/baseline-profile.md`
- `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/slot-profile.md`
- `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/optimization-plan.md`

## Scope

### Included

- Package-local scaffold, prompts, artifacts, disposition files, and catalog
  update.
- Required-reading map and byte-count budget before implementation.
- Contract-first amendment to `SC-OFEROUTE-001` before production code:
  - analytic celerity formulas for Manning, laminar skin, Hirsch turbulent
    skin, and additive friction menus;
  - implicit-differentiation rule for `q = alpha(q, h) h^1.5`;
  - bounded Newton alpha solve, convergence/fallback criteria, dry/zero-slope
    behavior, and fail-closed handling for non-finite derivatives;
  - authorized `h * h.sqrt()` replacement for `h^1.5`;
  - any authorized `Re^0.45` approximation envelope, coefficient provenance,
    input range, max absolute/relative error, and test vectors;
  - named tolerance/fidelity obligations because bit identity is not retained.
- Contract-derived tests before production code for celerity, alpha, power
  substitution, approximation bounds, and failure modes.
- Implementation in the active overland-flow routing numerics.
- Fidelity and conservation evidence:
  - Iwagaki oracle ladder within current rev-25/rev-26 tolerances at every
    rung;
  - exact booked-ledger conservation;
  - TV(q) transient bound;
  - 19-OFE class-fixture sweep with exact conservation;
  - H2637 active endpoint and rev-27 day-closure hard-fails green;
  - named hydrograph peak, timing, outlet-volume, and routed-shape deltas
    versus the pre-change trajectory.
- Timing evidence with exact release binary provenance and the D14/D15A
  two-instrument protocol: persistent timing slots plus `perf`.
- Dual independent review, finding disposition, dual verification, line-count
  governance, and final disposition.

### Excluded

- Tier 2 mesh-policy work, target-`dx` promotion, or coupled space-time
  adjudication.
- Tier 3/hybrid/SIMD work, implicit subsystem revival, GPU/vector rewrite, or
  whole-solver architecture replacement.
- Default activation policy, D16 coefficient/source acquisition, or active
  selection policy changes.
- Sediment process-physics changes, erosion consumer changes, crop/climate/
  soil/management source tuning, or wepppy orchestration changes.
- Closure/CFL tolerance relaxation unless `SC-OFEROUTE-001` is amended first
  with explicit thresholds, units, tests, and evidence.
- Silent fallback to shadow/DC01 routing, fallback wrappers that mask missing
  required dependencies, unbounded fast-math, or unproven `f32` substitutions.

## Dependencies

- Backlog authority:
  `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`.
- Current OFE routing authority:
  `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- D10B oracle/conservation harness and Iwagaki ladder.
- D14/D15A timing and profiling artifacts listed under required reading.
- Current active-router correctness packages through rev 41+ solver positivity
  and rev 27 day-closure hard-fails.

The backlog recommends sequencing Tier 1 after D16 default-promotion
adjudication, or alongside it if fleet economics demand. This scaffold records
the package for execution when the operator elects to proceed; it does not
claim D16 has closed.

## Intended Write Set

Package and catalog:

- `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/**`
- `docs/work-packages/README.md`

Contract authority:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`

Primary implementation:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs` only if
  counters must distinguish old/new alpha/celerity/pow work.

Focused validation and active consumer evidence:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs` only if the
  oracle harness needs a contract-derived comparison surface.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
  only if execution must expose counters or prove the active consumer uses the
  new numerics.
- `crates/openwepp-runner/src/hillslope/laned_active.rs` only for H2637 active
  evidence plumbing or counters.
- `tests/integration/laned_shadow_h2637.rs` and focused integration tests.

Protected:

- No edits to mesh-policy/default-promotion packages except package-local
  cross-reference updates.
- No wepppy, watershed/channel routing, climate, crop, management, soil, or
  sediment process changes.
- No fixture/cohort required-case binding edits unless explicitly justified and
  followed by anti-evasion gates.

## Phase Plan

### Phase A - Intake, Source Map, and Baseline

1. Record `git status --short --branch` and identify unrelated dirty files.
2. Produce `artifacts/required-reading-map.md` with required-reading byte
   counts and threshold disposition.
3. Record pre-change source map for alpha, celerity, power, skin friction, and
   active consumer call sites.
4. Build an exact release runner binary and record path, mtime, size, and hash
   before any timing evidence.
5. Capture pre-change H2637 active timing and persistent-slot/profile evidence
   unless a current, same-revision baseline is already valid and cited.

### Phase B - Contract-First Authority

1. Amend `SC-OFEROUTE-001` before code.
2. Add or update contract change-log entries and invariant/test obligations.
3. Define the analytic formulas and derivative/fallback envelope in end-user
   legible terms for future maintainers.
4. Define the fidelity surfaces that replace bit-identity acceptance.

### Phase C - Contract-Derived Tests and Pre-Implementation Gate

1. Add tests that fail on current code where the new authority is not yet
   implemented, without weakening existing D10B/D15A acceptance.
2. Cover dry cells, zero slope, Manning, laminar skin, Hirsch skin, additive
   friction, non-finite derivative, and approximation-bound cases.
3. Run the pre-implementation contract gate and record it in
   `artifacts/pre-implementation-contract-gate.md`.

### Phase D - Implementation

1. Implement analytic celerity and remove the perturbed-depth second alpha
   fixed point from the hot path.
2. Implement bounded Newton alpha solve and retain fail-closed behavior for
   invalid/non-convergent states authorized by the contract.
3. Replace `h.powf(1.5)` in hot routing paths with `h * h.sqrt()` where the
   contract and tests authorize the substitution.
4. Implement the `Re^0.45` optimization only inside the proven range and only
   if the approximation evidence is complete. If evidence is insufficient,
   close that candidate as out-of-envelope while still executing the rest of
   Tier 1.
5. Update counters/profiles only where needed to prove work reduction.

### Phase E - Focused Fidelity and Conservation Validation

1. Run friction/numerics unit tests.
2. Run D10B/Iwagaki oracle ladder and prove every rung remains within
   rev-25/rev-26 tolerances.
3. Prove exact booked-ledger conservation and TV(q) transient bound.
4. Run the 19-OFE class-fixture sweep and record exact conservation.
5. Run H2637 active endpoint and rev-27 day-closure hard-fail checks.
6. Record named deltas versus pre-change trajectory in
   `artifacts/fidelity-delta.md`.

### Phase F - Timing and Performance Adjudication

1. Rebuild the exact release runner and record binary provenance.
2. Run H2637 active before/after timing using the D14/D15A two-instrument
   protocol: persistent slots and `perf`.
3. Record alpha-evaluation counts, solver CFL time, pow/libm attribution,
   user/wall/sys time, and repeatability limits.
4. Decide whether the achieved speedup satisfies the Tier 1 performance goal
   without violating fidelity gates.

### Phase G - Review, Verification, and Closure

1. Complete line-count governance and owned-file manifest.
2. Complete dual independent reviews and disposition accepted findings before
   final verification.
3. Complete dual verification, including at least one verifier who checks
   authority/test/evidence alignment rather than re-running only commands.
4. Run final closure gates.
5. Write `artifacts/final-disposition.md` and `artifacts/worker-handoff.md`.

## Required Gates

Always record:

- `git status --short --branch`
- `git diff --check`
- Markdown/doc lint for touched package, contract, and catalog docs.
- Contract/profile/BEI checks required by touched `SC-*` authority.
- Focused unit tests for `ofe_routing::friction` and
  `ofe_routing::kinematic_wave`.
- D10B/Iwagaki oracle ladder.
- 19-OFE class-fixture conservation sweep.
- H2637 active endpoint and rev-27 day-closure hard-fail tests.
- Exact release build of the runner used for timing/comparator evidence,
  including path, mtime, size, and hash.
- H2637 active timing with persistent timing slots.
- H2637 active timing under `perf` or a documented local equivalent if `perf`
  is unavailable.
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
verification, comparator/timing, numerics-review, and contract-authority
subagents. Expected outputs are package-local artifacts. Write access is
bounded to package-local artifacts unless a subagent is explicitly assigned an
implementation fix.

Heavy comparator, timing, and final full-closure gates must be delegated to a
`comparator_suite_runner` subagent when available. If no such subagent/tool is
available in the execution environment, the parent executor may run the gates
directly and must record the unavailability and commands in
`artifacts/gate-results.md`.

## Exit Criteria

Close as `EXECUTED-COMPLETE-TIER1-NUMERICS` only if all of the following are
true:

- `SC-OFEROUTE-001` authorizes every landed numerical-method change.
- Contract-derived tests fail before implementation or otherwise demonstrate
  that the new authority is actually exercised.
- The active production router consumes the new numerics on the H2637 active
  path; no shadow-only or producer-only proof is used for closure.
- Iwagaki oracle ladder, exact booked-ledger conservation, TV(q) transient
  bound, 19-OFE class-fixture conservation, H2637 active endpoint, and rev-27
  day-closure hard-fails pass.
- Fidelity deltas versus the pre-change trajectory are named, measured, and
  accepted under the amended contract.
- Timing evidence shows the achieved Tier 1 speedup or records a justified
  performance hold without weakening correctness gates.
- Final Rust closure gates and docs gates pass.
- Accepted review findings are fixed and verified before final disposition.

Legitimate hold outcomes include:

- `EXECUTED-HOLD-CONTRACT-AUTHORITY`: the required numerical method cannot be
  authorized from current sources.
- `EXECUTED-HOLD-FIDELITY`: oracle, conservation, closure, or named fidelity
  gates fail after in-envelope fixes.
- `EXECUTED-HOLD-PERFORMANCE`: correctness passes but the measured speedup does
  not justify promotion.
- `EXECUTED-HOLD-IMPLEMENTATION`: the implementation cannot satisfy typed
  guards, convergence, or fail-closed requirements without a larger package.
- `EXECUTED-HOLD-APPROXIMATION-ENVELOPE`: the `Re^0.45` approximation lacks a
  defensible bounded-error envelope, even if the other Tier 1 changes land.
