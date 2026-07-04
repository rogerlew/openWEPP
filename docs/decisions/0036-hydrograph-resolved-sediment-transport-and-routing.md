# ADR-0036: Hydrograph-Resolved Sediment Transport and Channel Routing

Status: **Proposed** (authored 2026-07-04 by Claude Code on operator direction
"scaffold the E.2 ADR"; pending operator ratification + Codex design review).
Contract-first sequencing binds: the SC-SED-001 / SC-INFILE-HBP-001 /
SC-ROUTE-001 amendments this ADR mandates are authored **before** any code.

Deciders: Roger Lew, Claude Code

Provenance:
[`docs/backlog/20260704-hydrograph-resolved-sediment-and-routing.md`](../backlog/20260704-hydrograph-resolved-sediment-and-routing.md)
(operator-directed concept, 2026-07-04);
[`increment-2-entry-gate.md`](../work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/increment-2-entry-gate.md)
§5 (the coupled multi-OFE design this substrate feeds);
[`docs/ROADMAP.md`](../ROADMAP.md) §E.2.

Relates: `SC-SED-001` (erosion consumes the discharge surface), `SC-ROUTE-001`
(`REF-ROUTE-CH13-PEAKIN` / `INV-ROUTE-005`, the triangular reconstruction being
demoted), `SC-INFILE-HBP-001` + `hbp-file.spec.md` (the interchange being
extended), `SC-RUNOFFPART-001` (`wb14_hourly_excess_m`, `INV-RUNOFFPART-030`
clamp / `INV-RUNOFFPART-031` hourly basis), ADR-0011 (contract-first), ADR-0017
(comparator-as-flag), ADR-0033 (OFE-by-OFE overland routing — the per-OFE lane
this rides), ADR-0035 (the erosion port this continues).

## Context

WEPP-lineage erosion solves sediment continuity at a **single representative
steady discharge** (the event peak) over the normalized profile. openWEPP
inherited that collapse through the 1b-C flip even though the direct runtime
*already models* the hourly infiltration-excess profile
(`wb14_hourly_excess_m[24]`, plus the 1b-C `wb14_hourly_rainfall_m[24]`) — the
information is discarded three times:

1. **Erosion (SC-SED-001):** the Wave-1 solve consumes WB16 `peakro` +
   duration; the hourly profile is not an operand. A single peak has no falling
   limb, so recession deposition cannot be represented, and the multi-OFE
   reinfiltration case (`qout < qin`) is held by the interim
   `INV-RUNOFFPART-030` clamp — which discards the flow decrease instead of
   depositing the sediment the lost flow was carrying.
2. **HBP interchange:** the EVENT payload serializes peak + volume + duration —
   a lossy scalar summary of a modeled shape.
3. **Channel routing (SC-ROUTE-001):** having only the scalars, routing
   *re-synthesizes* a triangular hydrograph (`REF-ROUTE-CH13-PEAKIN`,
   `INV-ROUTE-005` currently **mandates** it hard-fail) and superposes the
   triangles at inlets.

This is a discretization / information-loss problem, not a model-soundness
hole: the deposition physics is correct and already deposits on capacity drop
(the Wave-1 solver carries the full decreasing-flow machinery — the
`xinflo` negative-`qostar` branches, `depc`/`depend`/`depos`). openWEPP has
strictly more information than legacy ever did and currently spends it nowhere.

Two E.1-recorded intake items also land at this boundary, because the HBP
EVENT schema is **designed once**:

- the npart-resolved per-class sediment surfaces (`sedcon`/`frcflw`), deferred
  from E.1 (the current HBP EVENT is the single-class schema-1 payload);
- the `peak_runoff_m3_s` unit question (the WB16-fed frame surface behaves as a
  depth-rate m/s despite the suffix; legacy writes `peakro·harea`, a true
  m³/s).

## Decision

Carry the modeled hourly flow through the stack as a first-class, versioned
surface: **hillslope hydrograph → erosion solve → HBP EVENT serialization →
channel routing.** Five sub-decisions:

### D1 — The erosion solve form: per-hour quasi-steady Wave-1

The Wave-1 continuity solve runs **per excess hour** (≤ 24 quasi-steady solves
per OFE-day, only on hours with excess), consuming that hour's mean discharge
from the hourly basis; daily totals are the hour sums
(`tdet/tdep/export = Σ_h`). This — not a hydrograph-integrated transport — is
the production form, for single-OFE and (in E.3) multi-OFE alike.

Rationale:
- A time-integrated form **cannot represent a falling limb by construction** —
  it is the peak collapse re-parameterized. The acceptance driver (recession /
  reinfiltration deposition without a clamp) rules it out.
- Hour resolution turns the reinfiltration case into the solver's **existing**
  decreasing-flow case: an hour where a downslope OFE absorbs runon
  (`qout < qin`) is an ordinary negative-`qostar` solve that deposits, retiring
  the `INV-RUNOFFPART-030` clamp as a *fix*, not a bound.
- Cost is bounded and small: excess hours are sparse (storm days, a few hours),
  the solve is a 101-point march over ~5 particle-blind coefficients, and the
  per-OFE-day physics budget is sub-µs-class in the array-native runtime.
- One solve form everywhere avoids the two-physics inconsistency the
  Increment-2 entry gate names as a standing risk (§3/§8).

Quasi-steady per hour is a **deliberate refinement beyond legacy source-intent**
(legacy solves once at the peak). SC-SED-001 already labels the steady-state
transposition as an explicit Chapter-11 simplification (`INV-SED-011`); the
amendment narrows that label to the hourly quantum. If per-hour proves
untenable at gate time (cost or discretization noise), the recorded fallback is
a **step-hydrograph reduction** (a small set of quasi-steady levels preserving
volume and the falling limb) — never a return to the single peak.

The daily peak-based solve is retired as a physics arm but **retained behind a
comparator flag** for one transition window (cross-check, then delete),
mirroring the EROD14 retirement pattern in the Increment-2 entry gate §3.

### D2 — HBP: one additive, versioned EVENT extension

The HBP EVENT payload gains a versioned **hourly-discharge surface at the
hillslope exit** (a bounded 24-slot channel on the same basis as
`wb14_hourly_excess_m`), added **together with** the other EVENT-schema items
so the interchange migrates once:

- the npart-resolved `sedcon`/`frcflw` per-class surfaces (E.1 deferral);
- the peak-discharge unit clarification (serialize a true m³/s or an
  explicitly-named depth-rate — resolved in the SC-INFILE-HBP-001 amendment,
  with the frame-side misnomer renamed at the same boundary);
- a generic hourly-surface container shape, so the stream-temperature backlog
  (`20260627`) rides the same extension rather than a second migration.

The extension is **additive** under the HBP spec's versioning (consumers
feature-detect; schema major/minor mechanics are the SC-INFILE-HBP-001
amendment's to fix). Per-OFE hydrographs stay **internal** (frame surfaces, not
serialized): the routing consumer needs the hillslope-exit shape; E.3's per-OFE
chaining consumes the frame directly. If E.3 falsifies that, extending the
payload is a second additive minor — not a redesign.

### D3 — Routing: route the serialized modeled hydrograph

`openwepp-cli-watershed` routes the **serialized hourly hydrograph** when the
shard carries it — superposing real per-hour inflows at channel inlets — and
falls back to the triangular reconstruction only for shards lacking the
surface. `INV-ROUTE-005` is amended from "the triangular procedure must be
used" (hard-fail) to a **conditional**: modeled-hydrograph superposition when
the surface is present, triangular as the explicit legacy-shard fallback. The
peak/volume summaries remain serialized (diagnostics + fallback inputs).

### D4 — Conservation policy: volume ties, peak does not

The **only** hard gate tying the hourly surface to the closed water balance is
`Σ hourly = runoff volume` (the extension refines a closed balance; it adds no
degree of freedom). WB16 `peakro` is a **separate analytical estimator**
(`vave·qpstar`), not the max of the hourly profile: `max(hourly) ≠ peakro` is
**not** an error, and the hourly profile is **not rescaled** to reconcile with
it — a rescale would fabricate flow to match a diagnostic. `peakro` remains a
diagnostic / fallback surface with its own WB16 lineage.

### D5 — Comparator posture: Investigation tier

Routing the modeled shape will diverge from the legacy triangular result **even
when both are correct**. All legacy comparisons on hydrograph-shape-affected
surfaces (outlet peaks, event sediment, channel deposition) are
**Investigation-tier flags** (ADR-0017), never acceptance gates. The retained
peak-based arm (D1) is the transition cross-check inside openWEPP itself.

## Consequences

- **Structural closure:** falling-limb and reinfiltration deposition become
  ordinary hour-resolved solves; the `INV-RUNOFFPART-030` clamp is retired as a
  fix. This is the substrate E.3 (multi-OFE chaining) and, later, Hairsine-Rose
  (`d_i = v_s·c`, an hour-resolvable settling term) build on.
- **Contract work precedes code:** amendments to SC-SED-001 (hourly-discharge
  operand + the narrowed steady-state label + per-hour mass-closure
  invariants), SC-INFILE-HBP-001 + `hbp-file.spec.md` (the additive EVENT
  extension, per-class surfaces, peak units), and SC-ROUTE-001 (conditional
  `INV-ROUTE-005`, hydrograph intake). Then the staged 2b execution per the
  Increment-2 entry gate §7 (shadow-first, conservation-hard-stop, non-hourly
  consumers byte-stable via the fallback until the flip).
- **Single-OFE production numbers change** when the hourly form becomes the
  production solve (hourly mean rates ≠ the peak): p61 / DFF-WS3 magnitudes
  shift. Acceptance is carried by the conservation gates, the per-hour closure,
  the directional burn law, and the E.1 reconstruction identity
  (`tdet = Σ sedcon × runvol` on zero-deposition days holds for daily
  aggregates independent of the solve quantum) — not by magnitude match to the
  peak-form numbers (E.5 posture unchanged; hydrograph *shape* fidelity stays
  attributionally separate from the water-*magnitude* gap).
- **HBP consumers migrate once:** watershed intake, `totalwatsed3`, and the
  replay/comparator tier read one extended EVENT schema; legacy shards stay
  readable (fallback path).
- **Risk — schema regret:** the EVENT extension is the one hard-to-reverse
  commitment (it crosses the binary boundary). Mitigation: the amendment lands
  with the per-class + peak-units + generic-hourly-container items resolved
  together, and 2b's gate includes a serialization round-trip on real shards
  before any consumer flips.

## Alternatives considered

1. **Hydrograph-integrated transport (single solve, integrated operands).**
   Rejected: cannot produce falling-limb deposition by construction — it is
   the status-quo collapse with extra steps. (Kept only as the shape of the
   *step-hydrograph* fallback if per-hour proves untenable, which preserves
   the limb.)
2. **Keep the peak solve + the INV-030 clamp.** Rejected: leaves a standing
   structural gap the roadmap ordering (structure before magnitude) exists to
   close; the clamp discards mass-carrying flow information.
3. **Sub-hourly / dynamic-wave hydraulics.** Rejected as a non-goal: the
   OFE/segment/channel topology and the daily/event tick are preserved; this
   ADR refines the within-event discharge representation only.
4. **Rescale the hourly profile so `max(hourly) = peakro`.** Rejected: peakro
   is an independent analytical estimator; forcing agreement fabricates flow
   and would distort a valid modeled hydrograph (D4).
5. **Serialize per-OFE hydrographs now.** Deferred: the serialized consumer
   (routing) needs the hillslope exit only; per-OFE stays on the frame. An
   additive minor extension remains open if E.3 falsifies this.
6. **Bundle Hairsine-Rose adoption.** Rejected: HR benefits from the substrate
   but does not gate it (backlog `20260526` stands alone, concept-stage).

## Execution note

Executor assignment for the 2b work-package follows the ADR-0035 pattern
(Claude Code authored the E-sequence increments to date under the
operator-authorized exception, with full AGENTS gates + Codex review) unless
the operator redirects at kickoff. This ADR itself makes no code change;
production outputs are unchanged until the contract amendments land and the 2b
gates pass.
