# MOFE Effective-Length / Equivalent-Plane Routing and its Transport-Capacity Coupling — 2026-07-04

Status: Draft
Last updated: 2026-07-04
Evidence mode: Static (read of `wepp-forest_260430_baseline/src`, the wepp-forest work-package record, and the openWEPP crates; reasoned. No binary executed, no `cargo test`, no numerical parity diff.)
Scope: **In scope** — the legacy WEPP multi-OFE *effective-length / equivalent-plane* overland-flow construction and its coupling into sediment transport capacity (`wepp-forest_260430_baseline`); the forest-port work-package record's root-cause findings on the MOFE water "blowup"; and two named openWEPP porting cut-points (the `efflen/totlen` `peakro` rescale, and rill-vs-field width in the sediment dimensionalization). **Out of scope** — numerical parity against legacy, watershed channel routing, and any "what should we do" sequencing (that is a planning artifact, not an audit — see the companion note `docs/planning/mofe-water-balance-sequencing.md`).

## 1. Purpose

The MOFE (multiple-OFE) water balance has a long-observed magnitude "blowup": per-OFE event-day closure spikes and whole-hillslope daily closures exceeding 1000 mm on multi-OFE hillslopes, with the outlier always at the terminal OFE. This audit answers three current-state questions:

1. **What is the legacy multi-OFE effective-length construction actually computing**, and what is its transport-capacity coupling?
2. **Where is the unsoundness** — is the MOFE water blowup an arithmetic bug, an input/authority problem, or a model-class property of the construction itself?
3. **Does openWEPP's current erosion code reproduce the two porting hazards** this construction is known to carry: the `efflen/totlen` `peakro` rescale, and the rill-vs-field-width sediment dimensionalization?

It does **not** answer "what should openWEPP do about it" — that is deliberately excluded per this directory's charter.

## 2. Method

- **Delegated static reviews (3 agents), whose load-bearing citations I re-verified first-hand:** one agent read the forest-port MOFE work-package record (`/workdir/wepp-forest/docs/work-packages/`, `docs/ablation/`); one read the legacy Fortran (`/workdir/wepp-forest_260430_baseline/src/`); one reconciled the openWEPP docs/code record. Their conclusions are attributed as such below; every legacy `file.for:line` and openWEPP `crate…#Lnn` citation carried into this audit was opened and confirmed by me this session.
- **Legacy Fortran re-verified directly:** `cefflen.inc`, `irs.for:330-364` and `:740-764`, `xinflo.for:124-193`, `sloss.for:158-171`; plus greps establishing the capacity-symbol vocabulary (`grep -nE '\b(tcap|dcap|tcend)\b' runge.for erod.for param.for`) and that **no literal `qcap` token exists** anywhere in `src/`.
- **openWEPP code read directly:** [erosion_seed.rs](../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_seed.rs#L238), [erosion_continuity.rs](../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs#L1763), [runoff.rs](../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs#L820).
- **Did not**: execute any binary; run `cargo test`/`cargo check`; diff openWEPP numerically against legacy; audit the watershed channel path.

## 3. What the construction computes

### 3.1 Effective length is a *length* — the "equivalent plane" abstraction

`efflen` is defined verbatim as *"the effective flow length … for continuous flow planes — efflen is the sum of the plane lengths having flow"* (`cefflen.inc:16-20`). WEPP is a steady-state, within-storm, single-plane kinematic model; to represent a chain of OFEs (differing soil / slope / cover) without solving unsteady routing between them, it **collapses a run of continuous-flow OFEs into one equivalent plane** whose length is the accumulated slope length, and whose infiltration parameters are length-weighted averages over that plane:

```fortran
! irs.for:335-353
if (iuprun(iplane).eq.0) then          ! no upslope runon → fresh flow group
  ibpln = iplane
  efflen(iplane) = slplen(iplane)
else                                    ! upslope runon → accumulate
  efflen(iplane) = efflen(iplane-1) + slplen(iplane)
end if
aveks(iplane) = sumks / efflen(iplane)  ! equivalent-plane averaged soil
avesm(iplane) = sumsm / efflen(iplane)
```

So within a continuous-flow group, `efflen(k) = Σ slplen(i)` from the group start. A "case 1–4" runon-runoff taxonomy (`irs.for:355-362`) classifies each OFE break — case 2 begins flow (`re>0`), case 3 continues it (`q(iplane-1)>0, re=0, q(iplane)>0`), case 4 is where runoff dies out. `efflen` resets only when a group ends; it is bounded only by the physical `totlen` (`efflen ≤ totlen`), not by any artificial cap.

### 3.2 Water crosses OFEs with no routing; `efflen` sets the discharge

There is **no St. Venant / kinematic routing between OFEs**. The previous OFE's outflow is handed to the next as inflow, and the current outflow is rebuilt from `peakro·efflen`:

```fortran
! xinflo.for:130,150,186
qin  = qout                              ! entire downslope water hand-off
qout = peakro(iplane) * efflen(iplane)   ! specific discharge (m^2/s)
qshear = qout * rspace(iplane)           ! per-rill discharge (m^3/s)
```

`peakro` itself carries a recently-added length normalization (`irs.for:745-756`, *A. Srivastava 4/17/2026*):

```fortran
if (contrs(...).ne.0) then
  peakro(l) = runoff(l)/effdrn(l)                       ! contoured: unscaled
else
  peakro(l) = (runoff(l)*efflen(l)/totlen(l))/effdrn(l) ! non-contoured: efflen/totlen rescale
endif
```

whose own comment states it exists *"to map equivalent-plane runoff back to OFE area … For a single OFE, EFFLEN=TOTLEN, so this reduces to the original PEAKRO = RUNOFF / EFFDRN expression."* Combining, non-contoured `qout = runoff·efflen²/(totlen·effdrn)`.

### 3.3 Two different "capacities" — and "qcap" is not a legacy symbol

**There is no `qcap` token anywhere in the legacy source** (grep returns zero hits); it is work-package vocabulary. Two physically distinct capacities exist, and conflating them is the classic error:

| Concept | Symbol | Units | Where | Meaning |
|---|---|---|---|---|
| Hydrology runoff cap | (WB-05A "q-cap") | mm | `watbal_hourly.for` | mass-conservation clamp on hourly runoff; enforcement **bypassed when `efflen ≤ slplen`** (the WB-05A defect) |
| Sediment transport capacity | `tcend`, `tcap`, `tottc` | kg·m⁻¹·s⁻¹ | `param.for:229`, `runge.for:102`, `yalin.for` | limits sediment load to what the flow can carry |
| Detachment capacity | `dcap` | — | `runge.for:89`, `erod.for:188` | excess-shear law `dcap = eata·(shr−tauc)` |

Transport capacity is `tcend = kt·shrsol^1.5` (`param.for:229`), driven by shear `shrsol = shears(qshear, slope)` — i.e. it depends on the water discharge, and therefore on `efflen`, only through the shear. The `param.for:184-379` block titled *"Transport Capacity continuous at OFE breaks"* is a **numerical-continuity device** (forcing `tc` to vary smoothly across the slope/soil discontinuity at an OFE boundary) and is dense with anti-overflow guards (`shrati` capped at 1e12, denominators floored, *"prevent … model bomb"*).

### 3.4 The efflen → transport coupling: real, monotonic, but sub-linear

Tracing the exponents as coded (delegated agent's chain, spot-verified against `shears.for`/`param.for`):

`efflen` ↑ (linear) → `qout = peakro·efflen` ↑ → `qshear = qout·rspace` ↑ → `shrsol ∝ qshear^~0.47` → `tcend ∝ shrsol^1.5 ∝ efflen^~0.7`.

The transport feedback is **sub-linear in `efflen`** and bounded three ways: rill width capped at `rspace` (`shears.for:88`), RK4 load capped at `tcap` in the continuity RHS (`runge.for:107`), deposition parameter `phi` hard-limited to ±1e5 (`param.for:624-625`). **Nothing in the transport chain is unbounded or super-linear.**

### 3.5 The reconciliation — where the blowup actually lives

Two careful static reads reached apparently-opposite verdicts, and both are correct because they describe different quantities under different normalizations:

| Quantity | Formula | Normalizer | Behavior on a 19-OFE continuous chain |
|---|---|---|---|
| **QOFE** (reported runoff *depth*, outlet-referenced) | `runoff·efflen/slplen` (`watbal.for:1099`) | terminal OFE's own **small** `slplen` | amplifies by `efflen/slplen = totlen/slplen ≈ 10.9×`; `qofe` observed to 5943 mm |
| **Transport discharge** | `qout = runoff·efflen²/(totlen·effdrn)` | whole-hillslope **`totlen`** | `efflen/totlen ≤ 1` → a *reduction*; `tcend ∝ efflen^~0.7` |

**The blowup is on the water / QOFE / closure-audit surface** (the `runoff·efflen/slplen` amplification), **not on the transport-capacity surface.** QOFE takes all the accumulated flow and references it to just the terminal OFE's small footprint. Transport capacity is a *downstream inheritor* of the resulting non-physical hydraulic state — the forest-port register states this directly: *"sediment yield numbers derived from non-physical hydraulic state"* (CONFLICT-006). The operator's recollection that "effective length interacts with transport capacity" identifies the right pivot (`efflen`), with the refinement that `efflen` amplifies the **runoff depth**, and transport capacity is the victim, not the amplifier.

## 4. Findings

1. **The MOFE water blowup is a model-class property, not an arithmetic bug.** The equivalent-plane / effective-length construction has **no relief valve along the cascade** — no re-infiltration or transmission loss *between* OFEs — so upslope runoff accumulates undiminished onto the terminal OFE. The forest-port record registers this as **CONFLICT-005, "1D OFE cascade has no relief valve at the terminal OFE," dispositioned out-of-scope** as a model-class limitation (*"cannot fix a model-class limitation … route those use cases to a different model"*), and **CONFLICT-001, "efflen amplification has no physical bound"** (`wepp_hydrology_assumption_register.md:47`).

2. **The amplification is bounded-but-large, and lives in the QOFE normalization.** `efflen ≤ totlen` (bounded by physical length), but `QOFE = runoff·efflen/slplen` amplifies by `totlen/slplen ≈ 11×` at a 19-OFE outlet. This drives the >1000 mm closure spikes and the WB-05A OFE19 catastrophe.

3. **The transport-capacity coupling is bounded and sub-linear** (`tcend ∝ efflen^~0.7`, capped three ways). It is not itself an amplifier; it inherits non-physical hydraulic state from the QOFE side.

4. **The forest port confirms "patching symptoms," in the record's own words.** The `efflen/totlen` `peakro` rescale (`irs.for:745`, consumer-side, 4/17/2026); a state-side water cap (`U6C`) **deployed then withdrawn** (`wepp_260501` removed 2026-05-03, self-labeled *"another band-aid"*); the true producer-side fix (bounding the `irs.for:335-346` recurrence) **named but declined**; dozens of *"prevent model bomb"* guards; and CONFLICT-008: *"Years of un-audited runs have undocumented closure debt."* The ">1000 mm carved-letter cohort" assessment puts *physical-defect confidence* **Low**, attributing the numbers primarily to audit-contract / area-normalization mismatch.

5. **openWEPP cut-point 2 (rill-vs-field width): CLEAN / legacy-faithful.** The Wave-1 sediment dimensionalization at [erosion_continuity.rs:1771](../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs#L1771) is `denorm = effdrn_s · tcend · width_m / rspace_m` — the **rill** `width_m` (grown from `shears`) over rill spacing, faithful to `sloss.for:166`. Field width enters **only** as the final kg/m → kg plane-width multiplier: `total_detachment_kg = total_detach_kg_m · field_width_m` ([:1859](../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs#L1859)); the interrill surface ([:1831](../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs#L1831)) does the same. The `param.for:239-241` *"always use rill width or you get sediment > soil loss"* 10–100× inflation hazard is **avoided**.

6. **openWEPP cut-point 1 (`efflen/totlen` `peakro` rescale): no current gap; latent at multi-OFE.** openWEPP's erosion discharge is a faithful `qout = peakro·efflen` / `qshear = qout·rspace` ([erosion_seed.rs:238-239](../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_seed.rs#L238)). Its `peakro` is the **WB16 kinematic peak-rate estimator** `vave·qpstar` ([runoff.rs:840,850](../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs#L840)) — carrying `efflen` but **no `totlen` rescale** — whereas legacy feeds the `efflen/totlen`-scaled `runoff/effdrn`. This is an intentional operand divergence, but it is **inert on the enabled path**: the Wave-1 erosion solve is **single-OFE-only**, where `efflen = totlen` and the legacy rescale is identity by construction (per its own comment). The divergence becomes load-bearing only when multi-OFE Wave-1 chaining is wired (roadmap §E.3): the cross-OFE discharge hand-off (`qin/qout`, `efflen` accumulation, the `peakro` basis) is presently **undefined for the multi-OFE erosion case** in production — the legacy EROD14/Wave-2 multi-OFE path emits zero sediment (per `WSHED-W7DC01` / roadmap §E.3).

## 5. Caveats

- **Static only.** No numerical parity was computed. Exponent estimates (`shrsol ∝ qshear^~0.47`, `tcend ∝ efflen^~0.7`) are read off the coded power laws, not measured; the "~11×" QOFE factor is `totlen/slplen` arithmetic on the cited geometry, not a run.
- **Delegated evidence.** The WP-record root causes (CONFLICT-001/005/006/008, the U6C withdrawal, the >1000 mm cohort verdict) come from a delegated static read of the wepp-forest record; I re-verified the legacy Fortran and openWEPP code citations but did **not** independently re-open every WP artifact — the CONFLICT-* quotes are attributed to that record as cited.
- **Baseline recency.** `wepp-forest_260430_baseline` is a 2026-04-30 snapshot; the Srivastava `efflen/totlen` rescale is dated 4/17/2026 and is present in it. A newer baseline could differ.
- **openWEPP recency.** Cut-point citations are current as of 2026-07-04 on `erosion-e1-inc1c-fidelity`; the Wave-1 single-OFE enable gate and the zero-sediment multi-OFE path are as recorded in roadmap §E.3 at that date.

## 6. Recommended follow-ups (not performed in this audit)

- Define the multi-OFE erosion discharge hand-off **before** wiring roadmap §E.3 (cut-point 1): whether openWEPP's per-OFE-native `peakro` needs any equivalent-plane "map-back" normalization, or whether the per-OFE-lane water architecture supersedes it. This belongs in the Increment-2 entry gate, not here.
- The forward framing (treating the resumed MOFE water-balance work as *completing* the equivalent-plane replacement rather than chasing legacy parity) is a planning question — see `docs/planning/mofe-water-balance-sequencing.md`.
