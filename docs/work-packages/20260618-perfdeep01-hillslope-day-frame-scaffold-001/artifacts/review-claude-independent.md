# PERFDEEP01 — Independent Review (Claude Code)

Verdict: **CONDITIONAL GO.** Stage 0's substance is sound — the frame + seed/flush + shadow harness exist,
the three migration ledgers are genuinely useful, the frame is correctly shadow-only (no hot-path change),
and gates/endpoint/identity are green. **But the disposition's "Stage-0 acceptance is *fully* met" is an
overclaim against the package's own bar**, on two points I reconcile below. Stage 1 may proceed, but it
inherits one deferred gate.

Evidence mode: **Static** (read `day_frame.rs`, the round-trip test, all artifacts) + **Ran** (none;
relied on Codex's gate runs).

## What is genuinely delivered (sound)

- **Frame is correctly shadow.** `+5` lines in `lib.rs` (a `mod` decl), no scheduler edit — the frame is a
  separate module, the maps stay authoritative. Endpoint flat (669.06 s vs 669.97 s, −0.14%; the 671.10 s
  was the determinism rerun, both in-noise); `.hbp`/`.wat`/`.loss`/`.plot` byte-identical, `pass` Arrow-equal.
  No production risk. ✓
- **Guard-tier catalogue (Finding 1) is a real closure:** 236-site exhaustive `WritebackField::bounded`
  inventory + a clear STATIC_BOUND vs RUNTIME_DERIVED_BOUND policy + a preserved-diagnostic-attribution
  contract. This is the right shape for Stage-1 guard migration. ✓
- **Publication-operand ledger (Finding 2)** credibly enumerates the WB13/WAT/PASS + HBP + manifest reads
  and maps each to a slot / typed-capture / array home. ✓
- **Contract-transition map (Finding 3)** present. ✓
- Workspace gates + `cargo deny` + markdown + determinism rerun: green. ✓

## Reconciliation 1 — the frame is slots, not "typed fields"; spec amended (not a defect)

The disposition says "`HillslopeDayFrame` now has a **typed schema**." It does not — it is
`state_slots`/`flux_slots: Vec<Option<BoundaryValue>>` keyed by `SymbolRegistry` id (the frame-schema
artifact states this honestly; only the disposition's wording is loose). This **deviates from ratified
spec §4.1** (named unit-typed fields, "no `Option`, no id indirection").

**My judgment: the slot representation is correct, and the spec was over-specified.** PERFARCH03 measured
*exactly this* `Vec<Option<BoundaryValue>>`-by-`SymbolId` representation at 0.96 µs/OFE-day = 146× — well
inside the ≤5× budget — and it eliminates phase-to-phase seams (the core thesis) identically to a
typed-field frame. Typed-named-field promotion is a *second-order* micro-optimization, not a viability
requirement. **I amended spec §4.1 to ratify the slot baseline and demote typed-fields to an open fork**
(commit accompanying this review). Codex made the right engineering call (build what was measured); my spec
over-specified. No defect — a reconciliation.

## Reconciliation 2 — the round-trip gate was NOT met as specified (becomes Stage 1's first gate)

Package §2 + Acceptance required: *"seed a frame from a **real** H2637 OFE-day state/flux surface, flush it
back, assert `to_bits()` equality on **every** symbol."* What shipped: `perfdeep01_h2637_like_warm_rain_surface()`
hand-builds **~20 synthetic warm-rain scalars** — no frost/snow/irrigation/MOFE families, not a real
surface. Two consequences:

- The **family-specific paths** (the MOFE hourly-array capture, the typed I/O-edge capture) are exercised
  only on a toy that may not contain those symbols — so their fidelity on the real symbol set is unproven.
- For a generic slot frame the round-trip is **near-tautological** (slots store `BoundaryValue` losslessly,
  as the schema notes), so even where it passes it proves the *plumbing*, not full-symbol coverage.

This is a **genuine completeness gap** — the exact "complete, not representative" bar the package set to
avoid the PERFMIG02 trap. It is **low-risk** (the frame is shadow; the slot mechanism is uniform), and it
is **cheap to close** (dump a real H2637 OFE-day surface — including a frost/snow day and a MOFE multi-OFE
day — and round-trip it). Because the frame does not become authoritative until Stage 1, the right place to
close it is **PERFDEEP02's first entry gate**, before any phase is migrated onto the frame. It must not be
silently dropped.

## Disposition

CONDITIONAL GO — land the Stage-0 scaffold (sound, shadow-only, zero production risk) + the spec amendment
+ this review. **Authorize PERFDEEP02 (Stage 1 — hydrology island) with two carried conditions:**
1. **First Stage-1 gate:** the real-surface, every-symbol seed/flush round-trip (incl. frost/snow/MOFE
   families) — the deferred Stage-0 completeness item.
2. **Inherited representation:** the slot frame (spec §4.1 amended); typed-field promotion stays an open
   fork pursued only if a later endpoint demands it.

The architecture is intact and the hard ledger work is done; the correction is to the *claim* ("fully
met"), not the direction. Stage 1 proceeds — it just opens by closing the one gate Stage 0 left ajar.
