# MOFE Water-Balance Sequencing: Completing the Equivalent-Plane Replacement

Status: **planning note** (2026-07-04). Author: Claude Code (operator-directed).
Companion diagnosis: [`docs/audits/20260704_mofe_effective_length_transport_capacity_audit.md`](../audits/20260704_mofe_effective_length_transport_capacity_audit.md).

Evidence mode: **Static** synthesis of the companion audit, the wepp-forest work-package record, and the openWEPP contract/code surface. This file is planning guidance, **not** science authority. Canonical authority remains the `SC-*` registry, ADR-0011/0017/0033, `SC-SUBHYD-001#INV-SUBHYD-033`, `SC-RUNOFFPART-001`, `SC-OFEROUTE-001`, and `docs/ROADMAP.md`.

## 0. TL;DR

> The MOFE water "blowup" is **not** a bug to chase to zero against legacy — it is a **model-class limitation of WEPP's equivalent-plane / effective-length routing**: a 1-D OFE cascade with **no relief valve at the terminal OFE** (audit Finding 1; wepp-forest CONFLICT-005). Legacy has patched its symptoms for years (audit Finding 4). **openWEPP has already begun the sound replacement** — genuine per-OFE lanes that reject the equivalent-plane collapse (`SC-RUNOFFPART-001#INV-RUNOFFPART-029`), a re-infiltration relief valve at the cascade (`INV-RUNOFFPART-031` / DC01), and an opt-in OFE-by-OFE hydraulic router (Lane D / `SC-OFEROUTE-001`). **The resumed MOFE water-balance work should be framed as *completing that replacement*, judged against the field-observed envelope (`INV-SUBHYD-033`), not against legacy's 55.5% / 127.7% outputs.**

## 1. The diagnosis, in one paragraph

WEPP represents a multi-OFE hillslope by collapsing a run of continuous-flow OFEs into one "equivalent plane" of length `efflen` (the accumulated slope length) and applying a steady-state kinematic solution; water crosses OFE boundaries with no routing (`qin=qout`), and the reported per-OFE runoff depth `QOFE = runoff·efflen/slplen` is referenced to the terminal OFE's *own* small footprint, so it amplifies by `efflen/slplen ≈ 11×` at a 19-OFE outlet. Because the equivalent plane has **no mechanism for water to leave the cascade except at the bottom**, upslope runoff piles undiminished onto the terminal OFE. That is the root of the >1000 mm closures and the OFE19 catastrophe. Sediment transport capacity is a *downstream inheritor* of the resulting non-physical hydraulic state, not the amplifier. Full mechanism, citations, and the two-read reconciliation are in the companion audit.

## 2. openWEPP already answers the unsoundness (architecturally)

This is the load-bearing point for sequencing: **the correct fix is architectural, and it is already partly in production.** openWEPP does not port the broken abstraction — it replaces it.

| Legacy failure mode | openWEPP's answer | Status |
|---|---|---|
| Equivalent-plane collapse (one plane, `efflen` accumulation, `QOFE` outlet-referenced amplification) | **Genuine per-OFE lanes** with typed `TransferInput`/`TransferOutput` and area-scaled runon; the equivalent-plane collapse is *explicitly forbidden* (`SC-RUNOFFPART-001#INV-RUNOFFPART-029`) | **Production** |
| No relief valve along the cascade (upslope runoff piles on the terminal OFE) | **Downslope runon re-infiltration** (the "filter-strip" process legacy lacks) — DC01, `INV-RUNOFFPART-031` | **Production** (default semantics; single-OFE byte-identical) |
| Steady-state, no genuine hydraulics between OFEs | **OFE-by-OFE TVD-MacCormack kinematic-wave routing + per-OFE Green-Ampt infiltration** — Lane D, `SC-OFEROUTE-001` (rev 6), ADR-0033 | **Opt-in / shadow** (default byte-flat; not the production path) |
| `QOFE × wrong area` publication over-scaling | `QOFE == Q` publication convention with matched-area export (`INV-RUNOFFPART-032`); the WSHED01 `QOFE × hillslope-area` over-scaling bug fixed | **Production** (fixed) |

So the water-magnitude question is no longer "why doesn't openWEPP match legacy" — it is "is the replacement complete, and does it land inside the field envelope." On H2637 post-DC01 it already does: not-contradicted on all four `INV-SUBHYD-033` tiers (annual yield 0.673, ET 863 mm/yr, event ratio 0.46 ascending, threshold shape present).

## 3. Provenance: the revision WEPP declined (operator-relayed)

The OFE-by-OFE routing openWEPP implements in Lane D is not a novel openWEPP invention — it is the MOFE revision the model's own author intended for WEPP, which WEPP never adopted.

> **Operator-relayed (2026-07-04), secondhand and undetailed — context, not authority.** From a conversation Erin Brooks and Anurag Srivastava had with Thanos Papanicolaou (~2026-05) and reported to the operator within a couple of days: Papanicolaou described his MOFE revisions, which he gave to Dennis Flanagan around 2018 to incorporate into WEPP; they were never incorporated. Brooks characterized the revisions as having *"fixed the MOFE issue"* — no further detail was relayed.

This is consistent with the audit's findings and sharpens the program framing:

- It confirms the root cause at the **institutional** level: a corrected model existed and was available to WEPP for ~8 years, yet legacy stayed on the unsound equivalent-plane construction and patched its symptoms instead (audit Finding 4). A concrete instance of the ADR-0017 posture — legacy is a flag, not authority.
- **openWEPP's Lane D is built on Papanicolaou et al. 2018** (frozen-library `R-63`) — i.e. it is that intended-but-never-incorporated revision. This strengthens the case for prioritizing Lane D's production activation over the WB16-estimator default (§5).
- Notable: the baseline `efflen/totlen` `peakro` band-aid (`irs.for:745`, *"A. Srivastava 4/17/2026"*, audit §3.2) is authored by the **same Anurag Srivastava** — the symptom-patch and the Papanicolaou conversation are the same person and time window.

**Open opportunity (not yet acted on):** the revisions Papanicolaou gave Flanagan may be more or newer than the 2018 paper. If the actual artifact (code / notes) can be obtained via Brooks or Srivastava, it would be a stronger external authority for `SC-OFEROUTE-001` than the published paper alone — and could inform the open shock-timing numerics gap (`GAP-OFEROUTE-005`).

## 4. The reframe

1. **Legacy is a flag, not a target** (ADR-0017). Legacy's own MOFE paths are non-conserving (`with_ui` = 127.7% of precip = the WB-05A `q-cap` blow-up) and carry undocumented closure debt (CONFLICT-008). Chasing parity to a known-unsound model is a category error.
2. **The magnitude bar is the field-observed envelope** `SC-SUBHYD-001#INV-SUBHYD-033`, judged on quickflow-separated components, only after conservation/routing/export closure hold. Not legacy 55.5%.
3. **"Complete the replacement," not "patch the symptom."** Every legacy band-aid in the audit (the `efflen/totlen` `peakro` rescale, the withdrawn U6C water cap, the `q-cap` clamp) is a consumer-side intervention on bad state produced by the equivalent-plane collapse. openWEPP's per-OFE lanes + relief valve remove the *producer* of the bad state; do not re-introduce the band-aids to mimic legacy numbers.

## 5. Sequencing implications

The existing MOFE program (roadmap §E for erosion; the MOFEFID campaign for water; Lane D for hydraulic routing) already carries most of this. Two things this note pins:

- **The multi-OFE erosion discharge hand-off is an open design item, and it is where the legacy `efflen/totlen` artifact must NOT be blindly inherited** (audit cut-point 1). openWEPP's Wave-1 erosion is single-OFE-only today, where the rescale is identity. When roadmap §E.3 wires multi-OFE Wave-1 chaining, the cross-OFE discharge basis (`qin/qout`, `efflen` accumulation, the `peakro` definition) must be defined on openWEPP's per-OFE-lane terms — likely superseding the equivalent-plane "map-back" entirely, consistent with `INV-RUNOFFPART-029`. **This belongs in the Increment-2 entry gate** ([`docs/work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/increment-2-entry-gate.md`](../work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/increment-2-entry-gate.md)); this note is the cross-reference, not the design.
- **The water-magnitude judgment precedes the erosion-magnitude judgment** (roadmap §E.5: rill detachment ∝ discharge, so multi-OFE runoff magnitude must be trustworthy before erosion magnitude is). The relief-valve architecture (DC01 + Lane D) is what makes the water magnitude trustworthy without the equivalent-plane amplification; erosion magnitude rides on top of it.
- **When Papanicolaou routing is active, the resolved kinematic peak must become the published peak runoff (activation requirement).** Verified 2026-07-04: the published `peakro` (pass parquet + HBP EVENT peak-discharge field) currently reads the WB16 `vave·qpstar` estimate (`row.runoff.peak_runoff_m3_s` / `row.erosion.peak_runoff_m3_s` — `05_runner_execution_and_outputs.rs:502,644-707`, `02_output_and_climate_helpers.rs:747-751`), while the Lane D `CascadeResult.peak_total_discharge_m3_s` reaches **no** output surface (shadow). Activating Papanicolaou in production **must** make the resolved kinematic peak supersede the WB16 estimate at that publication seam — units already match (both m³/s), so it is a clean supersede, not a conversion. This is not cosmetic: the HBP EVENT peak drives **watershed channel routing** (`SC-ROUTE-001` triangular-hydrograph reconstruction, `PEAKIN`) and `peakro` drives **erosion detachment** (`qout = peakro·efflen`). Publishing a WB16 peak while routing with Papanicolaou would hand the entire downstream stack a peak the routing never produced. The same obligation extends to the hydrograph's **time-to-peak and duration** (which feed the triangular reconstruction) — consistent with the hydrograph-resolved-routing direction (`docs/backlog/20260704-hydrograph-resolved-sediment-and-routing.md`; ADR-0036). Contract home: the `SC-OFEROUTE-001` production-activation gate.
- **Papanicolaou activation is BLOCKED until the router is coupled to the subsurface — baseflow + subsurface-excess-to-runoff (activation gate).** Operator-directed 2026-07-04, now `SC-OFEROUTE-001#INV-OFEROUTE-012` / `GAP-OFEROUTE-006`. The Lane D solver forcing is rainfall-excess + the upstream *surface* hydrograph only — no subsurface exfiltration source term, no baseflow export (the contract had scoped subsurface entirely out). On the subsurface-dominated steep-wet-forest hillslopes openWEPP targets (H2637 routes ~99% as lateral flow), a surface-only active router would drop the dominant pathway. Before activation: subsurface excess (return flow / saturation excess) must enter the routing as a source term, baseflow must be conserved/exported, and the full hillslope water balance must close — with the **subsurface-excess-to-runoff carrying a dedicated test fixture** plus an H2637-class closure vector. The coupling-seam design is open (which of `SC-OFEROUTE-001` / `SC-SUBHYD-001` / `SC-RUNOFFPART-001` owns exfiltration-into-surface and baseflow-export).

Suggested order (all already on the roadmap; this note only asserts the *framing* that ties them together):
1. Water side — confirm the per-OFE-lane + DC01 relief-valve architecture is the settled production path and that the equivalent-plane amplification cannot re-enter (it is forbidden by `INV-RUNOFFPART-029`, but the publication/erosion consumers must be audited for implicit `efflen/slplen` reads).
2. Erosion side — at §E.3, define the multi-OFE discharge hand-off natively (cut-point 1); retire the legacy EROD14/Wave-2 arm rather than porting its normalization.
3. Magnitude — judge against `INV-SUBHYD-033` (water) then the erosion envelope (§E.5), never legacy parity.

## 6. What NOT to do

- **Do not** port the legacy `efflen/totlen` `peakro` rescale into the multi-OFE erosion path to "match legacy" — it is a band-aid for an abstraction openWEPP rejected.
- **Do not** re-derive the blowup — it is attributed (audit Findings 1–4). Time spent re-confirming the >1000 mm cohort is spent on a known model-class limitation.
- **Do not** treat legacy 55.5% / 127.7% as a bar. The bar is `INV-SUBHYD-033`.
- **Do not** collapse per-OFE lanes back to an aggregate `Q` for any consumer (publication, erosion, watershed hand-off) — that silently re-introduces the equivalent-plane amplification (`INV-RUNOFFPART-029`).
- **Do not** publish the WB16 `vave·qpstar` peak while Papanicolaou routing is active — the published peak (pass parquet, HBP EVENT) must be the resolved kinematic peak, or the watershed router and erosion detachment consume a peak the routing never produced (see §5).
- **Do not** activate Papanicolaou routing on a hillslope with material subsurface flow until `INV-OFEROUTE-012` is met — a surface-only active router silently omits baseflow and subsurface-excess-to-runoff (return flow), which is the majority of the water on openWEPP's target forest hillslopes (see §5).

## 7. Cross-references

- Diagnosis: [`docs/audits/20260704_mofe_effective_length_transport_capacity_audit.md`](../audits/20260704_mofe_effective_length_transport_capacity_audit.md)
- Water campaign: [`docs/planning/mofe-fidelity-campaign-strategy.md`](mofe-fidelity-campaign-strategy.md)
- Erosion program: `docs/ROADMAP.md` §E; [`increment-2-entry-gate.md`](../work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/increment-2-entry-gate.md)
- Contracts: `SC-RUNOFFPART-001` (INV-028/029/031/032), `SC-SUBHYD-001` (INV-033), `SC-OFEROUTE-001`; ADR-0017, ADR-0033, ADR-0011.
