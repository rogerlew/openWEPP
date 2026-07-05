# GAP-OFEROUTE-006 — Subsurface-Coupling Seam Design

Author: Claude Code, 2026-07-05. Evidence: **Static** (SC-SUBHYD-001
surfaces + the direct-runtime DC01 saturation-carry lineage +
SC-OFEROUTE-001 rev 2). Design-only; implementation is the activation
work.

## D1 — The exfiltration source term IS `ui_SCrunf`

WEPP's model class has exactly one subsurface→surface pathway: the
hourly WB19 tail clips positive top-layer saturation excess into the
24-slot `ui_SCrunf` (`SC-SUBHYD-001#INV-SUBHYD-023` — "clip positive
top-layer saturation excess into 24-slot `ui_SCrunf`, and leave no
material post-clipping top-layer excess"). Return flow (subsurface
lateral inflow exceeding storage/transmission) surfaces THROUGH that
clip: inflow raises `st(1)` beyond `fzul` and exits as saturation
excess. There is no second exfiltration surface to design.

**Seam rule:** when the router is active, each OFE's kinematic-wave
lateral source RATE is
`s_h = (wb14_hourly_excess_m[h] + ui_SCrunf[h]) / 3600 s` (m s⁻¹ —
both carries are DEPTHS per hour slot; the depth→rate conversion is
the seam's one required, recorded unit helper), uniform over the
routing substeps within the hour — the SAME two limbs the DC01
transfer weights already unify (`runoff.rs`
`dc01_surface_runoff_hourly_weights`: `excess + saturation carry`),
now consumed as a rate series instead of a day-shape. No new physics,
no re-derivation: the router replaces the day-lump aggregation of an
existing, contract-governed surface.

## D2 — The inter-OFE subsurface carry stays subsurface

`ui_LfCrf` (the 24-slot lateral carry) continues to feed the DOWNSLOPE
OFE's soil column, exactly as today. The router owns only the SURFACE
inter-OFE path (`INV-OFEROUTE-009` already supersedes DC01's
daily-lump SURFACE runon); it never intercepts the subsurface carry.
Subsurface water surfaces only where D1 exfiltrates it — on whichever
downslope OFE the clip occurs. This resolves the seam-ownership
question with one sentence: **the router supersedes surface runon,
never subsurface transfer.**

## D3 — Baseflow export bypasses the router

The outlet OFE's non-exfiltrated lateral outflow (`latqcc` at the toe,
the `INV-SUBHYD-033` ENV-Y term) exports on the existing subsurface
channel (WB19 → watershed baseflow lane) untouched. The router MUST
NOT consume, rescale, or re-route it. Conservation is enforced by D4's
closure identity carrying the term explicitly.

## D4 — The activation closure identity

With the router active, per hillslope-day at the WB closure tolerance:

```
P + Q_surface_in = Q_surface_out + latqcc_outlet + ET + ΔS + deep_perc
```

where `Q_surface_out` is the routed toe hydrograph integral and
`latqcc_outlet` the D3 export. Both gate fixtures assert this identity;
runtime hard-fails on material non-closure (`INV-OFEROUTE-012` (c),
unchanged posture).

## D5 — Time-base precondition (hourly lane required)

The seam consumes the HOURLY carry arrays (`ui_SCrunf`, `ui_LfCrf` —
`INV-SUBHYD-023` surfaces). Daily-lane hillslopes do not publish them:
activation on a daily-lane hillslope FAILS CLOSED (a typed activation
error, not a silent fallback to daily lumps). This is an activation
precondition, not a new lane requirement — the hourly lane is already
the MOFE-current path.

## Gate fixture specifications (`INV-OFEROUTE-012`)

1. **Subsurface-excess-to-runoff fixture (crafted):** a two-OFE lane
   with a thin conductive layer over an impeding layer and high
   upslope lateral transfer, driven so hours with ZERO rainfall excess
   carry positive `ui_SCrunf` on the downslope OFE. Assert: the routed
   toe hydrograph is nonzero on those hours (exfiltration reaches the
   surface); the D4 identity closes; deactivating the router reproduces
   today's DC01 byte-identical surfaces (`INV-OFEROUTE-010`).
2. **Subsurface-dominated closure vector (H2637-class):** the real
   H2637 inputs (the MAGPARITY01 instrument; ~99% routed-lateral
   share). Assert: the D4 identity closes with the subsurface terms
   present; `latqcc_outlet` export is unchanged by activation within
   tolerance; ENV-Y (`INV-SUBHYD-033`) remains in-envelope — the router
   must not move the water-yield fraction of a subsurface-dominated
   hillslope materially, since it owns only the (here ~1%) surface
   share.

## What this design deliberately does NOT do

No new exfiltration physics (the `ui_SCrunf` clip is the model class);
no router ownership of subsurface transfer; no daily-lane synthesis;
no change to the D-val (`INV-OFEROUTE-011`) posture — the four-case
non-reproduction dispositions are a separate item.

## Codex review round 1 — response record (2026-07-05)

One Medium (three parts), CONFIRMED and fixed in SC-OFEROUTE-001
rev 4: the seam surfaces are now carried through the contract's
structural maps — a subsurface-seam required-inputs row (with the
depth-per-hour → m s⁻¹ rate conversion recorded as the required unit
helper, and `ui_LfCrf`/outlet `latqcc` explicitly listed as
NON-inputs), unit-governance rows for `ui_SCrunf`/
`wb14_hourly_excess_m`/`latqcc`, and the missing `INV-OFEROUTE-012`
guard-map row. This artifact's D1 seam rule is corrected to state the
rate conversion explicitly. Binding-exposure lint: PASS-DEFERRED
(pre-existing follow-on rows, unchanged count).
