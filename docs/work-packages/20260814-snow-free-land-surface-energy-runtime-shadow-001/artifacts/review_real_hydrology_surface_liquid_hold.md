# Independent Review: Real-Hydrology Surface-Liquid HOLD

Evidence class: `Static`

Review target:
`artifacts/real-hydrology-surface-liquid-hold-audit.md`

Path keys used below are exact:

- `SC` = `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
- `PKG` = `docs/work-packages/20260814-snow-free-land-surface-energy-runtime-shadow-001/package.md`
- `ORCH` = `crates/openwepp-hillslope-orchestrator/src`
- `DR` = `crates/openwepp-hillslope-orchestrator/src/direct_runtime`
- `CH2` = `crates/openwepp-hillslope-orchestrator/src/vegetation_real_hydrology_shadow.rs`
- `C3ADAPTER` = `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs`

## Disposition

`PASS` — the proposed HOLD is legitimate.

`FAIL` — no safe in-scope extraction from current production state can close the
Child-3 `forest_litter` real-owner requirement. Current production has a real,
day-persistent soil-layer liquid owner, but it has no persistent
per-`(OFE,tile,surface class)` litter/surface-liquid store. It also has no
accepted owner-candidate operation that applies the LSE transaction's typed
condensation credit to such a store at the immutable beginning-snapshot point.

Relabeling residue interception, depression retention, WAT5 output, snow liquid,
or an aggregate storage correction would invent ownership or change production
science. The bounded soil-layer route remains legitimate for bare mineral soil;
it does not satisfy forest-litter closure.

## Binding Requirement

The current contract is unambiguous:

- hydrology exclusively owns ponded/depression, litter-held, and soil-layer
  water, while LSE owns no water amount
  (`SC:421-430`);
- hydrology must supply every beginning water amount through an immutable owner
  snapshot (`SC:439-448`);
- `forest_litter` requires hydrology-owned `W_l`, bounded by a configured
  `W_l,max`, and blocks direct mineral-soil evaporation during the interval
  (`SC:614-629`);
- condensation has no withdrawal request and must credit the exact amount to
  the typed surface store (`SC:684-708`); and
- roots and ground may use only immutable beginning stores before current rain,
  runon, or canopy release; finalized debits and the condensation credit precede
  the once-only ingress partition (`SC:710-744`).

The package requires source-keyed D/A/F, real-owner authorization, restart,
rollback, and independent closure before exit
(`PKG:53-60`). It excludes production dispatch and scheduler edits
(`PKG:17-28`), so a
new production store and state point are not hidden implementation details that
Child 3 may infer locally.

## Exact Production State Audit

### Day-persistent owner state

`DirectLaneFrame` is the production day-to-day carry. Its complete state surface
includes aggregate water, transfer/publication state, subsurface layers, ET
stage state, plant state, winter/snow/frost carries, erosion carry, and day
inputs; it contains no litter or non-snow surface-liquid amount
(`DR/00_core_frames.rs:965-996`).
`DirectWaterState` contains only aggregate soil water and daily infiltration,
runoff, ET, drainage, and lateral-flow values
(`DR/02_state_reports.rs:31-52`).

The day seed copies `lane.water`, then explicitly carries production
`subsurface_layers`, ET stage state, winter/snow/frost state, and erosion state
(`DR/00_core_frames.rs:692-764`). The day commit writes aggregate
soil water and the final subsurface layers, then ET stage, plant, snow/frost,
and erosion carries (`DR/00_core_frames.rs:1088-1173`). Neither
path reads or writes a litter/surface-liquid carry. This is stronger than a
name search: the actual seed/commit persistence boundary has no candidate field.

The concrete persistent liquid inventory is
`DirectSubsurfaceLayerState::theta_m`, with frozen-water and layer-domain fields
alongside it (`DR/subsurface.rs:698-758`). The Child-2 immutable
snapshot consequently extracts facts only by `(OFE lane, SoilLayerId)` from
those seeded percolation layers
(`CH2:39-89,1011-1080`).
Authorization invokes the production layer-withdrawal kernel
(`CH2:340-389`), and candidate construction
mutates only `lane.subsurface_layers[layer].theta_m` and reconstructs aggregate
soil water (`CH2:519-579`). There is no tile,
surface-class, surface-store, capacity, credit, or litter ending-state identity
in that owner path.

The production runner constructs this same frame from lane constructor inputs
(`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:306-370`).
Those constructor inputs likewise enumerate subsurface, plant, winter,
snow/frost, and daily inputs but no litter/surface store
(`DR/00_core_frames.rs:139-162`). The direct frame types derive
`Clone/PartialEq`, not serialization (`DR/00_core_frames.rs:139-162,545-562,965-996`).
The only nearby explicit persistent snapshot serializer found is snow Stage-3's
typed `DirectSnowStage3PersistentState` serializer/restore pair
(`ORCH/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs:100-131`);
it is not a general surface-liquid restart endpoint.

### Route 1: `residue_interception_m`

`DirectEvapotranspirationComputeInputs::residue_interception_m` is an input
field and defaults to zero (`DR/evapotranspiration.rs:629-664`).
In active growth flow it is copied from the current growth downstream operands
into the ET compute inputs (`DR/growth.rs:212-234`); the growth
input/result surfaces carry it as a daily operand
(`DR/growth.rs:395-446,737-755`). It is not sourced from
`DirectLaneFrame` owner storage.

WB17 first treats the value as residue evaporation. Any unused amount is added
directly to the top soil layer, and the returned residue inventory is always
exactly zero (`DR/evapotranspiration.rs:417-460`). The exposed
`DirectEvapotranspirationSurfaceState::residue_interception_after_m` is therefore
a same-pass result/diagnostic, not a carried owner ending state
(`DR/evapotranspiration.rs:998-1029`). The day commit does not
persist it. It also lacks OFE/tile/surface-class capacity, transaction/source
identity, an immutable snapshot digest member, and a debit/credit operation.

Verdict: `FAIL` as a litter owner. Treating this input as `W_l` would both invent
custody and alter existing WB17 behavior, including its current unused-water
transfer into top-soil storage.

### Route 2: PMET storage return / possible condensation alias

The closest production addition is the PMET negative-soil-evaporation return:
negative `es_raw_m` becomes a nonnegative
`soil_evaporation_storage_return_m` (`DR/evapotranspiration.rs:728-745`),
which is added to the first soil layer (`DR/evapotranspiration.rs:1347-1363`)
and included in aggregate storage reconciliation
(`DR/evapotranspiration.rs:285-301` and `DR/storage.rs:797-833`).

This proves that production can add a legacy-PMET-computed correction to the
top-soil owner; it does **not** prove an LSE condensation-credit endpoint. The
amount is generated inside legacy PMET, is not keyed by the LSE transaction,
OFE/tile/surface/source identity, carries no condensate temperature or enthalpy,
and cannot target a litter/surface store. There is no operation accepting the
final joint solve's explicit credit and staging it in the same immutable owner
candidate as D/A/F.

Verdict: `FAIL` for the required typed condensation credit and `FAIL` for
forest litter. It may inform a future soil-layer credit implementation, but it
cannot be extracted or called as the present contract endpoint.

### Route 3: infiltration/depression state

WB14 computes `depression_storage_delta_m` from the current day's rainfall
excess and configured capacity, then removes that retained amount from the
current runoff profile (`DR/runoff.rs:1986-2031`). The state and
downstream operands contain only cumulative infiltration and the storage
**delta** (`DR/runoff.rs:2512-2559`); the input handoff defaults
both values to zero (`DR/runoff.rs:2341-2356`). Neither
`DirectLaneFrame` nor its commit carries a beginning depression amount into the
next immutable snapshot.

Verdict: `FAIL`. This is an interval partition result, not an owner inventory
with beginning/end state, debit, credit, tile/class identity, or restart lineage.

### Route 4: WAT5 retention

WAT5 allocates rainfall, infiltration, depression retention, and post-depression
excess into 288 five-minute bins (`DR/runoff.rs:1674-1702`). Its
profile is initialized to zero and filled by a chronological replay
(`DR/runoff.rs:1845-1869,2001-2019`). More decisively,
`DirectDayFrame` declares the optional WAT5 ledger diagnostic and explicitly
states that it does not participate in authoritative state
(`DR/00_core_frames.rs:1201-1212`).

Verdict: `FAIL`. A diagnostic replay cannot be promoted into owner storage.

### Route 5: snow and frost liquid

Production does persist snow liquid: `DirectSnowRuntimeCarry` contains
`liquid_water_retained_m` (`DR/00_core_frames.rs:164-174`), and
the snow-coupling span writes retained liquid back into the winter/snow carry
(`DR/storage.rs:528-558`). This is real state, but its owner and
branch are snow. The LSE contract requires snow absent at both endpoints and
rejects snow, frozen, or thawing material before calculation
(`SC:413-419`). Frost's optional liquid delta is an
aggregate soil-storage coupling input, not a snow-free surface/litter inventory
(`DR/storage.rs:615-647`).

Verdict: `FAIL`. Borrowing snow/frost state violates both branch exclusion and
owner identity.

## Current Child-3 Boundary Corroboration

The default-off adapter reflects the production boundary rather than filling
the gap: it only accepts `SoilLayerLiquid`, rejects surface/litter sources
(`C3ADAPTER:348-373`), debits only subsurface layers
(`C3ADAPTER:293-339`),
and rejects condensation because `DirectRunFrame` has no accepted mutation
endpoint (`C3ADAPTER:342-345,409-411`). This is a
correct fail-closed implementation of the audited HOLD, not independent proof
of the missing state.

## Attempted Safe Routes and Result

| Route | Actual owner/state | Same-snapshot D/A/F + credit? | Forest-litter legitimacy |
|---|---|---|---|
| Child-2/top soil layer | persistent hydrology layer | withdrawal D/A/F exists; no typed LSE credit operation | `FAIL` for litter; valid for bare dry mineral soil |
| residue interception | current growth/ET operand; zero ending residue | no | `FAIL` |
| PMET storage return | legacy-computed top-soil addition | no external typed credit; no litter target | `FAIL` |
| WB14 depression delta | current-interval partition delta | no | `FAIL` |
| WAT5 retention | explicitly diagnostic replay | no | `FAIL` |
| snow retained liquid | persistent snow owner | wrong branch/owner | `FAIL` |
| frost liquid delta | aggregate soil/frost coupling | wrong branch/identity | `FAIL` |
| shadow sidecar/test inventory | no production owner | synthetic | `FAIL` |

There is therefore no authority-preserving extraction that can complete the
forest-litter endpoint without adding new production-owned state or changing
existing hydrology/ET science and scheduling. A soil-only partial integration
must remain explicitly bounded and cannot satisfy the package's forest-floor
real-owner closure claim.

## First Lift Action

Open and authorize a separate hydrology-owner work package (and amend/freeze its
state contract before code) for one persistent, restart-representable
per-`(OFE,tile,surface class)` snow-free liquid store. The first implementation
increment must place that store on the actual production lane/run owner boundary
and prove uninterrupted-versus-restored day continuity. It must then expose, at
the exact Child-2 immutable day-start snapshot point:

1. typed beginning amount and capacity with owner/OFE/tile/surface/source identity;
2. common arbitration with root and ground beginning-store requests;
3. finalized debit and signed condensation mass/enthalpy credit in one candidate;
4. final current-ingress retention/overflow/infiltration/runoff routing exactly once;
5. atomic validation, rollback, and ending-state/restart serialization; and
6. scheduler ordering proving the same immutable snapshot feeds all claimants.

Only after that owner increment closes should Child 3 add the forest-litter
adapter. Deriving the new store from daily residue input or depression deltas is
not a lift action; it is a production-science change requiring explicit science
authority and migration rules.

No commands were run as behavioral validation; this artifact is a static code,
contract, persistence-path, and ownership review.
