# Modeled Hourly Flow Through the Erosion → HBP → Routing Stack (Hydrograph-Resolved Sediment)

## Status
- `state`: backlog → **ADR authored**:
  [ADR-0036](../decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md)
  (Proposed, 2026-07-04) resolves the open decisions below (per-hour
  quasi-steady form; additive HBP EVENT extension designed once with the
  per-class + peak-units items; conditional `INV-ROUTE-005`; Σhourly=volume
  only; Investigation-tier comparator). Pending ratification + Codex design
  review; contract amendments before code.
- `maturity`: concept / architecture direction (operator-directed 2026-07-04)
- `default_path`: not eligible (needs ADR ratification + contract sequencing before code)
- `evidence_mode`: Static (read SC-SED-001, SC-ROUTE-001, the HBP format, and the
  direct-runtime hourly surfaces; no execution)

## The core awkwardness — stated structurally

WEPP-lineage hillslope erosion is solved with a **single representative steady
discharge** (the event peak runoff `peakro`) over a normalized profile `x∈[0,1]`.
That collapse is the root of the "decreasing-flow" awkwardness: on a **falling
limb**, or where a downslope OFE **reinfiltrates runon** (`qout < qin`), the flow —
and therefore transport capacity — drops, and the correct response is **deposition**
(`load > Tc`). A single peak discharge cannot represent that recession: it has no
falling limb, so the recession deposition is not captured, and the multi-OFE
reinfiltration case is currently handled by the interim **INV-030 `qin`-clamp** (the
clamp discards the flow decrease rather than depositing the sediment the lost flow was
carrying).

Crucially this is **not** a deep model-soundness hole — the deposition physics is
correct and WEPP already deposits on capacity drop. It is a **discretization /
information-loss** problem: the hillslope *models* the hourly flow but *throws it
away* at two boundaries.

## Where the information is lost (three subsystems, one loss)

1. **Hillslope erosion (SC-SED-001).** The Wave-1 continuity solve consumes
   `peakro` (a scalar peak) + `effint`/`effdrr` (rainfall-intensity surfaces). The
   modeled **hourly** infiltration-excess profile already exists on the day frame as
   `wb14_hourly_excess_m[24]` (DC01 / INV-RUNOFFPART-031), and the 1b-C flip added the
   parallel `wb14_hourly_rainfall_m[24]`. The erosion solve does **not** consume them —
   it collapses to the WB16 peak.

2. **The HBP interchange (`SC-INFILE-HBP-001` / hbp-file spec).** The EVENT payload
   serializes **peak discharge + runoff volume + duration + sediment** — a scalar
   hydrograph summary. It does **not** serialize the modeled hourly flow. So the
   hillslope→watershed boundary is lossy by construction.

3. **Watershed channel routing (SC-ROUTE-001).** Because the HBP carries only
   peak+duration, the channel routing **reconstructs a triangular synthetic
   hydrograph** (`REF-ROUTE-CH13-PEAKIN`, Ch.13 §13.4.1) and superposes those triangles
   at inlets, then applies the Modified Rational outlet-peak method. The watershed
   never sees the real modeled shape — it rebuilds a lossy triangle from the two
   surviving scalars.

The same modeled hourly flow is discarded at (1)→solve, serialized-away at (2)→HBP,
and re-synthesized as a triangle at (3)→routing. openWEPP has *more* information than
legacy ever did (the hourly excess profile) and currently spends it nowhere.

## The direction — carry the modeled hourly flow through the stack

Replace the peak collapse with the modeled hourly flow as a first-class surface that
flows: **hillslope hydrograph → erosion solve → HBP serialization → channel routing.**
Concretely:

- **Erosion (SC-SED-001):** solve the sediment continuity against the **hourly-resolved
  discharge** rather than a single peak, so the falling limb deposits naturally and the
  multi-OFE `qin/qout` accumulation is an hour-by-hour balance (the reinfiltration case
  becomes ordinary hour-resolved deposition, retiring the INV-030 clamp as a real fix,
  not a bound). The *design* of how the continuity consumes the hydrograph (per-hour
  solve, time-integrated transport, or a reduced hydrograph parameterization) is the
  Increment-2 entry-gate's job — see
  [`increment-2-entry-gate.md`](../work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/increment-2-entry-gate.md).
- **HBP:** add a versioned **hourly-flow surface** to the EVENT payload (a bounded
  `[f64; 24]`-class channel, same basis as `wb14_hourly_excess_m`), so the interchange
  carries the modeled shape, not just peak+volume+duration. Schema **extension**, not a
  break — migrated per the HBP contract's versioning.
- **Routing (SC-ROUTE-001):** route the **serialized modeled hydrograph** instead of a
  triangular reconstruction, superposing real per-hour inflows at channel inlets. The
  triangular method remains the fallback for legacy shards that lack the hourly surface.

## Why it is worth doing (and why now)

- **It is the *structural* fix for the decreasing-flow deposition**, which is otherwise
  a standing interim clamp (INV-030). Structure-before-magnitude (ADR-0011 ordering):
  this closes a structural gap, independent of magnitude.
- **It is a genuine improvement over legacy**, not a port — legacy never had the hourly
  profile; openWEPP does, and discards it. Triangular hydrograph superposition is a
  known lossy step; routing the modeled shape is strictly more faithful.
- **It is the natural substrate for multi-OFE erosion (Increment 2)** — the `qin/qout`
  handoff and the reinfiltration/deposition case are inherently per-hour, so the
  multi-OFE architecture and the hourly-flow resolution are coupled and should be
  designed together.
- **It is the natural substrate for Hairsine-Rose** later — HR's deposition
  `d_i = v_s,i · c_i` is a continuous, hour-resolvable settling term; a hydrograph-
  resolved transport surface is exactly what HR wants (see
  [`20260526-hairsine-rose-multiclass-sediment-model.md`](20260526-hairsine-rose-multiclass-sediment-model.md)).

## Contract-first sequencing (non-negotiable per ADR-0011)

1. **ADR** on hydrograph-resolved sediment transport + routing: whether the hourly-flow
   surface supersedes the peak collapse or coexists (peak retained as a diagnostic/
   fallback), and the comparator-tier posture (routing the modeled hydrograph will not
   match the legacy triangular result even when both are correct — Investigation-tier,
   ADR-0017).
2. **Contract amendments:** SC-SED-001 (erosion consumes hourly discharge), the HBP
   format + SC-INFILE-HBP-001 (versioned hourly-flow EVENT surface), SC-ROUTE-001
   (route serialized hydrograph, triangular fallback). Author before code.
3. **Conservation gate:** the hourly surface must close to the existing runoff **volume**
   (`Σ hourly = volume`), so the extension is a refinement of the closed water balance,
   not a new degree of freedom. Note the WB16 `peakro` is a **separate** analytical peak
   estimator (`vave·qpstar` via the `tstar`/`vstar` branch), **not** the max of the
   hourly profile — the two need not be equal, and forcing `max(hourly) = peakro` would
   reject or distort a valid modeled hydrograph. WB16 `peakro` stays a **diagnostic /
   fallback** surface; whether the hourly profile is rescaled to reconcile with it is an
   explicit **ADR policy** choice, not a conservation requirement.
4. **HBP schema migration:** the hourly-flow surface is designed **once** and versioned;
   a premature commit before the per-OFE / per-event serialization semantics are settled
   risks a costly migration (same discipline the HR backlog flags for its deposited-layer
   state).

## Non-goals

- Not a dynamic-wave or 2D finite-element hydraulics rewrite. openWEPP's
  OFE/segment/channel topology and the daily/event tick are preserved; this refines the
  *within-event discharge representation* on that topology.
- Not a removal of the peak/volume summary surfaces — they remain (diagnostics, legacy
  fallback, and the conservation tie).
- Not coupled to Hairsine-Rose adoption — HR benefits from it but does not gate it.

## Open questions

1. Does the erosion continuity solve per-hour (24 solves/day) or consume a
   time-integrated transport derived from the hydrograph? (Cost vs. fidelity; the
   Increment-2 entry gate decides, with the recession-deposition case as the acceptance
   driver.)
2. HBP hourly-flow surface: per-OFE-exit, hillslope-exit only, or both? Routing needs
   the hillslope-exit hydrograph; multi-OFE erosion needs per-OFE. Design the payload
   for the routing consumer first, per-OFE as an internal (non-serialized) surface if
   that suffices.
3. Interaction with the water-magnitude contract-gap (forest lateral absolute
   magnitude): hydrograph *shape* fidelity is independent of the *magnitude* gap, but a
   magnitude-inflated hydrograph routed faithfully is still magnitude-inflated — keep the
   two attributions separate (shape = this item, magnitude = the deferred water item).

## Related work

- [`increment-2-entry-gate.md`](../work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/increment-2-entry-gate.md)
  — the multi-OFE erosion design that this substrate feeds.
- [`20260627-stream-water-temperature-surface-energy-balance.md`](20260627-stream-water-temperature-surface-energy-balance.md)
  — its open pickup question is *exactly* this: "can hourly water+temperature
  serialize across HBP and be consumed by `openwepp-cli-watershed`." The hourly-flow
  EVENT surface designed here is the shared vehicle — design it once so both the
  sediment hydrograph and an optional hourly water-temperature channel ride the same
  versioned schema extension rather than two separate migrations.
- [`20260526-hairsine-rose-multiclass-sediment-model.md`](20260526-hairsine-rose-multiclass-sediment-model.md)
  — the deposition model that most benefits from a hydrograph-resolved transport surface.
- `SC-SED-001`, `SC-ROUTE-001`, `SC-INFILE-HBP-001`, `docs/specifications/wepp-input-files/specs/hbp-file.spec.md`.
- [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md) (contract-first),
  [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) (comparator-as-flag).
