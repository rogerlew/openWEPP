# MOFEFID-D5 — OFE-by-OFE Cascade

Status: **EXECUTED — REVIEW-READY** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract: `SC-OFEROUTE-001` (rev 3). Owner: Claude Code. Worktree: `mofefid-d5`.
Activation: **opt-in / shadow-first**.

## What landed

`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`: the
OFE-by-OFE overland-flow cascade (SC-OFEROUTE-001 INV-OFEROUTE-008/009) —

- Routes each OFE with the D4 TVD-MacCormack solver; the upstream OFE's
  **outlet hydrograph becomes the downstream OFE's upstream boundary**
  (Papanicolaou assumption 1: route summit -> outlet), interpolated and
  **width-scaled** (`w_i / w_{i+1}`) for total-discharge continuity.
- Width-aware **cascade mass balance** (total m^3) with the clamp-adjusted
  conservation residual carried through from D4.
- Per-OFE outlet/received-runon volumes exposed for handoff verification.

Shadow-first: **not wired into any production phase span** (grep-verified);
default hillslope path byte-flat (`INV-OFEROUTE-010`).

## GAP-OFEROUTE-003 resolved: supersede-then-compose

The runon re-infiltration reconciliation with DC01 is **design-resolved**
(contract rev 3): when the routing subsystem is active it **owns the hourly
inter-OFE runon** and **supersedes** DC01's daily-lump admission
(`INV-RUNOFFPART-031`) so nothing double-counts; re-infiltration of the
routed runon is then the downstream OFE's **per-OFE hourly infiltration**
(`SC-RUNOFFPART-001`) acting on the routed hydrograph (**compose** at the
hourly step). D5 implements the routing + handoff; the runtime infiltration
composition + DC01-disable guard is the remaining **integration** scope.

## Validation (6 tests, no copyrighted data)

- **Cascade conservation (INV-006 across the cascade):** total rainfall
  excess = terminal outlet + total storage (width-aware), clamp-adjusted, to
  < 1%.
- **Handoff continuity (INV-008):** OFE i+1 received-runon volume = OFE i
  outlet volume (equal widths); and **width-change** handoff conserves total
  volume (unit discharge scales, volume invariant).
- **Downslope accumulation:** per-OFE peaks grow summit->outlet; terminal
  steady discharge ~ v x total length.
- **Case 3 (vegetation patchiness):** the vegetated strip has the same steady
  discharge (mass balance) but **backs up more water** (higher steady depth,
  monotone in f_eq) — the filter-strip signature judged on the invariant that
  survives to steady state, not the fragile transient peak.
- **Fail-closed:** empty cascade / non-positive width.

## D-val note (unchanged from D4)

Formal Ef-vs-observed (`INV-OFEROUTE-011`) remains the D-val stage: it needs
the SC-RUNOFFPART infiltration coupling (the GAP-003 compose step) for the
rainfall->runoff cases and digitized observed series. D5 lands the routing
mechanism the Ef stage will exercise once infiltration is composed.

## Gates

- `ofe_routing` 23/23 (6 friction + 12 solver + ... ); full orchestrator
  suite 171/171; clippy `-D warnings` 0; BEI PASS-DEFERRED; authority guards
  PASS; cascade shadow-first.

## Next (integration)

Compose the routing cascade with per-OFE hourly infiltration
(SC-RUNOFFPART) and disable DC01's daily-lump admission when active
(GAP-003 runtime), then run the D-val Ef.
