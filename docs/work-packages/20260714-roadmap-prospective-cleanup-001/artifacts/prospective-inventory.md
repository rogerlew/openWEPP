# Prospective Inventory

Status: `complete`

Static: This artifact classifies the pre-rewrite `docs/ROADMAP.md` at 1,124
lines / 125,845 bytes. Classification was completed before the roadmap body was
replaced.

## Classification Rules

- Retain an unfinished item only when the roadmap can name its state, owner or
  owning queue, and advancement trigger/dependency.
- Retain standing policy only when it changes how future work is selected or
  closed.
- Route completed or superseded execution to the work-package catalog and
  canonical decisions/contracts; do not summarize it again.
- Route unprioritized concepts to `docs/backlog/TRACKER.md`; a backlog entry is
  not an execution commitment.
- Preserve the scientific-assurance sequence and the minimum closure contract
  needed to scaffold `ASSURE-02`, because its replacement architecture has not
  yet been authored elsewhere.

## Section Inventory

| Pre-rewrite section | Classification | Terminal treatment |
| --- | --- | --- |
| Header and `How to read this queue` | Mixed standing policy and completed R4-R6/direct-runtime history | Retain a short operating contract; remove the program history. |
| Scientific Assurance Queue | Mixed: `ASSURE-01` history; `ASSURE-02` through `ASSURE-08` prospective; v2 requirements are next-package constraints | Remove `ASSURE-01`; retain a compact v2 direction/closure contract and rows `ASSURE-02` through `ASSURE-08`. |
| Watershed Runtime Performance Queue | Historical except recurring `CQR-NIGHTLY` | Remove the completed WSHED/Lane D/CQR campaign ledger; retain CQR as standing maintenance. |
| Generic `Queue` and current direct-runtime note | Historical | Remove all rows and narrative; route history to work packages, ADRs, and contracts. |
| Winter-column snow/frost sequence and fidelity adjudication | Historical program with backlog-only residual concepts | Remove terminal sequence; route unprioritized residuals to the backlog tracker. Snow/frost remains a future assurance flagship under `ASSURE-06`. |
| Per-OFE runoff magnitude and indexed runtime performance | Historical | Remove. Any new performance or fidelity work requires backlog promotion or a new authorized package. |
| Stage-2 physics magnitude | Backlog-only concepts, not current execution | Remove from roadmap and route to the backlog tracker. |
| Hillslope erosion sediment sequence | Historical program with backlog-only extensions | Remove terminal sequence; route optional extensions to the backlog tracker. |
| Keeping this current and authority pointers | Standing policy and routing | Retain in shorter form with canonical history/authority/backlog pointers. |

## Item Inventory

### Retained prospective execution sequence

| Item | Pre-rewrite state | Terminal classification |
| --- | --- | --- |
| `ASSURE-02` | next, documentation-only | Retain as the sole immediate program item and no-code gate. |
| `ASSURE-03` | queued behind `ASSURE-02` | Retain. |
| `ASSURE-04` | queued behind `ASSURE-02` and `ASSURE-03` | Retain. |
| `ASSURE-05` | queued behind `ASSURE-04` | Retain. |
| `ASSURE-06` | queued behind successful `ASSURE-05` | Retain; snow/frost is the flagship, not the tooling pilot. |
| `ASSURE-07` | queued after `ASSURE-05` | Retain as incremental portfolio work that need not block `ASSURE-06`. |
| `ASSURE-08` | deferred, mandatory pre-beta | Retain with its beta-release trigger and current handoff boundary. |
| `CQR-NIGHTLY` | queued for operator request | Retain as recurring maintenance, not as the immediate program priority. |
| `CANOPY-PHENOLOGY` | promoted/staged, no active package | Retain as queued plant/snow-frost work with an operator scheduling trigger, current-foundation reconciliation, and contract-first leaf-off/leaf-on sequence. |

### Removed historical table items

Static: The following rows are completed, terminally dispositioned,
superseded, or stale pointers into completed successors. Their packages and
execution records remain available from `docs/work-packages/README.md`.

- `ASSURE-01`
- `WSHED-W7`, `WSHED-W7DC01`, `WSHED-W7R`
- `M-T2A`, `M-T2P`, `M-T2Q`, `M-T2S`, `M-T2R`, `M-T2B`, `M-T2`
- `WSHED-W8`, `WSHED-W9`, `WSHED-W10`
- `CQR-FOLLOWUP-20260711`, `CQR-PREINT-20260711`
- `M`, `M-T1`, `M-T3`, `WSHED-W11`, `WSHED-W11A`, `WSHED-W11B`
- generic queue rows 1 through 6 and program rows `P`, `W`, and `K`

The pre-rewrite `M` row used the word `active`, but its stated next item was
`WSHED-W11`; W11A/W11B completed that chain. The `M-T1` hold concerned an
unratified optional approximation after the authorized local numerics had
landed; it is not an active package and its remaining concept is routed through
the backlog rather than represented as an unfinished implementation.

### Backlog-routed concepts

These ideas remain discoverable but are not promoted execution commitments:

| Concept | Durable route |
| --- | --- |
| `snowd.for` code/documentation magnitude review | `docs/backlog/20260605-snow-code-deferred-science-review.md` |
| Snow-free wet-heat / `Qwet` candidate | `docs/backlog/20260612-frost-heave-frozen-fringe-impedance-formulation.md` |
| Irrigation management-gated activation | `docs/backlog/20260617-irrigation-management-gated-activation.md` |
| Lane D optional numerics/performance tiers | `docs/backlog/20260706-laned-router-numerics-performance-tiers.md` |
| Cropland/tillage erosion activation | `docs/backlog/20260705-cropland-tillage-erosion-enable.md` |
| Per-class-hourly sediment interchange | `SC-SED-001#GAP-SED-008` plus the reconciled staged record at `docs/backlog/20260704-hydrograph-resolved-sediment-and-routing.md`; promote only for a named sub-daily class-composition consumer. |
| Hairsine-Rose alternative sediment model | `docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md` |

The forest lateral-flow magnitude note is not retained as deferred work:
`docs/backlog/TRACKER.md` marks it complete after promotion to
`SC-SUBHYD-001#INV-SUBHYD-033`. The old roadmap wording was stale.

The WB16 trace-event difference is also not retained as an open roadmap item.
`SC-SED-001#GAP-SED-009` records it as a bounded water-side Investigation flag
in a closed erosion gap. Channel-hourly routing is terminal W11 history; only
the separate `GAP-SED-008` per-class-hourly interchange remains an unpromoted
concept.

## Assurance Requirements Preserved In Compact Form

The rewrite must preserve these future constraints without carrying the full
design specification:

- scientific findings and a conventional manuscript lead the public product;
- public report, technical supplement, machine bundle, model-science
  authority, and application decision remain distinct;
- `ASSURE-02` is documentation-only and produces the architecture, report
  standard, source/build/lifecycle contract, evidence-led pilot selection,
  nonpublic real-evidence manuscript prototype, and v1 migration/decomposition;
- the first tooling pilot is bounded and non-snow, with linear groundwater
  reservoir recurrence only a preferred candidate pending inventory;
- no draft/candidate enters public `usersum`;
- claims are specific, quantitative results are mechanically traceable,
  verification is materiality-scoped, negative evidence remains visible, and
  tooling never adjudicates science;
- agent assistance is content-identified, retained, reviewed, and subordinate;
- accessibility, independent scientific review, mechanical publication review,
  and current-release transfer remain explicit; and
- snow/frost follows a successful pilot as the flagship synthesis, while
  downstream vendoring remains mandatory immediately before the WEPPcloud beta
  release and is not authorized now.
