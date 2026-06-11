# D3 Fine-Sublayer Port — M1 Scoping Phase Prompt

Status: executed (M1 scoping complete; no production code)
Author: Claude Code, 2026-06-11
Authority: `SC-SNOWFREEZE-001` v57 (`INV-SNOWFREEZE-006`/`-012`), superseded
for Increment A handoff wording by v58, FDHP01 package envelope, addenda 2e/3
and the D3 coarse-front failed-attempt evidence.

## Phase framing

This is an **M1 scoping artifact, not an implementation prompt.** The next
Codex pass executes this phase and fills the Deliverables sections below
(in this file). **No production code edits in this phase.** The completed
artifact becomes the **first required-reading item** of the subsequent
implementation pass.

Why this phase exists: two D3 shortcuts have now failed in a structured way —
the scalar-`frdp` model could not carry mass (addendum 2e), and the coarse
per-layer energy front ran clean but missed the envelope (mean max 643 mm vs
legacy 414, duration delta −428 days; recorded as failed evidence and backed
out, `efd2152b`). Both skipped the scoping step. The fine-sublayer port is
evidence-forced; this phase makes it land against a declared state shape
rather than an evolving one.

Subagent requirement: none for this phase (reading + authoring only; no
heavy runs).

## Required reading for this phase

- This package's `package.md` (envelope, protected boundaries) and
  `review_claude_fdhp01_closure_status_and_cohort_validation.md`
  (addenda 2e, 3 — why depth and mass decoupled).
- `SC-SNOWFREEZE-001.md` v57 (`INV-SNOWFREEZE-006`, `-012`, `-013`, the D3
  tightening) — the authority the map must trace to.
- Legacy pinned baseline `/workdir/wepp-forest_260430_baseline/src/`:
  `frostn.for` (entry handoff `:336`, hourly dispatch `:662–686`),
  `frwatc.for` (`wbtofs=0` frost→WB `:80–137`; `wbtofs=1` WB→frost `:139+`),
  `frzng.for`, `frznw.for`, `mlttp.for`, `mltbtm.for`, `watdst.for`
  (unfrozen-water redistribution under frost, Saxton–Rawls), plus the
  `watbalprint.for` publication (`soilw`/`soilf` summation, `:56–69`).
- CRM Ch. 3.8; Dun et al. 2010 (fine-layer discretization rationale).

## Deliverables (executed scoping results; all `Static:` legacy-source-traced)

### 1. Legacy state-machine map

Static: pinned baseline source read.

The legacy model is a fine-sublayer state machine. The load-bearing inversion
from addendum 2e is explicit: ice mass is accumulated in `slsic`/`frzw`, while
depth is recomputed from fine-layer flags and frozen thicknesses. Depth is not
allowed to allocate frozen water after the fact.

| Symbol | Meaning and units | Shape | Writes | Reads | Lifecycle / port note |
|---|---|---|---|---|---|
| `fgfrst` | Frost flag for each fine layer: `0` no frost, `1` fully frozen, `2` frozen at top, `3` frozen at bottom. Unitless. | fine sublayer x soil layer x plane | Initialized in `frostn.for:254-268` and `:290-333`; mutated by freeze/thaw in `frzng.for:539-552`, `mltbtm.for:419-432`, `mlttp.for:448-462`, cleared in thaw completion paths. Definition in `cflgfs.inc:7-24`. | Branch scans in `frzng.for:245-285`, `mltbtm.for:194-228`, `mlttp.for:295-356`, `watdst.for:320-494`. | Persistent while frost exists; rebuilt only on no-frost/initial-frost setup. This is the primary state for deriving `frdp`/`thdp`/sandwich geometry. |
| `slfsd` | Frozen thickness inside a fine layer, m. | fine sublayer x soil layer x plane | Initialized in `frostn.for:256-257`, `:291-293`, `:324-325`; advanced by `frzng.for:555-563`; reduced/cleared by `mltbtm.for:419-428`, `:431-446`, `mlttp.for:448-457`, `:459-478`. Definition in `cflgfs.inc:26`. | Aggregated by `frwatc.for:102-129`; scanned by `watdst.for:320-494`; used in `frznw.for:123-130`. | Persistent fine-state member; aggregate `frozen(i)` is derived, not authoritative. |
| `slsic` | Ice amount in a fine layer, m water equivalent. | fine sublayer x soil layer x plane | Initialized in `frostn.for:267`, `:305-308`, `:333`; increased by `frzng.for:566-619` and `frznw.for:153-164`; reduced/cleared by `mltbtm.for:419-428`, `:441-497`, `mlttp.for:448-528`. Definition in `cflgfs.inc:28`. | Aggregated by `frwatc.for:102-129`; thaw routines compute melt energy from it (`mltbtm.for:384-405`, `mlttp.for:415-434`). | Persistent frozen mass. This is the missing D3 mass/depth coupling store. |
| `slsw` | Liquid water content in the unfrozen part of a fine layer, volumetric fraction. | fine sublayer x soil layer x plane | Initialized from `soilw/dg` or `thetdr` in `frostn.for:258-267`, `:300-333`; adjusted by `frwatc.for:149-259`, `frzng.for:571-619`, `mltbtm.for:423-559`, `mlttp.for:451-592`, `watdst.for:156-278`. Definition in `cflgfs.inc:27`. | Conductivity and hydraulic redistribution in `frostn.for:432-443`, `:564-575`, `frzng.for:399-437`, `watdst.for:184-278`. | Persistent liquid state in fine layers; `frwatc(1)` updates it from daily water-balance deltas. |
| `sltime` | Per-fine-layer water redistribution time already consumed in the current hour, s. | fine sublayer x soil layer x plane | Reset at each hour in `frostn.for:342-348`; advanced in `watdst.for:184-189`. Definition in `cflgfs.inc:29`. | `watdst.for:173-178` subtracts prior elapsed time for mode `fgfzft=2`. | Hour-local; do not persist across days. |
| `nfine` | Number of fine layers per WEPP soil layer. Unitless count. | soil layer | Recomputed in `frostn.for:195-235`; declared in `cflgfs.inc:7-16`. Last layer derives from actual `dg`; top two layers use `fineTop`, deeper layers use `fineBot`. | `locate.for:48-134`, all fine-layer loops. | Deterministic topology from `dg`, `fineTop`, `fineBot`; publish for diagnostics but do not treat as mutable physics state. Fine thickness is `dg(i)/nfine(i)`. |
| `fineTop`, `fineBot` | Frost sidecar controls for fine-layer count in top two and lower layers. | scalar options | Parsed/provided as frost options; used in `frostn.for:205-223`. Definition in `cflgfs.inc:30-32`. | Fine-layer topology construction. | Existing `frost.options.fineTop`/`fineBot` inputs stay authoritative. |
| `frdp`, `thdp`, `tfrdp`, `tthawd` | Global frost bottom depth, surface thaw depth, top frost depth, and top thaw depth, m. | plane | Initialized/read in `frostn`; recomputed from fine-layer scan in `watdst.for:458-511`; thaw-through clears them in `watdst.for:499-511`. Definitions in `cwint.inc:45-58`. | Dispatch/locate in `frostn.for:359-365`, `:614-655`; publication consumes `frdp`. | Derived summary state. The implementation must derive these from fine layers after mutations, not mutate them first and project mass afterward. |
| `frozen` | Frozen thickness per coarse soil layer, m. | soil layer x plane | Recomputed by `frwatc(0)` as `sum slfsd` in `frwatc.for:95-129`; reduced by `frznw.for:153-161`, `mltbtm.for:530-561`, `mlttp.for:548-594` for liquid-in-frozen-zone handling. Definition in `cwint.inc:53-56`. | `frwatc(1)` allocates new water into frozen-zone liquid using it (`frwatc.for:198-218`); `watbal.for:1031-1033` builds `soilf`. | Coarse aggregate/cache for water-balance consumers; derived from fine state at handoff. |
| `frzw` | Frozen water above residual per coarse layer, m. | soil layer x plane | Recomputed by `frwatc(0)` as `sumice - thetdr*frozen` in `frwatc.for:127-132`; initialized elsewhere to zero. Definition in `cwint.inc:55-56`. | Capacity terms in `frwatc.for:198-218`; legacy water-balance/percolation consumers (`watbal.for:507-799`, `perc.for:135-150`); `soilf` publication build in `watbal.for:1026-1038`. | Derived aggregate/cache. Existing `wb18_perc_frzw_####` can survive as this aggregate, not as the primary fine-layer store. |
| `soilf` | Published frozen water per coarse layer, m: `frzw + thetdr*frozen` when `frdp > 0.001`, else zero. | soil layer x plane | Built in `watbal.for:1026-1038` and hourly form `watbal_hourly.for:1085-1098`. Definition in `cwater.inc:60-62`. | `watbalprint.for:56-69` sums `frozwt = Σ soilf`; `bigout.for`/`outeng.for` also publish it. | Publication aggregate; openWEPP WAT `frozwt` must remain this lineage. |
| `soilw` | Unfrozen/liquid soil water per coarse layer, m. | soil layer x plane | `frwatc(0)` recomputes it from unfrozen fine-layer water plus `nwfrzz` in `frwatc.for:102-132`; initial fine state reads it in `frostn.for:258-333`. Definition in `cwater.inc:60-61`. | Water-balance publication `watbalprint.for:56-69`; frost setup and `frwatc(1)`. | This is `Total-Soil`/`SoilWaterTotal` lineage and excludes frozen water. |
| `st` | Available water storage per coarse layer, m. | soil layer x plane | `frwatc(0)` recomputes `st = sumst + nwfrzz` in `frwatc.for:121-132`; water-balance routines mutate it. Definition in `cwater.inc:40-42`. | `frwatc(1)` uses `varwat = st - yst` as the daily exchange delta in `frwatc.for:149-164`. | Must be represented or exactly derivable at the seam; aggregate `wb11_soil_water` alone is insufficient for a faithful `frwatc(1)`. |
| `yst` | Prior handoff available storage per coarse layer, m. | soil layer x plane | Initialized in `frostn.for:266`, `:303`, `:332`; updated by `frwatc(0)` in `frwatc.for:127-132`. Definition in `cwater.inc:62`. | `frwatc(1)` computes `st - yst` in `frwatc.for:158-160`. | Persistent across frost days; required to avoid double-applying WB deltas. |
| `nwfrzz` | Unfrozen liquid water residing in the frozen zone, m. | soil layer x plane | Zeroed/updated in `frwatc.for:149-218`; frozen into `slsic` by `frznw.for:123-164`; released by thaw in `mltbtm.for:530-561`, `mlttp.for:548-594`. Definition in `cwint.inc:81-85`. | `frzng.for:193-220`, `:245-263`; `frwatc` capacity calculations. | Persistent layer store. It is separate from `slsic` and must be ported with `frznw`; omitting it loses migration-water refreeze behavior. |
| `watpdg`, `watbtm` | Ponded thaw water at surface and bottom/profile exit, m. | plane | `frzng.for:132-163` freezes surface ponding; top thaw writes `watpdg` in `mlttp.for:491-542`; bottom thaw writes `watbtm` in `mltbtm.for:459-524`. Definitions in `cwint.inc:48-49`. | Freeze/thaw routines. | Carry through the fine-layer port because these are the overflow paths for thawed water. |
| `frzflg` | Dispatch arm flag: `0` balanced, `1` bottom freeze, `2` top freeze + bottom thaw, `3` both/top thaw, `4` bottom thaw. | plane/current hour | Selected in `frostn.for:614-655`; definition in `cpfrst.inc:19-24`. | Dispatch in `frostn.for:660-681`; freeze/thaw routines branch on it (`frzng.for:167-189`, `:448-455`, `mltbtm.for:233-366`). | Hour-local control state, but useful as diagnostic `frost.hourly.frzflg_####`. |
| `qdry`, `qhtout`, `qwet`, `qoutdm`, `dmfrsn` | Heat-flow terms: lower unfrozen-soil heat, surface/frozen-path heat, migration-water latent term, surface path resistance denominator, snow/residue denominator. Units W m^-2 for fluxes and m2 C W^-1 for denominators. | plane/current hour | `frostn.for:382-458` computes `qdry`; `frostn.for:466-607` computes `qhtout`/`qoutdm`/`dmfrsn`; `frzng.for:381-455` computes `qwet`; definitions in `cpfrst.inc:7-38`. | Dispatch and subroutine energy equations. | Existing hourly `qsrf`/`quf` diagnostics survive, but implementation needs signed internal terms matching the branch equations. |
| `kftill`, `kfutil`, `kres`, `ksnow`, `ksoilf`, `tilld` | Layer thermal conductivities/adjustments and tillage boundary. | scalar/plane | `frostn.for:185-193`, `:479-504`, `:530-535`; definitions in `cpfrst.inc:36-38`, `cwint.inc:88-91`. | Heat path resistance in `frostn`, `frzng`, `mlttp`, `mltbtm`. | Existing constants/options can survive; the resistance sum must be rebuilt over fine-layer geometry, not just aggregate depth. |
| `amtfrz`, `fcycle`, `fgcycl`, `nwfzfg` | Frost-heave/freeze-cycle/new-water diagnostics and flags. | plane | `frzng.for:471-479`, `watdst.for:516-527`; definitions in `cwint.inc:50-64`, `:81-85`. | Diagnostics/legacy reports. | Not first-order for D3 closure, but implementation should not preclude later publication. |

### 2. Routine sequence and trigger map

Static: pinned baseline source read.

Observed dispatch skeleton:

```text
winter -> frostN(hour)
  topology/state setup
  frwatc(1) at active-day hour 1 when existing frost state is present
  reset sltime for the hour
  compute qdry and qhtout
  choose frzflg
  dispatch one of four arms
  watdst(...) updates liquid redistribution and derived frost depths
  frwatc(0) at hour 24 or fgthwd = 1
```

The actual pinned call to `frwatc(1)` is guarded by `hour.eq.1`
(`frostn.for:335-337`), while `frwatc(0)` fires at day-end or thaw-complete
(`frostn.for:686-687`). If implementation interprets the contract phrase
"hourly entry" more broadly than this baseline source line, the contract must
be amended first.

Trigger and branch selection:

| Step | Static source | Required behavior |
|---|---|---|
| Build fine topology | `frostn.for:195-235`; `locate.for:48-134` | Recompute `nfine` from `fineTop`/`fineBot` and the actual last-layer `dg`; locate depths as start or end points with `tpbtfg`. |
| Initialize no-frost state | `frostn.for:247-268` | If `frdp < 0.001` and `slsic(1,1) < 1e-5`, clear flags/depth/ice, initialize `slsw` from `soilw/dg`, and set `yst = st`. |
| Initialize nonzero starting frost | `frostn.for:269-333` | If `fsdfg == 0` with nonzero `frdp`, locate the front, mark frozen fine layers, set `slfsd`, initialize ice from layer soil water, and initialize unfrozen layers separately. |
| `frwatc(1)` entry handoff | `frostn.for:335-337`; `frwatc.for:139-267` | At active frost day ingress, map WB storage delta `st - yst` into fine-layer liquid state, draining `nwfrzz` first for negative deltas and allocating positive deltas to frozen-zone liquid plus unfrozen fine layers. |
| Hour reset | `frostn.for:342-348` | Reset `sltime` before each hour so `watdst` mode `2` can subtract within-hour elapsed redistribution only. |
| Lower heat path `qdry` | `frostn.for:382-458` | Estimate temperature 1 m below frost, harmonic-mean unfrozen conductivity through fine layers, and lower heat flux. `tmpbl <= 0` makes `qdry = 0`. |
| Surface/frozen path `qhtout` | `frostn.for:466-607` | Cap positive surface temperature at zero under snow, add snow/residue resistance, then add frozen or thawed/frozen fine-layer path depending on `surtmp`, `thdp`, and `tfrdp`. |
| Arm selection | `frostn.for:614-655` | Sandwich frost (`tthawd > 0.001` or `thdp > 0.001`) uses `qhtout` sign: `<0` -> arm 2, `=0` plus `qdry` -> arm 0/4, `>0` -> arm 3. Single frost uses `qdry + qhtout` when `qhtout <= 0`: `<0` -> arm 1, `=0` -> arm 0, `>0` -> arm 4; positive `qhtout` -> arm 3. No frost starts arm 1 only when `qdry + qhtout < 0`. |

Dispatch arms:

| `frzflg` | Source | Calls and arguments | Meaning |
|---|---|---|---|
| `1` | `frostn.for:660-663` | `frzng(hour) -> watdst(0.0, 3600., 2)` | Bottom/front freezing. |
| `2` | `frostn.for:665-669` | `frzng(hour) -> if qdry > 0 then mltbtm(hour) -> watdst(0.0, 3600., 2)` | Top freezing with possible bottom thaw. This is the fourth arm missing from the initial three-arm simplification. |
| `3` | `frostn.for:671-676` | `mlttp(hour) -> if qdry > 0 && fgthwd != 1 then mltbtm(hour) -> watdst(0.0, 3600., 0)` | Top thaw / both-end thaw. |
| `4` | `frostn.for:678-681` | `mltbtm(hour) -> watdst(0.0, 3600., 0)` | Bottom thaw. |

Subroutine semantics:

- `frzng(hour)` is not a scalar depth increment. It first freezes surface
  ponding into `slsic` if present (`frzng.for:132-163`), starts at the bottom
  or top-freeze front depending on `frzflg` (`frzng.for:166-189`), refreezes
  `nwfrzz` by calling `frznw` before moving a front (`frzng.for:193-220` and
  `:245-263`), recomputes surface resistance as the front moves
  (`frzng.for:287-342`), computes migration-water heat `qwet`
  (`frzng.for:381-437`), and spends remaining energy on `slfsd`/`slsic`
  updates (`frzng.for:458-630`).
- `frznw(lyn,flyn,engtm,frztm,hour)` freezes liquid water already in the
  frozen zone. It partitions `nwfrzz` over `frozen`, limits by capacity
  `ul/dg*slfsd - slsic`, adds frozen water to `slsic`, and reduces
  `nwfrzz`/`frozen` (`frznw.for:123-164`). This must run before ordinary
  frost-front extension so infiltrated frozen-zone water is not double
  counted or left liquid.
- `mltbtm(hour)` starts at the bottom frost front (`mltbtm.for:128-159`),
  moves upward through fine layers, sets `fgthwd` on thaw-through
  (`mltbtm.for:176-188`, `:197-212`, `:491-511`), computes bottom thaw
  energy from `qdry + qhtout` for arm 4 or `qdry` for mixed arms
  (`mltbtm.for:358-366`), melts `slsic` back into `slsw` while reducing
  `slfsd` (`mltbtm.for:384-447`), and releases `nwfrzz`
  (`mltbtm.for:530-561`).
- `mlttp(hour)` starts at surface thaw or the current `thdp`
  (`mlttp.for:112-125`), computes surface-to-front resistance through
  residue and thawed fine layers (`mlttp.for:187-247`), advances the top thaw
  front downward (`mlttp.for:283-356`), melts `slsic` into `slsw` or
  `watpdg` (`mlttp.for:409-528`), and releases `nwfrzz`
  (`mlttp.for:548-594`).
- `watdst(qwater, flxtim, fgfzft)` implements Saxton-Rawls redistribution
  under frost. Its flags are defined in source as `0` no frozen front, `1`
  around frozen front, `2` unfrozen layers with a frozen front in the profile
  (`watdst.for:20-25`). Mode `1` is used inside `frzng` for migration-water
  redistribution around the moving front (`frzng.for:624-630`) and returns
  when it reaches another frozen layer (`watdst.for:285-290`). Mode `2`
  subtracts per-layer `sltime` for the first frozen-to-unfrozen boundary
  (`watdst.for:173-178`). Modes `0` and `2` then scan all fine layers and
  recompute `frdp`, `thdp`, `tfrdp`, `tthawd`, `frsttk`, and `fcycle`
  (`watdst.for:300-530`).

### 3. openWEPP state-shape proposal

Static: legacy and current openWEPP source read.

Current Rust carries a coarse `FrostLayerWaterState` with `frozen_depth_m` and
`frzw_m` per WEPP layer (`coupling.rs:4-15`) and applies a target aggregate
`frdp_m` into those coarse layers after the hourly heat loop
(`coupling.rs:59-147`, `:966-973`). That was sufficient for D2 publication
but is the D3 anti-pattern: the fine-layer routines must mutate mass and depth
during the hourly branch, then derive aggregate depth/publication state.

Proposed Rust internal state:

```text
FrostFineLayerState {
  layer_index: usize,
  fine_index: usize,
  fine_thickness_m: f64,
  fgfrst: FrostFineLayerFlag,   // 0/1/2/3 legacy values
  slfsd_m: f64,
  slsic_m: f64,
  slsw_theta: f64,
  sltime_s: f64,                // hour-local
}

FrostLayerExchangeState {
  layer_index: usize,
  nfine: usize,
  dg_m: f64,
  thetdr: f64,
  ul_m: f64,
  st_m: f64,
  yst_m: f64,
  nwfrzz_m: f64,
  frozen_m: f64,                // derived at frwatc(0)
  frzw_m: f64,                  // derived at frwatc(0)
  soilf_m: f64,                 // derived publication aggregate
}
```

Flattened runtime symbols should follow the existing scalar-boundary style:

| Proposed symbol | Legacy alias | Persistence | Notes |
|---|---|---|---|
| `frost.runtime_fgfrst_LLLL_FFFF` | `fgfrst(j,i,iplane)` | persistent while frost exists | Integer flag encoded as scalar with domain `[0,3]`. |
| `frost.runtime_slfsd_m_LLLL_FFFF` | `slfsd(j,i,iplane)` | persistent | Fine-layer frozen thickness. |
| `frost.runtime_slsic_m_LLLL_FFFF` | `slsic(j,i,iplane)` | persistent | Fine-layer ice store; primary frozen-water mass. |
| `frost.runtime_slsw_theta_LLLL_FFFF` | `slsw(j,i,iplane)` | persistent | Volumetric liquid water for unfrozen fine-layer part. |
| `frost.runtime_sltime_s_LLLL_FFFF` | `sltime(j,i,iplane)` | hour-local diagnostic | Reset before each hourly branch; not carried as day state. |
| `frost.runtime_nfine_LLLL` | `nfine(i)` | deterministic topology | Existing `frost.runtime_nfine_####` root can survive (`03_kernel_support_00_support_helpers.rs:293-295`). |
| `frost.runtime_fine_thickness_m_LLLL` | `dg(i)/nfine(i)` | deterministic topology | Existing root can survive. |
| `frost.runtime_nwfrzz_m_LLLL` | `nwfrzz(i,iplane)` | persistent | Required for `frznw` and thaw release. |
| `frost.runtime_yst_m_LLLL` | `yst(i,iplane)` | persistent | Required for `frwatc(1)` delta `st - yst`. |
| `wb18_perc_frozen_depth_####` | `frozen(i,iplane)` | derived aggregate | Survives as derived aggregate written after `frwatc(0)`, not as the authoritative fine-state source. |
| `wb18_perc_frzw_####` | `frzw(i,iplane)` | derived aggregate | Survives for `SC-WATBAL-001` v152 `soilf` lineage. |
| `wb18_perc_theta_####` / `wb11_soil_water` | `soilw(i,iplane)` / `watcon` | derived liquid aggregate | `frwatc(0)` must recompute from fine unfrozen water and `nwfrzz`. |
| `frost.runtime_soilf_m_####` | `soilf(i,iplane)` | derived publication aggregate | Optional diagnostic. WAT `frozwt` can continue to consume the existing aggregate diagnostic. |
| `frost.runtime_frdp_m`, `frost.runtime_thdp_m`, `frost.runtime_tfrdp_m`, `frost.runtime_tthawd_m` | `frdp`, `thdp`, `tfrdp`, `tthawd` | derived summary | Recomputed from the fine-layer scan equivalent to `watdst.for:300-511`. |
| `frost.runtime_fgthwd_flag` | `fgthwd` | persistent/current day | Existing symbol survives (`03_kernel_support_00_support_helpers.rs:288-295`). |
| `frost.hourly.qsrf_w_m2_####`, `frost.hourly.quf_w_m2_####`, `frost.hourly.ksrf_w_m_k_####` | `qhtout`, `qdry`, `ktopf` family | hourly diagnostic | Existing roots survive (`03_kernel_support_00_support_helpers.rs:281-287`). Add `frost.hourly.frzflg_####` if branch audit needs direct evidence. |

Implementation ownership should stay in
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
and write through
`hydrology_phase_runoff_reconciliation.rs`, where current frost diagnostics and
layer aggregates are emitted (`hydrology_phase_runoff_reconciliation.rs:768-935`).
The runner WAT surface should remain unchanged except for consuming the now
fine-derived `frost.runtime_frwatc_frozen_water_after_m`
(`02_output_and_climate_helpers.rs:818-841`).

Alias table to carry into `SC-SNOWFREEZE-001` during the implementation pass:

| Legacy | openWEPP alias | Authority rule |
|---|---|---|
| `slsic` | `frost.runtime_slsic_m_LLLL_FFFF` -> aggregate `wb18_perc_frzw_####` -> `frost.runtime_frwatc_frozen_water_after_m` | Accumulated by freeze/thaw energy, never derived from `frdp * theta`. |
| `slfsd`/`fgfrst` | `frost.runtime_slfsd_m_*` / `frost.runtime_fgfrst_*` -> aggregate `frost.runtime_frdp_m` | Depth derives from deepest active frozen fine layer. |
| `soilw` | `wb18_perc_theta_####` + `wb11_soil_water` | Unfrozen liquid store; excludes `soilf`/`frozwt`. |
| `soilf` | `frost.runtime_frwatc_frozen_water_after_m` -> WAT `frozwt` | `soilf = frzw + thetdr*frozen`; additive to `Total-Soil`. |
| `frzflg` | proposed `frost.hourly.frzflg_####` | Branch audit diagnostic for `INV-SNOWFREEZE-012`. |

### 4. Seam mapping — `frwatc(1)` / `frwatc(0)` onto existing openWEPP seams

Static: legacy and current openWEPP source read.

`frwatc(1)` maps water-balance state into fine frost state:

- Source direction: `wbtofs=1` means "from water balance to frost routine"
  (`frwatc.for:139-140`).
- Zero available water path: if `st(i) == 0`, legacy clears `nwfrzz(i)` and
  sets every fine-layer `slsw` to `thetdr` (`frwatc.for:149-154`).
- Delta path: legacy computes `varwat = st(i) - yst(i)`
  (`frwatc.for:158-160`). Negative deltas drain `nwfrzz` first
  (`frwatc.for:163-173`), then drain unfrozen fine-layer liquid toward
  `saxfc`/residual (`frwatc.for:175-193`, `:231-259`). Positive deltas first
  allocate capacity into `nwfrzz` when the coarse layer is frozen
  (`frwatc.for:196-220`), then distribute remaining water over unfrozen fine
  thickness (`frwatc.for:223-259`).
- openWEPP mapping: this requires per-layer `st_m`, `yst_m`, `nwfrzz_m`, and
  fine `slsw`/`slfsd`, not just `wb11_soil_water`. Current layer inputs
  `wb18_perc_theta_####`, `wb18_perc_ul_####`, `thetdr_####`, `dg_####`, and
  the existing fine topology roots can seed it (`coupling.rs:530-611`), but
  the implementation must add the missing persistent `yst` and `nwfrzz`
  surfaces or derive them from an explicitly equivalent layer-state store.

`frwatc(0)` maps fine frost state back to water-balance state:

- Source direction: `wbtofs=0` means "from frost to water balance routines"
  (`frwatc.for:89-90`).
- Legacy sums fine state per coarse layer: `sumfzd += slfsd`, `sumice +=
  slsic`, `sumsw += slsw * unfrozen_depth`, and `sumst += (slsw - thetdr) *
  unfrozen_depth` (`frwatc.for:95-123`).
- It then writes `frozen = sumfzd`, `frzw = sumice - thetdr*frozen`,
  `st = sumst + nwfrzz`, `soilw = sumsw + nwfrzz`, `yst = st`, and clears
  `nwfrzz` when `frozen < 0.001` (`frwatc.for:127-134`).
- openWEPP mapping: after the fine-layer branch, write the existing
  `wb18_perc_frozen_depth_####` and `wb18_perc_frzw_####` from these
  aggregates, write `wb18_perc_theta_####`/`wb11_soil_water` from the
  recomputed unfrozen liquid lineage, and write `frost.runtime_frwatc_*`
  diagnostics from before/after sums. The current writeback sites already
  cover these surfaces (`hydrology_phase_runoff_reconciliation.rs:768-935`);
  their producer must be re-bound from coarse target projection to the exact
  fine-state aggregation.

Publication and closure:

- Legacy daily water balance builds `soilf = frzw + thetdr*frozen` only when
  `frdp > 0.001`, else zero (`watbal.for:1026-1038`; hourly analog
  `watbal_hourly.for:1085-1098`), and `watbalprint` publishes
  `watcon = Σ soilw`, `frozwt = Σ soilf` (`watbalprint.for:56-69`).
- Current openWEPP WAT publication already consumes
  `frost.runtime_frwatc_frozen_water_after_m` for `frozwt` and
  `frost.runtime_frdp_m` for `frdp`
  (`02_output_and_climate_helpers.rs:818-841`). Keep that surface.
- `SC-WATBAL-001` v152 remains the gate: `Total-Soil`/`SoilWaterTotal`
  excludes frozen water; annual frost-active storage uses
  `Total-Soil + frozwt`. Years 2-6 closure at noise must survive every
  increment, and the year-7 boundary residual remains a D3 watch item.

Survive / rebind / retire:

| Category | Symbols |
|---|---|
| Survive unchanged at the boundary | `frost.runtime_frwatc_soil_water_before_m`, `frost.runtime_frwatc_soil_water_after_m`, `frost.runtime_frwatc_frozen_water_before_m`, `frost.runtime_frwatc_frozen_water_after_m`, freeze/thaw debit/credit/net delta, `frost.runtime_frdp_m`, WAT `frozwt`, WAT `frdp`. |
| Survive as derived aggregates | `wb18_perc_frozen_depth_####`, `wb18_perc_frzw_####`, `frost.runtime_ws_frz`, `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nfine_####`, `frost.runtime_fine_thickness_m_####`. |
| Add as authoritative fine state | `frost.runtime_fgfrst_*`, `frost.runtime_slfsd_m_*`, `frost.runtime_slsic_m_*`, `frost.runtime_slsw_theta_*`, `frost.runtime_nwfrzz_m_####`, `frost.runtime_yst_m_####`. |
| Retire as authority | `apply_layered_frost_target`-style target-depth projection from aggregate `frdp_m` into layer water (`coupling.rs:59-147`, `:966-973`). It may remain only as a deleted/backed-out reference in package evidence, not production physics. |

### 5. Tests to author first (red, before implementation)

Static: contract/test-surface scoping only; no tests authored in this phase.

Author these as red tests before production code. The current D2 tests in
`tests/integration/clim06_frost_frozen_soil_kernel_contract.rs` already cover
coarse layered-store publication and exchange diagnostics; D3 needs tests that
fail because the fine-state symbols and routine sequencing do not exist yet.

| Test name | Expected pre-port failure | Pass criterion |
|---|---|---|
| `fdhp01_fine_sublayer_frwatc_round_trip_conserves_mass` | Missing `frost.runtime_slsic_*`/`slsw_*`/`yst_*`/`nwfrzz_*` surfaces or coarse-only round trip. | Seed a 2-layer profile with mixed frozen/unfrozen fine layers; run `frwatc(1)` then `frwatc(0)` with no freeze/thaw energy; recover `soilw`, `st`, `frzw`, `frozen`, and `soilf` to <= `1e-12 m`. |
| `fdhp01_frostn_dispatch_arms_match_inv_snowfreeze_012` | No direct branch diagnostic and current simplified loop cannot prove arm 2. | Force four signed-flux states and assert arm calls: `1` -> `frzng/watdst(2)`, `2` -> `frzng/optional mltbtm/watdst(2)`, `3` -> `mlttp/optional mltbtm/watdst(0)`, `4` -> `mltbtm/watdst(0)`. |
| `fdhp01_fine_sublayer_freeze_front_steps_by_energy_and_resistance` | Aggregate `frdp` changes before fine `slsic`/`slfsd` and coarse projection permits dry outliers. | Under fixed cold forcing, front advances fine layer by fine layer; each increment consumes `lhfh2o * slsw * dz`; increasing frozen `Σ dz/k` slows later hourly advance. |
| `fdhp01_fine_sublayer_frznw_refreezes_nwfrzz_once` | `nwfrzz` is absent or thaw/freeze exchange double counts. | Seed liquid in a frozen zone above a front; `frznw` reduces `nwfrzz`, increases `slsic`, respects capacity, and leaves total `soilw + soilf` unchanged to noise. |
| `fdhp01_fine_sublayer_mltbtm_bottom_thaw_recedes_front` | Warm lower flux only thaws aggregate depth or fails to release fine ice. | Positive `qdry` retreats bottom fine-layer flags/depth, converts `slsic` to `slsw`/`watbtm`, decreases `frdp`, and preserves mass. |
| `fdhp01_fine_sublayer_mlttp_top_thaw_sets_sandwich_and_fgthwd` | Top thaw lacks sandwich flags or early exit. | Positive surface heat creates `fgfrst=3` top-thaw geometry over remaining frost; complete thaw sets `fgthwd=1` and clears `frdp`/`tfrdp`/`tthawd`. |
| `fdhp01_watdst_mode_flags_update_depths_and_sltime` | Current code has no `fgfzft` semantics. | Mode `1` stops around the moving front, mode `2` subtracts `sltime`, mode `0` performs no frozen-front special case, and modes `0`/`2` recompute global depths from fine flags. |
| `fdhp01_fine_sublayer_multiday_additive_closure_preserved` | Fine state not present or additive identity regresses. | Multi-day freeze/thaw profile keeps `Total-Soil + frozwt` closed to years-2-6 noise while `SoilWaterTotal` remains unfrozen-only. |
| `fdhp01_d3_cohort_gate_depth_duration_and_year7_boundary` | Existing D3 hold reproduces profile-bound pinning and duration under-persistence. | 43/43 single-OFE cohort clean; depth no longer pins at profile bound; correlation rises from the FDMC01/Addendum 3 floor; duration delta collapses from -518/-428 days toward zero; year-7 residual is explained or removed. |

### 6. Sizing and phase recommendation

Static: scope recommendation.

This is a substantial physics transplant, not a cleanup patch. The required
implementation spans:

- one new fine-layer state shape and symbol family;
- faithful `frwatc(1)` and `frwatc(0)` handoffs;
- `watdst` redistribution plus derived depth recomputation;
- `frzng` + `frznw` freezing;
- `mlttp` + `mltbtm` thawing;
- existing WAT/conservation publication preservation; and
- the 43-prefix D3 cohort gate.

Recommended implementation sequencing inside the next pass:

1. Amend `SC-SNOWFREEZE-001` alias/binding text for the fine-state symbols and
   add the red tests above.
2. Add typed fine-layer state helpers and topology/locate helpers with no
   comparator tuning.
3. Implement `frwatc(1)`/`frwatc(0)` and keep D2 years-2-6 closure at noise.
4. Implement `watdst` mode semantics and depth derivation from fine flags.
5. Implement `frzng`/`frznw`, then `mlttp`/`mltbtm`.
6. Run focused contract tests, full package Rust gates, and the single-OFE
   cohort with D2 additive closure plus D3 depth/duration metrics.

Legitimate external phase line: only a landed phase that closes the
depth+duration gap may advance the package. A partial helper/state commit is
acceptable as an implementation checkpoint only if it is either non-production
or the package remains explicit `executed-hold`; it must not be presented as a
D3 closure while depth still pins or duration remains severely under-persistent.
The next implementation pass should expect multiple internal increments, but
the acceptance boundary is still one D3 outcome: cap-free depth in the
heat-flow envelope, duration residual materially collapsed, years-2-6
conservation at noise, and the year-7 boundary item classified.

## Hard boundaries (this phase and the implementation pass it scopes)

- **No coarse scalar or coarse per-layer substitute.** v57 + two recorded
  failures (`efd2152b`, addendum 2e) close that route. A third
  simplification attempt is grind, not scoping.
- **Conservation is non-negotiable:** years 2–6 additive-identity closure
  at ≤ ~3e-11 mm must survive every increment; the year-7 boundary watch
  item (addendum 3) must be explained or eliminated by the port.
- **No comparator-match tuning** (ADR-0017): legacy depth/duration is the
  envelope flag; the heat-flow contract is the authority.
- FQ-4 activation gate, kfactor magnitude, forest `ksatadj`, MOFE remain
  protected per `package.md`.

## Acceptance for this scoping phase

Sections 1–6 filled with source-traced content (`Static:` labels, file:line
citations against the pinned baseline); the legacy↔openWEPP alias table
present; the test list concrete enough to write red tests directly; sizing
recommendation stated. No production code edits. On completion, this
artifact becomes required-reading item 1 for the implementation pass.

## Addendum C1a — seam accounting specification (2026-06-11)

Evidence: Ran + Static. Diagnostic production edits: none retained.

Increment C1a ran a temporary env-gated ledger on the current B-boundary source
without the comparator subagent. The targeted p43 and p1 runs both stopped at
`HKERNEL-WB14-RUNOFF-E-003` on simulation day 94 (`1990-04-04`). The first
illegal write is the previous runoff-reconciliation frost pass on day 93
(`1990-04-03`):

| Prefix | Day-93 aggregate `frzw` | Aggregate `ul` | `frzw - ul` excess | Fine `slsic` capacity excess |
|---|---:|---:|---:|---:|
| p43 | `50.58972525883585 m` | `0.543517677999698 m` | `50.049070656902806 m` | `50.324510845402415 m` |
| p1 | `51.18301848887181 m` | `0.543517677999698 m` | `50.644102740198335 m` | `50.91552839450165 m` |

The first fine-layer capacity excess was already present on day 1:
`0.041949772970434 m` for p43 and `0.042174177930601 m` for p1. The largest
observed shadow `frwatc(1)` residual before the day-93 re-freeze was
`33.4009943366675 m` for p43 and `33.79382883453257 m` for p1. The archived C1
p43 aggregate-cap smoke still matters: it collapsed published storage to
`ProfilePorosityCap`, but annual closure still missed by up to
`200.39845415539014 mm`, so an aggregate clamp after the handoff is not a valid
accounting repair.

Static attribution:

- `apply_shadow_frwatc_ingress` applies `st - yst` into fine state
  (`coupling.rs:301`).
- `aggregate_shadow_layer` recomputes the shadow coarse state and sets `yst`
  (`coupling.rs:361`).
- The read side correctly rejects `frzw > ul` (`coupling.rs:1228`).
- `freeze_fine_front` adds `slsw_theta * dz` to `slsic` without the legacy
  remaining-capacity bound `ul/dg * slfsd - slsic` (`coupling.rs:672`).
- Runoff writeback writes aggregate `frzw` and the fine shadow arrays as
  separate surfaces (`hydrology_phase_runoff_reconciliation.rs:949` and
  `:1006`).

Accounting specification for C1b:

1. **Single daily ingress.** Apply `st - yst` once at the day ingress point
   corresponding to legacy `frwatc(1)`. Non-owning diagnostic calls must not
   re-apply the handoff.
2. **Fine-state ownership.** From ingress until egress, fine state owns liquid
   and ice. Coarse `theta`/`frzw`/`frozen_depth` are derived outputs, not
   independently mutable stores.
3. **Capacity-bound freeze.** `frzng`/`frznw` moves liquid to ice only up to
   `ul/dg * slfsd - slsic` for each fine layer, debiting `slsw` or `nwfrzz`
   in the same ledger entry and carrying unused energy forward without creating
   ice beyond capacity.
4. **Thaw and redistribution double-entry.** `mlttp`, `mltbtm`, and `watdst`
   convert `slsic` to fine-layer liquid, redistribute within capacity, and
   route any overflow to named `watpdg`/`watbtm` surfaces. Those surfaces must
   enter the WAT identity explicitly; they cannot disappear outside
   `Total-Soil + frozwt`.
5. **Wholesale egress.** The egress corresponding to legacy `frwatc(0)`
   recomputes coarse `theta`, `frzw`, `frozen_depth`, `soil_water`, `st`,
   `yst`, and `nwfrzz` from the fine state. It must not patch a scalar delta
   on top of a separately mutated coarse pool.
6. **Writeback guards.** Egress writebacks must enforce `frzw <= ul` and
   `slsic <= ul/dg * slfsd`; any valid overflow path must be named and balanced
   before the read-side guard sees the next day.

C1b acceptance starts by passing the day-94 p43 and p1 boundary with zero
aggregate `frzw > ul` rows and shadow `frwatc` residuals at numerical noise.
Only then should the full years-2-6 additive-identity cohort gate be re-run.
