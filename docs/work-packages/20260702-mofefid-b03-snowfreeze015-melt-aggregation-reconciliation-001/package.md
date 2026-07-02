# MOFEFID-B03 — SC-SNOWFREEZE-015 Melt-Aggregation Reconciliation

Status: **EXECUTED — REVIEW-READY** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane B
(B01 finding B10 disposition). Owner: Claude Code. Worktree: `mofefid-b03`.
Scope: **contract text only** — no production code change.

## Objective

Close B01's B10: `INV-SNOWFREEZE-015` reads as requiring the legacy
net-melt-scaling (`route pstvML + ngtvML`) while the production
implementation (`redistribute_daily_signed_snowmelt`) routes the
positive-parts sum under the SNOWSCI-S1 single-source store. On a
positive+negative coexist day these differ.

## Finding: not a defect, not a live contract conflict

Investigation (Static, this package): **`INV-SNOWFREEZE-019` (SNOWSCI-S1,
HPHYS0284/0285/0303) already resolves this.** It:
- single-sources runtime SWE from the post-hourly depth/density store;
- states for the positive+negative coexist case that "negative raw melt
  may affect diagnostic signed-melt lineage but must not create a second
  SWE debit after the depth/density store has already recorded pack loss";
- requires routed melt = authoritative storage loss (`old SWE + snowfall +
  retained rain − new SWE`) — i.e. positive-parts-consistent;
- **explicitly "supersedes the separate openWEPP SWE-debit
  interpretation"**, and lists `INV-SNOWFREEZE-015` in its own authority.

The implementation (`redistribute_daily_signed_snowmelt`: routed =
`snowpack_state_loss` = positive melt total) satisfies `INV-SNOWFREEZE-019`.
So B10 is a **text-clarity gap**: `INV-SNOWFREEZE-015` never cross-referenced
its own superseding invariant, so read alone it appears to mandate the
net-algebra.

## Physics grounding (why positive-parts is correct, not a coin-flip)

Under the `INV-SNOWFREEZE-002` density gate, sub-350 kg m⁻³ hours retain
positive melt **in-pack** (density increase, never exported); export happens
only above the gate. So a below-gate refreeze is of **in-pack** liquid and
must not be netted against **above-gate exported** melt — different water.
Legacy `winter.for` net-scaling came from an ungated melt model; applying it
would wrongly subtract in-pack refreeze from exported melt. Positive-parts
routing is therefore the correct openWEPP behavior, and equals the
net-algebra on the density-gate-consistent reachable domain (no openWEPP
production OFE-day observed to reach a mixed exported-melt + refreeze branch;
consistent with the brief's 0/21.7M and H2637 byte-stability).

## Amendment (SC-SNOWFREEZE-001 rev 116)

`INV-SNOWFREEZE-015` text now states the net-melt-scaling is the legacy
porting reference, and for the coexist case defers explicitly to
`INV-SNOWFREEZE-019` (routed melt = storage loss, positive-parts-consistent)
grounded in the `INV-SNOWFREEZE-002` density gate — with a tripwire: if the
mixed exported-melt + refreeze branch becomes reachable, re-adjudicate under
INV-019, don't silently ship. `INV-SNOWFREEZE-019` and `-002` added to
INV-015's authority list.

## Acceptance

- No production code/test change (contract text only).
- Amendment is internally consistent with the already-ratified
  `INV-SNOWFREEZE-019`/`-002`; no invariant now contradicts another.
- Codex review at close.
