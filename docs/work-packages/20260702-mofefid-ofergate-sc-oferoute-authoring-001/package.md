# MOFEFID-OFERGATE — SC-OFEROUTE-001 Authoring

Status: **EXECUTED — REVIEW-READY** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Owner: Claude Code. Worktree: `mofefid-ofergate`. Scope: **contract text only**.

## Objective

Author `SC-OFEROUTE-001` — the canonical top-down science authority the
`ADR-0033` narrowed ratification requires **before** D4 solver work. This is
the prerequisite gate: D4 (single-OFE KWE/TVD solver) and D5 (OFE-by-OFE
cascade) are blocked until this contract is authored **and ratified**.

## What landed

- `SC-OFEROUTE-001.md`: purpose/scope (hillslope overland routing, distinct
  from watershed-channel `SC-ROUTE-001`); 11 authority anchors; variables +
  units; algorithm spec (friction menu eqs. 2-7, KWE A1-A2, TVD-MacCormack
  8-14, CFL 12); 11 invariants (`INV-OFEROUTE-001..011`); guard map; 3 gaps;
  revision history v1.
- Registered in `science-contracts/index.md`.

## Design anchors

- **`INV-OFEROUTE-001`** anchors the already-landed shadow-first friction
  kernels (`ofe_routing::friction`) retroactively — top-down order restored.
- **Solver/cascade invariants** (`005-009`) are the D4/D5 gates: KWE fidelity,
  scheme conservation, CFL stability, per-OFE hydrograph handoff over the
  `INV-RUNOFFPART-029` seam, and the DC01 runon re-infiltration coupling.
- **`INV-OFEROUTE-010`** keeps the default path byte-flat (opt-in only, per
  the operator decision + ADR-0033); default activation is a separate gate.
- **Frozen-library posture** is explicit in the anchors + `GAP-OFEROUTE-001/002`:
  eqs. 2-3 constants, eq. 4 bounds, k_o tables, and the TVD numerics cited
  secondary (via R-63 / KINEROS); eq. 4/5/6 primaries (R-77/72/78) in hand;
  unit conventions confirmed empirically by D-val, not asserted from an
  un-acquired primary.

## Named prerequisite carried forward

`GAP-OFEROUTE-003` / `INV-OFEROUTE-009`: the runon re-infiltration
reconciliation — when the routing subsystem is active it must own the hourly
runon supply without double-counting DC01's daily-lump admission
(`INV-RUNOFFPART-031`). Design (supersede vs. compose) is D5 scope.

## Acceptance

- Contract-text only; no production code/test change.
- Registered; internally consistent; the D3 kernels are now contract-anchored.
- Codex review + ratification decision at close (ratification unblocks D4).
