# MOFEFID-D15 Blocker Resolution - Terminal-Bin and Active Owner

Status: **EXECUTED-HOLD-ACTIVE-OWNER-TIMING-BUDGET** (2026-07-06).
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md)
Lane D. Contract focus: `SC-OFEROUTE-001` opt-in production activation
blockers.

Base intake: repository head `f49ce13308d0` (`main` / `origin/main`) with the
D15 rerun hold package merged. Do not create or switch branches.

Execution result: terminal-bin/day-boundary blocker resolved for the H2637
shadow/timing path. The D10B-corrected opt-in routed path now completes, but
the release endpoint timing is `91.59 s` user / `1:31.67` wall (`+89.10 s`
user over default/off and about `3.06x` D14's `~29.9 s` wall shadow budget).
Production activation was not flipped because the active production owner path
is still absent. The timing regression is adjudicated as a current hold; the
next package must resolve active production ownership and optimize/adjudicate
D15 timing before activation.

## Objective

Resolve the blockers recorded by the D15 rerun package:

1. Close the H2637 D10B-corrected routed timing-path failure
   (`NegativeOutletBin` on day 88, full-day window, source active through hour
   24) with contract-authorized terminal-bin/day-boundary semantics.
2. If the corrected routed path completes and the refreshed timing budget is
   acceptable, implement the opt-in active production owner path required for
   D15 activation.

If either blocker cannot be safely closed in this package, stop at
`EXECUTED-HOLD-*` with direct evidence, not a partial production flip.

## Rationale

The D15 rerun at
`../20260706-mofefid-d15-opt-in-production-activation-rerun-001/` held on two
blockers: the D10B-corrected H2637 shadow path fails before endpoint timing
with `NegativeOutletBin`, and the current production runtime still has only
diagnostics-only shadow/candidate surfaces rather than an active Lane D water
owner. D16/default promotion remains blocked until opt-in activation has real
consumer evidence.

## Correction Authority Envelope

### Defect IDs and observed failures

- D15 rerun `HOLD-1`: H2637 day-88 `NegativeOutletBin` on the D10B
  conservative bin-series path before endpoint timing/profile emission.
- D15 rerun `HOLD-2`: no active production owner path; DC01 daily-lump runon
  still feeds production and the D13 routed-hydrograph consumer is candidate
  only.

### In-scope authority

- `SC-OFEROUTE-001` rev 26, especially `INV-OFEROUTE-008..012`,
  per-OFE handoff rev-26, DC01-supercede/runon ownership, routed hydrograph
  shape consumer obligations, and subsystem-off byte identity.
- D10B conservative bin-series handoff and `GAP-OFEROUTE-005` closure.
- D11 rev-21 friction operand closure, D12 source-shape closure, and D13
  routed-hydrograph erosion-shape closure.
- D15 rerun timing/readiness/hold artifacts.

### Authorized edit classes

- Amend `SC-OFEROUTE-001` only if day-boundary or active-owner semantics need
  canonical clarification before implementation.
- Fix conservative outlet-bin handoff, publication, and downstream injection
  code in `crates/openwepp-hillslope-orchestrator/src/ofe_routing/**` and
  `crates/openwepp-runner/src/hillslope/laned_shadow.rs`.
- Add focused tests for terminal-bin non-negativity, exact total preservation,
  H2637 shadow completion, and timing/profile evidence.
- If Phase A passes: add an explicit opt-in active selector and wire real
  production consumers in the runner/direct-runtime path so Lane D owns
  surface-water routing for active lanes.
- Update package-local evidence and high-level campaign/catalog status docs.

### Protected boundaries

- No D16/default promotion or policy-scoped default activation.
- No surrogate physics, tuned reshaping, compatibility wrapper, silent
  fallback, or truncation that hides a negative/missing routed-bin condition.
- No activation claim from producer-only, shadow-only, counter-only, or
  candidate-only evidence.
- Default/off behavior and protected outputs must remain byte-identical.
- Do not loosen fail-closed guards without contract-first authority and tests.

## Scope

### Included

- Package scaffold, catalog updates, artifacts, and active kickoff prompt.
- Reproduce and diagnose the day-88 terminal-bin failure.
- Contract-first correction if the failure is an in-envelope boundary/handoff
  defect with clear authority.
- H2637-class timing refresh after the correction, including endpoint
  wall/user/sys time, solver counters, step counts, and slot/profile evidence
  when available.
- Activation readiness re-audit after timing closure.
- Opt-in active-owner implementation only if the corrected timing/readiness
  surface is green and authority-backed.
- Dual review, finding disposition, dual verification, gate results, line-count
  governance, worker handoff, and final disposition.

### Excluded

- Default activation, D16 policy, release rollout, and wepppy orchestration.
- New public schemas unless an active-consumer gate proves they are necessary
  and contract-authorized.
- Watershed/channel routing.
- D11/D12/D13 source-policy re-adjudication unless an implementation defect is
  found in their active consumption surfaces.

## Dependencies

- Prior D15 rerun hold:
  `../20260706-mofefid-d15-opt-in-production-activation-rerun-001/`.
- D10B hold-lift:
  `../20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/`.
- D14 runtime package:
  `../20260705-mofefid-d14-laned-runtime-profile-optimization-001/`.
- D11/D12/D13 packages named in the D15 rerun package.
- `docs/work-packages/AGENTS.md`,
  `docs/specifications/science-contracts/AGENTS.md`, and
  `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.

## Intended Write Set

Primary:

- `docs/work-packages/20260706-mofefid-d15-blocker-resolution-001/`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md`
- `docs/ROADMAP.md`

Conditional terminal-bin correction:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/**`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- Focused `ofe_routing` and `laned_shadow_h2637` tests.

Conditional active-owner implementation:

- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- Active consumer tests under `tests/` or crate-local modules.

## Phase Plan

1. **S0 - Scaffold and intake.** Create package assets, read authority, and
   record the baseline blockers.
2. **S1 - Reproduce terminal-bin failure.** Run the focused H2637 shadow test
   and capture day/window/bin context without retaining diagnostic-only code.
3. **S2 - Contract-first terminal-bin correction.** Add any required contract
   clarification, tests, and production correction for non-negative exact-total
   terminal bins and downstream injection.
4. **S3 - Timing refresh.** Re-run the H2637 default/off and opt-in routed
   timing/profile path and compare to D14/D15-rerun budgets.
5. **S4 - Activation readiness decision.** Re-audit every D15 activation
   precondition against real consumers.
6. **S5 - Active-owner implementation or hold.** If S3/S4 are green, wire the
   opt-in active owner path and prove closure/DC01-disable/D13 consumer/default
   identity. Otherwise write the hold legitimacy audit.
7. **S6 - Review, verification, and disposition.** Complete reviews,
   verification, line-count governance, gates, disposition, and worker handoff.

## Exit Criteria

Complete blocker-resolution plus D15 opt-in activation requires:

- H2637 D10B-corrected opt-in routed path completes without
  `NegativeOutletBin`.
- Corrected outlet-bin series is non-negative and exact-total preserving; any
  terminal/day-boundary tail semantics are contract-authorized.
- H2637 timing refresh is recorded and acceptable.
- `INV-OFEROUTE-010` subsystem-off protected outputs remain byte-identical.
- `INV-OFEROUTE-012` active closure hard-fail is live and includes
  `ui_SCrunf` source consumption plus `latqcc` bypass accounting.
- DC01 daily-lump runon is disabled for active routed lanes; no double-feed.
- Rev-21 friction operands and D12 source-shape limbs are consumed by the real
  active path.
- Routed hydrograph shape feeds the D13 erosion consumer when routing owns the
  water path.
- Dual review/verification and required gates pass.

Terminal-bin-only completion is allowed only if the active-owner blocker is
proven outside this package after timing closure. In that case, the package
must close `EXECUTED-HOLD-ACTIVE-OWNER` and name the first active-owner
follow-on with exact evidence.

## Required Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Contract/profile/BEI checks required by touched `SC-*` contracts.
- Focused Lane D / `ofe_routing` tests.
- `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture`
- H2637-class timing run with opt-in routed path enabled.
- Protected-output byte identity with subsystem off.
- Active-mode closure evidence for `INV-OFEROUTE-012` if active owner is
  implemented.
- DC01-disable / no-double-feed proof for active lanes if active owner is
  implemented.
- Routed-hydrograph-to-erosion consumer proof if active owner is implemented.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Authority anti-evasion guards if required-case binding, cohort fixture, or
  external-authority suite posture is touched:
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` gate blocks a complete
disposition.

## Required Artifacts

- `artifacts/reproduction.md`
- `artifacts/terminal-bin-correction.md`
- `artifacts/timing-refresh.md`
- `artifacts/activation-readiness-audit.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-codex.md`
- `artifacts/review-qa.md`
- `artifacts/verification-codex.md`
- `artifacts/verification-qa.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `explorer`,
`rust_code_reviewer`, and `rust_qa_reviewer` subagents for timing/comparator
runs, source/authority audit, implementation review, verification, and heavy
gate execution. Expected outputs are compact findings, timing metrics, gate
metrics, log paths, and package-local review or verification artifact text.
Write access is read-only unless a subagent is explicitly assigned a bounded
implementation fix in the intended write set.

## HOLD Legitimacy

If the package holds, the hold artifact must name the exact blocker, cite direct
evidence, list the in-envelope route considered, explain why that route cannot
close now, and provide the first actionable follow-on package/action. A missing
active production owner path remains an in-scope implementation obligation only
after the terminal-bin/timing path is green.
