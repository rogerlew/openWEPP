# PERFARCH02 — Independent Review (Claude Code)

Verdict: **Sound CONDITIONAL GO — honest and well-disciplined.** The honest-measurement
hard-stop I set genuinely held, the result is real, and the disposition does **not**
overclaim. But be clear-eyed: this package did **not** measure the floor I scoped — it
measured the *blocked writeback/guard surface in isolation* (~49.9× faster), not the
integrated WB11 per-OFE-day flow. So **5×, and even ≤10×, remain unproven**; the real
floor measurement is the next package's job. That is exactly what the disposition says.

Evidence mode: **Static** (all 9 artifacts + the prototype `main.rs`/`Cargo.toml`) +
**Ran** (verified the harness wiring).

## The honest-measurement hard-stop held — I checked the harness

This was the thing I said I'd watch (did the prototype strip the guards to look fast?). It
did not:

- `logical_current` calls the **real production** `evaluate_kernel_writeback` /
  `apply_kernel_writeback` from `openwepp-kernel-contract` (path dependency to the actual
  crate, not a reimplementation). The 49.9× is a fair like-for-like, not a strawman. ✓
- The array path does the **real** finite / lower-upper-domain / known-slot / state-flux-class
  checks — same semantics as the production evaluator, just on dense slots. ✓
- **Identity validated before timing:** apply the same payload through both paths, export
  the array to maps, require **exact** state+flux map equality. **Failure-path parity**
  checked: invalid non-finite/domain payloads reject with the same message-id class and
  lazy subject resolution. So the measured array path is a *correct* replacement, not a
  faster-but-different one. ✓

## What it actually proved (real, but narrow)

- The **PERFIDX03/05-blocked class** — string-keyed writeback application + guard evaluation
  — is ~**49.9×** faster array-authoritative *and* preserves exact exported maps + failure
  semantics. That **rejects more read-mirror/id-table work** and **validates the
  array-authoritative direction** + the dual-write/export fixes.
- Crucially, the surface is **not the floor**: the array writeback/guard cost is **0.657 µs**
  per iteration vs the **193 µs/OFE-day ≤5× budget** — far below. So removing the
  writeback/guard string machinery is *necessary but nowhere near sufficient*; the floor is
  whatever the **rest** of the WB11 flow costs.

## What it did NOT prove (honestly disclosed)

- The **integrated WB11 floor**. The prototype is a synthetic surface (3072 state / 1024 flux
  symbols, 96+64 updates/iter) — it does **not** execute WB11 runoff, frost, plant, lateral
  drainage, erosion, storage, scheduler state movement, consumer-boundary validation, or
  publication. So the H2637 ratio is **not** extrapolated, ≤10× is a *target* not a
  demonstrated result, and **5× remains unproven**. The disposition, floor-measurement, and
  floor-prototype artifacts all say this plainly — no overclaim.

This is **narrower than the floor measurement I scoped** (I asked for the integrated WB11
flow). But it is honestly scoped, de-risks the single highest-value point first (the blocked
class works + isn't the floor), and correctly defers the real floor to an integrated pilot.
Acceptable — it is a disclosed scoping result, not a corner-cut sold as complete.

## The design + plan are substantive

- **Redesign shape** addresses all three problems concretely (`ArrayHotState`,
  `ArrayWritebackPayload`, `LogicalSurfaceView` at boundaries only; export moved *outward*;
  mirror removed; id-backed guards with lazy failure-path names). The `export_once` cost
  (0.001 µs) is measured separately and confirms boundary materialization is negligible —
  good evidence the PERFIDX03 export seam is solvable.
- **Staged plan** (A contract-shell → B WB11 integrated pilot → C scheduler flip → D family
  expansion → E publication boundary → F remove mirror) uses shadow-then-flip per family and
  — importantly — has explicit **Stop Conditions**: write a NO-GO if the integrated pilot
  shows the correct array path is still above the ≤10× budget, or conservation/guard/publication
  dominates such that **73× is the honest floor**, or it needs broad `SC-*` changes. That is
  the right anti-grind discipline.

## The answer to "can we reach 5×" — still pending one measurement

PERFARCH02 confirms the *direction* and clears the blocked class, but the 5× answer now
hinges entirely on **PERFARRAY01** (the integrated WB11 pilot): it measures the actual
per-OFE-day floor with real hydrology + conservation + scheduler + publication on array
state. Until then, ≤10× is the credible target and 5× is unproven — correctly stated.

## Disposition

Land the record (docs + the artifact-local prototype harness; `target/` git-ignored). Next:
**PERFARRAY01** — integrated WB11 array-authoritative pilot, which is where the floor (and
the real 5× verdict) gets measured. Its Stage-B stop conditions must be honored.
