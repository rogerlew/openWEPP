# Backlog Tracker

State of the `docs/backlog/` concept notes. This is the at-a-glance index;
each note holds the authoritative detail. Update the row when a note's state
changes (promote, prune, block). See [README.md](README.md) for the
concept → work-package → contract-authoring promotion path.

**State vocabulary:**
- `concept` — concept note exists; not yet prioritized for a work-package.
- `breadcrumb` — deliberately-thin placeholder to return to; not being planned.
- `staged` — partially addressed; a dimension is implemented/active, remainder concept.
- `blocked` — concept, gated on a named prerequisite.
- `complete` — superseded by completed work; **prune candidate** (move detail to the
  execution log and delete the note).

Last updated: 2026-08-09 (Codex — native-forest authority reframe complete;
implementation successor remains blocked on constitutive authority).

| Item | State | Gate / blocker | Notes |
|---|---|---|---|
| [1.5D top-slope trapezoidal kinematic wave](20260512-1p5d-top-slope-trapezoidal-kinematic-wave.md) | `concept` | not prioritized | Hillslope-geometry kinematic-wave extension. |
| [Residue moisture storage — full state](20260512-residue-moisture-storage-full-state.md) | `concept` | not prioritized | Full residue moisture-storage state (beyond the current residue cover). |
| [Hairsine–Rose multiclass sediment model](20260526-hairsine-rose-multiclass-sediment-model.md) | `concept` | not prioritized | Alternative multiclass erosion model. Benefits from the hydrograph-resolved substrate below (its `d_i=v_s·c` deposition is hour-resolvable). |
| [Hydrograph-resolved sediment + routing (modeled hourly flow through the stack)](20260704-hydrograph-resolved-sediment-and-routing.md) | `staged` | Consumer need for hour-resolved class composition; contract-first additive interchange extension | Hillslope hourly erosion, paired HBP hourly water/total-sediment surfaces, and channel-hourly routing are implemented and contract-authorized. Only `SC-SED-001#GAP-SED-008` remains: a consumer-pulled per-class-hourly interchange channel. The WB16 trace-event difference is a bounded, closed Investigation flag under `GAP-SED-009`, not an open erosion item. |
| [Snow-code deferred science review](20260605-snow-code-deferred-science-review.md) | `staged` | Stage-2 snowd.for review concept | **Substantially addressed** by the snow-fidelity arc (Harder–Pomeroy phase, holding-capacity, density compaction; density is legacy-as-built-equivalent). The `snowd.for` Eq. 3.7.5 code-vs-doc divergence review (Stage 2) remains. |
| [Frost-depth model — heat-flow parity](20260607-frost-depth-model-heat-flow-parity.md) | `complete` | — | **FDHP01 done; frost ratified** (`INV-SNOWFREEZE-047/048/050`, default-activated). Prune candidate — detail is in the `20260608-fdhp01-*` and `20260629-frost-*` work-packages. |
| [Frost-heave / frozen-fringe / impedance (`Qwet`)](20260612-frost-heave-frozen-fringe-impedance-formulation.md) | `concept` | ungated (FDHP01 done); deferred | The deferred wet-heat/`Qwet` candidate for the **2 snow-free frost cells** the frost arc bounded. Legacy `frzftp=0` (no authority); needs external/literature authority. |
| [Irrigation management-gated activation](20260617-irrigation-management-gated-activation.md) | `concept` | deferred (runs only when management declares it) | Out of scope for the perf migration; activate when irrigation is prioritized. |
| [Forest lateral-flow absolute-magnitude authority](20260618-forest-lateral-flow-absolute-magnitude-authority.md) | `complete` | — | **Promoted 2026-07-02** to the four-tier observed-authority envelope `SC-SUBHYD-001#INV-SUBHYD-033` (WS10/Maimai/Panola/Weiler); FARPOINT01 magnitude now judged against field data, not legacy. Judgment run = MOFEFID-C03. Prune candidate. |
| [Native-vegetation evapotranspiration process model](20260803-native-vegetation-et-process-model.md) | `concept` | Vendor Stevens Canyon cohort; admit component-resolved forest authority; amend `SC-EVAP-001` | Replace agricultural LAI/crop-coefficient partitioning in explicit native mode with conservative live-canopy transpiration, soil evaporation, and canopy/litter interception processes. Legacy remains compatibility/diagnostic behavior, not native truth. |
| [Canopy mutation peak-runoff discontinuity](20260807-canopy-peak-runoff-discontinuity.md) | `staged` | Execute [hourly peak-runoff authority closure](../work-packages/20260809-hourly-peak-runoff-authority-closure-001/package.md) | Legacy replication retired after the Topanga 1,088-trial census established a general daily-return retiming defect. The active package makes modeled hourly runoff authoritative, preserves hourly saturation return, fixes units, and reruns the small-mutation hillslope design with openWEPP. |
| [Frost / daylength canopy decline + leaf-on/leaf-off + residue cover](20260626-frost-daylength-canopy-decline-hemisphere-robust.md) | `staged` | empirical calibration and assurance under the [canopy phenology assurance roadmap](../planning/canopy-phenology-assurance-roadmap.md) | **Mechanics implemented:** dynamic residue coupling and `CANOPY-PHENOLOGY-02` native GSI leaf-on/leaf-off, canopy, LAI, litter, and real-consumer integration are complete. Remaining: Bill Elliot reproduction, field calibration, litter-source adjudication, canopy-gradient congruence, independent Southern Hemisphere evaluation, and `CANOPY-ASSURE-01`. |
| [RHESSys-derived vegetation crate](20260806-rhessys-derived-vegetation-crate.md) | `blocked` | Admit the selected constitutive families contract-first through the held coupled successor | The authority reframe and exact workspace pass. Site values and compatible state are caller configuration, not universal-value research blockers. Production remains blocked on complete schema/process authority and independent canopy, wet-canopy, forest-floor, and root-layer closure. |
| [Canopy snow interception / sublimation](20260627-canopy-snow-interception-sublimation.md) | `concept` | not load-bearing yet | The canopy side of sublimation (distinct from the surface-pack side below). |
| [Stream water temperature — surface energy balance](20260627-stream-water-temperature-surface-energy-balance.md) | `breadcrumb` | prerequisites met; not being planned | Surface-EB foundation + opt-in meltwater-temperature source exist. Open question on pickup: can hourly water+temperature serialize across HBP and be consumed by `openwepp-cli-watershed`. |
| [Multilayer surface-EB sublimation + longwave](20260629-multilayer-surface-sublimation-longwave.md) | `breadcrumb` | sequenced after frost | Surface-pack sublimation via the per-layer surface energy balance; the streamflow/ET-tuning case makes it more defect-shaped than a fidelity nicety. |
| [Hillslope sub-5× performance assessment](20260701-hillslope-sub5x-performance-assessment.md) | `complete` | — | **Ran** 2026-07-01: H2637 direct 71.4 s vs legacy 9.65 s (7.40×); perf profile attributes ~64% to the winter subsystem — frost partition solved **twice** per winter OFE-day (F1, ~21 s), success-path guard-symbol `String` formatting (F2, ~7–11 s), duplicate hourly forcing + cacheable curve fits (F3/F5). **Executed same day: WP-1 (mechanical, byte-identical) 71.4→46.7 s, then WP-2 (frost single-solve, rubric-passed) →32.8 s = 3.52× — inside the projected 3.5–4.5× composite.** Day-frame lifecycle measured non-problem (~3%). Prune candidate. |

## Active program (not backlog — tracked here for cross-reference)

| Program | State | Authority | Notes |
|---|---|---|---|
| **Kernel-boundary typing program** | `complete; CQR burndown closed` (2026-07-01) | array-native spec §8.2; ADR-0031; [kernel-boundary-cqr-burndown-execplan.md](../work-packages/kernel-boundary-cqr-burndown-execplan.md) | Symbol-map runtime (`scheduler.rs`/`day_frame.rs`/carriers) **deleted** on main (`a381702b`, merged `c588023e`); production direct-only, `compatibility_edge_invocations=0`, H2637 byte-identical. The nine-row CQR burndown is complete: all owned row scopes measure `0` production functions above CRAP 30, H2637 remained byte-identical per row, and final full-workspace CRAP evidence is recorded in the execplan. Obsolete symbol-map-runtime tests were retired with justification; retained ratified-contract assertions were restored at typed runtime/parser surfaces. ROADMAP item K closed. |
