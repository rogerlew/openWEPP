# Modeled Hourly Flow Through the Erosion → HBP → Routing Stack (Hydrograph-Resolved Sediment)

## Status
- `state`: **staged; core path implemented, one additive interchange gap
  remains**. Hillslope hourly erosion (E.2–E.4), paired HBP hourly water and
  total-sediment surfaces, and W11 channel-hourly routing are implemented and
  contract-authorized. Current remaining scope is only
  `SC-SED-001#GAP-SED-008`: a consumer-pulled per-class-hourly interchange
  channel. See *Current remaining scope* below. Originally: backlog →
  **ADR-0036 RATIFIED**:
  [ADR-0036](../decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md)
  (Accepted 2026-07-04 after a two-round Codex design review) resolves the
  open decisions below: per-hour quasi-steady form on hydraulically-active
  hours (`excess_h > 0 ∨ qin_h > 0`); one additive HBP EVENT extension
  carrying PAIRED hourly runoff volume `V_h` (m³) + sediment mass `S_h` (kg)
  — hourly water alone would force an implicit sediment-timing rule in the
  channel — designed once with the per-class + peak-units items; conditional
  `INV-ROUTE-005`; integral closures `Σ V_h = runvol` / `Σ S_h = event mass`;
  Investigation-tier comparator.
- `maturity`: staged residual concept (operator-directed 2026-07-04; reconciled
  to current contracts and W11 completion 2026-07-15)
- `current authority`: `SC-SED-001#INV-SED-013..014` and
  `SC-ROUTE-001#INV-ROUTE-015..020`; `GAP-SED-008` owns the remaining
  per-class-hourly interchange limitation.
- `WB16 posture`: the trace-event `peakro` difference is a bounded water-side
  Investigation flag in closed `GAP-SED-009`; it is not an open erosion defect
  or an unfinished part of this program.

The design narrative below is retained as historical rationale and uses
then-current tense. It is not the current execution plan; the status above and
the final *Current remaining scope* section control backlog promotion.

## The core awkwardness — stated structurally

WEPP-lineage hillslope erosion is solved with a **single representative steady
discharge** (the event peak runoff `peakro`) over a normalized profile `x∈[0,1]`.
That collapse is the root of the "decreasing-flow" awkwardness: on a **falling
limb**, or where a downslope OFE **reinfiltrates runon** (`qout < qin`), the flow —
and therefore transport capacity — drops, and the correct response is **deposition**
(`load > Tc`). A single peak discharge cannot represent that recession: it has no
falling limb, so the recession deposition is not captured, and the multi-OFE
reinfiltration case is currently handled by the bounded interim `qin`-clamp
(`INV-RUNOFFPART-031` compatibility scope under the `INV-RUNOFFPART-030` hold) — (the
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
  becomes ordinary hour-resolved deposition, retiring the bounded interim
  `INV-RUNOFFPART-031` clamp — held under the `INV-RUNOFFPART-030` governance hold — as
  a real fix, not a bound). *Resolved by ADR-0036 D1:* per-hour quasi-steady solves on
  **hydraulically-active hours** (`excess_h > 0 ∨ qin_h > 0`, covering the
  full-reinfiltration `qout = 0 / qin > 0` hour), daily totals = hour sums.
- **HBP:** add the versioned **paired hourly surfaces** to the EVENT payload
  (*ADR-0036 D2*): hour-integrated runoff **volume** `V_h` (m³) + exported sediment
  **mass** `S_h` (kg) on one shared 24-slot time base — hourly water alone would force
  an implicit sediment-timing rule in the channel. Schema **extension**, not a break —
  migrated once per the HBP contract's versioning, bundled with the per-class and
  peak-units items.
- **Routing (SC-ROUTE-001):** route the **serialized modeled hydrograph** instead of a
  triangular reconstruction, superposing real per-hour inflows at channel inlets. The
  triangular method remains the fallback for legacy shards that lack the hourly surface.

## Why it is worth doing (and why now)

- **It is the *structural* fix for the decreasing-flow deposition**, which is otherwise
  a standing interim clamp (`INV-RUNOFFPART-031`). Structure-before-magnitude (ADR-0011 ordering):
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

1. **ADR** on hydrograph-resolved sediment transport + routing: whether the hourly
   surfaces supersede the peak collapse or coexist (peak retained as a diagnostic/
   fallback), and the comparator-tier posture (routing the modeled hydrograph will not
   match the legacy triangular result even when both are correct — Investigation-tier,
   ADR-0017). *Done: ADR-0036 (supersede, peak-arm retained-as-flag then deleted;
   D1–D5).*
2. **Contract amendments:** SC-SED-001 (erosion consumes hourly discharge), the HBP
   format + SC-INFILE-HBP-001 (versioned paired `V_h`/`S_h` EVENT surfaces), SC-ROUTE-001
   (route serialized hydrograph, triangular fallback). Author before code.
3. **Conservation gate:** the hourly surface must close to the existing runoff **volume**
   (`Σ hourly = volume`), so the extension is a refinement of the closed water balance,
   not a new degree of freedom. Note the WB16 `peakro` is a **separate** analytical peak
   estimator (`vave·qpstar` via the `tstar`/`vstar` branch), **not** the max of the
   hourly profile — the two need not be equal, and forcing `max(hourly) = peakro` would
   reject or distort a valid modeled hydrograph. WB16 `peakro` stays a **diagnostic /
   fallback** surface; whether the hourly profile is rescaled to reconcile with it is an
   explicit **ADR policy** choice, not a conservation requirement.
4. **HBP schema migration:** the paired hourly surfaces are designed **once** and versioned;
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
2. HBP hourly surfaces: per-OFE-exit, hillslope-exit only, or both? (*Resolved by
   ADR-0036 D2: hillslope-exit serialized; per-OFE stays frame-internal.*) Routing needs
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
  serialize across HBP and be consumed by `openwepp-cli-watershed`." The hourly
  EVENT surface designed here is the shared vehicle — design it once so both the
  sediment hydrograph and an optional hourly water-temperature channel ride the same
  versioned schema extension rather than two separate migrations.
- [`20260526-hairsine-rose-multiclass-sediment-model.md`](20260526-hairsine-rose-multiclass-sediment-model.md)
  — the deposition model that most benefits from a hydrograph-resolved transport surface.
- `SC-SED-001`, `SC-ROUTE-001`, `SC-INFILE-HBP-001`, `docs/specifications/wepp-input-files/specs/hbp-file.spec.md`.
- [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md) (contract-first),
  [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) (comparator-as-flag).

## Current remaining scope (updated 2026-07-15)

The hydrograph-resolved hillslope, HBP hourly water/total-sediment interchange,
and channel-hourly routing path are implemented. W11A ratified channel-hourly
authority and W11B closed its production consumer and conservation path. They
are execution history, not backlog scope.

Only the E.4 `SC-SED-001#GAP-SED-008` remainder stays open: an additive HBP
surface carrying hour-resolved particle-class composition. The solver already
has per-quantum enriched composition; current interchange serializes hourly
total sediment mass and event-level class mass. A future consumer must not
infer enriched hourly class timing from the event-level blend.

Promotion trigger: a named downstream consumer requires sub-daily particle-
class composition. Promotion is contract-first and must define the additive
interchange, real consumer, per-class/hour closure, versioning, and protected
current total-mass behavior. WB16 peak fidelity is not bundled into that work:
`GAP-SED-009` records the trace-event difference as a bounded, closed
Investigation flag.
