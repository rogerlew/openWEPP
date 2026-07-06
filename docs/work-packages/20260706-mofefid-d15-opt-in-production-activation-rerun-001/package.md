# MOFEFID-D15 Rerun - Opt-In Production Activation

Status: **EXECUTED-HOLD-TIMING-ACTIVE-PATH** (2026-07-06). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: `SC-OFEROUTE-001` opt-in production activation gate.

Base intake: repository head `4ad74af7b913` (`main`, newer than the
requested D10B review-response base). Do not create or switch branches.

Execution result: held before Phase C. The D10B-corrected H2637-class Lane D
shadow path fails before endpoint timing with `NegativeOutletBin`, and the
current production runtime still has only diagnostics-only shadow/candidate
surfaces, not an active Lane D production owner path. No activation selector,
DC01 disable, routed publication cutover, active closure hard-fail, or output
schema change was implemented.

## Objective

Refresh the D14 endpoint timing on the D10B-corrected Lane D routing path,
then either complete the D15 opt-in production activation flip or close in an
executed hold with exact blockers, evidence, and the first actionable
follow-on.

The activation claim is allowed only if the real active path owns surface-water
routing for opt-in lanes, disables the old DC01 daily-lump runon feed on those
lanes, hard-fails material active-mode closure violations, feeds D13 routed
hydrograph shape to erosion when routing owns water, consumes the rev-21/D12
authorized sources, and preserves byte-identical subsystem-off behavior.

## Rationale

The first D15 package held on `SC-OFEROUTE-001#GAP-OFEROUTE-005` /
`INV-OFEROUTE-011`. D10B has now resolved that blocker at rev 25/26, but D10B
also changed the profiled routing path (true-celerity CFL and conservative
bin-series handoff). Strategy §6.1 therefore requires a D14 endpoint-timing
refresh before D15 can make an activation claim.

## Scope

### Included

- Scaffold package artifacts, prompts, and work-package catalog updates.
- Phase A timing refresh:
  - H2637-class default/off and Lane D opt-in/shadow endpoint timing on the
    D10B-corrected path.
  - Wall/user/sys timing, solver counters, step counts, and slot-profile
    evidence when available.
  - Comparison against D14's previous budget and a proceed/hold decision.
- Phase B activation readiness audit:
  - `INV-OFEROUTE-010` subsystem-off protected-output byte identity.
  - `INV-OFEROUTE-011` D-val / `GAP-OFEROUTE-005` D10B closure.
  - `INV-OFEROUTE-012` active subsurface seam: `ui_SCrunf` source term,
    `latqcc` bypass, runtime closure hard-fail.
  - DC01 daily-lump runon disabled for active routed lanes.
  - Rev-21 friction operands consumed by the active production path, not only
    shadow.
  - D12 source-shape and D13 routed-hydrograph erosion-shape obligations in the
    active consumer.
  - Missing runtime surfaces or stale shadow-only paths.
- Phase C implementation only if Phase A/B prove activation is in-envelope and
  authority-backed.

### Excluded

- No D16 default-promotion policy or default activation.
- No surrogate, provisional, proxy, empirical stand-in, heuristic physics, or
  compatibility wrapper carrying a production activation claim.
- No production activation while a current activation precondition is unmet.
- No D10B numerical-method/tolerance changes, D11/D12/D13 source-policy
  changes, watershed/channel routing work, or public schema changes unless a
  current activation gate proves they are necessary and contract-authorized
  inside this package.

## Dependencies

- `SC-OFEROUTE-001` rev 26.
- D14 runtime package and artifacts:
  `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/`.
- Original D15 hold:
  `docs/work-packages/20260705-mofefid-d15-opt-in-production-activation-001/`.
- D10B hold-lift:
  `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/`.
- D11 rev-21 friction operand closure:
  `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/`.
- D12 melt-limb source-shape closure:
  `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/`.
- D13 routed-hydrograph erosion-shape closure:
  `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/`.
- Lane D seam/runtime shadow packages enough to verify `INV-OFEROUTE-012`
  machinery and current shadow-only surfaces.

## Intended Write Set

Primary:

- `docs/work-packages/20260706-mofefid-d15-opt-in-production-activation-rerun-001/`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md`
- `docs/ROADMAP.md`

Conditional, only if Phase C proceeds:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/**`
- Focused Lane D / `ofe_routing` / active-consumer tests under `tests/` or
  crate-local test modules.

Protected unless contract-first evidence explicitly authorizes change:

- Default/off runtime behavior and protected outputs.
- Public pass/HBP schemas.
- D10B numerical-method authority and D11/D12/D13 source-policy semantics.
- D16 default-promotion policy.

## Phase Plan

1. **S0 - Scaffold and intake.** Create package-local artifacts/prompts,
   update the work-package catalog, read required authority, and dispatch
   timing/audit subagents where useful.
2. **S1 - D14 endpoint-timing refresh.** Re-run the H2637-class default/off
   and Lane D opt-in/shadow timing path on the D10B-corrected code; record
   endpoint timing, counters, slot profile, and comparison to the D14 budget.
3. **S2 - Activation readiness audit.** Trace every current activation
   precondition through real runtime consumers and tests. Name any missing
   active selector, stale shadow-only path, or consumer still reading the old
   compatibility/DC01 path.
4. **S3 - Flip-or-hold decision.** If every precondition is in-envelope and
   the timing refresh passes, implement contract-authorized production
   activation. If not, stop at `EXECUTED-HOLD-*` and write the hold-legitimacy
   audit.
5. **S4 - Implementation and evidence.** For an activation flip only: add
   contract-derived tests first, wire active routing, prove closure/DC01
   disable/routed erosion consumer/default identity, and document
   implementation evidence.
6. **S5 - Review, verification, gates, disposition.** Complete dual review,
   finding disposition, dual verification, required gates, line-count
   governance, worker handoff, and final disposition.

## Exit Criteria

Complete activation requires:

- D10B-corrected H2637 timing refresh is recorded and acceptable for opt-in
  production rerun.
- `INV-OFEROUTE-010` subsystem-off protected outputs remain byte-identical.
- `INV-OFEROUTE-011` is accepted as closed by D10B; no stale blocker language
  remains in package disposition.
- Active `INV-OFEROUTE-012` runtime closure hard-fail is live and includes
  `ui_SCrunf` source consumption plus `latqcc` bypass accounting.
- DC01 daily-lump runon is disabled for active routed lanes; no double-feed.
- Active path consumes rev-21 friction operands and D12 source-shape limbs.
- Routed hydrograph shape feeds the D13 erosion consumer when routing owns the
  water path.
- Default/off remains byte-identical and no default promotion is claimed.
- Dual review/verification and required gates pass, or every failed/blocked
  gate is tied to an executed hold.

If any precondition cannot be closed in-envelope, the package must stop at
`EXECUTED-HOLD-*` with a hold-legitimacy audit naming the exact blocker,
evidence proving it, why it is not safely closeable here, and the first
actionable follow-on.

## Required Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Contract/profile/BEI checks required by touched `SC-*` contracts.
- Focused Lane D / `ofe_routing` tests.
- H2637-class timing run with opt-in routed path enabled.
- Protected-output byte identity with subsystem off.
- Active-mode closure evidence for `INV-OFEROUTE-012`.
- DC01-disable / no-double-feed proof for active lanes.
- Routed-hydrograph-to-erosion consumer proof.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Authority anti-evasion guards if required-case binding, cohort fixture, or
  external-authority suite posture is touched:
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` gate blocks complete
disposition.

## Conservation / Output Acceptance

This package is conservation-sensitive if Phase C proceeds. Activation evidence
must prove real downstream consumers read the active routed path and must not
close on producer-only, shadow-only, counter-only, or self-consistency evidence.
Record operand lineage for active closure surfaces, reject old/DC01 aliasing,
and include independent reconstruction of the active closure identity. The
subsystem-off gate remains byte-exact.

## HOLD Legitimacy

A hold is legitimate only after the package proves a current activation
precondition cannot be safely closed inside this envelope. Valid hold classes
include missing active runtime selector/path, missing contract authority,
timing refresh exceeding the opt-in budget without an in-envelope
behavior-preserving remedy, missing downstream consumer surface, or a required
gate failure that cannot be fixed without D16/default-promotion or unrelated
process-family work.

The hold artifact must name the blocker, cite direct evidence, list the
in-envelope route considered, explain why it cannot close now, and provide the
first actionable follow-on package/action.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `explorer`,
`rust_code_reviewer`, and `rust_qa_reviewer` subagents for timing/comparator
runs, source/authority audit, implementation review, verification, and heavy
gate execution. Expected outputs are compact findings, timing metrics, gate
metrics, log paths, and package-local review or verification artifact text.
Write access is read-only unless a subagent is explicitly assigned a bounded
implementation fix.

Subagent requirement: `comparator_suite_runner` is required for H2637 timing
and full-suite gates when available. If a role is unavailable, record the
tooling block and run the equivalent gate locally only when package governance
permits.
