# MOFEFID-D15 - Opt-In Production Activation

Status: **EXECUTED-HOLD-SOURCE-AUTHORITY** (2026-07-06). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: `SC-OFEROUTE-001` opt-in production activation gate.

## Objective

Make the Lane D router the opt-in production owner of the surface-water path:
disable DC01 daily-lump runon on active lanes, route the hourly surface source
series through `ofe_routing`, hard-fail the active runtime closure identity,
publish active outputs from the routed path, feed the D13 routed-hydrograph
erosion consumer, and keep subsystem-off protected outputs byte-identical.

## Execution Result

D15 executed the pre-implementation authority gate and closed in
`EXECUTED-HOLD-SOURCE-AUTHORITY`. The production flip is not currently
authorized: `SC-OFEROUTE-001` rev 23 still leaves `INV-OFEROUTE-011` open
through Case 4 and records `GAP-OFEROUTE-005` as
`EXECUTED-HOLD-SOURCE-AUTHORITY`, explicitly blocking active routed-water
publication/default promotion and requiring a source-authority reconciliation
before production activation can use the H2637 diagnostics as acceptance.

No Rust runtime code, selector, publication path, DC01 disable, output schema,
or `SC-*` contract semantics were changed.

## Rationale

D9-D14 closed the non-numerics validation pieces, friction operand authority,
hourly source-shape coverage, routed-hydrograph erosion consumer, and runtime
cost budget. Those are necessary activation inputs, but they do not close the
remaining D10 shock-numerics/source-authority hold. `SC-OFEROUTE-001` names
that hold as blocking active routed-water publication and default promotion.

D15 therefore cannot truthfully claim opt-in production activation yet. The
correct execution is a preflight hold with a narrow handoff: close
`GAP-OFEROUTE-005` / the Case-4 source-authority reconciliation, then rerun the
D15 activation package against the D14 timing budget.

## Scope

### Included

- Pre-implementation authority gate over `SC-OFEROUTE-001`, Lane D strategy,
  D10-D14 dispositions, and the current runtime activation surfaces.
- Confirmation that D14 delivered the runtime budget and that D11-D13
  consumer inputs exist for a future active path.
- Hold-legitimacy audit naming the contract blocker and the smallest
  follow-on route.
- Package artifacts, work-package catalog, Lane D planning, and roadmap
  updates.

### Excluded

- No production/default Lane D activation.
- No D15 selector, env gate, config gate, DC01 disable, routed-path
  publication cutover, manifest activation claim, or output schema change.
- No D10 `GAP-OFEROUTE-005` shock-numerics correction, limiter/handoff
  method change, Case-4 acceptance, tolerance change, or source-authority
  reconciliation.
- No D11 friction-source policy change.
- No D12 melt-limb source-shape change.
- No D13 erosion-shape semantic change.
- No D14 profiling/optimization change.
- No D16 default-promotion policy.
- No surrogate, provisional, proxy, empirical stand-in, heuristic physics, or
  numerical-method shortcut.

## Dependencies

- `SC-OFEROUTE-001` rev 23:
  - `INV-OFEROUTE-011`
  - `INV-OFEROUTE-012`
  - `GAP-OFEROUTE-005`
  - `GAP-OFEROUTE-006`
  - `GAP-OFEROUTE-007`
- `SC-SED-001` rev 53 (`INV-SED-013` routed-hydrograph erosion consumer).
- D10 hold:
  `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/`.
- D11 dynamic friction closure:
  `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/`.
- D12 melt-limb source-shape closure:
  `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/`.
- D13 routed-hydrograph erosion-shape closure:
  `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/`.
- D14 runtime budget:
  `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/`.

## Intended Write Set

Executed write set:

- `docs/work-packages/20260705-mofefid-d15-opt-in-production-activation-001/`
- `docs/work-packages/README.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md`
- `docs/ROADMAP.md`

Protected:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- Rust runtime and output code.
- Tests and fixtures.
- Public pass/HBP schemas.

## Phase Plan

1. **D15-S0 - Intake and authority gate.** Read `SC-OFEROUTE-001`, Lane D
   strategy, and D10-D14 dispositions. Decide whether production activation is
   contract-authorized.
2. **D15-S1 - Activation surface audit.** If authorized, trace active selector,
   DC01 disable, routed publication, closure hard-fail, D13 erosion consumer,
   and default/off identity surfaces. If not authorized, document the blocking
   contract surface.
3. **D15-S2 - Contract-first amendment or HOLD.** If authorized, amend
   contracts before code. If not authorized, close in `HOLD` with evidence.
4. **D15-S3 - Implementation.** Only if authorized: wire active opt-in
   production routing and contract-derived tests.
5. **D15-S4 - Evidence and closure.** Run package gates, review/verification,
   final disposition, and handoff.

Actual execution: D15 stopped at S2 with a source-authority HOLD.

## Exit Criteria

For a complete activation, D15 would require:

- Opt-in active H2637/real-vector evidence.
- Default/off byte identity.
- Runtime closure hard-fail when active closure is materially non-closing.
- DC01 daily-lump runon disabled for active lanes.
- Routed-path publication and manifest provenance.
- D13 erosion consumer reading `routed_hydrograph_runoff_fraction`.
- Rev-21 D11 friction operands and D12 melt limb proven in the active
  consumer.
- D14 runtime budget retained or refreshed.
- No default promotion.

Current result: these activation criteria are **BLOCKED** by
`GAP-OFEROUTE-005` / `INV-OFEROUTE-011`; the package closes as
`EXECUTED-HOLD-SOURCE-AUTHORITY`.

## Required Gates

Executed D15 hold gates:

- `git diff --check`
- Markdown lint for touched docs.
- Static authority audit over `SC-OFEROUTE-001` rev 23.
- Static write-set audit confirming no runtime, contract, schema, fixture, or
  test files changed.
- Hold-legitimacy audit.

Full activation gates remain deferred to the hold-lifted D15 rerun.

## Conservation / Output Acceptance

D15 would be conservation-sensitive if activation were authorized: active
routing would own the water path, `latqcc` bypass closure, and erosion
hydrograph shape. Because D15 holds before runtime edits, no water/sediment
output changed. Any future D15 activation rerun must include independent
closure reconstruction and protected-output identity, not only self-consistent
manifest assertions.

## HOLD Legitimacy

This hold is legitimate because the blocking authority is canonical and
outside D15's permitted write set. `SC-OFEROUTE-001#GAP-OFEROUTE-005` requires
a source-authority reconciliation for the reduced KWE limiter/handoff and
Iwagaki friction mapping before active routed-water publication can be
accepted. D15 explicitly excludes that D10 work.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` subagents for read-only source/authority audit,
review, verification, fixture inspection, H2637/Lane D evidence, and heavy
gate execution. Expected outputs are compact findings, gate metrics, log
paths, and package-local review or verification artifact text. Write access is
read-only unless the operator assigns a bounded write set.

D15 did not dispatch subagents because execution stopped at the static
authority gate and no runtime code or heavy fixture gate was in scope.
