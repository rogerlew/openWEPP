# MOFEFID-D15A - Active Owner and Timing Optimization

Status: **EXECUTED-COMPLETE** (2026-07-06; Claude Code end-to-end; final
call in `artifacts/final-disposition.md`; Codex re-check 2026-07-07
GO-WITH-AMENDMENTS with this status line as the remaining amendment).
Originally scaffolded 2026-07-06. Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: `SC-OFEROUTE-001` opt-in production activation.

Base intake: start from `main` / `origin/main` at
`94a7ac3aff003a89328701e4a6daf3abd98c8fe3` or newer, with
`20260706-mofefid-d15-blocker-resolution-001` merged. Do not create or switch
branches unless the operator explicitly asks.

## Objective

Resolve the two remaining D15 holds without a partial activation flip:

1. Optimize or explicitly adjudicate the D10B-corrected H2637 opt-in routed
   timing regression recorded by the prior blocker-resolution package.
2. Implement the contract-authorized opt-in active production owner path so
   Lane D owns surface-water routing for active lanes and the real downstream
   consumers read that path.

If either condition cannot be safely closed inside this package, stop at
`EXECUTED-HOLD-*` with a hold legitimacy audit. Do not land a partial
production activation.

## Rationale

The D15 blocker-resolution package closed the terminal-bin/day-boundary
`NegativeOutletBin` failure. The D10B-corrected H2637 path now completes, but
release timing is `91.59 s` user / `1:31.67` wall, about `3.06x` the D14
optimized shadow budget. The same package also proved the active production
owner is still absent: the current runtime remains a diagnostics-only shadow
surface, DC01 daily-lump runon still feeds production, active closure is not
live in production, and D13 erosion still receives the DC01 source shape.

D15A is the hold-lift package. It keeps the D14 optimization structure for the
timing portion, then performs the active-owner cutover only when timing and
authority evidence support it.

## Scope

### Included

- Package-local scaffold, catalog updates, artifacts, and kickoff prompt.
- Required-reading map covering `AGENTS.md`, work-package governance,
  `SC-OFEROUTE-001`, Lane D strategy, D10B, D11, D12, D13, D14, D15 rerun,
  and D15 blocker-resolution artifacts.
- D14-shaped timing optimization:
  - reproducible H2637 default/off and opt-in baseline timing;
  - persistent slot/profile evidence;
  - empirical attribution of the post-D10B regression;
  - behavior-preserving optimization plan and implementation;
  - before/after endpoint timing and output-preservation proof.
- Active-owner architecture and operand-lineage audit before production edits.
- Contract-first amendments only if active-owner semantics, timing budget, or
  consumer obligations need canonical clarification.
- Opt-in active owner implementation only when authority-backed:
  - explicit opt-in selector;
  - routed path owns surface-water routing for active lanes;
  - DC01 daily-lump runon is disabled for active lanes;
  - active `INV-OFEROUTE-012` closure hard-fail is live;
  - `latqcc` bypass and `ui_SCrunf` source terms are in closure;
  - rev-21 friction operands, D12 source-shape limbs, and D13 routed
    hydrograph erosion shape are consumed by the real active path.
- Dual review, finding disposition, dual verification, line-count governance,
  gate results, final disposition, and worker handoff.

### Excluded

- No D16/default promotion or policy-scoped default activation.
- No wepppy orchestration, release rollout, watershed/channel routing, or
  public API/schema expansion unless a current active-consumer gate proves it
  necessary and contract-authorized.
- No D10 shock-numerics method change, D11 operand-policy re-adjudication, D12
  source-shape rule change, or D13 erosion semantic change outside a proven
  implementation defect.
- No surrogate physics, tuned compatibility wrapper, silent fallback,
  shadow-only activation claim, or numerical-method shortcut for performance.
- No partial flip where the old DC01 path and new routed path both feed the
  same active lane.

## Dependencies

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- D10B hold-lift:
  `../20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/`.
- D11 friction operand closures:
  `../20260705-mofefid-d11-friction-operand-authority-001/` and
  `../20260706-mofefid-d11-gap007-dynamic-friction-closure-001/`.
- D12 melt-limb source-shape closure:
  `../20260705-mofefid-d12-melt-limb-hourly-shape-001/`.
- D13 routed-hydrograph erosion-shape closure:
  `../20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/`.
- D14 timing optimization:
  `../20260705-mofefid-d14-laned-runtime-profile-optimization-001/`.
- D15 activation preflight and rerun:
  `../20260705-mofefid-d15-opt-in-production-activation-001/` and
  `../20260706-mofefid-d15-opt-in-production-activation-rerun-001/`.
- D15 blocker-resolution hold:
  `../20260706-mofefid-d15-blocker-resolution-001/`.

## Intended Write Set

Primary:

- `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md`
- `docs/ROADMAP.md`

Conditional timing/profile optimization:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/**`
- Focused Lane D timing/profile/H2637 tests.
- Local profiling helpers under `tools/` only if existing profiler hooks are
  proven insufficient.

Conditional active-owner implementation:

- `crates/openwepp-runner/src/hillslope/direct_publication/**`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- Active consumer tests under `tests/` or crate-local modules.

Conditional contract authority:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` only if the
  routed-hydrograph-to-erosion active consumer semantics require canonical
  clarification.

Protected:

- Default/off behavior and protected output bytes.
- D16/default promotion policy.
- D10 numerical-method authority, D11 friction policy, D12 source-shape
  policy, and D13 erosion policy except for explicit contract-first
  amendments.
- Public pass/HBP schemas unless authority and consumer evidence require them.

## Optimization Phase Plan

The timing portion intentionally mirrors the D14 package structure.

1. **D15A-S0 - Intake and baseline.** Read authority, verify clean
   `main`/`origin/main`, and record default/off plus opt-in routed H2637
   baseline timing from the D10B-corrected path.
2. **D15A-S1 - Slot instrumentation.** Reuse or extend persistent timing
   diagnostics to attribute the regression to explicit slots without changing
   outputs or control flow.
3. **D15A-S2 - Empirical profile.** Run H2637 and any justified reduced
   fixture under instrumentation. Record wall/user/sys time, solver counters,
   step counts, slot totals, event/OFE counts, and repeatability limits.
4. **D15A-S3 - Optimization plan.** Pick only behavior-preserving candidates
   supported by measured profile evidence. Reject candidates that change
   numerical method, source authority, closure tolerance, activation semantics,
   or output values.
5. **D15A-S4 - Implement optimizations.** Land selected optimizations with
   focused tests, line-count governance, and before/after preservation proof.
6. **D15A-S5 - Optimization evidence and timing adjudication.** Re-run H2637
   endpoint timing and decide whether D15 active-owner work may proceed in this
   package. If timing remains above the accepted budget and the operator has
   not explicitly accepted it, hold before activation.

## Active-Owner Phase Plan

1. **D15A-P0 - Active-owner architecture and operand lineage.** Produce an
   operand-lineage table before production edits: source series, units,
   basis/denominator, owning frame/state object, downstream consumer, and old
   path that must be bypassed for each active lane.
2. **D15A-P1 - Contract/readiness audit.** Re-check every current
   `SC-OFEROUTE-001` activation precondition and amend contracts before code if
   semantics are missing or contradictory.
3. **D15A-P2 - Production phase integration.** Add the explicit opt-in active
   selector and restructure execution order only as needed so routed water is
   available before downstream runon admission and erosion shape consumption.
4. **D15A-P3 - Consumer cutover proof.** Prove DC01-disable/no-double-feed,
   active closure hard-fail, rev-21 friction operand consumption, D12
   source-shape consumption, and D13 routed-hydrograph erosion consumption in
   the real active production path.
5. **D15A-P4 - Activation gates.** Run protected-output identity, H2637 timing,
   focused Lane D tests, contract/profile checks, and full Rust closure gates.
6. **D15A-P5 - Review, verification, and disposition.** Complete dual review,
   disposition accepted findings, complete dual verification, write final
   disposition, and hand off D16/default-promotion only if D15 opt-in
   activation is complete.

## Exit Criteria

Complete disposition requires all of the following:

- H2637 default/off and opt-in active routed timing are recorded with exact
  commands, wall/user/sys time, solver counters, step counts, and slot/profile
  evidence.
- Timing is either optimized back into the accepted D15 budget or explicitly
  adjudicated as acceptable by current package authority/operator direction.
- `INV-OFEROUTE-010` subsystem-off protected outputs remain byte-identical.
- `INV-OFEROUTE-011` / `GAP-OFEROUTE-005` remains closed on the D10B path.
- `INV-OFEROUTE-012` active closure hard-fail is live in active mode and
  includes `ui_SCrunf` source consumption plus outlet `latqcc` bypass
  accounting.
- DC01 daily-lump runon is disabled for active routed lanes; no active lane is
  double-fed by both DC01 and Lane D routed water.
- Rev-21 friction operands are consumed by the real active production path,
  not only shadow/profile code.
- D12 source-shape limbs are consumed by the real active production path with
  daily-sum closure to lane-local supply.
- The D13 routed hydrograph feeds the erosion hourly substrate when routing
  owns the water path.
- Consumer-path evidence names producer source, in-memory state/frame object,
  runner handoff, downstream call site, output/API surface, and negative proof
  that the old compatibility path is not used for the activation claim.
- Accepted review findings are fixed and verified before completion.

If any criterion cannot be closed, write `artifacts/hold-legitimacy-audit.md`
and close `EXECUTED-HOLD-*`; do not partially flip production activation.

## Required Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Contract/profile/BEI checks required by touched `SC-*` contracts.
- Focused Lane D / `ofe_routing` tests.
- `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture`
- H2637-class endpoint timing with opt-in routed active path enabled.
- H2637-class slot/profile timing with opt-in routed path enabled.
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

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` gate blocks a complete
disposition.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/baseline-profile.md`
- `artifacts/slot-profile.md`
- `artifacts/optimization-plan.md`
- `artifacts/optimization-results.md`
- `artifacts/operand-lineage.md`
- `artifacts/active-owner-architecture.md`
- `artifacts/active-owner-implementation.md`
- `artifacts/consumer-path-proof.md`
- `artifacts/activation-readiness-audit.md`
- `artifacts/protected-output-byte-identity.md`
- `artifacts/gate-results.md`
- `artifacts/review-codex.md`
- `artifacts/review-qa.md`
- `artifacts/review-disposition.md`
- `artifacts/verification-codex.md`
- `artifacts/verification-qa.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`
- `artifacts/hold-legitimacy-audit.md` if any activation or timing criterion
  cannot close.

## Conservation / Output Acceptance

This package changes production ownership of surface water if active-owner
implementation proceeds. Before production edits, author an operand-lineage
table covering source terms, routed hydrograph, closure terms, runon admission,
erosion hourly shape, publication surfaces, units, denominators, and old-path
bypass. Acceptance must include independent reconstruction from produced
outputs and real consumer proof. Exact self-consistency, producer-only
counters, shadow diagnostics, or direct-runtime-internal evidence alone cannot
close activation.

## HOLD Legitimacy

The package may hold only for a named boundary that cannot be closed safely
inside this write set:

- behavior-preserving optimization cannot return the H2637 path to an
  accepted timing budget and no explicit timing acceptance exists;
- active-owner execution order requires a broader architecture package than the
  bounded direct-runtime/runner write set;
- canonical `SC-*` authority is missing or contradictory for an active
  production consumer;
- a required consumer still reads the old DC01/compatibility path after the
  in-envelope implementation route is exhausted.

The hold audit must name the blocker, cite evidence proving it, explain why the
in-envelope correction route cannot close it, and identify the first actionable
follow-on package/action.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
`comparator_suite_runner`, and `timing_comparator` subagents for read-only
source/authority audit, profiling review, optimization review, active-consumer
proof review, H2637 timing/comparator execution, and heavy gate execution.
Expected outputs are compact findings, timing metrics, comparator/gate metrics,
log paths, and package-local review or verification artifact text. Write access
is read-only unless the operator assigns a bounded write set for a named
implementation fix.

Subagent requirement: `comparator_suite_runner` or `timing_comparator` is
required for heavy H2637 endpoint/profile timing when available. If
session-level tooling prevents subagent dispatch, record that block and run
locally only when package governance permits.
