# 3. The hillslope run, end to end

What actually happens between launching `openwepp-cli-hill` and parquet
appearing on disk. This chapter follows the **direct runtime** — the
array-native typed execution path that has been the only production hillslope
runtime since ADR-0030/0031 — at the level of code you can click into.

Vocabulary used heavily here (lane, phase span, shadow projection, closure) is
defined in [chapter 5](05-concepts-glossary.md); the history of *why* the
runtime is typed-frame-shaped is [chapter 6](06-history-and-performance.md).

## 3.1 The shape in one picture

```text
                        once per run                            per OFE-day
┌──────────────────────────────────────────────┐   ┌──────────────────────────────────┐
│ runfile (TOML) + run-dir                     │   │ DirectProductionDayInputBuilder  │
│   ├─ parse soil/management/slope/climate     │   │   climate day + lane state       │
│   ├─ discover legacy sidecars (snow, pmet…)  │   │   → snow/frost authorities       │
│   ├─ build static runtime setup (lanes,      │   │   → DirectPublicationDayInput    │
│   │    geometry, climate span)               │   └────────────┬─────────────────────┘
│   ├─ DirectProductionSeedAuthority           │                │
│   │    (typed day-zero seeds)                │   ┌────────────▼─────────────────────┐
│   └─ DirectRunFrame (one DirectLaneFrame     │   │ DirectDayFrame                   │
│        per OFE)                              │   │   seed ← lane carry state        │
└──────────────┬───────────────────────────────┘   │   apply day input                │
               │                                   │   run ~23 phase spans (§3.4)     │
               ▼                                   │   → DirectPublicationDayRow      │
   day-major, OFE-minor loop                       │   commit → lane carry state      │
   for day { for lane { … } }                      └────────────┬─────────────────────┘
                                                                │
                                          streaming sink: parquet row groups (8192),
                                          summary accumulator → HBP + loss at run end
```

Two loops matter. The **outer loop** is day-major, OFE-minor
(`for day_index { for lane_index { … } }`), so upslope OFEs finish a day
before downslope OFEs of the same day consume their transfers. The **inner
cycle** is the per-OFE-day pipeline on the right: build inputs, seed a day
frame from the lane's carried state, run the phase sequence, publish one row,
commit state back to the lane.

## 3.2 Once per run: intake and setup

Entry point: `execute_hillslope_run` in
`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`,
feeding from intake in `00_runner_intake_and_lane_setup.rs`. In order:

1. **Runfile + inputs.** The TOML runfile (`openwepp-hillslope-runfile-v1`)
   names the WEPP input files. The parsers in `openwepp-input-contract`
   validate soil / management / slope / climate into typed surfaces — a parse
   error is a run-refusing error with a located message, never a default.
2. **Sidecar discovery.** With `--legacy-sidecar-discovery`, legacy `.txt`
   sidecars (snow, frost, pmetpara, wepp_ui, gwcoeff) are resolved from the
   run-dir (`openwepp-legacy-bridge`). Unknown sidecars produce explicit
   `LSB-W-*` warnings rather than silent acceptance.
3. **Static runtime setup.** Per-OFE lane slices, areas, runoff publication
   geometry (`Q`/`QOFE` scaling, effective lengths), and the climate span are
   computed once (`build_static_hillslope_runtime_setup`).
4. **Seed authority.** `DirectProductionSeedAuthority` computes the typed
   day-zero state — initial layer stores, controls, ET demand seeds — the
   ordered seed pipeline that used to live on the deleted symbol-map surface.
5. **Frame construction.** `build_direct_production_run_frame` builds one
   `DirectRunFrame` holding a `DirectLaneFrame` per OFE. In production the
   per-day inputs are *not* pre-materialized (that pre-allocation once cost
   ~909 MiB on a 34-year run; see chapter 6) — they are built lazily by a
   day-input builder closure as the loop reaches each OFE-day.

## 3.3 The three frame tiers

Defined in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`:

| Frame | Lifetime | Holds |
|---|---|---|
| `DirectRunFrame` | whole run | run identity (lane count, day count), the phase plan, all lanes, the inter-OFE transfer ledger |
| `DirectLaneFrame` | whole run, one per OFE | the state that **carries across days**: soil water, subsurface layer states, plant growth state, ET stage state, the winter column (snow + frost), transfer buffers, publication scalars |
| `DirectDayFrame` | one OFE-day | the working state for a single OFE-day: forcing, every phase's typed inputs / state / downstream operands / shadow projection |

This is the direct answer to the legacy `COMMON` block. Everything a phase
may read or write is a **named, typed field** on the day frame; what survives
the day is exactly what `commit_day` copies back into the lane frame — one
audited place where day state becomes carry state, with guard validation on
the way through (e.g. a negative reconciled storage refuses to commit).

`seed_day_frame` / `commit_day` (same file) are the day boundary:
seed copies lane carry state *in* (water, transfer, winter column, layer
states); commit copies the day's results *out* (reconciled storage, layer
state after root uptake, growth state, winter column, erosion operands).

## 3.4 The per-OFE-day phase sequence

`DirectFrameExecutor::run_day_spans`
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`)
runs a fixed, explicit sequence of **phase spans**. Each span is a method on
`DirectDayFrame` in a physics module of `direct_runtime/`; each follows the
same internal shape: *compute* (pure function of typed inputs) → *mutate*
(write typed state fields) → *produce downstream operands* (what later phases
may read) → *record a shadow projection* (an audit snapshot).

The production order, with the module that owns each:

| # | Span | Module | Physics |
|---|---|---|---|
| 1 | `r5b_normalization` | `normalization.rs` | forcing/carry normalization and bounds |
| 2 | `r5b_storage_bounds` | `normalization.rs` | storage bounds preparation |
| 3 | `r5c_decomposition` | `decomposition.rs` | residue decomposition |
| 4 | `r5c_residue_partition` | `decomposition.rs` | residue mass partition |
| 5 | `r5d_annual_growth` | `growth.rs` | annual crop growth |
| 6 | `r5d_perennial_growth` | `growth.rs` | perennial growth |
| 7 | `r4c_storage_input` | `storage.rs` | storage input accounting |
| 8 | `r4i_liquid_input` | `runoff.rs` | liquid water input (rain + melt handoff) |
| 9 | `r4j_runon_carry` | `runoff.rs` | run-on carry from upslope |
| 10 | `r4k_infiltration_depression` | `runoff.rs` | Green–Ampt infiltration + depression storage |
| 11 | `r4m_percolation` | `subsurface.rs` | vertical percolation through layers, deep seepage |
| 12 | `r4n_surface_et` | `evapotranspiration.rs` | surface evaporation |
| — | `r4x_winter_local_liquid` projection | (guarded) | winter pre-saturation liquid projection |
| 13 | `r4o_subsurface_compute` | `subsurface.rs` | lateral flow, tile drainage, saturation |
| 14 | `r4n_root_uptake` | `evapotranspiration.rs` | transpiration / root-zone withdrawal |
| 15 | `r4g_snow_coupling` | `storage.rs` | snowpack coupling into the water column |
| 16 | `r4l_saturation_addback` | `runoff.rs` | saturation-excess add-back |
| 17 | `r4a_runoff_partition` | `runoff.rs` | runoff partition (winter-frost-aware) |
| 18 | `r7d6_peak_runoff` | `runoff.rs` | peak runoff |
| 19 | `r4b_storage_reconciliation` | `storage.rs` | daily water-balance reconciliation |
| 20 | `r4pqz_hydrology_projection` | `projection.rs` | WB output projection + **closure check** |
| 21 | `r7d6_erosion` | `erosion.rs` | hillslope erosion (EROD13/14/15) |
| 22 | `r3b_water_ledger` | `00_core_frames.rs` | the day's water ledger |

(The `rN`/`r7dN` prefixes are rewrite-stage names from the R0–R7 program that
built this runtime — chapter 6. They are kept because the work-package log and
commit history speak this vocabulary.)

Cross-phase reads go through the *downstream operands* a producer phase
published — a consumer does `.as_ref().ok_or(MissingDirectUpstream)`, so a
mis-ordered phase plan fails loudly with the name of the missing producer
rather than reading a stale value. This is the runtime-enforced version of the
dependency ordering that legacy WEPP encoded only in `CALL` statement order.

**Failure semantics.** Every span validates its inputs and outputs against
typed guards (`validate_finite`, `validate_nonnegative_direct_m`, range
guards). Phase 20 additionally enforces **closure**: layer-aggregated storage
must reconcile with the ledger within contract tolerance, or the run fails
with a `DirectClosureToleranceExceeded` error carrying the full operand
decomposition (storage, precip, runoff, ET, seepage, frost delta …) for the
exact lane/day that broke. A conservation bug in openWEPP is therefore a
*located* bug.

## 3.5 The winter column (snow + frost)

Winter physics does not fit the one-shot phase shape — snowpack and frost are
**stateful hourly sub-systems** whose layers persist across days. They live as
a dedicated sub-solver (`DirectWinterColumnState`, hourly machinery under
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/`,
ratified by [ADR-0026](../decisions/0026-stateful-winter-column-sub-solver.md)
and [ADR-0029](../decisions/0029-commit-paradigm-2-multilayer-snow.md)):

- **Hourly forcing** for winter days is built from the climate surface
  (`runtime_inputs/06_simimpl28_hourly_forcing.rs`): hourly temperature and
  radiation curves, plus precipitation phase partition (Harder–Pomeroy
  psychrometric hydrometeor temperature).
- **Snow**: multilayer snowpack with density evolution, melt/refreeze, and a
  liquid-routing partition that hands melt water to the day's liquid input.
- **Frost**: an hourly fine-layer freeze/thaw column (`coupling/frost*.rs`)
  computing frost depth, frozen water, and frozen infiltration capacity, which
  gates infiltration and runoff partition on frozen days.

The day-input builder (next section) runs these winter authorities to produce
the day's typed winter inputs; the runoff-partition span re-evaluates the
frost partition against the day's evolved layer state. The winter column state
carries in the lane frame like all other carry state.

## 3.6 Per-day inputs: the production builder

`DirectProductionDayInputBuilder`
(`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`)
is the bridge between the parsed input surfaces and the engine's typed day
inputs. Per OFE-day it assembles a `DirectPublicationDayInput`: climate
forcing, hyetograph-derived intensities, percolation/subsurface layer inputs,
ET compute inputs, growth schedule activation, snow/frost winter inputs,
erosion inputs. It is deliberately **stateless across days except for named
authorities** (residue cover state, snow/frost lane authorities) — the
builder derives each day from the lane's committed state plus the static
setup, rather than keeping a second evolving copy of the world.

## 3.7 Publication: rows out, no run-length buffering

Each OFE-day produces one `DirectPublicationDayRow`
(`direct_runtime/01_publication.rs`) — a flat, typed, `Copy`-field row of
everything the output surface needs (water balance operands, runoff, ET,
soil temperature/frost, erosion). The executor hands it to a **streaming
sink** (`04_direct_publication.rs`):

- WAT / PASS parquet rows are appended to pre-sized chunks and flushed every
  8,192 rows (one parquet row group) — the run's memory footprint is
  **run-length-flat** (~80 MiB regardless of simulation years; it was
  1.13 GiB before the 2026-06-30 streaming arc).
- A summary accumulator observes every row and builds the HBP pass shard,
  loss report, and run manifest **once at run end**.
- The manifest records runtime counters (`day_frame_commits`,
  `compatibility_edge_invocations` — the latter must be 0) and provenance
  (source commit, binary checksum), so a run artifact is self-describing.

## 3.8 Multi-OFE routing within the hillslope

OFE-to-OFE transfer is explicit: after each OFE-day, surface runoff and hourly
lateral carries are published into the downstream lane's transfer buffers
(`publish_dynamic_transfer_to_downstream`, `03_executor.rs`), validated
against the lane topology (single outlet, consistent upstream/downstream
pairing — checked before the run starts). The day-major loop order guarantees
the downstream OFE sees its upslope inputs for the same simulated day.

## 3.9 Where to set a breakpoint

| Question | Place |
|---|---|
| "What did day N look like on OFE k?" | `run_day_spans` (`03_executor.rs`) with `day_index`/`lane_index` conditionals |
| "Why did the water balance break?" | the `DirectClosureToleranceExceeded` detail string already carries the decomposition; then `r4b_storage_reconciliation` in `storage.rs` |
| "What state crosses days?" | `DirectLaneFrame::commit_day` (`00_core_frames.rs`) |
| "What inputs did the engine get for a day?" | `DirectProductionDayInputBuilder::build` (runner, `00_builders_and_authority.rs`) |
| "What's in the output row?" | `DirectPublicationDayRow::from_day_frame` (`01_publication.rs`) |

There are also env-gated JSONL trace hooks on hot phases (percolation,
subsurface saturation, runoff rebalance, ET, storage — grep
`OPENWEPP_R7H_*_TRACE_PATH`) that dump per-day operand lines for a filtered
lane/day without a debugger.
