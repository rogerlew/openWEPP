# MOFEFID-D14 - Lane D Runtime Profiling and Optimization

Status: **SCAFFOLDED** (2026-07-06). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: Lane D opt-in production-activation precondition in
`SC-OFEROUTE-001`.

## Objective

Profile and optimize the Lane D runtime physics path before the opt-in
production flip. D14 must empirically break down the current H2637 shadow
overhead into solver math, per-day/OFE setup, allocation, hydrograph
sampling/interpolation, friction/forcing construction, handoff, diagnostics,
and runner/publication overhead. It may then land only behavior-preserving
optimizations that keep numerical authority, closure evidence, and protected
outputs intact.

D14 must finish with a fresh before/after runtime budget for D15. It must not
activate production routing, change the D10 shock-numerics posture, or tune
physics to improve timing.

## Rationale

D10-D13 settled the current opt-in candidate surfaces: the shock-numerics
source-authority hold is isolated, friction operands are sourced for opt-in
shadow, the melt-limb hourly shape is source-authorized, and erosion has a
routed-hydrograph consumer path for active routed water. The candidate path is
now coherent enough to profile, but the best current H2637 measurement still
shows about `+207 s` CPU overhead in the Lane D shadow path over default
execution.

That cost is too large to bury inside the production-activation package. D14
exists to make the runtime cost empirical and to remove behavior-preserving
overhead before D15 makes any activation claim.

## Scope

### Included

- Establish a reproducible H2637 baseline for:
  - default/off runtime,
  - current Lane D shadow runtime,
  - shadow-on protected-output identity, and
  - routed-path closure/diagnostic parity.
- Add or extend persistent timing diagnostics only as needed to attribute the
  Lane D runtime cost to explicit slots:
  - `ofe_routing::cascade` / `KinematicWaveSolver` solver math,
  - per-day/OFE setup,
  - allocation, cloning, and vector construction,
  - hourly hydrograph sampling/interpolation and source-rate construction,
  - friction/forcing operand construction,
  - handoff and closure diagnostics,
  - runner/publication integration.
- Profile the real H2637 fixture and any smaller focused fixture needed for
  fast iteration, recording command lines, wall/user/sys timing, and slot
  counters.
- Land behavior-preserving optimizations when the profile supports them.
- Prove that optimized output is unchanged where it must be unchanged:
  default/off protected outputs stay byte-identical, shadow diagnostics retain
  closure parity, and routed-path numerical outputs do not change except for
  explicitly approved diagnostics metadata.
- Update the package artifacts, work-package catalog, Lane D planning, and
  worker handoff.

### Excluded

- No production/default Lane D activation.
- No D15 active selector, DC01 disable, routed-path publication cutover, or
  production manifest activation claim.
- No D10 `GAP-OFEROUTE-005` shock-numerics correction, limiter/handoff
  method change, Case-4 acceptance, or tolerance loosening.
- No D11 friction-source policy change beyond preserving the rev-21 operand
  path.
- No D12 melt-limb source-shape rule change.
- No D13 routed-hydrograph erosion-shape semantic change.
- No D16 default-promotion policy.
- No watershed/channel routing work.
- No surrogate, provisional, proxy, empirical stand-in, heuristic physics, or
  numerical-method shortcut for performance.

## Dependencies

- `SC-OFEROUTE-001` rev 23.
- D10 final disposition and hold:
  `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/`.
- D11 friction operand closure:
  `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/`.
- D12 melt-limb source-shape closure:
  `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/`.
- D13 routed-hydrograph erosion-shape closure:
  `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/`.
- Lane D runtime shadow package:
  `docs/work-packages/20260705-mofefid-laned-activation-increment-001/`.
- Current runtime surfaces:
  - `crates/openwepp-runner/src/hillslope/laned_shadow.rs`,
  - `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/ofe_routing/seam.rs`,
  - `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`, and
  - `tests/integration/laned_shadow_h2637.rs`.

## Intended Write Set

Primary:

- `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md`
- `docs/ROADMAP.md`

Conditional, only for profiling or behavior-preserving optimization:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/seam.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- Focused Lane D timing, H2637, or profiler tests.
- Local profiling helpers under `tools/` only if the package records why the
  existing test/runner hooks are insufficient.

Protected:

- `SC-OFEROUTE-001` and other `SC-*` contract semantics, unless D14 discovers
  a timing-status documentation correction with no process-rule change.
- Production activation selector and default runtime policy.
- D10 numerical-method implementation and D-val acceptance.
- D11/D12/D13 semantic surfaces.
- D15/D16 activation/default-promotion policy.
- Public pass/HBP schemas.

## Phase Plan

1. **D14-S0 - Intake and baseline.** Read required authority, verify current
   branch/main state, and record default/off plus shadow-on H2637 baseline
   timing.
2. **D14-S1 - Slot instrumentation.** Add or locate persistent timing
   diagnostics that attribute Lane D cost to explicit slots without changing
   outputs or control flow.
3. **D14-S2 - Empirical profile.** Run H2637 and focused fixtures under the
   instrumentation. Record wall/user/sys time, slot totals, event/OFE counts,
   and repeatability limits.
4. **D14-S3 - Optimization plan.** Pick only behavior-preserving candidates
   justified by the measured profile. Reject candidates that change numerical
   method, source authority, closure tolerance, or activation semantics.
5. **D14-S4 - Implement optimizations.** Land the selected optimizations with
   focused tests and line-count governance for touched Rust files.
6. **D14-S5 - Evidence and closure.** Re-run before/after timing, protected
   output identity, routed-path closure/diagnostic parity, review,
   verification, and final disposition.

## Exit Criteria

- A slot-level timing artifact exists and attributes the H2637 Lane D runtime
  overhead to named code-path categories.
- Before/after H2637 endpoint timing is recorded with exact commands and
  wall/user/sys evidence.
- Default/off protected outputs remain byte-identical.
- Shadow/routed-path closure and diagnostic parity hold after optimization.
- Any timing diagnostics introduced by D14 are discoverable by local CI agents
  and do not require GitHub CI.
- D15 receives an explicit runtime budget and a list of remaining activation
  risks.
- No D10, D15, D16, surrogate physics, or numerical-method shortcut work
  occurs.
- Accepted review findings are fixed and verified before completion.

## Required Gates

Selection follows `docs/standards/local-ci-gate-selection.md` where relevant,
but D14 cannot close without recording:

- `git diff --check`
- Markdown lint for touched docs.
- Baseline H2637 default/off and Lane D shadow timing with exact commands.
- Slot-level profiling evidence for H2637 or a justified reduced fixture plus
  H2637 endpoint confirmation.
- Protected-output identity evidence for default/off and shadow-off paths.
- Routed-path closure/diagnostic parity evidence before and after
  optimization.
- Focused tests for any changed profiler, timing, routing, runner, or
  optimization code.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, fixture
  governance, or authority-suite posture are touched.

If heavy gates are delegated, record subagent output and log paths in
`artifacts/gate-results.md`.

## Conservation / Output Acceptance

D14 is performance-sensitive, not physics-authority-changing. A successful
optimization must preserve existing numerical outputs, closure diagnostics,
manifest semantics, and protected-output identity. Any output-affecting change
blocks the package unless a separate contract-backed package authorizes it.
Timing improvement alone is not sufficient evidence.

## HOLD Legitimacy

D14 may close in `HOLD` only for a named boundary that cannot be closed inside
this write set: profiling cannot isolate the overhead, optimization requires a
D10 numerical-method/source-authority decision, behavior-preserving changes do
not reduce runtime enough for D15, or the current candidate path is too costly
to activate without a larger architecture package. A hold must include the
measured profile, the candidates considered, why they are blocked, and the
smallest next package that can unblock activation.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` subagents for read-only source/authority audit,
profiling review, optimization review, verification, H2637/Lane D evidence,
and heavy gate execution. Expected outputs are compact findings, timing
metrics, gate metrics, log paths, and package-local review or verification
artifact text. Write access is read-only unless the operator assigns a bounded
write set.

Subagent requirement: `comparator_suite_runner` is required for heavy H2637
timing and full-suite gates when available. If session-level tooling prevents
subagent dispatch, record that block and run locally only when package
governance permits.
