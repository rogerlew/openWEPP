# Worker Handoff

Status: **HANDOFF-RECORDED — hold-lift packages required**.

## D16 / default-promotion status

D16 is **not** unblocked. Opt-in production activation did not complete.

## First actionable follow-on

Open a Lane D terminal-bin/day-boundary hold-lift package.

Objective:

- Reproduce H2637 day 88 `NegativeOutletBin` on the D10B conservative
  bin-series path.
- Bind contract-authorized semantics for source active in hour 24: exact
  non-negative outlet bin series, day-boundary storage/carry or drain-tail
  handling, and mapping to the 24-hour routed-hydrograph erosion consumer.
- Implement the correction without surrogate physics or silent fallback.
- Re-run the H2637 timing refresh and focused ignored H2637 evidence test.

Suggested write set:

- `SC-OFEROUTE-001` only if day-boundary/hydrograph-shape authority needs
  amendment.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
  and `cascade.rs` only if the bin-series/handoff correction is in scope.
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs` only if the shadow
  routing-window policy needs adjustment after contract authority is clear.
- Focused `ofe_routing` and `laned_shadow_h2637` tests.

## Second follow-on after timing blocker closes

Open the active-owner implementation package.

Objective:

- Add an explicit opt-in active selector.
- Route active lanes through Lane D as the surface-water owner.
- Disable DC01 daily-lump runon on active routed lanes and prove no double-feed.
- Construct active `INV-OFEROUTE-012` closure operands including `ui_SCrunf`
  source and `latqcc` bypass, with material residual hard-fail.
- Feed the D13 routed-hydrograph shape consumer.
- Prove rev-21 friction operands and D12 source-shape limbs are consumed by the
  active path.
- Prove subsystem-off/default protected-output byte identity.
