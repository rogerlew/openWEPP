# Hydrology and Erosion Kernel Stub-Audit — 2026-05-25

Status: Draft
Last updated: 2026-05-25
Evidence mode: Static (source inspection only; no kernel execution, no test run, no line-by-line diff against legacy Fortran)
Scope: All production hydrology and erosion kernels in [/workdir/openWEPP/crates/](../../crates/) plus their cross-reference to the SC contracts and the `wepp-forest_260430_baseline` provenance anchors.

## 1. Purpose

Confirm that the hydrology and erosion kernels shipping in openWEPP carry real physics rather than stubbed bodies, and that each kernel's algorithmic shape corresponds to the routines pinned by its SC contract to [/workdir/wepp-forest_260430_baseline/src/](../../../wepp-forest_260430_baseline/src/) under the contract-recorded hash `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## 2. Method

- Enumerated production kernel `impl` blocks via `grep -rn "impl HillslopeKernel\|impl WatershedKernel"`.
- Read entry-function bodies for every dispatch arm in [Wb11HydrologyKernel](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs) and [Ws10ChannelImpoundmentKernel](../../crates/openwepp-watershed-orchestrator/src/lib.rs).
- Scanned all source files under [crates/](../../crates/) (excluding `target/` and `.worktrees/`) for `todo!`, `unimplemented!`, `stub`, `placeholder`, `FIXME`, `TODO`.
- Cross-referenced each kernel against the corresponding SC contract in [docs/specifications/science-contracts/contracts/](../specifications/science-contracts/contracts/) for the `REF-*-LEGACY-*` baseline anchor.
- Did **not** execute `cargo test`, `cargo check`, or any kernel; did **not** line-by-line diff Rust against Fortran.

## 3. Inventory of production kernels

| Kernel | Location | Phase coverage |
|---|---|---|
| `Wb11HydrologyKernel` | [crates/openwepp-hillslope-orchestrator/src/hydrology.rs:742](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L742) | ET, percolation, lateral transfer, drainage, runoff reconciliation, storage reconciliation, peak runoff (peak runoff dispatches into Erod13 and Erod14) |
| `Ws10ChannelImpoundmentKernel` | [crates/openwepp-watershed-orchestrator/src/lib.rs:255](../../crates/openwepp-watershed-orchestrator/src/lib.rs#L255) | Channel routing, impoundment stage-area-discharge |

No other production simulation kernels exist in the tree. [openwepp-kernel-contract](../../crates/openwepp-kernel-contract/src/lib.rs) is interface-only (request types, phase classes, writeback payload, closure-check helpers); no physics lives there.

## 4. Stub scan result

- No `todo!()`, `unimplemented!()`, or empty-body kernel arms in production hydrology or erosion code. (`grep` for those tokens matches only test files.)
- The only `placeholder` references in production code are in [crates/openwepp-watershed-output/src/writers.rs:32](../../crates/openwepp-watershed-output/src/writers.rs#L32), where the writer **refuses** to emit empty placeholder parquet via `OWSOUT-E-004`. That is an anti-stub guard, not a stub.
- The fallthrough `_ =>` arm in [run_hillslope_phase at hydrology.rs:6172](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L6172) emits `HKERNEL-WB11-NOP-001` only for phase classes this kernel does not own (growth, decomposition, snow, frost, infiltration-hyetograph). Those classes are routed to other dispatchers by the scheduler; the NOP arm is by-class, not physics stubbed out.

## 5. Per-kernel physics summary

### 5.1 `Wb11HydrologyKernel::run_evapotranspiration` ([L3146](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L3146))

- Beer's-law LAI partition: `Es_potential = ET_demand · exp(-k·LAI)`, `Ep_potential = ET_demand - Es_potential`.
- Residue evaporation cap: `Er = min(residue_interception, Es_potential)`.
- Ritchie two-stage soil-evaporation with `S1/S2/TU/TV` state surface, both stage-1 (`S1 < TU`) and stage-2 (`S1 >= TU`) branches with infiltration coupling and surface-deficit transitions.
- Cascading actual evapotranspiration: `Es_actual = min(soil_water, Es_demand)`, then `Ep_actual = min(soil_after_Es, Ep_potential)`, then `ET = Er + Es_actual + Ep_actual`.
- Water-stress ratio: `ws = Ui / ETp` with `ws = 1` when `ETp ≤ 0`.

Contract anchor: [SC-EVAP-001.md:66-68](../specifications/science-contracts/contracts/SC-EVAP-001.md#L66-L68) — `evap.for:458-564` (stage memory), `evap.for:609-668` (layer extraction), `swu.for:122-191` (root-zone uptake).

### 5.2 `Wb11HydrologyKernel::run_percolation` ([L3511](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L3511))

- Per-layer PURK cascade with explicit `nsl` layer enumeration.
- Saturation ratio: `stz = θ / UL`.
- Shape function: `fx = max(stz^exponent, fx_min)` when `stz < saturation_threshold`, else `fx = 1`.
- Adjusted conductivity: `Ks_adj = ssc · fx`.
- Drainage step: `pei_pre = min(Δt · Ks_adj, excess)` capped by lower-layer saturation factor `sqrt(1 − θ_below/UL_below)` for all but the bottom layer.
- Bottom-up routing order (inline comment cites *"legacy WEPP percolation ordering in PURK"* at line 3632).
- Bottom-layer `pei` is emitted as `percolation_loss` (deep seepage).

Contract anchor: SC-PERC-001 family; baseline `purk.for`.

### 5.3 `Wb11HydrologyKernel::run_lateral_transfer` ([L3790](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L3790))

- Saturated-zone identification by `θ + ε ≥ θ_FC` per layer.
- Saturated-zone-weighted conductivity: `Ke = 86400 · Σ(ssc·dg) / Σ(dg)` (m/day units).
- Slope factor: `sin(atan(avgslp))`.
- Darcy-style lateral flux: `q = (sat_thickness · anisotropy · Ke · slope_factor) / slplen`.
- Available pool reconciled against drainable storage and recharge `Pe`.
- Top-down layer withdrawal via `wb19_withdraw_top_down`.

Contract anchor: [SC-SUBHYD-001.md:59](../specifications/science-contracts/contracts/SC-SUBHYD-001.md#L59) — Eq. 6.2.4 lateral-flow Darcy form; [L182](../specifications/science-contracts/contracts/SC-SUBHYD-001.md#L182) `INV-SUBHYD-003`.

### 5.4 `Wb11HydrologyKernel::run_drainage` ([L3956](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L3956))

- Drain-enabled gate keyed off `WB19_SYMBOL_DRAIN_ENABLED`.
- Water-table depth derived bottom-up through saturated layers (`watbl`, `dep2watbl`).
- Tile-layer enumeration by cumulative `dg` until drain depth.
- Hooghoudt equivalent-depth `de`:
  - Spacing ratio `≤ 0.3`: radius-ratio formulation `1 + (drain_depth/r)·…`.
  - Otherwise: `de = drain_depth / (ln(S/r) − 1.15)` correction.
- Drainage flux: `q = 8 · Ke · de · h / S²` (cm/h → consistent unit conversion).
- Saturated-zone-weighted `dranks` conductivity for `Ke`.
- Withdrawal sequencing via `wb19_withdraw_tile_to_surface`.

Contract anchor: [SC-SUBHYD-001.md:61-63](../specifications/science-contracts/contracts/SC-SUBHYD-001.md#L61-L63) — Eq. 6.2.10-6.2.13 drainage-flux + equivalent-depth + anisotropy semantics; [L184](../specifications/science-contracts/contracts/SC-SUBHYD-001.md#L184) `INV-SUBHYD-006`.

### 5.5 `Wb11HydrologyKernel::run_runoff_reconciliation` ([L4278](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L4278))

- Hyetograph integration with point-count guard and per-interval `rate · duration` accumulation.
- Snow and frost coupling outcomes consumed (`compute_active_snow_coupling`, `compute_active_frost_coupling`).
- Irrigation coupling (depletion vs fixed-date schedule sources) added to liquid input.
- Canopy interception subtracted before infiltration (`compute_canopy_interception_depth`).
- Green-Ampt-style coupled infiltration: [compute_coupled_infiltration_depth at L2697](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2697) drives per-interval Green-Ampt with [solve_ponded_cumulative_infiltration at L2891](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2891) for ponded-regime Newton solution.
- Runoff equation: `q_runoff = liquid + runon − F_cum − ΔDS`.
- Forward-solver-lane mode runs an observed-side closure delta against the kernel-produced `q_runoff` and gates by `closure_tolerance`.

Contract anchors: SC-WATBAL-001, SC-INFILT-HYET (per WB14).

### 5.6 `Wb11HydrologyKernel::run_storage_reconciliation` ([L4689](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L4689))

- Daily soil-water-balance closure: `S' = S + P + S_snow + I − R − ET − Pe − L_sub`.
- Forward-solver-lane mode runs the same equation in observed-side form and compares against the kernel emission within `closure_tolerance`.
- Storage state writeback through `compute_storage_reconciled_with_interception`.

Contract anchor: [SC-WATBAL-001.md](../specifications/science-contracts/contracts/SC-WATBAL-001.md).

### 5.7 `Wb11HydrologyKernel::run_peak_runoff` ([L5727](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L5727))

- Effective duration `effdrr` from hyetograph time-span.
- Mean runoff rate `vave = Q / effdrr`.
- Maximum rainfall intensity `remax` from hyetograph peak.
- Normalised excess intensity `vstar = vave / remax`.
- KINEROS kinematic-wave time-to-equilibrium: `te = (efflen / (α · vave^(m-1)))^(1/m)`, `tstar = te / effdrr`.
- Two-branch peak shape function `qpstar(tstar, vstar)` for `tstar ≥ 1` vs `tstar < 1`.
- Peak discharge: `peakro = vave · qpstar`, floored by `WB16_PEAKRO_FLOOR`.
- Event duration: `watdur = min(Q / peakro, max_duration)`.
- Sequenced dispatch into `run_erod13_wave1_core(...)` then `run_erod14_wave2(...)` with the same request.

Contract anchor: SC-RUNOFFPART-001; baseline `pkflow.for`-family routines.

### 5.8 `run_erod13_wave1_core` ([L4844](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L4844))

Foster-Meyer single-OFE rill detachment/deposition physics:
- Effective rill shear: `τf = τfe · (fs/ft)`.
- Rill erodibility composite: `η = (cntlen · Kr · kradjf · shrsol) / tcend`.
- Critical shear adjusted: `τcn = (tcadjf · shcrit) / shrsol`.
- Theta and phi composites: `θ = (cntlen · detinr / tcend) · (effdrr / effdrn)`, `φ = (β · veleff) / pkro`.
- Transport capacity: `tc = tcadjf · k · τf^m`.
- Detachment branch (`τf > τcn` and `g < tc`): `Dc = η(τf − τcn)`, `Df = Dc · (tc − g)/tc`.
- Deposition branch (`g > tc`): `Df = −(β · vf / q)(g − tc)`.
- Sediment-load continuity check: `dgdx ≈ Df + Di` within `EROD13_CONTINUITY_TOLERANCE`.
- Independent continuity check on `watdur` against `Q / peakro`.

Contract anchor: SC-SED-001; baseline `eros.for` / `param.for` rill family.

### 5.9 `run_erod14_wave2` ([L5085](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L5085))

Multi-OFE particle-size-class enrichment, gated by `EROD14_SYMBOL_CLASS_COUNT`:
- `case_number` (1-4) validated against the four canonical state arrangements of `qj_minus_1`, `vj`, `qj`, `fh - fp` with `EROD14_CASE_TOLERANCE`.
- Per-class arrays: `fall`, `frcflw`, `fidel`, `tcf1`, `ssa_class`, `ftheta`, `gu`, plus `gend` and `sedmax` accumulators.
- `theta` consumed from Erod13's writeback when present, otherwise from the state surface.
- Enrichment ratio `er` and class-area `ssa_soil` propagated as writebacks.

Contract anchor: SC-SED-001 (multi-OFE/enrichment extensions); work package [docs/work-packages/20260525-erod14-multiofe-and-enrichment-kernel-001](../work-packages/20260525-erod14-multiofe-and-enrichment-kernel-001/).

### 5.10 `Ws10ChannelImpoundmentKernel::run_channel_node` ([L652](../../crates/openwepp-watershed-orchestrator/src/lib.rs#L652))

- Routing-gain ratio: `(1 + control_slope) / (1 + roughness)`.
- Baseflow term: `cbase · nchnum · (1 + conductivity · dtchr)`.
- Four `ipeak` branches, each fully implemented:
  - Rational (constant peak).
  - CREAMS: `√(routing_gain / (1 + conductivity·dtchr))` attenuation.
  - Kinematic Wave: `1 + roughness·dtchr + conductivity·nchnum` storage factor.
  - Muskingum-Cunge: separate translation `1 + conductivity·dtchr` and storage `1 + roughness·dtchr + slope·nchnum` factors.
- Channel runoff volume `roff = qpo · durrof`; event duration `durrof = available_peak · event_duration / qpo`.

Contract anchor: [SC-HYDRAULICS-001.md:71](../specifications/science-contracts/contracts/SC-HYDRAULICS-001.md#L71) — `wshdrv.for`, `wshpek.for`, `wshchr.for` ipeak-branch authority.

### 5.11 `Ws10ChannelImpoundmentKernel::run_impoundment_node` ([L879](../../crates/openwepp-watershed-orchestrator/src/lib.rs#L879))

- Stage-area relation: `A = a0 + a1 · H^a2`.
- Composite outflow:
  - Drop spillway above `ha`.
  - Culvert above `ht`.
  - Riser above `hlm`.
- Seepage loss `qinf` consumed from state surface.
- Continuity outflow: `Qo + qinf`.

Contract anchor: [SC-IMPOUND-001.md:278-282](../specifications/science-contracts/contracts/SC-IMPOUND-001.md#L278-L282) — `imphnw.for:75-143`, `impflo.for:94-175`, `wshiqi.for:74-179`, `wshimp.for:207-218`.

## 6. Contract-to-baseline anchoring

The wepp-forest_260430_baseline routines are pinned **in the SC contract layer**, not (predominantly) in source comments. Each relevant contract under [docs/specifications/science-contracts/contracts/](../specifications/science-contracts/contracts/) carries an Authority Anchors table with `REF-*-LEGACY-*` rows pointing at specific [/workdir/wepp-forest_260430_baseline/src/](../../../wepp-forest_260430_baseline/src/) files and line ranges under hash `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, followed by hard-fail `INV-*-NNN` invariants tied to those references.

Source-level provenance comments are sparse:
- [crates/openwepp-climate-runtime-adapter/src/lib.rs:22](../../crates/openwepp-climate-runtime-adapter/src/lib.rs#L22) cites `stmget.for:176-183`.
- [hydrology.rs:3632](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L3632) cites *"legacy WEPP percolation ordering in PURK"* inline.

This is consistent with [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md): the contract is the authority, the legacy binary is a flagging signal, not an oracle.

## 7. Findings

1. **No stubbed physics found.** Every dispatch arm in `Wb11HydrologyKernel` and `Ws10ChannelImpoundmentKernel` contains substantive numerical computation that matches the algorithmic shape of the routines cited by its SC contract.
2. **Guard density is high.** Every input/output passes through `require_state_scalar` / `require_state_range` / `require_flux_range` checks plus continuity assertions (e.g. `dgdx ≈ Df + Di` for Erod13, observed-side balance for runoff/storage reconciliation). A stub silently emitting zeros would be caught by these guards before producing a clean output.
3. **Erosion is wired through peak runoff.** Erod13 and Erod14 execute inside the WB16 peak-runoff dispatch path; their writebacks merge into the hydrology writeback. There is no separate `ErosionKernel` impl, by design.
4. **Watershed output writer is intentionally strict.** `OWSOUT-E-004` rejects empty placeholder parquet — confirms the kernels are expected to emit real data, not stub records.

## 8. Caveats

- **Static review only.** No `cargo test`, no kernel invocation, no parquet comparison.
- **Sampling.** [hydrology.rs](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs) is 9048 lines; [watershed lib.rs](../../crates/openwepp-watershed-orchestrator/src/lib.rs) is ~2000 lines; [kernel-contract lib.rs](../../crates/openwepp-kernel-contract/src/lib.rs) is 1871 lines. Entry function bodies and inner physics blocks were read; helper functions were spot-checked but not exhaustively traced.
- **No Rust-vs-Fortran line-by-line diff.** "Algorithmic shape matches" is the strongest claim this audit supports. Numerical-parity verification against each `REF-*-LEGACY-*` anchor remains a per-WP exercise, not an audit pass.
- **Recency.** Erod14 wave-2 multi-OFE activation landed in `386312c Execute MOFE3 wave2 activation and close package` (2026-05-25). MOFE02 cross-file OFE parity landed shortly before. Some of the erosion physics is freshly merged and still inside its hardening window.

## 9. Recommended follow-ups (not performed in this audit)

- Per-kernel numerical-parity work packages running each Rust kernel against its `REF-*-LEGACY-*` Fortran routine on the WEPP fixture set, validated by the `INV-*` invariants.
- Codifying the contract→source linkage with mandatory `// Provenance:` comments at each kernel entry (only one such comment exists today, at `climate-runtime-adapter/src/lib.rs:22`).
- A targeted closure-violation regression for the forward-solver-lane closure checks in `run_runoff_reconciliation` and `run_storage_reconciliation`.

## 10. Cross-reference against wepp-forest_260430_baseline — gaps and inconsistencies

Static reading of [/workdir/wepp-forest_260430_baseline/src/](../../../wepp-forest_260430_baseline/src/) (233 `.for` files) compared against the openWEPP Rust surface. The baseline routine purpose statements were sampled from the leading comment blocks; the Rust surface was queried by `grep` for canonical concept names.

### 10.1 Method note

A "gap" here means: the baseline routine has no Rust analog with the same algorithmic shape that is reachable from a production kernel. The kernel may consume a state surface representing the routine's output (e.g. ET demand, snow accumulation, frost depth), but the *computation* is absent. This is a structural observation, not a quality claim — many of these gaps are deliberate where the orchestrator currently expects upstream-prepared state.

### 10.2 Missing computations (no Rust analog reachable from a kernel)

| Baseline routine(s) | Purpose | openWEPP status | Evidence |
|---|---|---|---|
| `evappm.for`, `pmetcoef.for` | Penman-Monteith dual-coefficient ET demand (FAO 56) | **Missing.** `et_demand` is computed in [hillslope/mod.rs:1629](../../crates/openwepp-runner/src/hillslope/mod.rs#L1629) as `0.00128·(Rn/58.3)·(Δ/(Δ+γ))` — a Priestley-Taylor-style form, not PM. The `pmetpara` parser at [parsers/pmetpara.rs:75](../../crates/openwepp-input-contract/src/parsers/pmetpara.rs#L75) reads `kcb` but the result is discarded (`let _pmetpara = parse_pmetpara_file(...)` at [hillslope/mod.rs:592](../../crates/openwepp-runner/src/hillslope/mod.rs#L592)). | parser-without-consumer; PM kernel arm absent |
| `snowd.for`, `melt.for`, `sndrft.for`, `mltbtm.for`, `mlttp.for` | Snow drift / hourly energy-balance snow melt (radiation, wind, residue/canopy coupling, hourly meteorology) | **Replaced by reduction.** `compute_active_snow_coupling` at [hydrology.rs:2496](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2496) uses linear-temperature rain/snow partition + degree-day-like melt: `melt_fraction = ΔT/(ΔT+1) · (newsnw/ssd)`. This is a placeholder reduction, not a wepp-forest snow energy balance. | no input radiation, no canopy/residue energy terms |
| `frostn.for`, `frsoil.for`, `frwatc.for`, `frzng.for`, `frznw.for`, `getfreezecond.for`, `winter.for`, `winthd.for` | Fine 10-layer frost solver with energy flow between frozen layers, per-layer `fgfrst(10,mxnsl,mxplan)` state, infiltration impedance | **Replaced by reduction.** `compute_active_frost_coupling` at [hydrology.rs:2330](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2330) sets `dfrost = if tmin <= 0 { FROST_MAX_DEPTH } else { 0 }`, `infcap_frz = ssc · (1 − freeze_fraction · (1 − kfactor_floor))`. This is a binary on/off with linear impedance, not the wepp-forest per-fine-layer frost solver. | no per-layer state; constant `FROST_MAX_DEPTH_M`; no thermal solver |
| `chnero.for` | Channel sediment detachment / deposition (called from WSHDRV per channel element) | **Missing.** `Ws10ChannelImpoundmentKernel::run_channel_node` emits `qpo`/`durrof`/`roff` only — no shear, detachment, transport-capacity, or deposition fields. SC-ROUTE-001 §13.5 Eq. [13.5.13]-[13.5.29] is unimplemented. | grep for `detach`/`deposit`/`sediment` in `watershed-orchestrator/src/lib.rs` returns zero hits |
| `route.for`, `rtpart.for` | Hillslope sediment routing with upper-end detach-or-deposit branch over slope segments (called from CONTIN) | **Partial.** `run_erod14_wave2` covers the case-1-4 routing and per-class transport, but the per-segment upper-end detach-vs-deposit branching of `route.for` (MSHEAR computed-GOTO cases) is not visible in the Rust impl. Needs algorithmic comparison. | function names match; control-flow shape not yet diffed |
| `impflo.for:94-147` adaptive timestep retry, `impflo.for:151-175` regime-transition reset, `imphnw.for:141-143,357-362` RK4 stage update | Adaptive half-step/full-step retry with next-step proposal policy and transition-safe retry | **Missing.** `run_impoundment_node` computes a single-shot stage-area-discharge `A = a0 + a1·H^a2` + drop/culvert/riser thresholds. No `RK4`, no adaptive timestep, no retry — grep for `RK4|runge_kutta|adaptive_timestep|half_step` returns zero hits. SC-IMPOUND-001.md:278-282 cites these as required. | grep negative |
| `furadv.for`, `furgps.for`, `furlea.for`, `furrec.for`, `furrow.for`, `furrun.for`, `irflow.for`, `irinpt.for`, `irprnt.for`, `irrig.for`, `irs.for` | Furrow irrigation kinematic-wave advance / recession; surface irrigation flow | **Missing.** openWEPP irrigation is event-scheduling only (`IRRIG_SYMBOL_DAILY_IRRIGATION` applies a uniform depth via `ActiveIrrigationEvent` at [hydrology.rs:776](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L776)). No kinematic-wave furrow model. | event geometry only |
| `disag.for`, `brkpt.for` | Breakpoint / climate disaggregation from STMGET | **Missing in Rust.** Hyetograph is consumed as a state surface (`resolve_hyetograph_point_count`). If disaggregation happens upstream of openWEPP today, this is a *boundary* gap, not a kernel gap; if it must run inside openWEPP for stand-alone simulation, it is a routine gap. | climate-runtime-adapter is the only climate-side crate; only one provenance line ([climate-runtime-adapter/src/lib.rs:22](../../crates/openwepp-climate-runtime-adapter/src/lib.rs#L22)) cites `stmget.for:176-183` |
| `watbal_hourly.for` | Hourly water-balance variant (Brooks/Dun) | **Missing.** openWEPP is daily. The wepp-forest comment ("needs work, has not been tested with new winter or subsurface code") marks this as experimental — likely an intentional omission, but worth recording. | n/a |
| `kostia.for`, `infile.for`, `infpar.for` (Kostiakov calibration; Green-Ampt parameter file) | Modified Kostiakov infiltration calibration; Green-Ampt parameter assembly | **Partial.** openWEPP has Green-Ampt-style ponded-regime solver at [hydrology.rs:2891](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2891) and `compute_coupled_infiltration_depth` at [L2697](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2697). No Kostiakov path; `infpar.for` parameter calibration not visible. | only Green-Ampt; no alternate solver |
| `psiinv.for`, `psis.for`, `saxfun.for`, `saxpar.for` | Matric potential inversion + Saxton soil-hydraulic functions | **Missing.** Pedotransfer / matric-potential helpers absent. Field capacity / wilting point are consumed as state symbols (`thetfc`, `thetdr`), implying any Saxton conversion happens upstream. | no Rust analog |
| `wshtc.for`, `wshscs.for`, `wshcqi.for`, `wshirs.for` | Watershed transport capacity / SCS channel routing / channel runon assembly / interrill source | **Partially missing.** Ws10 covers `wshpek`/`wshchr`/`wshimp`/`imphnw`-shape and partial `wshiqi`. `wshcqi.for` channel runon assembly (`rvolat`, `rvotop`, `rvolon`) and SC-ROUTE-001 REF-ROUTE-WSHCQI-RUNON are not visibly implemented. `wshscs.for` SCS routing absent. | partial coverage |
| `yalin.for` | Yalin transport-capacity equation via TRCOEF / SHIELD | **Not used.** `run_erod13_wave1_core` uses `tc = tcadjf · tc_k · τf^m` (Foster-Meyer power-law form, SC-SED-001 REF-SED-CH11-TC). Yalin is referenced in baseline but not invoked from the Rust kernel — this is a deliberate choice if the contract specifies the simpler form, but worth flagging. | algorithmic-shape divergence |

**Notes on 10.2:**

- **Note 2026-05-25 (PM row):** wepp-forest dispatches PM only when `pmetpara.txt` is present ([infile.for:1538](../../../wepp-forest_260430_baseline/src/infile.for), [watbal.for:494](../../../wepp-forest_260430_baseline/src/watbal.for)). The fallback `evap.for` is itself a Hargraves/PT/Penman dispatcher keyed by wind-data availability. openWEPP implements only the no-wind PT branch — actual gap is **PM + Penman**.
- **Note 2026-05-25 (snow row):** drop `sndrft.for` from the gap list. Call site at [winter.for:313-315](../../../wepp-forest_260430_baseline/src/winter.for) has been commented out since 1994-08-01; drift is dead in wepp-forest too.
- **Note 2026-05-25 (watbal_hourly row):** correction — openWEPP has hourly lane infrastructure. [TimestepPolicy at hillslope/mod.rs:262](../../crates/openwepp-runner/src/hillslope/mod.rs#L262) defines `Daily | Hourly | SubHourly` variants, [ExecutionLane at L238](../../crates/openwepp-runner/src/hillslope/mod.rs#L238) exposes `Daily` and `Hourly` (sub-hourly is scaffold-only, `physics_enabled() = false`). SIMIMPL09 delivered the hourly lane foundation. [ComparatorSurfaceClass::HourlyWaterBalance](../../crates/openwepp-comparator-metadata/src/lib.rs#L46) routes hourly comparisons to the Investigation confidence tier per ADR-0011. What is **not** verified by this audit is whether `Wb11HydrologyKernel` runs different physics at hourly vs daily cadence, or whether hourly mode just re-ticks the daily kernels — that needs a separate check before claiming Brooks/Dun `watbal_hourly.for` parity.

### 10.3 Algorithmic-shape divergences worth confirming against contracts

| Concept | wepp-forest form | openWEPP form | Audit risk |
|---|---|---|---|
| ET demand | Penman-Monteith with `kcb` basal coefficient (`evappm.for`) | Priestley-Taylor `0.00128·Rn·γ/(58.3·(γ+Δ))` ([hillslope/mod.rs:1629](../../crates/openwepp-runner/src/hillslope/mod.rs#L1629)) | `pmetpara.kcb` is parsed but unused. Either Priestley-Taylor is intentional and PM should be dropped, or PM must be wired. |
| Transport capacity | Yalin (`yalin.for`) called from `trcoef.for` via `shield.for` | Power-law `tcadjf · k · τf^m` ([hydrology.rs:5008](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L5008)) | Contract SC-SED-001 REF-SED-CH11-TC explicitly cites the power-law form — this is a contract-sanctioned simplification, but downstream tooling that compares against wepp-forest will see a parity gap. |
| Frost | Per-fine-layer thermal solver (`frostn.for`) | Binary tmin-threshold + constant `FROST_MAX_DEPTH_M` ([hydrology.rs:2430-2448](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2430-L2448)) | Output state will not match wepp-forest under freeze/thaw conditions. The runtime guards accept this output, so the closure check is satisfied even with non-physical frost. |
| Snow | Hourly energy-balance with radiation/wind/canopy (`melt.for`) | Linear-temperature partition + degree-day-like melt × density factor ([hydrology.rs:2543-2596](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2543-L2596)) | Same posture as frost — guards pass, but snow accumulation/melt trajectory will diverge. |
| Impoundment | RK4 with adaptive timestep, regime-transition retry | Direct stage-area-discharge with composite outflow | Single-step approximation; transient regime behavior will not match. |
| Hyetograph | `disag.for`/`brkpt.for` disaggregation from daily | Consumed as pre-prepared state surface | Possibly upstream-of-openWEPP; needs a documented contract boundary. |
| PURK unsat fx exponent | `fx = stz ** hk(k1)` — **per-layer** exponent [perc.for:131](../../../wepp-forest_260430_baseline/src/perc.for) | `fx = stz ** WB18_PERC_SHAPE_EXPONENT` — single global constant ([03_kernel_support.rs:2935](../../crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs#L2935)) | per-layer attenuation lost; layered profiles will diverge |
| PURK saturation incl. frzw | `stz = (st + frzw) / ul` includes frozen water [perc.for:130](../../../wepp-forest_260430_baseline/src/perc.for) | `stz = θ / UL` (liquid only) | under frost, openWEPP under-counts saturation → over-estimates drainage |
| PURK saturated-lower-layer override | `meblfc=1 → fx=1` when lower layer ≥ 95% saturated (Brooks/Dun 2007, ui_run mode) [perc.for:145-163](../../../wepp-forest_260430_baseline/src/perc.for) | not present | openWEPP applies fx reduction even when lower layer is saturated |

### 10.4 Parser-without-consumer surfaces

- `pmetpara.rs` → `kcb`, basal-coefficient table parsed at [parsers/pmetpara.rs:501](../../crates/openwepp-input-contract/src/parsers/pmetpara.rs#L501), discarded at [hillslope/mod.rs:592](../../crates/openwepp-runner/src/hillslope/mod.rs#L592). [SC-INFILE-PMETPARA-001.md](../specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md) defines the input but no SC-EVAP path wires it to the ET kernel.
- `soil.rs` → `ksatadj`, `ksatfac_mm_h`, `ksatrec_per_day` parsed at [parsers/soil.rs:238-242](../../crates/openwepp-input-contract/src/parsers/soil.rs#L238-L242). wepp-forest applies these in [infpar.for:618-647](../../../wepp-forest_260430_baseline/src/infpar.for) (A. Srivastava) to mutate saturated K via three regimes — `solwpv==9001` exponential recovery, `solwpv==9002` Saxton-Rawls Brooks-Corey `keff = ks · sat_frac^(2λ+3)`, `solwpv==9003` with burn-severity floor `lkeff`. No openWEPP kernel consumes these fields; the disturbed-land / forest / burn-recovery K adjustment is silently inert. Distinct from the PURK `fx` unsaturated shape function — `fx` scales drainage flux per day, `ksatadj` mutates the saturated K once at input time before Green-Ampt.
- `snow.rs`, `frost.rs` parsers exist under [parsers/](../../crates/openwepp-input-contract/src/parsers/) but the runtime snow/frost computations are the reductions described above; the parsed-file detail likely does not flow into the runtime computation.

These should either be wired into kernels or excised; leaving them as parser-only is a hidden-contract risk because the input files appear active but their content is silently inert.

### 10.5 Phase-class taxonomy observation

[HillslopeKernelPhaseClass at kernel-contract/src/lib.rs:1060](../../crates/openwepp-kernel-contract/src/lib.rs#L1060) has 12 variants: 8 hydrology + 2 growth + 2 decomposition. There is **no `Erosion` class, no `Snow` class, no `Frost` class, no `Canopy` class, no `Infiltration` class**. Erosion executes inside `HydrologyPeakRunoff`; snow/frost execute inside `HydrologyRunoffReconciliation` as coupling helpers; canopy interception is an inline helper. This compresses many wepp-forest top-level routines into the hydrology phase tree.

If snow, frost, and channel-erosion are intended to become first-class kernels, the phase-class enum will need extension and the scheduler graph will need to learn the new dispatches. Today, the absence of those phase classes is the structural signal that the corresponding wepp-forest physics has not been re-implemented.

### 10.6 Routines outside this audit's scope

- **Plant / management / decomposition** routines (`plant`, `mngmt`, `decomp`, `grow`, `growop`, `rgrcur`, `cutgrz`, `rburn`, `tilage`, …) are covered by `GrowthAnnualTransition`, `GrowthPerennialTransition`, `DecompositionTransition`, `ResiduePartitionTransition` phase classes and `pl_*` symbol families. Cross-reference is out of scope for this hydrology/erosion audit.
- **Climate** (`stmget`, `stmtim`, `rdat`, `idat`, `tmpadj`, `tmpcft`, `tmpfun`, `radcur`, `sunmap`, `aspect`) — partially covered by `openwepp-climate-runtime-adapter` and `parsers/climate.rs`. Same — out of scope here.
- **I/O** (`bigout`, `annout`, `monout`, `hbpout`, `bighdr`, `outfil`, …) — covered by the output crates and the HBP-format contract. Out of scope.

### 10.7 Bottom line for cross-reference

- **Confirmed-implemented physics** (matches algorithmic shape): ET (Priestley-Taylor no-wind branch), Ritchie two-stage soil evaporation, PURK percolation (with the per-layer / frzw / meblfc gaps noted below), Darcy lateral, Hooghoudt drainage, Green-Ampt coupled infiltration, KINEROS peak runoff, Foster-Meyer rill detachment/deposition, particle-class enrichment, ipeak-branch channel routing, stage-area-discharge impoundment.
- **Confirmed-missing physics**: Penman-Monteith ET + Penman wind-data branch, energy-balance snow, fine-layer frost, channel sediment (chnero), RK4 impoundment timestep, furrow irrigation, hyetograph disaggregation, watershed-side `wshcqi`/`wshscs` routines, `ksatadj` disturbed-land K adjustment (three regimes).
- **Confirmed reductions/placeholders** (guard surface satisfied, physics simplified): snow and frost coupling outcomes.
- **Confirmed parser-without-consumer**: `pmetpara.kcb`; `soil.ksatadj/ksatfac/ksatrec`; likely `snow`/`frost` parser detail.
- **Confirmed PURK fx narrow divergences** (fx itself matches Eq. 7.4.3; the per-layer exponent, frzw inclusion, and meblfc override do not): see 10.3 rows.
- **Algorithmic-shape divergence to confirm**: Yalin transport capacity (intentional per SC-SED-001 simpler form).

### 10.8 Caveats specific to Section 10

- The wepp-forest routine purpose strings were sampled from leading comments. They are authoritative for what the routine does, but Rust analog absence is only confirmed for the specific concept keywords I searched.
- "Not visible by grep" is not the same as "demonstrably absent." A few of the gaps (channel erosion, Yalin) are categorical and grep-confirmed; a few (route.for per-segment branching, wshcqi inflow assembly) are softer — they need a direct algorithmic diff to call definitively missing.
- All claims about *what wepp-forest does* are static reads of `.for` source; I did not run the legacy binary.
- All claims about *what openWEPP does* match Section 5 of this audit and are grep-corroborated for the negative cases.

