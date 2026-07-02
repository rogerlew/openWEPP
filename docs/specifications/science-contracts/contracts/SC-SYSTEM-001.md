---
contract_id: SC-SYSTEM-001
title: System Integration Boundary and Watershed Assembly Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 83
producer_scope:
  - Hillslope-to-watershed pass-file state/flux surfaces
  - Channel and impoundment boundary assembly surfaces
  - Cross-component event/daily closure and handoff semantics
consumer_scope:
  - Channel hydrology/erosion and impoundment routing consumers
  - Watershed outlet hydrograph/sediment-yield accounting consumers
  - Comparator/replay and governance-gate consumers
evidence_level: Static
last_reviewed: 2026-06-14
supersedes: []
superseded_by: []
---

# SC-SYSTEM-001 System Integration Boundary and Watershed Assembly Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for cross-component integration in WEPP
watershed simulation: hillslope pass-file payload completeness, channel/impoundment
runon-runoff assembly semantics, hydrograph merge behavior, and mass-continuity
constraints that must hold at integration boundaries.

## Scientific Scope

In scope:
- Handoff obligations between hillslope outputs and channel/impoundment inputs.
- Runon/runoff assembly, peak-runoff composition, and event-duration harmonization
  rules used when watershed elements are coupled.
- Impoundment stage-routing and sediment-mass continuity constraints required for
  valid downstream boundary publication.
- System-level guard obligations for explicit invalid-state failure (no silent
  boundary repair).

Out of scope:
- Internal kernel constitutive equations owned by domain contracts
  (`SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-SED-001`, `SC-IMPOUND-001`).
- Rust API naming finalization beyond canonical symbol-to-identity aliasing.
- Large-watershed extrapolation beyond chapter-declared applicability bounds.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-SYSTEM-CH1-WATERSHED | `references/50201000/chap1.pdf` §1.1 and Fig. 1.1.1 | Watershed decomposition into hillslopes, channels, and impoundments plus linkage semantics. | `[DIRECT][Static]` |
| REF-SYSTEM-CH1-COMPONENTS | `chap1.pdf` §1.4 and §1.4.11 | Continuous simulation component coupling and watershed extension assumptions. | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-PASSFILE | `references/50201000/chap13.pdf` intro paragraph before §13.2 | Required hillslope pass-file payload fields consumed by channel/impoundment components. | `[DIRECT][Static]` |
| REF-SYSTEM-HBP-FORMAT | `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` (`EVENT Payload`) | Canonical HBP payload field names and units for hillslope-to-watershed routing-boundary coupling (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3[npart]`, `particle_diameter_m[npart]`, `particle_flow_fraction[npart]`). | `[DIRECT][Static]` |
| REF-SYSTEM-HBP-READER | `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md` (`Read Contract`, `Required Invariants`) | Watershed reader fail-closed payload-completeness semantics and no-text-fallback posture. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SYSTEM-CH13-RUNON | `chap13.pdf` §13.2, Eq. [13.2.1]-[13.2.3] | Channel runon decomposition, depth conversion, and event-duration maximum rule. | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-TRANSLOSS | `chap13.pdf` §13.2, Eq. [13.2.4]-[13.2.6] | Case-partitioned transmission-loss and final-runoff assembly semantics. | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-PEAKIN | `chap13.pdf` §13.4.1, Eq. [13.4.1]-[13.4.2] | Triangular-hydrograph merge semantics when multiple watershed elements contribute runon. | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-PEAKOUT | `chap13.pdf` §13.4.2, Eq. [13.4.3]-[13.4.7], [13.4.25]-[13.4.26] | Channel/watershed outlet peak-runoff and duration semantics, including method options. | `[DIRECT][Static]` |
| REF-SYSTEM-WSHDRV-ORDER | `/workdir/wepp-forest_260430_baseline/src/wshdrv.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Watershed execution-order authority for channel lane (`wshcqi -> wshirs -> wshrun/wshpek`) including direct `wshchr` routing path when `ipeak > 2` and local channel runoff is absent. | `[DIRECT][Static]` |
| REF-SYSTEM-WSHPEK-IPEAK | `/workdir/wepp-forest_260430_baseline/src/wshpek.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | `ipeak`-selected outlet branch authority (`1` Rational, `2` CREAMS, `>=3` wave routing) and routed peak/duration publication semantics. | `[DIRECT][Static]` |
| REF-SYSTEM-WSHCHR-WAVE | `/workdir/wepp-forest_260430_baseline/src/wshchr.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy-equivalent channel wave-routing authority (linear kinematic-wave and Muskingum-Cunge branches, storage closure, routed `peakot`/`runvol`/`rundur` outputs). | `[DIRECT][Static]` |
| REF-SYSTEM-LEGACY-WATBAL | `/workdir/wepp-forest_260430_baseline/src/watbal.for:958-967` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | WB11 aggregate recomputation authority linking layer-water state to published daily soil-water totals consumed by WB13 outputs. | `[DIRECT][Static]` |
| REF-SYSTEM-LEGACY-OUTFIL | `/workdir/wepp-forest_260430_baseline/src/outfil.for:623-643` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | WB13 publication authority for ET components (`Ep`, `Es`, `Er`) and soil-water aggregates (`Total-Soil`, `SoilWaterTotal`). | `[DIRECT][Static]` |
| REF-SYSTEM-CH13-SEDCONT | `chap13.pdf` §13.5.5, Eq. [13.5.17] | Channel sediment continuity with upstream/lateral inflow and flow detachment/deposition terms. | `[DIRECT][Static]` |
| REF-SYSTEM-CH14-HYDCONT | `references/50201000/chap14.pdf` §14.2, Eq. [14.2.1]-[14.2.5] | Impoundment hydraulic continuity and stage-discharge/stage-area coupling. | `[DIRECT][Static]` |
| REF-SYSTEM-CH14-ADAPT | `chap14.pdf` §14.2.3, Eq. [14.2.7]-[14.2.9] | Adaptive timestep semantics and mandatory minimum-step reset at regime transitions. | `[DIRECT][Static]` |
| REF-SYSTEM-CH14-OUTFLOW | `chap14.pdf` §14.3.8, Eq. [14.3.18] | Total outflow as explicit sum of active outlet-structure contributions. | `[DIRECT][Static]` |
| REF-SYSTEM-CH14-SEDCONT | `chap14.pdf` §14.6.1, Eq. [14.6.1] | Impoundment sediment mass continuity for effluent concentration computation. | `[DIRECT][Static]` |
| REF-SYSTEM-CH2-BRKPT | `references/50201000/chap2.pdf` §2.2, Table 2.2.1 convention text | Breakpoint rainfall sequence conventions required by channel infiltration/runoff assembly. | `[DIRECT][Static]` |
| REF-SYSTEM-INFILE-WEPPUI | `docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md` §4, §8, §11 | Cross-contract authority for requested/effective `wepp_ui` mode propagation to runtime lane selection and publication provenance. | `[DIRECT][Static]` |
| REF-SYSTEM-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative volumetric fluxes, explicit branch behavior, and conservation-consistent handoff are required for physically valid integration. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `durstorm` | `s` | Hillslope storm duration exported in pass file. | hillslope component | channel/impoundment runon assembly |
| `tc_h` | `h` | Overland-flow time of concentration exported from hillslope. | hillslope component | channel peak-flow assembly |
| `alpha` | `fraction` | Dimensionless Rational-equation shape parameter from contributor element. | hillslope/channel/impoundment element | downstream channel peak-flow assembly |
| `qdepth` | `m` | Runoff depth exported by contributing element. | hillslope/channel/impoundment element | downstream runon assembly |
| `rof` | `m^3` | Runoff volume exported by contributing element. | hillslope/channel/impoundment element | downstream runon/hydrograph assembly |
| `qp` | `m^3 s^-1` | Peak runoff from contributing element. | hillslope/channel/impoundment element | downstream hydrograph merge |
| `Ep`, `Es`, `Er` | `mm` (daily depth-equivalent publication units) | Daily ET component publications consumed by replay/comparator surfaces and downstream summaries. | hillslope daily hydrology closure | WB13/reporting/replay consumers |
| `Total-Soil`, `SoilWaterTotal` | `mm` | Daily soil-water aggregate publications derived from runtime layer-state lineage plus frozen/snow components. | hillslope daily hydrology closure | WB13/reporting/replay consumers |
| `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore` | `mm` | Daily full-profile publications where `ProfileDepth`/`ProfilePorosityCap` derive from baseline-authoritative soil preprocessing + aggregation lineage, `ProfileFCStore` derives from layer aggregation plus explicit normalized-tail contribution (`Σ(thetfc_i*dg_i)*1000 + wb13_profile_fc_tail_mm`) under HPHYS0216D, and `ProfileWPStore` derives from normalized-profile storage symbol (`wb13_profile_wp_store_mm`). | hillslope daily hydrology closure | WB13/reporting/replay consumers |
| `mofe_hourly_carry` manifest object | manifest metadata | Hillslope-run manifest provenance object declaring MOFE hourly carry-array activation, 24-slot policy, required array family names, and final aggregate carry evidence. | hillslope runner publication | watershed contributor intake validation |
| `total_detachment_kg` | `kg` | Total hillslope detachment payload at event endpoint. | hillslope erosion component | channel sediment-load assembly |
| `total_deposition_kg` | `kg` | Total hillslope deposition payload at event endpoint. | hillslope erosion component | watershed sediment bookkeeping |
| `particle_class_count` | `count` | Particle-class cardinality for event payload vectors. | hillslope erosion component | watershed routing payload validator |
| `sediment_concentration_kg_m3,k` | `kg m^-3` | Sediment concentration for particle class `k` at handoff. | hillslope/channel/impoundment element | downstream sediment-routing component |
| `particle_diameter_m_k` | `m` | Representative particle diameter for class `k` at handoff. | hillslope/channel/impoundment element | downstream sediment-routing component |
| `particle_flow_fraction_k` | `fraction` | Fraction of particle class `k` in eroded sediment payload. | hillslope/channel/impoundment element | downstream sediment-routing component |
| `rov`, `rol`, `roi` | `m^3` | Total, lateral, and inlet runon volumes for channel inlet assembly. | channel integration routine | channel runoff assembly |
| `rod` | `m` | Runon depth computed from runon volume and channel area. | channel integration routine | channel runoff case logic |
| `Ach` | `m^2` | Physical channel area used for depth conversion. | channel geometry surface | channel runon depth conversion |
| `durc`, `durrunon`, `durchan`, `durirrig` | `s` | Channel event duration and duration contributors (runon max, local storm, irrigation). | integration/climate/irrigation surfaces | channel runon/runoff and peak-flow logic |
| `qci`, `qcf` | `m` | Initial and final channel runoff depth during case partition logic. | channel runoff routine | channel runoff volume/peak computation |
| `tl` | `m^3` | Channel transmission-loss volume. | channel runoff routine | channel water-balance accounting |
| `tb`, `tp` | `min` | Synthetic-hydrograph base time and time-to-peak for contributor hydrographs. | hydrograph merge routine | channel/impoundment peak assembly |
| `Aw`, `qa`, `qpi` | `m^2`, `m`, `m^3 s^-1` | Contributor area, average runoff depth, and peak flow used for synthetic hydrograph construction. | contributor element set | hydrograph merge routine |
| `qpo` | `m^3 s^-1` | Peak runoff at channel/watershed outlet. | channel outlet routine | downstream routing/reporting |
| `tc`, `tcc`, `tcs`, `tci` | `h` | Total time of concentration and channel/overland/impoundment components. | channel outlet routine | peak-flow method |
| `Qi`, `Qo` | `ft^3 s^-1` | Impoundment inflow and outflow rates in continuity expression. | impoundment routing routine | stage and sediment routing |
| `H`, `Aimp` | `ft`, `ft^2` | Impoundment stage and stage-dependent area. | impoundment routing routine | hydraulic continuity integration |
| `Qtotal` | `ft^3 s^-1` | Sum outflow from all active outflow structures at current stage. | impoundment outlet routine | downstream hydrograph and sediment routing |
| `M`, `Ci`, `Co`, `Dep` | `lb`, `lb ft^-3`, `lb ft^-3`, `lb` | Impoundment sediment mass, influent concentration, effluent concentration, and deposited mass. | impoundment sedimentation routine | watershed sediment accounting |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SYSTEM-001 | Pass-file completeness invariant: all required hillslope boundary fields listed in Chapter 13 intro (storm duration, overland `tc`, `alpha`, runoff depth/volume/peak, endpoint detachment/deposition, particle-class concentrations, particle diameters, and size fractions) are present with declared units before channel/impoundment consumption. | hard-fail | REF-SYSTEM-CH1-WATERSHED, REF-SYSTEM-CH13-PASSFILE, REF-SYSTEM-HBP-FORMAT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-002 | Channel runon decomposition invariant: runon identity and depth conversion hold (`rov = rol + roi`, `rod = rov/Ach`) with non-negative volumes/area and explicit contributor accounting. | hard-fail | REF-SYSTEM-CH13-RUNON, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-003 | Channel duration harmonization invariant: event duration uses the explicit max rule `durc = max(durrunon, durchan, durirrig)` and all duration terms share consistent event basis/units. | hard-fail | REF-SYSTEM-CH13-RUNON | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-004 | Channel runoff-case invariant: channel runoff must follow the four-case partition of §13.2, including transmission-loss equations (`tl`) for applicable cases and explicit zero branch for Case IV (`qci = 0`, `rod = 0` implies `qcf = 0`, `rof_f = 0`). | hard-fail | REF-SYSTEM-CH13-TRANSLOSS | `[DIRECT][Static]` |
| INV-SYSTEM-005 | Hydrograph-merge invariant: with one upstream contributor, inlet peak is direct pass-through; with multiple contributors, SCS triangular synthetic hydrographs are computed and superimposed before selecting inlet peak. For `ipeak <= 2`, peak/duration calculations are skipped when `rof_f <= 0.001 m^3`; for `ipeak >= 3`, routed channel flow may still be evaluated from incoming hydrograph when local runoff is absent. | hard-fail | REF-SYSTEM-CH13-PEAKIN, REF-SYSTEM-CH13-PEAKOUT, REF-SYSTEM-WSHDRV-ORDER, REF-SYSTEM-WSHPEK-IPEAK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-006 | Outlet method-branch invariant: channel/watershed outlet routing uses exactly one declared `ipeak` branch (`1` modified Rational, `2` CREAMS, `3` linear kinematic-wave, `>=4` Muskingum-Cunge) with explicit branch identity retained in outputs and no implicit fallback switching. | hard-fail | REF-SYSTEM-CH13-PEAKOUT, REF-SYSTEM-WSHPEK-IPEAK, REF-SYSTEM-WSHCHR-WAVE, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-007 | Impoundment hydraulic continuity invariant: stage update obeys continuity (`dV/dt = Qi - Qo`, `dH/dt = (Qi - fQo(H))/fA(H)`), and adaptive timestep reset to minimum is enforced whenever outflow regime changes or inflow starts/ends. | hard-fail | REF-SYSTEM-CH14-HYDCONT, REF-SYSTEM-CH14-ADAPT | `[DIRECT][Static]` |
| INV-SYSTEM-008 | Impoundment outflow aggregation invariant: total outflow equals sum of active outlet-structure contributions (`Qtotal`) and each inactive structure contributes exactly zero when stage is below its inlet threshold. | hard-fail | REF-SYSTEM-CH14-OUTFLOW | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-009 | Sediment continuity invariant: channel sediment continuity and impoundment sediment continuity equations are enforced with explicit upstream/lateral loads and deposition terms; no untracked mass creation/loss across handoff boundaries is allowed. | hard-fail | REF-SYSTEM-CH13-SEDCONT, REF-SYSTEM-CH14-SEDCONT, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-010 | Breakpoint forcing invariant: when channel runoff is event-driven, rainfall inputs preserve breakpoint sequence semantics required by Chapter-2 conventions (explicit interval times/intensities and end-of-storm zero-intensity termination). | governance-fail | REF-SYSTEM-CH13-RUNON, REF-SYSTEM-CH2-BRKPT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-011 | INT10 cross-lane publication invariant: hillslope boundary publication to watershed/channel integration is allowed only after canonical daily plant/hydrology lane closure (`decomp -> growth -> watbal`) has completed without typed ordering/state-transfer violations. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-CH13-PASSFILE, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-012 | PL14 strict replay provenance invariant: Tier-A replay lane execution must explicitly surface missing required replay artifacts (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`), strict-diff status, and provenance-hash gaps as typed gate failures or `HOLD` signals; no synthetic fallback artifact substitution is allowed. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-013 | PL15 Tier-A closeout governance invariant: unresolved strict Tier-A deltas remain blocking unless explicit risk-acceptance approval reference (owner, rationale, and scope) is recorded; silent tier down-classification and implicit risk acceptance are forbidden. | governance-fail | REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-014 | PL14R strict replay rerun reproducibility invariant: post-closure-wave Tier-A rerun must stage required interchange surfaces (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`), persist strict comparator JSON artifacts, and record command/binary/tool/output hashes; missing required surfaces, comparator artifacts, or provenance hashes must hard-fail replay disposition and keep the lane in `HOLD`. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-015 | PL15R Tier-A recloseout supersession invariant: refreshed hold-lift governance must classify residual Tier-A deltas from the latest PL14R schema-aligned strict replay evidence set (parquet comparator JSON artifacts plus day-by-day parity evidence) and must treat stale pre-supersession strict-failure signatures as historical context, not active blockers. If unresolved Tier-A blockers remain after supersession, explicit risk-acceptance reference is still mandatory. | governance-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-016 | WB20 forward-solver parity publication invariant: Tier-A forward-solver lane publication must include explicit lane-manifest evidence showing `wb20_forward_solver_lane_enabled = 1` and confirming observed closure targets are excluded from acceptance-driving inputs; missing manifest/no-substitution evidence is an unresolved governance blocker. | governance-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-017 | PL14S semantic replay-evidence invariant: Tier-A hillslope replay publication must persist semantic comparator evidence (`h5_wat_semantic_comparator.json`) with row-key presence deltas, per-column tolerance verdicts, and top divergent-key diagnostics for investigation; missing or malformed semantic report evidence, or suppressed strict-comparator skip/execution status in provenance output, is a hard-fail/HOLD condition. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-018 | SIMPIPE production runner ownership invariant: system-boundary publication and replay candidate staging for hillslope products must consume outputs from an executed runner -> scheduler/kernel lifecycle; publication from non-executed projection-only synthesis paths is invalid. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-019 | SIMMODE mode-propagation manifest invariant: requested/effective `wepp_ui` mode must be propagated into runtime lane selection and exposed in publication provenance with deterministic lane identity (`daily`/`hourly`); missing mode provenance or lane/mode mismatch is invalid. | hard-fail | REF-SYSTEM-INFILE-WEPPUI, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-020 | SIMOUT simulation-owned publication invariant: required replay candidate surfaces (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`) must be simulation-owned outputs emitted from executed runtime lanes with explicit provenance; synthetic/bootstrap substitution or projection-only reconstruction is invalid. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-021 | SIMCONS selective consolidated-intake governance invariant: consolidated-kernel/policy intake from candidate sources must remain selective and triaged with explicit `adopt`/`defer`/`reject` decisions and typed guard posture; wholesale adoption or untriaged qcap-style policy intake is forbidden. | governance-fail | REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-022 | SIMIMPL14 replay-span comparability invariant: runner publication provenance must demonstrate continuous multi-day execution closure (`executed_day_count == climate_day_count`), replay-surface row closure (`wb13_row_count == executed_day_count`), monotonic key progression (`sim_day_index = 1..N`), and simulation-year row-key semantics for `Y`; missing continuity proofs or key-domain mismatch keeps replay comparability in hard-fail/HOLD posture. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-INFILE-WEPPUI, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-023 | SIMIMPL15 replay-lane policy/provenance invariant: comparator suite provenance must explicitly encode strict/parquet lane policy mode and candidate source classification (`native-runtime-dat`, `conversion-derived-dat`, `native-runtime-parquet`) with deterministic no-default behavior. Missing/ambiguous policy metadata is a hard-fail/HOLD condition; conversion-derived dat strict evidence is non-promotable for final Tier-A closeout claims. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-024 | SIMIMPL15 semantic report structural-continuity invariant: parquet semantic reports must resolve required investigation alias continuity for `Total-Soil` and publish observed row-width diagnostics comparable to dat lanes; alias drift or placeholder width diagnostics is a hard-fail/HOLD evidence defect. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-025 | SIMIMPL16 replay contract-derived test-coverage invariant: system replay closeout claims must be backed by contract-derived tests covering `SIMIMPL13-TEST-001..005`, including span overlap closure, key-domain alignment, parquet alias continuity, strict-lane compensation when parquet strict is skipped, and conversion-derived dat provenance row-consistency checks. Missing/failed closure tests keep replay governance evidence non-authoritative. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-026 | SIMIMPL18 baseline-year policy and full-span precipitation parity invariant: replay comparability claims across legacy baseline and openWEPP candidate must publish explicit baseline-year policy that yields a declared common keyed horizon, preserve identical input/sidecar provenance references, and evaluate precipitation (`P`) parity across the full keyed span (not overlap-only subsets). Missing policy metadata or unmatched-span `P` claims are hard-fail/HOLD defects. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-INFILE-WEPPUI, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-027 | SIMIMPL21/HPARITY02 WB13 ET/soil-water/profile publication-lineage invariant: published `Ep`, `Es`, `Er`, `Total-Soil`, `SoilWaterTotal`, `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, and `ProfileWPStore` surfaces must be simulation-owned outputs traceable to canonical WB11/WB13 lineage with explicit alias continuity and no projection-side surrogate reconstruction. | hard-fail | REF-SYSTEM-LEGACY-WATBAL, REF-SYSTEM-LEGACY-OUTFIL, REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-028 | HPHYS0241 MOFE hourly carry manifest invariant: multi-OFE hourly hillslope publications must include `mofe_hourly_carry` manifest provenance proving active 24-slot carry-array execution for `ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`, and `ui_LfCrf`; watershed contributor intake must reject missing, inactive, malformed, or non-24-slot carry metadata before routing dispatch. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-PHYS-BOUNDS, SC-WATBAL-001#INV-WATBAL-033, SC-RUNOFFPART-001#INV-RUNOFFPART-013 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-029 | HPHYS0255 MOFE WB13 storage-lineage provenance invariant: MOFE WB13/H.wat manifests must declare the storage lineage policy used by canonicalized single-row publication; under the current architecture this is `single-runtime-wb11-state`, and consumers must not reinterpret aggregate `Area` as proof of area-weighted dynamic storage aggregation. | hard-fail | REF-SYSTEM-LEGACY-WATBAL, REF-SYSTEM-CH1-COMPONENTS, SC-WATBAL-001#INV-WATBAL-042 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-030 | MOFE01 M-E0 per-OFE dynamic-state publication-policy manifest invariant: transition from MOFE04 aggregate to per-OFE WB13/H.wat publication must be manifest-gated by `publication_ofe_policy = "per-ofe-dynamic-water-balance-state"`, `contributor_ofe_count`, `per_ofe_record_count`, `per_ofe_state_policy`, `transfer_identity_status`, `per_element_identity_status`, `aggregate_identity_status`, and `storage_lineage_policy = "per-ofe-dynamic-wb-state"`. Consumers fail closed when multi-OFE publication lacks OFE-keyed records, policy fields are missing or malformed, row cardinality disagrees with contributor count, or aggregate-only rows are relabeled as per-OFE records. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-LEGACY-WATBAL, SC-WATBAL-001#INV-WATBAL-097, SC-RUNOFFPART-001#INV-RUNOFFPART-029, INV-SYSTEM-028, INV-SYSTEM-029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-031 | MOFE01 M-F-REDO per-OFE publication anti-clone and M-F-REDO2 runoff-normalization manifest/consumer invariant: multi-OFE WB13/H.wat publication manifests and consumer gates must not treat row cardinality, monotonic OFE keys, or conservation residual closure alone as proof of per-OFE genuineness. Publication must provide or reference anti-clone evidence for active routed days: lane-local source lineage, nonzero adjacent surface handoff from independently stored transfer operands, non-identical hydrology vectors, raw non-cloned local runoff across OFEs, and and public runoff-normalization consistency (`Q = runoff * efflen / totlen`, with `QOFE == Q` the canonical published convention per `SC-RUNOFFPART-001#INV-RUNOFFPART-032` / MOFE04 canonicalized policy; the retained per-OFE local-length basis feeds only byte-invariant `H.pass.runvol`/peak). Missing anti-clone evidence, all-OFE-identical active-day hydrology or raw local runoff, WB14 multistep-lane acceptance of seeded/stale `wb12_infiltration`, zero-on-zero transfer acceptance, or public-row synthesis of transfer fields hard-fails publication promotion and downstream consumption. **Superseded (2026-07-02, MOFEFID-B02):** the former `QOFE == Q where slplen != totlen` rejection is removed — genuineness is proven by the distinctness/lineage evidence above, not by `QOFE != Q`. | hard-fail | REF-SYSTEM-CH1-COMPONENTS, REF-SYSTEM-LEGACY-WATBAL, SC-WATBAL-001#INV-WATBAL-098, INV-SYSTEM-030 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SYSTEM-032 | MOFE01 M-G EROD14 `qin` manifest-boundary invariant: hillslope run manifests and downstream consumers must distinguish water-transfer-only EROD14 `qin` compatibility seeding from accepted sediment-coupled `qin` closure. When Wave-2 runs without prior-OFE erosion `qout` plus particle/class-fraction handoff evidence, manifests must publish `erod14_qin_source_policy = "water-transfer-only-mofe01-mg-sediment-coupling-follow-on"` and `erod14_qin_sediment_coupled = false`; downstream consumers must not infer sediment coupling from `UpStrmQ`, `SubRIn`, public WB13/WAT rows, aggregate runoff, or Wave-2 kernel execution alone. | governance-hold | REF-SYSTEM-CH1-COMPONENTS, SC-RUNOFFPART-001#INV-RUNOFFPART-030, SC-WATBAL-001#INV-WATBAL-099, SC-SED-001#INV-SED-012, INV-SYSTEM-028, INV-SYSTEM-031 | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-SYSTEM-001` | runtime | Boundary payload validator at hillslope->channel/impoundment handoff | Typed hard error on missing field, malformed units, or absent class vectors | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-002` | runtime | Channel inlet runon assembler | Typed hard error on decomposition/area/volume domain failure | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-003` | runtime | Channel event-duration assembly | Typed hard error on inconsistent duration basis or max-rule violation | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-004` | runtime | Channel runoff case-switch and transmission-loss calculator | Typed hard error when case outputs violate required equations or zero branch | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-SYSTEM-005` | runtime | Hydrograph merge and threshold/routed-gating logic | Typed hard error on invalid contributor-merge branch or threshold/routed-gating handling | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-006` | runtime | `ipeak` outlet branch selector/executor | Typed hard error on ambiguous, mixed, or undefined outlet branch in single evaluation path | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-007` | runtime | Impoundment continuity integrator and timestep controller | Typed hard error on continuity mismatch or missing minimum-step reset at regime transitions | Tier-B gate | `[DIRECT][Static]` |
| `INV-SYSTEM-008` | runtime | Impoundment outflow aggregator | Typed hard error on structure-sum mismatch or inactive-structure non-zero contribution | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-009` | runtime | Channel and impoundment sediment continuity routines | Typed hard error on continuity residual beyond tolerance or missing load term | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-010` | governance | Review/disposition/verification and promotion checklist | Promotion `HOLD` until breakpoint forcing semantics are contractually closed with companion climate/runoff contracts | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-011` | runtime | Hillslope daily-lane closure gate at system boundary publish handoff | Typed hard error and publish block when coupled plant/hydrology lane ordering or transfer closure fails | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-012` | runtime + governance | Comparator replay harness staging + disposition gate | Typed hard error or explicit `HOLD` when required replay artifacts/provenance hashes are missing, or strict comparator failure is masked | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-013` | governance | PL15 closeout criteria matrix + final decision record + conditional risk-acceptance artifact reference | Promotion `HOLD` when unresolved Tier-A deltas lack explicit approval reference; reject implicit risk-accept posture | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-014` | runtime + governance | PL14R rerun staging + provenance manifest gate | Typed hard error / explicit `HOLD` when required replay interchange surfaces, strict comparator JSON artifacts, or reproducibility hashes are missing | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-015` | governance | PL15R recloseout criteria matrix + refreshed hold-lift decision record + supersession references | Governance `HOLD` when active Tier-A blocker classification is derived from stale pre-supersession deltas, or when post-supersession unresolved blockers lack explicit approval reference | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-016` | governance | WB20 forward-solver lane publication checklist + disposition evidence gate | Governance `HOLD` when forward-lane manifest/no-substitution evidence is missing or does not prove observed-target exclusion from acceptance logic | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-017` | runtime + governance | PL14S semantic comparator publication gate + provenance integrity checklist | Typed hard error / explicit `HOLD` when semantic comparator JSON or required investigation diagnostics are missing, or when provenance omits strict-comparator skipped/executed posture | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-018` | runtime | Runner-execution provenance gate for system publication/replay staging | Typed hard error (`WS-SIMPIPE-E-001`) when publication is attempted without executed runner->scheduler lifecycle evidence | SIMIMPL execution gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-019` | runtime | Mode-propagation provenance and lane-identity closure guard | Typed hard error (`WS-SIMMODE-E-001`) on missing requested/effective mode or lane/mode mismatch in publication manifest | SIMIMPL execution gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-020` | runtime + governance | Simulation-owned replay-surface provenance gate | Typed hard error / explicit `HOLD` (`WS-SIMOUT-E-001`) when required candidate surfaces are projection/synthesis-first | Tier-A replay integrity gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-021` | governance | Consolidated intake triage governance gate | Governance `HOLD` (`WS-SIMCONS-E-001`) when candidate consolidated kernels/policies are adopted without explicit triage/provenance disposition | Consolidated-intake gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-022` | runtime + governance | Continuous replay-span/key provenance closure gate | Typed hard error / explicit `HOLD` (`WS-SIMOUT-E-001`) when execution-day, publication-row, monotonic-key, or simulation-year key-domain continuity assertions are missing or violated | SIMIMPL replay comparability gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-023` | runtime + governance | Replay-lane policy + candidate-source provenance classifier gate | Typed hard error / explicit `HOLD` (`WS-SIMOUT-E-001`) when strict/parquet lane policy or candidate source classification is missing/ambiguous; conversion-derived dat strict evidence is non-promotable for final Tier-A closeout | SIMIMPL replay tooling alignment gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-024` | runtime + governance | Semantic alias and row-width structural continuity gate | Typed hard error / explicit `HOLD` (`WS-SIMOUT-E-001`) when `Total-Soil` alias continuity is unresolved or semantic width diagnostics use placeholder sentinel classes | SIMIMPL replay tooling alignment gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-025` | governance | Replay contract-derived closure-test coverage gate | Typed hard error / explicit `HOLD` (`WS-SIMOUT-E-001`) when required SIMIMPL13 blind-spot closure tests are missing/failing, including strict-lane compensation and conversion-derived dat row-consistency assertions | SIMIMPL replay contract-test closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-026` | runtime + governance | Baseline-year policy + full-span precipitation comparability gate | Typed hard error / explicit `HOLD` (`WS-SIMOUT-E-001`) when baseline-year expansion/rekey policy is missing, input-provenance parity is unproven, or full-span keyed `P` comparability is reduced to overlap-only subsets | SIMIMPL replay span-policy gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-027` | runtime + governance | WB13 ET/soil-water/profile publication-lineage validator for ET components, aggregate soil-water outputs, and profile-capacity outputs | Typed hard error / explicit `HOLD` (`WS-SIMOUT-E-001`) when required WB13 ET/soil-water/profile outputs are not traceable to simulation-owned WB11/WB13 lineage with declared aliases | SIMIMPL ET/soil-water/profile publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-028` | runtime + governance | Hillslope manifest publisher plus watershed contributor manifest validator | Typed hard error / explicit `HOLD` (`CLIWAT-E-037`) when multi-OFE hourly contributor metadata lacks active 24-slot carry-array provenance, required array family names, or finite non-negative aggregate evidence | HPHYS MOFE hourly carry-array intake gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-029` | runtime + governance | Hillslope manifest publisher plus downstream publication consumers | Typed hard error / explicit `HOLD` when MOFE storage lineage policy is absent, malformed, or inconsistent with WB11/WB13 simulation-owned storage provenance | HPHYS0255 MOFE storage-lineage provenance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-030` | runtime + governance | Hillslope WB13/H.wat manifest publisher plus downstream publication consumers and watershed contributor metadata validators | Typed hard error / explicit `HOLD` when per-OFE publication policy is asserted without OFE-keyed records, identity statuses, matching row cardinality, or `per-ofe-dynamic-wb-state` storage lineage, or when aggregate-only rows are relabeled as per-OFE records | MOFE01 M-E0 per-OFE publication-policy transition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-031` | runtime + governance | Hillslope WB13/H.wat manifest publisher plus downstream publication consumers and watershed contributor metadata validators | Typed hard error / explicit `HOLD` when per-OFE publication evidence lacks anti-clone hydrology-vector checks, raw local-runoff operand distinctness, nonzero active adjacent surface handoff, lane-local source lineage, WB14 multistep infiltration lineage, or public `QOFE = runoff * efflen / slplen` / `Q = runoff * efflen / totlen` evidence; consumers must not accept aggregate-cloned records or seeded/stale WB14 multistep infiltration (the `QOFE == Q` rejection is superseded by MOFEFID-B02) | MOFE01 M-F-REDO anti-clone and M-F-REDO2 runoff-publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SYSTEM-032` | runtime + governance | Hillslope manifest publisher plus downstream EROD14/WAT/consumer gate | Explicit `HOLD` when manifests omit the M-G `erod14_qin_source_policy`, assert `erod14_qin_sediment_coupled = true` without SED-owned prior-OFE `qout` and class-fraction handoff evidence, or downstream consumers treat water-transfer-only `qin`/Wave-2 execution as sediment-coupled closure | MOFE01 M-G erosion `qin` manifest-boundary gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols follow chapter authority notation. openWEPP boundary/API field
names for active watershed integration surfaces are now fixed to ARCH22 typed
symbol families. Source-model identity aliases remain only where fields are
consumed directly as parser payload terms instead of projected runtime symbols.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `durstorm`, `tc_h`, `alpha`, `qdepth`, `rof` | identity names | hillslope pass-file payload (source-model intake) | chapter-declared units preserved | `[DIRECT][Static]` |
| `qp`, `watdur` | `hs{ID}_peakro`, `hs{ID}_watdur` (`WatershedProductionStateSymbol::{HillslopeContributorPeak,HillslopeContributorDuration}`) | contributor peak/duration runtime ingress | `m^3 s^-1`, `s` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `total_detachment_kg`, `total_deposition_kg` | `hs{ID}_total_detachment_kg`, `hs{ID}_total_deposition_kg` (`WatershedProductionStateSymbol::{HillslopeContributorTotalDetachmentKg,HillslopeContributorTotalDepositionKg}`) | contributor sediment-total runtime ingress | `kg` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `particle_class_count` | `hs{ID}_particle_class_count` (`WatershedProductionStateSymbol::HillslopeContributorParticleClassCount`) | contributor class-cardinality runtime ingress | count semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `sediment_concentration_kg_m3,k`, `particle_diameter_m_k`, `particle_flow_fraction_k` | `hs{ID}_sediment_concentration_kg_m3_{class:04}`, `hs{ID}_particle_diameter_m_{class:04}`, `hs{ID}_particle_flow_fraction_{class:04}` (`WatershedProductionStateSymbol` class-index families) | contributor per-class runtime ingress | `kg m^-3`, `m`, and `fraction` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `rov`, `rol`, `roi`, `rod`, `Ach` | identity names | channel runon-runoff assembly internals | `m^3`/`m`/`m^2` preserved | `[DIRECT][Static]` |
| `durc`, `durrunon`, `durchan`, `durirrig` | identity names | channel event-duration harmonization internals | `s` preserved | `[DIRECT][Static]` |
| `qci`, `qcf`, `tl` | identity names | channel runoff-case and transmission-loss internals | `m` and `m^3` preserved | `[DIRECT][Static]` |
| `tb`, `tp`, `Aw`, `qa`, `qpi` | identity names | synthetic hydrograph merge internals | `min`, `m^2`, `m`, `m^3 s^-1` preserved | `[DIRECT][Static]` |
| `qpo`, `durrof`, `roff` | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff` (`WatershedProductionStateSymbol::ChannelNode`, `WatershedProductionFluxSymbol::ChannelNode`) | channel-node runtime publication/consumption boundaries | `m^3 s^-1`, `s`, `m^3` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `H`, `Hfull`, `deltat`, `Qinf`, `Qo`, `durout`, `Hnext`, `outflow_volume` | `ws10_impoundment_{id}_{h,hfull,deltat,qinf,qo,durout,hnext,outflow_volume}` (`WatershedProductionStateSymbol::ImpoundmentNode`, `WatershedProductionFluxSymbol::ImpoundmentNode`) | impoundment-node runtime publication/consumption boundaries | `ft`, `s`, `ft^3 s^-1`, and volume semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ep`, `Es`, `Er` | `Ep`, `Es`, `Er` (identity) | WB13 ET component publication surfaces | `mm` daily publication units preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Total-Soil`, `SoilWaterTotal` | `Total-Soil`, `SoilWaterTotal` (identity; legacy semantic alias `Total-Soil Water` accepted in comparator tooling only) | WB13 aggregate soil-water publication surfaces | `mm` publication units preserved with WB11 lineage continuity | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`, `ui_LfCrf` | `mofe_hourly_carry.required_arrays[]` plus runtime symbols `ui_*_{hour:04}` | MOFE hourly carry-array provenance from hillslope runner into watershed contributor validation | 24 scalar `m` entries per family preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Qi`, `Aimp`, `Qtotal`, `M`, `Ci`, `Co`, `Dep` | identity names | impoundment hydraulic/sediment process internals | chapter-declared units preserved | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| No-runon/no-runoff channel event | Case IV branch with `qci = 0`, `rod = 0`, `qcf = 0`, and `rof_f = 0`. | Explicitly defined valid zero-flow branch in §13.2 case logic. | `[DIRECT][Static]` |
| Single-contributor merge | Exactly one watershed element contributes runon and inlet peak equals that contributor's peak. | Explicit §13.4.1 merge rule for single source. | `[DIRECT][Static]` |
| Sub-threshold runoff event | `rof_f <= 0.001 m^3` yields zero peak/duration and skip to downstream element. | Explicit §13.4.1 threshold behavior. | `[DIRECT][Static]` |
| Inactive outlet structures | Outflow contribution from a structure remains zero when stage is below structure inlet threshold. | Explicit §14.3.8 summation logic. | `[DIRECT][Static]` |
| Zero sediment inflow interval | `Qi*Ci = 0` with continuity reducing to effluent/deposition evolution only. | Consistent with Eq. [14.6.1] mass-balance form. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invalid States

- Missing any required pass-file field or particle-class vector required for
  downstream handoff. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative or undefined runon/runoff volumes, area, or stage/area terms in
  continuity equations. `[DIRECT][Static] + [INFERENCE][Static]`
- Channel duration computed without max-rule basis, or with mixed units across
  duration contributors. `[DIRECT][Static] + [INFERENCE][Static]`
- Peak-runoff merge branch inconsistent with contributor count (single-source
  vs multi-source synthetic hydrograph behavior). `[DIRECT][Static] + [INFERENCE][Static]`
- Outflow-regime transition processed without minimum timestep reset in
  impoundment integration. `[DIRECT][Static]`
- Sediment continuity evaluated without explicit upstream/lateral load terms or
  with unexplained continuity residual beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Replay provenance omits deterministic strict/parquet lane-policy classification
  or candidate source classification for candidate surfaces (`INV-SYSTEM-023`).
  `[DIRECT][Static] + [INFERENCE][Static]`
- Semantic parquet evidence omits `Total-Soil` alias continuity or publishes
  placeholder-only width diagnostics (`INV-SYSTEM-024`).
  `[DIRECT][Static] + [INFERENCE][Static]`
- Replay closeout evidence is asserted without contract-derived closure tests
  for span/key overlap, strict-lane compensation, alias continuity, and
  conversion-derived dat provenance row-consistency (`INV-SYSTEM-025`).
  `[DIRECT][Static] + [INFERENCE][Static]`
- Replay comparability evidence claims full-span parity without explicit
  baseline-year policy metadata or without full-span keyed precipitation (`P`)
  diagnostics under that policy (`INV-SYSTEM-026`).
  `[DIRECT][Static] + [INFERENCE][Static]`
- WB13 ET/soil-water/profile publication outputs (`Ep`, `Es`, `Er`,
  `Total-Soil`, `SoilWaterTotal`, `ProfileDepth`, `ProfilePorosityCap`,
  `ProfileFCStore`, `ProfileWPStore`) are emitted from projection-side
  surrogates or alias-only reconstruction without traceable simulation-owned
  WB11/WB13 lineage
  (`INV-SYSTEM-027`).
  `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-SYSTEM-P-001: Publish complete pass-file payload with declared units and
  particle-class semantics before channel/impoundment consumption.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-002: Enforce runtime guards for all `INV-SYSTEM-*` hard-fail
  invariants prior to publishing downstream boundary outputs.
  `[INFERENCE][Static]`
- OBL-SYSTEM-P-003: Surface typed integration errors (missing payload,
  continuity failure, branch ambiguity) without silent defaulting/clamping.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-004: Persist method-branch identity and key closure terms in
  diagnostic payload for comparator/replay traceability.
  `[INFERENCE][Static]`
- OBL-SYSTEM-P-005: Comparator replay producers must emit command traces,
  strict comparator JSON artifacts, and reproducible tool/binary/output hashes
  for each required interchange replay surface.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-006: PL14S comparison-suite producers must emit semantic
  comparator artifacts with row-key presence deltas, per-column tolerance
  verdicts, top divergent keys, and explicit strict-comparator skip/execution
  provenance status; silent omission of these diagnostics is forbidden.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-007: Production runner publication paths must emit explicit
  execution provenance proving runner -> scheduler/kernel lifecycle execution
  before system-boundary publication or replay staging.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-008: Publication provenance must include requested/effective
  `wepp_ui` mode and selected runtime lane identity; lane/mode divergence is a
  typed hard-fail condition.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-009: Consolidated kernel/policy intake claims must remain
  selective with explicit triage dispositions (`adopt`/`defer`/`reject`) and
  may not silently import qcap-style clamp policy overlays.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-010: Continuous replay publication must expose run-span
  continuity assertions (climate day count, executed day count, WB13 row
  count, first/last replay row keys, and monotonic `sim_day_index` verdict)
  and must encode replay row-year keys as simulation-year ordinals rather than
  absolute calendar-year keys.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-011: Replay provenance producers must publish explicit
  strict/parquet lane policy mode and candidate source classification for each
  comparison run; implicit policy defaults are forbidden and
  conversion-derived dat strict evidence must be marked non-promotable for
  final Tier-A closeout claims.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-012: Semantic report producers must preserve `Total-Soil` alias
  continuity and publish observed row-width diagnostics for parquet lanes;
  placeholder sentinel width diagnostics are non-authoritative.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-013: Replay governance/test producers must maintain and enforce
  contract-derived closure tests for `SIMIMPL13-TEST-001..005`, including
  span/key comparability assertions, strict-lane compensation checks, alias
  continuity checks, and conversion-derived dat provenance row-consistency
  gates before promotable Tier-A replay closure claims are published.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-014: Replay provenance producers must publish explicit
  baseline-year policy metadata (including keyed-span expansion/rekey behavior
  when legacy baseline clamps) and must report full-span keyed precipitation
  parity diagnostics (`P`) under that policy before promotable parity claims.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-P-015: WB13 publication producers must preserve simulation-owned
  lineage for `Ep`/`Es`/`Er`, `Total-Soil`/`SoilWaterTotal`, and
  `ProfileDepth`/`ProfilePorosityCap`/`ProfileFCStore`/`ProfileWPStore`,
  including explicit alias continuity to baseline WB13 semantics;
  projection-side surrogate reconstruction is forbidden.
  `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-SYSTEM-C-001: Channel and impoundment consumers must reject malformed
  handoff payloads explicitly and propagate invariant IDs.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-C-002: Downstream routing/reporting consumers must preserve units
  and branch semantics for peak-flow and sediment payloads.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SYSTEM-C-003: Consumers must not reinterpret absent contributor payloads
  as zero unless the governing branch explicitly defines that behavior.
  `[INFERENCE][Static]`
- OBL-SYSTEM-C-004: Comparator/replay consumers must classify violations at
  tier-appropriate gates and preserve `HOLD` when governance guards remain open.
  `[INFERENCE][Static]`
- OBL-SYSTEM-C-005: Reporting/replay consumers must reject WB13 ET and
  soil-water publications that cannot be traced to simulation-owned WB11
  lineage with declared alias continuity.
  `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Pass-file completeness and runon decomposition (`INV-SYSTEM-001/002/003`) | hillslope->channel/impoundment boundary ingest | Hard error with invariant ID and field-level diagnostics; block boundary publish | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Channel runoff/peak-flow branch semantics (`INV-SYSTEM-004/005/006`) | channel assembly and outlet-peak routines | Hard error on case-logic or method-branch mismatch | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Impoundment hydraulic/outflow continuity (`INV-SYSTEM-007/008`) | impoundment routing integration loop | Hard error on continuity/regime-transition/outflow-sum failure | Tier-B gate | `[DIRECT][Static]` |
| Sediment continuity (`INV-SYSTEM-009`) | channel and impoundment sediment routines | Hard error on unresolved continuity residual | Tier-B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Breakpoint forcing governance (`INV-SYSTEM-010`) | review/verification/promotion cycle | Governance `HOLD` until companion forcing/route contracts close boundary semantics | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Coupled lane publication closure (`INV-SYSTEM-011`) | hillslope->watershed publish boundary | Hard error when publish is attempted after failed/invalid plant-water coupled lane closure | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Strict replay artifact/provenance completeness (`INV-SYSTEM-012`) | comparator staging + replay disposition boundary | Hard error / `HOLD` when required replay artifacts or provenance hashes are missing; no fallback artifact substitution | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL15 residual Tier-A governance closeout (`INV-SYSTEM-013`) | comparator disposition and PL08 hold-lift decision boundary | Governance `HOLD` unless unresolved Tier-A deltas are either closed or explicitly risk-accepted with approval reference; no silent down-classification | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL14R rerun reproducibility completeness (`INV-SYSTEM-014`) | comparator rerun staging + provenance publication boundary | Hard error / `HOLD` when required interchange surfaces, strict comparator JSON outputs, or binary/tool/output hashes are missing from rerun evidence | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL15R recloseout supersession governance (`INV-SYSTEM-015`) | comparator re-disposition and refreshed PL08 hold-lift decision boundary | Governance `HOLD` when active blocker classification ignores superseding schema-aligned strict replay evidence, or when post-supersession unresolved blockers lack explicit risk-acceptance reference | Tier-A closeout gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB20 forward-solver parity lane governance (`INV-SYSTEM-016`) | parity-lane evidence publication and disposition boundary | Governance `HOLD` when WB20 lane-manifest/no-substitution evidence is absent or does not prove observed-target exclusion from acceptance-driving closure logic | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| PL14S semantic replay diagnostics completeness (`INV-SYSTEM-017`) | semantic comparator artifact publication and provenance boundary | Hard error / `HOLD` when semantic comparator report content or strict-skip provenance status is absent, malformed, or silently suppressed | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMPIPE runner execution ownership closure (`INV-SYSTEM-018`) | runner publication and replay-staging boundary | Hard error when required publication surfaces are emitted without executed runner->scheduler lifecycle provenance | SIMIMPL execution gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMMODE publication lane-provenance closure (`INV-SYSTEM-019`) | publication provenance manifest boundary | Hard error when requested/effective mode provenance is absent or lane identity diverges from effective mode | SIMIMPL execution gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMOUT simulation-owned replay-surface closure (`INV-SYSTEM-020`) | replay candidate publication boundary | Hard error / `HOLD` when required candidate surfaces are projection/synthesis-first instead of simulation-owned execution outputs | Tier-A replay integrity gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMCONS consolidated-intake governance closure (`INV-SYSTEM-021`) | consolidated-kernel adoption boundary | Governance `HOLD` when intake claims lack explicit triage disposition or include untriaged policy overlays | Consolidated-intake gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL14 replay-span and key-domain closure (`INV-SYSTEM-022`) | runner manifest + replay-surface publication boundary | Hard error / `HOLD` when climate-span execution, WB13 row-span, monotonic key progression, or simulation-year key-domain assertions fail | SIMIMPL replay comparability gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL15 replay-lane policy/source closure (`INV-SYSTEM-023`) | replay provenance manifest boundary | Hard error / `HOLD` when strict/parquet lane policy metadata or candidate source class is absent/ambiguous; conversion-derived dat strict evidence remains non-promotable for final Tier-A closeout | SIMIMPL replay tooling alignment gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL15 semantic alias/diagnostic structural closure (`INV-SYSTEM-024`) | semantic report publication boundary | Hard error / `HOLD` when `Total-Soil` alias continuity is unresolved or width diagnostics use placeholder sentinel classes instead of observed row widths | SIMIMPL replay tooling alignment gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL16 replay contract-derived test-coverage closure (`INV-SYSTEM-025`) | replay governance/test evidence boundary | Hard error / `HOLD` when closure tests for span/key overlap, strict-lane compensation, alias continuity, or conversion-derived dat row-consistency are missing/failing | SIMIMPL replay contract-test closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL18 baseline-year policy and precipitation full-span closure (`INV-SYSTEM-026`) | replay provenance + semantic parity publication boundary | Hard error / `HOLD` when baseline-year adaptation policy is absent/implicit or when keyed full-span precipitation comparability (`P`) is not demonstrated under the declared policy | SIMIMPL replay span-policy gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| SIMIMPL21/HPARITY02 WB13 ET/soil-water/profile publication lineage closure (`INV-SYSTEM-027`) | WB13/reporting publication boundary | Hard error / `HOLD` when required ET/soil-water/profile WB13 outputs are not simulation-owned outputs with traceable WB11/WB13 lineage and declared alias continuity | SIMIMPL ET/soil-water/profile publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not bitwise
parity). Integration-boundary interpretation tolerances for review/comparator
surfaces are:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-SYSTEM-001 | Runon volume closure residual for Eq. [13.2.1] | `<= 1e-9 m^3` | Comparator interpretation bound; runtime still hard-fails on material mismatch. | `[INFERENCE][Static]` |
| TOL-SYSTEM-002 | Runon-depth conversion residual for Eq. [13.2.2] | `<= 1e-12 m` | Applied after unit-normalized conversion. | `[INFERENCE][Static]` |
| TOL-SYSTEM-003 | Outlet-flow continuity residual for Eq. [14.2.1]/[14.2.5] over one integration step | `<= 1e-8 ft^3` | Comparator-only tolerance for numerical integration noise. | `[INFERENCE][Static]` |
| TOL-SYSTEM-004 | Outflow-summation residual for Eq. [14.3.18] | `<= 1e-10 ft^3 s^-1` | Inactive-structure contribution remains exact zero by branch rule. | `[INFERENCE][Static]` |
| TOL-SYSTEM-005 | Sediment continuity residual for Eq. [13.5.17] and Eq. [14.6.1] | `<= 1e-8` in native mass units | Unit-aware residual check by component (`lb` vs `kg` surfaces). | `[INFERENCE][Static]` |
| TOL-SYSTEM-006 | Tier-A strict replay numeric tolerance for PL14 closeout lane | `abs_tol = 0`, `rel_tol = 0` | Comparator-lane authority is strict diff detection; residual disposition belongs to PL15. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-SYSTEM-007 | Tier-A semantic replay tolerance profile for PL14S investigation lane | `default abs=0.1`, `default rel=0.02`, with per-column overrides from `tools/owcmp/configs/pl14s_wat_tolerances.json` | Investigation-grade semantic comparator tolerances are explicit evidence artifacts and do not replace strict-diff structural checks. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Binding Exposure Index

Status: `scstruct05-partial-adjudication-hold`
Evidence mode: `Static`

This index conserves binding residue during SC-SYSTEM context reduction. Rows
mapped to existing IDs are resolved for binding exposure but may remain
core-resident when they carry active guard, schema, vector, or integration
detail. Rows still routed to `science-review-follow-on` are narrower HOLDs with
named owners and next evidence gates; their narrative remains in the binding
core and is not sidecar-eligible.

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `WS11-CHANNEL-ROUTING-PHYSICS-EQUIVALENCE-INTEGRATION-ADDENDUM` | `SC-SYSTEM-001.md#ws11-channel-routing-physics-equivalence-integration-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-001, INV-SYSTEM-005, INV-SYSTEM-006` | `none` | SCSTRUCT05 map-in-core: payload completeness, hydrograph merge/routed gating, and outlet branch identity cover the binding residue; detailed WS11 guard/vector text stays core-resident. Authority: `REF-SYSTEM-CH13-PASSFILE`, `REF-SYSTEM-CH13-PEAKIN`, `REF-SYSTEM-CH13-PEAKOUT`, `REF-SYSTEM-WSHPEK-IPEAK`, `REF-SYSTEM-WSHCHR-WAVE`. |
| `WS12-IMPOUNDMENT-PHYSICS-EQUIVALENCE-INTEGRATION-ADDENDUM` | `SC-SYSTEM-001.md#ws12-impoundment-physics-equivalence-integration-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-007, INV-SYSTEM-008` | `none` | SCSTRUCT05 map-in-core: impoundment continuity/adaptive-step and outflow aggregation cover the binding residue; WS12 vector text stays core-resident. Authority: `REF-SYSTEM-CH14-HYDCONT`, `REF-SYSTEM-CH14-ADAPT`, `REF-SYSTEM-CH14-OUTFLOW`. |
| `ARCH22-TYPED-PRODUCTION-SURFACE-ADDENDUM` | `SC-SYSTEM-001.md#arch22-typed-production-surface-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: typed accessor/signature obligations are not exactly exposed by current `INV-SYSTEM-*` rows. Owner: `SCSTRUCT05-ARCH22-BEI-PROMOTION`. Next gate: promote/map typed production-surface boundary authority before relocation. |
| `EROD12-CROSS-DOMAIN-OWNERSHIP-AND-GUARD-CLOSURE-ADDENDUM` | `SC-SYSTEM-001.md#erod12-cross-domain-ownership-and-guard-closure-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: multi-contract ownership/guard lanes need exact system/producer/consumer binding exposure. Owner: `SCSTRUCT05-CROSSDOMAIN-BEI-PROMOTION`. Next gate: map to precise SYSTEM/SED/HYDRAULICS/ROUTE binding IDs before relocation. |
| `SIMIMPL03-PRODUCTION-RUNNER-AND-PUBLICATION-PROVENANCE-ADDENDUM` | `SC-SYSTEM-001.md#simimpl03-production-runner-and-publication-provenance-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-018, INV-SYSTEM-019, INV-SYSTEM-020, INV-SYSTEM-021` | `none` | SCSTRUCT05 map-in-core: runner execution ownership, mode propagation, simulation-owned replay surfaces, and selective consolidated intake are directly exposed by existing invariants and guard map rows. |
| `SIMIMPL14-CONTINUOUS-REPLAY-SPAN-AND-KEY-DOMAIN-ADDENDUM` | `SC-SYSTEM-001.md#simimpl14-continuous-replay-span-and-key-domain-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-022` | `none` | SCSTRUCT05 map-in-core: continuous span, row closure, monotonic key progression, and simulation-year key semantics are exposed by `INV-SYSTEM-022`. |
| `SIMIMPL15-REPLAY-COMPARATOR-TOOLING-ALIGNMENT-ADDENDUM` | `SC-SYSTEM-001.md#simimpl15-replay-comparator-tooling-alignment-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-023, INV-SYSTEM-024` | `none` | SCSTRUCT05 map-in-core: strict/parquet policy, candidate-source classification, alias continuity, and row-width diagnostics are exposed by `INV-SYSTEM-023` and `INV-SYSTEM-024`. |
| `SIMIMPL16-REPLAY-CONTRACT-DERIVED-TEST-COVERAGE-CLOSURE-ADDENDUM` | `SC-SYSTEM-001.md#simimpl16-replay-contract-derived-test-coverage-closure-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-025` | `none` | SCSTRUCT05 map-in-core: replay contract-derived closure-test coverage is exposed by `INV-SYSTEM-025`. |
| `SIMIMPL18-BASELINE-YEAR-POLICY-AND-PRECIPITATION-SPAN-CLOSURE-ADDENDUM` | `SC-SYSTEM-001.md#simimpl18-baseline-year-policy-and-precipitation-span-closure-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-026` | `none` | SCSTRUCT05 map-in-core: baseline-year policy and full-span precipitation parity are exposed by `INV-SYSTEM-026`. |
| `SIMIMPL21-WB13-ET-SOIL-WATER-PUBLICATION-LINEAGE-ADDENDUM` | `SC-SYSTEM-001.md#simimpl21-wb13-etsoil-water-publication-lineage-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-027` | `none` | SCSTRUCT05 map-in-core: WB13 ET, soil-water, and profile-capacity simulation-owned lineage is exposed by `INV-SYSTEM-027`. |
| `HPHYS0202-WB13-PROFILE-FC-WP-PUBLICATION-LINEAGE-ADDENDUM-HISTORICAL` | `provenance/SC-SYSTEM-001-provenance.md#hphys0202-wb13-profile-fcwp-publication-lineage-addendum-historical` | `historical` | `historical-or-superseded` | `INV-SYSTEM-027` | `none` | SCSTRUCT05 relocated: historical layer-aggregation/projection narrative is superseded by retained HPHYS0207 and live `INV-SYSTEM-027` lineage authority. |
| `HPHYS0205-CORRECTED-LAYER-PROJECTION-ADDENDUM-HISTORICAL` | `provenance/SC-SYSTEM-001-provenance.md#hphys0205-corrected-layer-projection-addendum-historical` | `historical` | `historical-or-superseded` | `INV-SYSTEM-027` | `none` | SCSTRUCT05 relocated: historical corrected-layer projection narrative is superseded by retained HPHYS0207 and live `INV-SYSTEM-027` lineage authority. |
| `HPHYS0206-NORMALIZED-LAYER-MAPPING-AND-FAIL-CLOSED-ADDENDUM-HISTORICAL` | `provenance/SC-SYSTEM-001-provenance.md#hphys0206-normalized-layer-mapping-and-fail-closed-addendum-historical` | `historical` | `historical-or-superseded` | `INV-SYSTEM-027` | `none` | SCSTRUCT05 relocated: historical normalized-layer mapping narrative is superseded by retained HPHYS0207 and live `INV-SYSTEM-027` lineage authority. |
| `HPHYS0207-NORMALIZED-PROFILE-FC-WP-DEPTH-AUTHORITY-ADDENDUM` | `SC-SYSTEM-001.md#hphys0207-normalized-profile-fcwp-depth-authority-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-027` | `none` | SCSTRUCT05 map-in-core: normalized-profile FC/WP and ordering continuity are covered by `INV-SYSTEM-027`; detailed depth-authority text remains core-resident. |
| `HPHYS0216D-PROFILEFC-LAYER-TAIL-BOUNDARY-AUTHORITY-ADDENDUM` | `SC-SYSTEM-001.md#hphys0216d-profilefc-layertail-boundary-authority-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-027` | `none` | SCSTRUCT05 map-in-core: ProfileFC layer+tail publication authority maps to `INV-SYSTEM-027`; detailed tail-boundary rule stays core-resident. |
| `HPHYS0203-WB13-ROBUSTNESS-GOVERNANCE-ADDENDUM` | `SC-SYSTEM-001.md#hphys0203-wb13-robustness-governance-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: robustness vector scope includes subsurface-loss families beyond exact current `INV-SYSTEM-027` exposure. Owner: `SCSTRUCT05-HPHYS0203-BEI-PROMOTION`. Next gate: promote/map robustness obligations before relocation. |
| `HPHYS0208-COUPLED-WB13-PUBLICATION-LINEAGE-ADDENDUM` | `SC-SYSTEM-001.md#hphys0208-coupled-wb13-publication-lineage-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: coupled WB13 lineage spans `ProfileFCStore`, `Dp`, `latqcc`, soil-water, and threshold seeds beyond exact current system binding exposure. Owner: `SCSTRUCT05-HPHYS0208-BEI-PROMOTION`. Next gate: promote/map coupled lineage before relocation. |
| `HPHYS0218-WB19-DRFC-THRESHOLD-GOVERNANCE-ADDENDUM` | `SC-SYSTEM-001.md#hphys0218-wb19-drfc-threshold-governance-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: WB19 `drfc` threshold and `coca` guard authority need exact WATBAL/SUBHYD/SYSTEM exposure. Owner: `SCSTRUCT05-WB19-THRESHOLD-BEI-PROMOTION`. Next gate: promote/map before relocation. |
| `HPHYS0221-WB19-COUPLED-SATURATED-DEPTH-GOVERNANCE-ADDENDUM` | `SC-SYSTEM-001.md#hphys0221-wb19-coupled-saturated-depth-governance-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: WB19 `solwpv`, saturated-depth, and lateral writeback coupling need exact binding exposure. Owner: `SCSTRUCT05-WB19-SATDEP-BEI-PROMOTION`. Next gate: promote/map before relocation. |
| `HPHYS0209-PROFILEWP-NEAR-CLOSED-PUBLICATION-ADJUDICATION-ADDENDUM` | `SC-SYSTEM-001.md#hphys0209-profilewp-near-closed-publication-adjudication-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-027` | `none` | SCSTRUCT05 map-in-core: ProfileWP simulation-owned publication lineage and profile geometry non-regression map to `INV-SYSTEM-027`; adjudication caveats remain core-resident. |
| `MOFE04-MULTI-OFE-WB13-WAT-PUBLICATION-BOUNDARY-CARRY-ADDENDUM` | `SC-SYSTEM-001.md#mofe04-multi-ofe-wb13wat-publication-boundary-carry-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: MOFE row identity, contributor cardinality, and area-policy obligations are not fully exposed by current `INV-SYSTEM-029`. Owner: `SCSTRUCT05-MOFE04-BEI-PROMOTION`. Next gate: promote/map publication policy authority before relocation. |
| `HPHYS0255-MOFE-STORAGE-LINEAGE-PUBLICATION-ADDENDUM` | `SC-SYSTEM-001.md#hphys0255-mofe-storage-lineage-publication-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-029` | `none` | SCSTRUCT05 map-in-core: MOFE storage-lineage policy and area/storage separation are exposed by `INV-SYSTEM-029`; detailed policy text remains core-resident. |
| `MOFE01-M-E0-PER-OFE-DYNAMIC-STATE-PUBLICATION-POLICY-ADDENDUM` | `SC-SYSTEM-001.md#mofe01-m-e0-per-ofe-dynamic-state-publication-policy-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-030` | `none` | MOFE01 M-E0: per-OFE dynamic-state publication policy, row cardinality, identity-status manifest gates, and aggregate-row relabel rejection are directly exposed by `INV-SYSTEM-030`. |
| `MOFE01-M-F-REDO-PER-OFE-PUBLICATION-ANTI-CLONE-ADDENDUM` | `SC-SYSTEM-001.md#mofe01-m-f-redo-per-ofe-publication-anti-clone-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-031` | `none` | MOFE01 M-F-REDO/M-F-REDO-CLONE/M-F-REDO2: per-OFE publication manifest/consumer gates require anti-clone hydrology-vector and raw local-runoff evidence, nonzero active surface handoff, WB14 multistep infiltration lineage, public `Q`/`QOFE` length-normalization evidence, and lane-local lineage beyond row cardinality or conservation closure. |
| `MOFE01-M-G-EROD14-QIN-MANIFEST-BOUNDARY-ADDENDUM` | `SC-SYSTEM-001.md#mofe01-m-g-erod14-qin-manifest-boundary-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-032` | `none` | MOFE01 M-G: manifests and downstream consumers must distinguish water-transfer-only EROD14 `qin` compatibility seeding from accepted sediment-coupled `qin`, exposing `erod14_qin_source_policy` and `erod14_qin_sediment_coupled` until the SED-owned prior-OFE `qout`/particle-fraction follow-on closes. |
| `MOFE05-WATERSHED-CONTRIBUTOR-METADATA-INTAKE-VALIDATION-ADDENDUM` | `SC-SYSTEM-001.md#mofe05-watershed-contributor-metadata-intake-validation-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: contributor metadata intake shape, consistency, and test vectors extend beyond exact `INV-SYSTEM-028/029` exposure. Owner: `SCSTRUCT05-MOFE05-BEI-PROMOTION`. Next gate: promote/map intake validation authority before relocation. |
| `HPHYS0241-MOFE-HOURLY-CARRY-METADATA-ADDENDUM` | `SC-SYSTEM-001.md#hphys0241-mofe-hourly-carry-metadata-addendum` | `active` | `maps-to-existing-INV` | `INV-SYSTEM-028` | `none` | SCSTRUCT05 map-in-core: active 24-slot MOFE hourly carry metadata and watershed intake rejection posture are exposed by `INV-SYSTEM-028`. |
| `EROD13-WAVE-1-ACTIVE-BOUNDARY-CARRY-ADDENDUM` | `SC-SYSTEM-001.md#erod13-wave-1-active-boundary-carry-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: Wave-1 hydrology-to-erosion boundary-carry authority requires exact RUNOFFPART/WATBAL/SED/SYSTEM exposure. Owner: `SCSTRUCT05-EROD13-BEI-PROMOTION`. Next gate: promote/map before relocation. |
| `EROD14-WAVE-2-ACTIVE-BOUNDARY-CARRY-ADDENDUM` | `SC-SYSTEM-001.md#erod14-wave-2-active-boundary-carry-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: Wave-2 sediment enrichment/class-conservation boundary-carry authority requires exact SED/SYSTEM exposure. Owner: `SCSTRUCT05-EROD14-BEI-PROMOTION`. Next gate: promote/map before relocation. |
| `EROD15-WAVE-3-HBP-BOUNDARY-CARRY-ADDENDUM` | `SC-SYSTEM-001.md#erod15-wave-3-hbp-boundary-carry-addendum` | `active` | `undecidable` | `none` | `science-review-follow-on` | SCSTRUCT05 narrower HOLD: Wave-3 HBP routing-boundary payload authority requires exact SED/ROUTE/SYSTEM exposure, including `SC-ROUTE-001#INV-ROUTE-011`. Owner: `SCSTRUCT05-EROD15-BEI-PROMOTION`. Next gate: promote/map before relocation. |

## WS11 Channel-Routing Physics-Equivalence Integration Addendum

### WS11 Integration Runtime Aliases

| Surface | Symbols |
|---|---|
| Channel runtime controls | `ws10_channel_{id}_chnn`, `ws10_channel_{id}_ctlslp`, `ws10_channel_{id}_chnk`, `ipeak` |
| Channel segment/hydraulic scaffold controls | `ws10_channel_{id}_nslpts`, `ws10_channel_{id}_x_{point:04}`, `ws10_channel_{id}_slope_{point:04}`, `ws10_channel_{id}_depa_{point:04}`, `ws10_channel_{id}_depb_{point:04}`, `ws10_channel_{id}_wida_{point:04}`, `ws10_channel_{id}_widb_{point:04}` |
| Channel runtime outputs | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff` |
| Impoundment runtime controls | `ws10_impoundment_{id}_h`, `ws10_impoundment_{id}_hfull`, `ws10_impoundment_{id}_deltat`, `ws10_impoundment_{id}_qinf` |
| Impoundment runtime outputs | `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout`, `ws10_impoundment_{id}_hnext`, `ws10_impoundment_{id}_outflow_volume` |
| Hillslope contributor payloads | `hs{ID}_peakro`, `hs{ID}_watdur` |

### WS11 Integration Rules

1. Watershed-node execution order must honor topology dependencies and consume
   upstream WS11 payloads through explicit symbol references, not implicit
   fallback defaults.
2. WS11 channel integration authority must preserve explicit `ipeak` branch
   semantics (`1` Rational, `2` CREAMS, `3` kinematic wave, `>=4`
   Muskingum-Cunge) with deterministic branch identity.
3. WS11 routing authority must not collapse channel execution into the
   pre-WS11 gain-factor surrogate `(1 + ctlslp) / (1 + chnn)` or equivalent
   single-gain substitutions.
4. Missing/non-finite/out-of-domain WS11 boundary payloads are hard-fail system
   integration states and must propagate typed guard IDs unchanged.
5. System integration must preserve deterministic publish ordering and make
   WS11 dependency payload availability observable at the node boundary.
6. WS11 integration pathways must not silently clamp or synthesize replacement
   state/flux values to repair invalid boundary inputs.
7. WS11 Muskingum-Cunge integration (`ipeak >= 4`) must preserve prior
   wave-state memory continuity when prior channel wave-state symbols are
   available (`ws10_channel_{id}_qin`, `ws10_channel_{id}_q1`), and must
   preserve finite signed MC coefficient publication semantics (`c1/c2/c3`)
   without non-physical non-negative clamp repair.
8. WS11 `ipeak = 5` integration must execute variable-parameter
   Muskingum-Cunge dynamic-coefficient refresh semantics for the current
   single-segment lane, rather than reusing static `ipeak = 4` coefficient
   families when dynamic refresh inputs are valid.

### WS11 Guard Families

| Kernel lane | Guard IDs |
|---|---|
| Channel lane | `WKERNEL-WS10-CHANNEL-E-001..003` |
| Impoundment lane | `WKERNEL-WS10-IMPOUNDMENT-E-001..003` |

### WS11 Contract-Derived System Vectors

Minimum WS11 integration vectors:
1. Deterministic `channel -> impoundment -> downstream channel` topology run
   emits finite non-negative WS11 channel outputs at each node boundary.
2. `ipeak = 3` and `ipeak >= 4` vectors preserve routed closure semantics
   (`roff = qpo * durrof` within tolerance) on published channel payloads.
3. Missing required WS11 symbol halts node execution with `-E-001`.
4. Non-finite WS11 symbol halts node execution with `-E-002`.
5. Domain/dependency WS11 violation halts node execution with `-E-003`.
6. `ipeak >= 4` vectors with seeded prior channel wave-state symbols
   (`ws10_channel_{id}_{qin,q1}`) produce deterministic branch-response deltas
   versus no-prior-state vectors while preserving finite/non-negative routed
   outputs (`qpo`, `durrof`, `roff`).
7. `ipeak >= 4` vectors preserve finite MC coefficient-state publication
   (`c0..c4`) with signed-coefficient continuity for `c1/c2/c3` where branch
   physics yields negative values.
8. `ipeak = 5` vectors demonstrate dynamic MC coefficient refresh continuity
   (dynamic `c0..c4` recomputation) and branch-output divergence from static
   `ipeak = 4` coefficient behavior under matched forcing/prior-state seeds.

## WS12 Impoundment Physics-Equivalence Integration Addendum

### WS12 Integration Runtime Aliases and Coefficient Families

| Surface | Symbols |
|---|---|
| Impoundment runtime controls | `ws10_impoundment_{id}_h`, `ws10_impoundment_{id}_hfull`, `ws10_impoundment_{id}_deltat`, `ws10_impoundment_{id}_qinf` |
| Impoundment required parser-projected coefficient families | canonical impoundment coefficient/threshold surfaces (`a,b,c,d,e,ha,ht,hlm,a0,a1,a2,l0,l1,l2`) from `SC-INFILE-WATERSHED-IMPOUNDMENT-001` |
| Impoundment runtime outputs | `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout`, `ws10_impoundment_{id}_hnext`, `ws10_impoundment_{id}_outflow_volume` |
| Upstream dependency payloads | `hs{ID}_peakro`, `hs{ID}_watdur`, `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout` |

### WS12 Integration Rules

1. Impoundment-node integration authority is continuity + stage-discharge
   routing with adaptive-step retry/regime-transition handling; the WS10
   headroom surrogate is non-authoritative for WS12 parity claims.
2. Regime-boundary crossings must retry with initial timestep reset before
   publish; unresolved crossings are hard-fail integration states.
3. Outflow assembly must preserve structure-control minima and additive
   contribution semantics required by `SC-IMPOUND-001` WS12 authority.
4. Missing/non-finite/out-of-domain WS12 impoundment boundary payloads must
   propagate typed guard IDs unchanged (`WKERNEL-WS10-IMPOUNDMENT-E-001..003`).
5. WS12 integration pathways must not silently clamp/synthesize replacement
   state/flux values to repair invalid impoundment boundary inputs.

### WS12 Contract-Derived System Vectors

Minimum WS12 integration vectors:
1. Deterministic topology run (`channel -> impoundment -> downstream channel`)
   emits finite non-negative WS12 impoundment outputs and preserves dependency
   provenance.
2. Regime-transition crossing vector triggers retry/reset behavior prior to
   publication; invalid unresolved transition fails with `-E-003`.
3. Missing required WS12 impoundment symbol/coefficient payload halts node
   execution with `WKERNEL-WS10-IMPOUNDMENT-E-001`.
4. Non-finite WS12 impoundment symbol/intermediate halts node execution with
   `WKERNEL-WS10-IMPOUNDMENT-E-002`.
5. Domain/continuity violation halts node execution with
   `WKERNEL-WS10-IMPOUNDMENT-E-003`.

## ARCH22 Typed Production-Surface Addendum

### Typed Runtime Surface Authority

1. Covered production system-integration interfaces must resolve boundary-state
   and boundary-flux surfaces via ARCH22 typed symbol families:
   `HillslopeProductionStateSymbol`, `HillslopeProductionFluxSymbol`,
   `WatershedProductionStateSymbol`, and `WatershedProductionFluxSymbol`.
2. Covered production guard/accessor helper signatures must not accept raw
   `&str` symbol identifiers where typed ARCH22 symbols exist.
3. Typed migration must preserve deterministic publication ordering, dependency
   visibility, and existing hard-fail boundary classes/message IDs.

### Contract-Derived Migration Vectors

1. Static migration proof: covered production integration accessors use typed
   symbol families, not stringly `&str` parameters.
2. Nominal migration vector: coupled hillslope/watershed production execution
   preserves deterministic state/flux publication semantics.
3. Failure migration vectors: missing/non-finite/domain/dependency violations
   preserve existing typed hard-fail boundary classes and guard IDs.

## EROD12 Cross-Domain Ownership and Guard Closure Addendum

| Cross-domain lane | Producer ownership | Consumer guard ownership | Closure posture | Evidence |
|---|---|---|---|---|
| Hydrology forcing to erosion/routing (`Q`, `peakro`, `watdur`, `wb16_*`) | `SC-RUNOFFPART-001` + `SC-WATBAL-001` | `SC-SED-001`, `SC-HYDRAULICS-001`, `SC-ROUTE-001` | Required Wave-0 ownership/guard semantics are explicit in canonical companion contracts. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Hydraulics-to-erosion coupling (`fr`, `fi/fe`, `w`, `fs`, `ft`, `τf/τfe`) | `SC-HYDRAULICS-001` | `SC-SED-001` | Producer and consumer guard ownership is canonicalized for required boundaries. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Sediment export to routing (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_flow_fraction_i`) | `SC-SED-001` | `SC-ROUTE-001` | Routing consumer guard ownership for sediment handoff completeness is explicit. | `[DIRECT][Static] + [INFERENCE][Static]` |

## SIMIMPL03 Production Runner and Publication-Provenance Addendum

### System-Level Execution Ownership Rules

1. System publication authority for hillslope replay candidate surfaces is
   execution-owned and requires successful runner -> scheduler/kernel lane
   execution provenance.
2. Projection-first helper publication is non-authoritative for required
   candidate surfaces once execution-owned publication is claimed.
3. Required publication provenance minimum:
   - requested/effective `wepp_ui` mode;
   - selected runtime lane identity (`daily`/`hourly`);
   - execution result status and typed guard code when failed.

### Consolidated Intake Governance Rules

1. Consolidated intake from `/workdir/wepp-forest/fpm-src` is selective and
   must be explicitly triaged per kernel/policy family before integration
   claims are accepted.
2. qcap-style clamp policy overlays remain non-authoritative until explicit
   triage disposition and guard mapping are recorded.
3. Missing triage disposition is a governance blocker for downstream execution
   promotion.

## SIMIMPL14 Continuous Replay-Span and Key-Domain Addendum

1. Runner publication claims for replay-ready hillslope surfaces require
   continuous day progression across the full climate forcing span for the run.
2. System provenance must expose, at minimum, `climate_day_count`,
   `executed_day_count`, `wb13_row_count`, first/last replay row keys, and
   monotonic `sim_day_index` verdict.
3. Replay row keys must remain comparator-aligned with simulation-year
   semantics for `Y` (`calendar_year - start_year + 1`), not absolute
   calendar-year keys.
4. Any span/key closure failure is a typed hard-fail/HOLD condition under
   `WS-SIMOUT-E-001`; silent fallback to one-day or calendar-year keyed output
   is non-authoritative.

## SIMIMPL15 Replay Comparator Tooling Alignment Addendum

1. System replay provenance must publish deterministic strict/parquet lane
   policy metadata for each comparison run (`strict-required` for `.dat`,
   `strict-equivalent-required` for `.parquet`).
2. Candidate source classification is required at provenance boundary and must
   use canonical classes: `native-runtime-dat`, `conversion-derived-dat`, or
   `native-runtime-parquet`.
3. Conversion-derived dat strict evidence remains diagnostic-only and cannot be
   treated as promotable final Tier-A closeout evidence.
4. Semantic parquet evidence must preserve `Total-Soil` investigation-column
   continuity across accepted aliases and publish observed row-width
   diagnostics for format comparability with dat lanes.
5. Missing policy/source metadata or unresolved alias/diagnostic continuity is
   a hard-fail/HOLD condition under `WS-SIMOUT-E-001`.

## SIMIMPL16 Replay Contract-Derived Test-Coverage Closure Addendum

1. System replay governance evidence is promotable only when contract-derived
   tests for `SIMIMPL13-TEST-001..005` are present and passing.
2. Span/key closure tests must explicitly fail on replay row-span collapse and
   simulation-year key-domain mismatch before comparator promotion assertions.
3. Strict-lane governance tests must compensate for parquet strict-lane skips
   by requiring strict-equivalent semantic evidence readiness.
4. Conversion-derived dat provenance tests must assert row-consistency against
   baseline replay spans before final Tier-A promotability claims.
5. Alias continuity tests must preserve `Total-Soil` investigation lineage and
   block regressions that reopen semantic column-mapping drift.

## SIMIMPL18 Baseline-Year Policy and Precipitation-Span Closure Addendum

1. Replay provenance must publish explicit baseline-year policy metadata when
   legacy baseline execution clamps to one year (for example declared expansion
   or rekey behavior used for full-span comparability).
2. Baseline/candidate parity artifacts must retain identical input/sidecar
   provenance references and fail closed when lane input identity diverges.
3. Full-span keyed precipitation (`P`) parity claims must be evaluated over the
   declared common keyed horizon under that explicit policy, not overlap-only
   subsets.
4. First-day keyed diagnostics (`P`, `RM`, `Snow-Water`, `Total-Soil`,
   `frozwt`, `SoilWaterTotal`) and multi-day storage mutation diagnostics are
   required evidence surfaces for follow-on hydrology closure waves.

## SIMIMPL21 WB13 ET/Soil-Water Publication Lineage Addendum

1. System publication authority explicitly includes WB13 ET components
   (`Ep`, `Es`, `Er`) and aggregate soil-water outputs (`Total-Soil`,
   `SoilWaterTotal`) as simulation-owned runtime products.
2. Publication lineage must remain traceable to canonical WB11 layer-water
   closure (`st(i)`/`soilw(i)` -> `watcon`) plus declared frozen/snow
   composition semantics before report/replay emission.
3. Comparator/replay/reporting pathways may canonicalize accepted aliases
   (for example legacy `Total-Soil Water`) but must not fabricate surrogate
   values when required simulation-owned symbols are missing.
4. Contract-derived follow-on tests must include lineage-preservation vectors
   that fail closed on missing alias continuity or projection-side
   reconstruction.

## HPHYS0207 Normalized-Profile FC/WP Depth-Authority Addendum

1. System-boundary publication authority for WB13 `ProfileFCStore` and
   `ProfileWPStore` is normalized-profile runtime storage symbols
   `wb13_profile_fc_store_mm` and `wb13_profile_wp_store_mm`.
2. These FC/WP storage symbols must be runtime-owned, baseline-corrected,
   normalized-profile aggregates sharing depth authority with
   `wb13_profile_depth_mm` and `wb13_profile_porosity_cap_mm`.
3. Residual normalized-tail depth beyond OFE parser-layer publication depth is
   consumed by normalized-profile storage projection authority; silent
   normalized-tail truncation and parser-domain fallback publication are
   prohibited.
4. Required WB13 profile ordering continuity remains:
   `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.

## HPHYS0216D ProfileFC Layer+Tail Boundary Authority Addendum

1. System-boundary publication authority for WB13 `ProfileFCStore` is
   `Σ(thetfc_i * dg_i) * 1000 + wb13_profile_fc_tail_mm`.
2. `wb13_profile_fc_tail_mm` is a required runtime-owned boundary symbol that
   represents normalized-profile residual FC contribution not covered by parser
   layer aggregation.
3. `wb13_profile_fc_store_mm` remains a diagnostic/reconciliation carry symbol
   and must reconcile with the combined layer+tail authority above; it is not
   a direct publication-driving value.
4. Missing/non-finite/negative `wb13_profile_fc_tail_mm` is a typed hard-fail
   WB13 boundary violation.
5. Required WB13 profile ordering continuity remains:
   `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.

## HPHYS0203 WB13 Robustness Governance Addendum

1. System-level closure evidence for WB13 hydrology publication families must
   include contract-derived robustness vectors for:
   - profile publications (`ProfileDepth`, `ProfilePorosityCap`,
     `ProfileFCStore`, `ProfileWPStore`),
   - soil-water aggregates (`Total-Soil`, `SoilWaterTotal`),
   - subsurface-loss publications (`latqcc`, `Dp`).
2. Robustness evidence must show conservation/order/domain continuity and typed
   hard-fail posture for missing/non-finite/domain-invalid publication inputs.
3. Deterministic regression fixtures used for robustness evidence must remain
   reproducible and explicitly scoped to the targeted publication families.
4. Semantic comparator outputs remain diagnostic signals and must not override
   process-authoritative robustness gate outcomes.

## HPHYS0208 Coupled WB13 Publication Lineage Addendum

1. System-level closure claims for coupled WB13 publication families
   (`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`) require
   explicit WB11 threshold-lineage ownership evidence:
   `sat`, `por_####`, `cpm_####`, `thetfc_####`, `thetdr_####`, `dg_####`.
2. WB13 coupled publication outputs must remain traceable through seeded
   storage-consumer lineage:
   `st(i)` -> `wb18_perc_theta/fc/ul_####` -> WB18/WB19 consumers ->
   WB13 publication families.
3. Missing/non-finite/domain-invalid threshold-lineage seeds are typed
   hard-fail system-boundary states for coupled WB13 closure claims; surrogate
   publication fallback is prohibited.

## HPHYS0218 WB19 `drfc` Threshold Governance Addendum

1. Coupled WB13 subsurface-loss closure claims (`latqcc`, `Dp`) must remain
   traceable through WB19 threshold authority:
   `drfc_i = wb18_perc_fc_i + (1-coca_i)*dg_i`.
2. System-boundary evidence for WB19 subsurface publication must include
   fail-closed enforcement of required `coca_####` domains (`0 < coca <= 1`)
   with no FC-only fallback threshold execution path.
3. WB19-layer mutation lineage for coupled claims must remain:
   `st(i)` seed -> `wb18_perc_theta/fc_####` + `coca_####` -> WB19
   `drfc_i` threshold execution -> WB13 `latqcc`/`Dp` publication.

## HPHYS0221 WB19 Coupled Saturated-Depth Governance Addendum

1. Coupled WB13 subsurface-loss closure claims must include WB19 `solwpv`
   branch authority in system lineage evidence:
   - `solwpv = 2006`: all saturated layers eligible for lateral partition.
   - `solwpv != 2006`: contiguous near-surface saturated block only.
2. WB19 system-coupled state lineage for lateral partition/saturated-depth
   evolution is explicit:
   - `por_####`, `coca_####`, `wb18_perc_fc_####`, `dg_####`
     -> `wb19_watyld`
   - `q` + `wb19_watyld` + pre-update `fcdep`
     -> `wb19_fcdep`, `wb19_unsdep`.
3. WB19 lateral writeback publications (`wb19_watyld`, `wb19_fcdep`,
   `wb19_unsdep`) are required system-boundary evidence surfaces for
   adjudicating coupled `latqcc`/`Dp`/soil-water residual families.
4. Missing/non-finite/domain-invalid branch/coupling symbols are typed
   hard-fail boundary states with no fallback substitution.

## HPHYS0209 ProfileWP Near-Closed Publication Adjudication Addendum

1. System-boundary authority for `ProfileWPStore` remains direct publication
   from runtime `wb13_profile_wp_store_mm` under HPHYS0207; no adjudication
   path may remap this column through alternate projection formulas.
2. Lane-level adjudication for isolated `ProfileWPStore` residuals is
   diagnostic-only and must preserve:
   - fail-closed WB13 publication guard posture, and
   - non-regressing `ProfileDepth`/`ProfilePorosityCap` profile-geometry
     families.
3. Any residual spread beyond the isolated lane, guard softening, or profile
   geometry/capacity regression invalidates expected-delta classification and
   must retain `HOLD` at system disposition.

## MOFE04 Multi-OFE WB13/WAT Publication Boundary-Carry Addendum

1. System-boundary publication authority for hillslope WB13/H.wat outputs must
   carry explicit MOFE04 canonicalized publication policy provenance:
   - `publication_ofe_policy`
   - `contributor_ofe_count`
   - `area_policy`
   - `publication_area_m2`.
2. In MOFE multi-OFE contexts, published `OFE = 1` is interpreted as canonical
   output-row identity only; contributor topology dimensionality must be taken
   from explicit `contributor_ofe_count` provenance rather than inferred from
   row key alone.
3. System-level replay/reporting consumers must fail closed on missing or
   malformed MOFE04 publication policy provenance fields and must not silently
   reinterpret canonicalized row ids as topology cardinality.
4. `Area` publication semantics at system boundary remain
   `sum-ofe-geometry-area` under MOFE04 and may not regress to
   primary-OFE-only assumptions when `contributor_ofe_count > 1`.

## HPHYS0255 MOFE Storage-Lineage Publication Addendum

1. MOFE publication provenance must include `storage_lineage_policy`.
2. The current policy value is `single-runtime-wb11-state`, meaning WB13/H.wat
   storage fields are simulation-owned WB11/WB13 runtime outputs and are not
   static area-weighted aggregates of OFE soil rows.
3. Downstream system consumers must treat `Area` aggregation and storage
   lineage as separate dimensions. `publication_area_m2` can aggregate
   contributor geometry while `Total-Soil`, `SoilWaterTotal`, and profile
   stores remain tied to the declared storage-lineage policy.
4. If per-OFE dynamic hydrology state is migrated later, the manifest policy
   must change only after canonical contracts define the state vectors,
   aggregation operator, and fail-closed validation behavior.

## MOFE01 M-E0 Per-OFE Dynamic-State Publication Policy Addendum

1. MOFE04 aggregate publication policy remains active until real OFE-keyed
   dynamic water-balance state exists and this addendum's manifest gates pass.
2. The per-OFE policy value is
   `per-ofe-dynamic-water-balance-state`. It is valid only with
   `publication_ofe_policy`, `contributor_ofe_count`, `per_ofe_record_count`,
   `per_ofe_state_policy`, `transfer_identity_status`,
   `per_element_identity_status`, `aggregate_identity_status`, and
   `storage_lineage_policy = "per-ofe-dynamic-wb-state"`.
3. For multi-OFE contributors, `per_ofe_record_count` must equal
   `contributor_ofe_count` for each executed day, and publication row
   cardinality must match after the policy flip.
4. Consumers must reject a per-OFE policy assertion when OFE-keyed records are
   missing, identity statuses are not green, storage lineage is still aggregate,
   or aggregate rows are relabeled as per-OFE records.
5. M-E0 tests must fail on the current aggregate publication architecture until
   manifest-gated per-OFE state and matching row cardinality exist.

## MOFE01 M-F-REDO Per-OFE Publication Anti-Clone Addendum

1. Per-OFE publication manifests and consumers must treat row cardinality and
   monotonic `(day, OFE)` keys as structural checks only. They do not prove that
   records are lane-local or non-cloned.
2. Promotion requires anti-clone evidence for active routed days: lane-local
   source lineage, nonzero adjacent surface handoff from independently stored
   transfer operands, hydrology vectors that are not all-OFE-identical, raw
   non-cloned local runoff, and public runoff-normalization evidence unless a
   documented same-physics rationale is verified.
3. Conservation residual closure is necessary but not sufficient. Manifests and
   downstream consumers must hold when identities close on aggregate-duplicated
   rows, when handoff proof is zero-on-zero only, or when transfer fields are
   synthesized from public WAT rows.
4. M-F-REDO-CLONE requires runtime evidence that declared multistep MOFE/hourly
   WB14 lanes compute lane-local infiltration instead of accepting seeded or
   stale `wb12_infiltration` as a producer-published result.
5. M-F-REDO2 requires publication evidence that `QOFE` is the OFE-local depth
   (`runoff * efflen / slplen`) and `Q` is the cumulative-length depth
   (`runoff * efflen / totlen`). Manifests and consumers must reject downstream
   `QOFE == Q` aliases where `slplen != totlen`, while conservation identities
   remain anchored to raw transfer/runoff operands.

## MOFE01 M-G EROD14 `qin` Manifest Boundary Addendum

1. Wave-2 execution status is not evidence that downstream `erod14_qin` is
   sediment-coupled.
2. Hillslope manifests must publish `erod14_qin_source_policy` and
   `erod14_qin_sediment_coupled` whenever execution provenance reports
   `erod14_wave2_enabled`.
3. The current MOFE01 water package policy is
   `water-transfer-only-mofe01-mg-sediment-coupling-follow-on`, with
   `erod14_qin_sediment_coupled = false`.
4. Downstream consumers must hold any sediment-coupling acceptance claim that
   derives from `UpStrmQ`, `SubRIn`, public WB13/WAT rows, aggregate runoff,
   or Wave-2 kernel status without SED-owned prior-OFE erosion `qout` and
   incoming particle/class-fraction evidence.

## MOFE05 Watershed Contributor Metadata Intake Validation Addendum

1. Watershed contributor intake authority must support per-contributor MOFE
   publication metadata carry through `inputs.hillslopes_block[]` at CLI
   boundary, referencing hillslope run manifest lineage when provided.
2. Multi-OFE contributor intake (`hbp.nofe > 1`) must fail closed unless
   contributor metadata is supplied and parseable from the declared metadata
   source surface.
3. Contributor metadata intake must hard-fail on missing/malformed required
   fields:
   - `wb13_publication.publication_ofe_policy`
   - `wb13_publication.contributor_ofe_count`
   - `wb13_publication.area_policy`
   - `wb13_publication.publication_area_m2`.
4. Contributor metadata consistency must be enforced at watershed intake:
   - `contributor_ofe_count` must equal contributor `hbp.nofe`,
   - `publication_ofe_policy` must remain
     `single-row-canonicalized-hillslope-aggregate`,
   - `area_policy` must remain `sum-ofe-geometry-area`,
   - `publication_area_m2` must be finite and `> 0`.
5. Metadata intake violations are typed hard-fail boundary errors and must
   terminate before watershed routing dispatch; no silent defaults, coercion,
   or inferred substitutions are authorized.

### MOFE05 Contract-Test Vectors

1. Multi-OFE contributor with missing metadata source surface hard-fails at
   watershed intake with typed metadata-missing boundary code.
2. Contributor metadata source with malformed or missing required publication
   fields hard-fails at watershed intake with typed metadata-shape boundary
   code.
3. Contributor metadata `contributor_ofe_count` mismatch versus `hbp.nofe`
   hard-fails at watershed intake before routing dispatch.
4. Valid contributor metadata for multi-OFE intake passes metadata gates and
   reaches downstream watershed execution/output boundaries.

## HPHYS0241 MOFE Hourly Carry Metadata Addendum

1. Hillslope runner manifests must publish a `mofe_hourly_carry` object with:
   - `policy = baseline-wathour-24-slot-copy-forward`,
   - `active` carry status,
   - `substep_count = 24` for active MOFE hourly lanes,
   - `required_arrays = ["ui_SUrunf", "ui_SCrunf", "ui_LfUrf", "ui_LfCrf"]`,
   - finite non-negative aggregate evidence for upstream and current carry
     totals.
2. For multi-OFE contributors, watershed intake must require active
   `mofe_hourly_carry` metadata before routing dispatch and must validate the
   policy string, substep count, required-array family names, and finite
   non-negative aggregate totals.
3. Single-OFE contributors may publish inactive carry metadata, but the object
   must remain parseable so downstream tooling can distinguish inactive
   single-OFE lanes from missing multi-OFE carry evidence.
4. Missing, inactive, malformed, non-24-slot, or aggregate-only carry metadata
   for multi-OFE contributors is a typed hard-fail watershed boundary error and
   cannot be repaired by inferring topology from canonicalized WB13 `OFE=1`
   rows.

## EROD13 Wave-1 Active Boundary-Carry Addendum

1. System integration boundaries carrying hillslope runtime outputs must
   preserve Wave-1 erosion-core forcing surfaces (`Q`, `peakro`, `watdur` and
   WB16 branch diagnostics) without mutation when `erod13_core_enabled = 1`.
2. Producer ownership for those coupling symbols remains in
   `SC-RUNOFFPART-001` and `SC-WATBAL-001`; system-boundary consumers must
   preserve typed failure posture when required Wave-1 coupling payloads are
   absent or malformed.
3. Cross-component publication pathways must not synthesize replacement erosion
   forcing values to bypass Wave-1 guard failures
   (`HKERNEL-EROD13-CORE-E-001..003`).

## EROD14 Wave-2 Active Boundary-Carry Addendum

1. System integration boundaries carrying hillslope erosion outputs must
   preserve Wave-2 enrichment/class-conservation exports
   (`sed_frac_*`, `ER`, `erod14_sumg`, and class-wise closure surfaces)
   without mutation when `erod14_wave2_enabled = 1`.
2. Producer ownership remains in `SC-SED-001`; system-boundary consumers must
   preserve typed hard-fail posture for missing/non-finite/domain-invalid
   Wave-2 payload symbols using `HKERNEL-EROD14-WAVE2-E-001..003` continuity.
3. Cross-component publication pathways must not synthesize replacement
   class-fraction or enrichment payloads to bypass Wave-2 guard failures.
4. Hillslope runfile execution boundary must deterministically carry
   `erod14_wave2_enabled` and required Wave-2 ingress symbol families into
   scheduler execution from canonical runner-owned activation/seeding policy
   defined in `SC-SED-001`; missing derivation inputs are hard-fail states.
5. Existing Wave-1 boundary-carry requirements remain active and additive.

## EROD15 Wave-3 HBP Boundary-Carry Addendum

1. System integration boundaries carrying hillslope erosion outputs must
   preserve Wave-3 HBP routing-boundary payload symbols without mutation when
   `erod14_wave2_enabled = 1`:
   `total_detachment_kg`, `total_deposition_kg`, `particle_class_count`,
   `sediment_concentration_kg_m3_{class:04}`, and
   `particle_flow_fraction_{class:04}`.
2. Producer ownership remains in `SC-SED-001`; system-boundary routing
   consumers must preserve typed hard-fail payload-completeness posture from
   `SC-ROUTE-001` (`INV-ROUTE-011`) for missing/non-finite/domain-invalid
   Wave-3 symbols.
3. Cross-component publication pathways must not synthesize replacement HBP
   payload values when required Wave-3 contributor symbols are absent or
   invalid.
4. Wave-3 coupling continuity preserves existing WS10 guard families
   (`WKERNEL-WS10-CHANNEL-E-001..003`,
   `WKERNEL-WS10-IMPOUNDMENT-E-001..003`) for routing-boundary failures.
5. Existing Wave-1 and Wave-2 boundary-carry requirements remain active and
   additive.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SYSTEM-001 | WSHEDIMPL39 re-baselined companion-contract dependency posture: required cross-domain ownership/guard closure for watershed assembly is explicit, while remaining companion non-promotable gaps are domain-local (for example impoundment calibration/alias refinement rows) rather than unresolved system-boundary ownership ambiguity. | System-boundary promotion is no longer blocked by generic companion-maturity ambiguity; residual companion domain risks remain tracked in owning contracts and require explicit review when claiming full production readiness. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SYSTEM-002 | WSHEDIMPL39 ratified concrete ARCH22 alias mappings for active watershed integration boundaries (hillslope contributor payload ingress plus channel/impoundment runtime publication families), replacing prior identity-placeholder posture for these surfaces. | System-boundary symbol continuity for active runtime integration surfaces is now explicit and contract-bound; identity aliases that remain are source-model/internal process terms, not unresolved boundary-name blockers. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SYSTEM-003 | Chapter 13 notes that separate climate files for hillslope and channel/impoundment components are possible but "not been tested" in cited text. | Cross-file forcing consistency risk remains for mixed-forcing configurations. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-SYSTEM-004 | CREAMS outlet peak-flow method is statistical and chapter-cited dataset support is for watersheds in the `70 ha` to `6200 ha` range. | Method-selection risk exists when applied outside referenced dataset conditions. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SYSTEM-005 | WSHEDIMPL14 implemented a baseline-authoritative end-to-end `openwepp-cli-watershed` comparator lane in `watershed_cli_behavior_contract`, seeded from baseline `ebe_pw0` fixture authority and asserting dispatch/branch/publication continuity at emitted parquet boundaries. | System integration comparator-lane closure is now explicit and executable for baseline-authoritative watershed CLI end-to-end evidence scope. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SYSTEM-006 | WSHED08 activated watershed row-model-backed parquet publication for all required watershed outputs and removed valid-lane placeholder blocking on `OWSOUT-E-004`. | Required watershed publication surfaces now emit non-placeholder parquet outputs; residual system-level hold posture is governed by remaining non-WSHED08 gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SYSTEM-007 | WSHED10 exported active impoundment branch payload families, WSHED11 projected reduced coefficients, and WSHED13 completed WS12 runtime projection of full function families (`f01..f15`) with kernel 15-function min-controller composition at watershed runtime boundaries. | Active-lane structure-family parity closure is complete for WS12 runtime/kernel integration scope; residual watershed HOLD posture is governed by remaining out-of-scope blocker (`GAP-SYSTEM-008`). | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SYSTEM-008 | WSHEDIMPL38 closed the remaining system-level watershed channel sediment integration seam by retiring unresolved-detachment diagnostics publication (`ws20_detachment_unmigrated_segment_count`, `ws21_detach_unmigrated_segment_count`) and replacing residual WS20/WS21 invalid-segment fallback continuation with typed fail-closed guard behavior (`ws20_case12_next_flux_{class:04}`, `ws21_case3_next_flux_{class:04}`, `ws21_case4_next_flux_{class:04}`) under canonical `chnero/chnrt/detach` routing authority. | End-to-end watershed sediment continuity now executes without unresolved-detachment surrogate counters; numeric/domain violations in the migrated segment lanes surface as explicit typed guard failures. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SYSTEM-009 | WSHEDIMPL40 identified residual WS11 system-integration drift for Muskingum-Cunge branch continuity: prior wave-state memory ingress and signed coefficient publication semantics were not explicitly preserved at node boundaries. | Without closure, multi-event WS11 integration vectors for `ipeak >= 4` can lose deterministic boundary memory continuity and misrepresent MC coefficient families at integration boundaries. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SYSTEM-010 | WSHEDIMPL41 migrated WS11 `ipeak = 5` variable-parameter Muskingum-Cunge dynamic-coefficient refresh behavior into the current single-segment WS10 integration lane with explicit dynamic reference-flow lineage and per-step coefficient refresh publication semantics. | System-boundary `ipeak = 5` routing no longer collapses to static `ipeak = 4` coefficient handling when dynamic refresh inputs are valid; integration parity closure is explicit for the current WS10 lane. | closed | `[DIRECT][Static] + [Ran]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-06-14` | `83` | `Codex` | MOFE01 M-G amendment: added `INV-SYSTEM-032` and manifest-boundary authority requiring `erod14_qin_source_policy` / `erod14_qin_sediment_coupled` provenance so water-transfer-only Wave-2 continuity is not mistaken for sediment-coupled `qin` closure. |
| `2026-06-14` | `82` | `Codex` | MOFE01 M-F-REDO2 amendment: required per-OFE publication evidence for public `QOFE = runoff * efflen / slplen` and public `Q = runoff * efflen / totlen`, rejecting downstream `QOFE == Q` aliases while keeping conservation identities on raw transfer operands. |
| `2026-06-13` | `81` | `Codex` | MOFE01 M-F-REDO-CLONE amendment: tightened `INV-SYSTEM-031` to require raw local-runoff anti-clone evidence and reject seeded/stale WB14 multistep infiltration acceptance at publication/consumer gates. |
| `2026-06-13` | `80` | `Codex` | MOFE01 M-F-REDO amendment: added `INV-SYSTEM-031` and the per-OFE publication anti-clone manifest/consumer gate requiring active-handoff and hydrology-vector genuineness evidence beyond row cardinality or conservation closure. |
| `2026-06-13` | `79` | `Codex` | MOFE01 M-E0 amendment: added `INV-SYSTEM-030` and the per-OFE dynamic-state publication-policy manifest gate for transition from MOFE04 aggregate publication to per-OFE WB13/H.wat rows. |
| `2026-06-02` | `78` | `Codex` | HPHYS0255 amendment: added `INV-SYSTEM-029` requiring explicit MOFE WB13/H.wat `storage_lineage_policy` provenance and preserving separation between aggregate area and simulation-owned storage lineage. |
| `2026-06-01` | `77` | `Codex` | HPHYS0241 amendment: added `INV-SYSTEM-028` and `mofe_hourly_carry` manifest/watershed-intake authority requiring active 24-slot carry-array provenance for multi-OFE hourly contributors and fail-closed validation before watershed routing dispatch. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-17 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with integration invariants, guard map, alias map, tolerances, and gap register for SCI-17 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: normalized evidence metadata, added duration-family alias coverage, added evidence labels for degenerate/tolerance rows, and clarified CREAMS dataset applicability range in gap text. |
| `2026-05-23` | `3` | `Codex` | INT10 amendment: added cross-lane publication invariant (`INV-SYSTEM-011`) and guard/disposition authority requiring successful `decomp -> growth -> watbal` closure before system-boundary publication. |
| `2026-05-23` | `4` | `Codex` | PL14 amendment: added strict replay artifact/provenance completeness invariant (`INV-SYSTEM-012`), replay-lane guard/disposition authority, and explicit strict Tier-A tolerance authority (`abs_tol=0`, `rel_tol=0`) for closeout staging. |
| `2026-05-23` | `5` | `Codex` | PL15 amendment: added Tier-A residual closeout governance invariant (`INV-SYSTEM-013`) requiring explicit risk-acceptance references for unresolved blockers and prohibiting silent down-classification/implicit risk acceptance. |
| `2026-05-23` | `6` | `Codex` | WS10 amendment: added system integration authority for WS10 production watershed runtime aliases, deterministic dependency payload publication rules, and WS10 typed guard-family/test-vector requirements. |
| `2026-05-23` | `7` | `Codex` | ARCH22 amendment: added typed production-surface authority requiring covered system integration interfaces to consume boundary symbols via ARCH22 typed symbol families while preserving deterministic publication and existing failure-class/message continuity. |
| `2026-05-23` | `8` | `Codex` | PL14R amendment: added strict replay rerun reproducibility invariant (`INV-SYSTEM-014`) requiring required include-surface staging plus persisted comparator/provenance hash evidence for post-closure-wave Tier-A rerun authority. |
| `2026-05-23` | `9` | `Codex` | PL15R amendment: added refreshed Tier-A recloseout supersession invariant (`INV-SYSTEM-015`) requiring blocker classification from latest schema-aligned strict replay evidence and explicit risk-acceptance reference only when post-supersession blockers remain. |
| `2026-05-23` | `10` | `Codex` | WB20 amendment: added forward-solver parity governance invariant (`INV-SYSTEM-016`) requiring explicit lane-manifest and no-observed-target-substitution evidence before Tier-A parity-lane disposition can close. |
| `2026-05-23` | `11` | `Codex` | EROD12 amendment: added cross-domain ownership/guard closure addendum covering required erosion-lane Wave-0 boundaries and refined `GAP-SYSTEM-001` to distinguish remaining non-Wave-0 system promotability holds from resolved Wave-0 ownership semantics. |
| `2026-05-24` | `12` | `Codex` | CLI02 amendment: replaced required strict-replay include-surface authority from legacy candidate files to simulation-driven partitioned interchange surfaces (`interchange/H.wat.parquet`, `interchange/H.pass.parquet`) and aligned supersession evidence wording to parquet comparator artifacts. |
| `2026-05-24` | `13` | `Codex` | WS12 amendment: added impoundment physics-equivalence integration authority requiring continuity/regime-driven impoundment routing with parser-projected coefficient families and preserved WS10 guard-family continuity for boundary failures. |
| `2026-05-24` | `14` | `Codex` | WS11 amendment: added channel-routing physics-equivalence integration authority requiring explicit `ipeak` branch execution (Rational/CREAMS/KW/MC), prohibited pre-WS11 gain-factor surrogate substitution, and anchored integration semantics to pinned legacy routing provenance while preserving existing WS10 guard-family continuity. |
| `2026-05-24` | `15` | `Codex` | PL14S amendment: added semantic replay diagnostics invariant (`INV-SYSTEM-017`), semantic/provenance publication guard authority, semantic replay producer obligations, and explicit semantic-tolerance profile authority for investigation-grade Tier-A replay evidence. |
| `2026-05-24` | `16` | `Codex` | SIMIMPL03 amendment: added production runner execution ownership, mode-propagation manifest closure, simulation-owned replay-surface provenance, and selective consolidated-intake governance invariants (`INV-SYSTEM-018..021`) with typed guard families (`WS-SIMPIPE/SIMMODE/SIMOUT/SIMCONS`). |
| `2026-05-25` | `17` | `Codex` | EROD13 amendment: added Wave-1 system-boundary carry authority for erosion-core forcing surfaces under `erod13_core_enabled`, preserving producer/consumer ownership continuity and typed hard-fail guard posture (`HKERNEL-EROD13-CORE-E-001..003`). |
| `2026-05-25` | `18` | `Codex` | EROD14 amendment: added Wave-2 system-boundary carry authority for enrichment and class-conservation payload exports under `erod14_wave2_enabled` with typed hard-fail guard continuity (`HKERNEL-EROD14-WAVE2-E-001..003`). |
| `2026-05-25` | `19` | `Codex` | EROD15 amendment: added Wave-3 HBP boundary-carry authority for routing payload exports (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, class-indexed concentration/fraction arrays) with explicit WS10 guard-family continuity at watershed routing boundaries. |
| `2026-05-25` | `20` | `Codex` | SIMIMPL14 amendment: added continuous replay-span/key-domain closure invariant (`INV-SYSTEM-022`) requiring full climate-span execution provenance, WB13 row-span closure, monotonic `sim_day_index`, and simulation-year key semantics for replay comparability authority. |
| `2026-05-25` | `21` | `Codex` | SIMIMPL15 amendment: added replay-lane policy + candidate-source provenance invariant (`INV-SYSTEM-023`), semantic alias/row-width structural continuity invariant (`INV-SYSTEM-024`), and explicit producer obligations for deterministic strict/parquet policy publication and non-promotable conversion-derived dat classification. |
| `2026-05-25` | `22` | `Codex` | SIMIMPL16 amendment: added replay contract-derived test-coverage closure invariant (`INV-SYSTEM-025`) plus explicit producer/governance obligations requiring blind-spot closure tests for span/key overlap, strict-lane compensation, alias continuity, and conversion-derived dat row-consistency gating. |
| `2026-05-25` | `23` | `Codex` | SIMIMPL18 amendment: added baseline-year policy and full-span precipitation comparability invariant (`INV-SYSTEM-026`), explicit replay-provenance obligations for declared span-policy metadata, and addendum authority requiring first-day and multi-day storage diagnostics for hydrology closure evidence. |
| `2026-05-25` | `24` | `Codex` | SIMIMPL21 amendment: added WB13 ET/soil-water publication-lineage invariant (`INV-SYSTEM-027`), explicit producer/consumer alias-lineage obligations for `Ep`/`Es`/`Er`/`Total-Soil`/`SoilWaterTotal`, and addendum authority prohibiting projection-side surrogate publication reconstruction. |
| `2026-05-29` | `65` | `Codex` | HPARITY02 amendment: extended `INV-SYSTEM-027` lineage scope to include WB13 profile-capacity outputs (`ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`) and updated producer/invalid-state/disposition authority accordingly. |
| `2026-05-29` | `66` | `Codex` | HPHYS0202 amendment: made system publication authority explicit that `ProfileFCStore`/`ProfileWPStore` are simulation-owned layer aggregates (`thetfc/thetdr` with `dg`) and that FC/WP adapter seeds remain non-authoritative diagnostics. |
| `2026-05-29` | `67` | `Codex` | HPHYS0205 amendment: required corrected-layer projection authority for WB13 FC/WP publication symbols (`thetfc_####`/`thetdr_####`) with reconciliation obligations against diagnostic FC/WP seed surfaces. |
| `2026-05-30` | `68` | `Codex` | HPHYS0206 amendment: required deterministic normalized-layer mapping closure for authoritative FC/WP publication symbols and explicit fail-closed/no-raw-fallback boundary posture. |
| `2026-05-30` | `69` | `Codex` | HPHYS0207 amendment: promoted WB13 FC/WP publication authority to normalized-profile storage symbols (`wb13_profile_fc_store_mm`, `wb13_profile_wp_store_mm`) and added explicit normalized-tail consumption policy authority. |
| `2026-05-30` | `70` | `Codex` | HPHYS0203 amendment: added system-level WB13 robustness governance obligations for profile/soil-water/subsurface publication families, requiring conservation/order/domain/non-finite vectors plus deterministic regression-fixture evidence with parity treated as diagnostic-only. |
| `2026-05-30` | `71` | `Codex` | HPHYS0208 amendment: added coupled WB13 publication-lineage governance requiring threshold-seed ownership (`sat`, `por_####`, `cpm_####`, `thetfc_####`, `thetdr_####`, `dg_####`) across WB11/WB18/WB19 pathways for `ProfileFCStore`/`Dp`/`latqcc`/`Total-Soil`/`SoilWaterTotal` closure claims. |
| `2026-05-30` | `72` | `Codex` | HPHYS0209 amendment: codified near-closed `ProfileWPStore` publication adjudication governance as diagnostic-only expected process-correct evidence when isolated/stable, with explicit non-regression and fail-closed guard continuity requirements. |
| `2026-05-31` | `73` | `Codex` | HPHYS0216D amendment: reconciled WB13 `ProfileFCStore` system-boundary authority to layer aggregation plus explicit normalized-tail carry (`wb13_profile_fc_tail_mm`), retained `wb13_profile_fc_store_mm` as diagnostic/reconciliation lineage, and required fail-closed missing/non-finite/negative tail guard posture. |
| `2026-05-31` | `76` | `Codex` | HPHYS0221 amendment: added WB19 `solwpv` branch governance and coupled saturated-depth lineage authority (`wb19_watyld`, `wb19_fcdep`, `wb19_unsdep`) as required system-boundary evidence for `latqcc`/`Dp` residual adjudication. |
| `2026-05-31` | `75` | `Codex` | HPHYS0219 amendment: corrected coupled WB19 `drfc` threshold governance to baseline-authoritative `wb18_perc_fc_#### + (1-coca_####)*dg_####` lineage with fail-closed `coca_####` domain enforcement for `latqcc`/`Dp` closure claims. |
| `2026-05-31` | `74` | `Codex` | HPHYS0218 amendment: added coupled WB19 `drfc` threshold governance requiring `wb18_perc_fc_#### + (1-coca_####)*dg_####` execution authority and fail-closed `coca_####` domain enforcement for `latqcc`/`Dp` closure claims. |
| `2026-05-25` | `25` | `Codex` | MOFE03 amendment: added system-boundary authority requiring deterministic runner carry of Wave-2 activation/ingress seed surfaces into scheduler execution under canonical `SC-SED-001` policy with hard-fail posture on missing derivation inputs. |
| `2026-05-25` | `26` | `Codex` | MOFE04 amendment: added system-boundary carry authority for explicit multi-OFE WB13/H.wat canonicalized publication policy provenance (`publication_ofe_policy`, `contributor_ofe_count`, `area_policy`, `publication_area_m2`) and fail-closed dimensional interpretation requirements for canonicalized `OFE=1` output rows. |
| `2026-05-25` | `27` | `Codex` | MOFE05 amendment: added watershed contributor MOFE metadata intake authority requiring typed fail-closed validation for missing/malformed publication metadata and explicit `contributor_ofe_count == hbp.nofe` consistency gating before watershed routing dispatch. |
| `2026-05-27` | `28` | `Codex` | WSHEDIMPL01 amendment: normalized unresolved watershed implementation gaps for system integration (WS11/WS12 end-to-end vector coverage, parser-to-runtime coefficient projection closure, channel sediment integration closure, and `OWSOUT-E-004` parquet publication blocker) with explicit WSHED03/04/06/08/09 dependency mapping. |
| `2026-05-27` | `29` | `Codex` | WSHEDIMPL04 amendment: updated system-level WS12 seam posture to reflect removal of manual/synthetic coefficient seeding dependency for inactive-structure conformance lanes and explicit fail-closed residual blocker language for active structure branch projection payload gaps. |
| `2026-05-27` | `30` | `Codex` | WSHEDIMPL06 amendment: ratified WS11 channel sediment publication-family closure (`ws10_channel_{id}_qsed`, `ws10_channel_{id}_tc`) while preserving non-promotable `GAP-SYSTEM-008` posture for unresolved full `chnero/chnrt/detach` process-parity migration and validation closure. |
| `2026-05-27` | `31` | `Codex` | WSHEDIMPL07 amendment: synchronized WS12 system integration posture to reflect active RK4/adaptive/regime-transition continuity migration for supported inactive-structure coefficient domains while retaining explicit non-promotable active-structure projection blockers in `GAP-SYSTEM-007`. |
| `2026-05-27` | `32` | `Codex` | WSHEDIMPL08 amendment: ratified watershed parquet writer activation and row-model emission closure for required watershed outputs, dispositioning `GAP-SYSTEM-006` to `closed` while retaining non-WSHED08 blockers (`GAP-SYSTEM-005/007/008`). |
| `2026-05-27` | `33` | `Codex` | WSHEDIMPL09 amendment: recorded watershed rerun evidence and confidence-tier disposition; retained `GAP-SYSTEM-005` as non-promotable with updated evidence class (`[Ran]`) because baseline-authoritative end-to-end comparator lane closure remains outstanding. |
| `2026-05-27` | `34` | `Codex` | WSHEDIMPL10 amendment: recorded active-structure parser payload export closure evidence and narrowed `GAP-SYSTEM-007` to remaining runtime active-coefficient projection implementation scope. |
| `2026-05-27` | `35` | `Codex` | WSHEDIMPL11 amendment: synchronized system boundary posture to reflect active runtime reduced-family coefficient projection from exported impoundment branch payloads, and narrowed `GAP-SYSTEM-007` to residual full 15-function active-lane parity closure scope. |
| `2026-05-27` | `36` | `Codex` | WSHEDIMPL13 amendment: ratified full active-lane WS12 function-family projection and 15-function min-controller composition closure at watershed runtime boundaries, dispositioning `GAP-SYSTEM-007` to `closed` while preserving out-of-scope blockers (`GAP-SYSTEM-005`, `GAP-SYSTEM-008`). |
| `2026-05-27` | `37` | `Codex` | WSHEDIMPL14 amendment: ratified baseline-authoritative end-to-end watershed comparator lane closure in runner CLI contract tests (baseline `ebe_pw0` signature seeded vector with dispatch/branch/publication continuity assertions), dispositioning `GAP-SYSTEM-005` to `closed` while preserving residual blocker `GAP-SYSTEM-008`. |
| `2026-05-27` | `38` | `Codex` | WSHEDIMPL15 amendment: ratified WS15 watershed channel-sediment scaffold closure (runtime projection of channel sediment controls plus fail-closed kernel publication of baseline conversion states `crsh/depmid/depsid`) while preserving non-promotable `GAP-SYSTEM-008` posture pending full `chnero/chnrt/detach` process-parity migration. |
| `2026-05-27` | `39` | `Codex` | WSHEDIMPL16 amendment: ratified contributor `particle_diameter_m` payload ingress projection (`hs{ID}_particle_diameter_m_{class:04}`) with fail-closed WS10 guard continuity, and narrowed `GAP-SYSTEM-008` to remaining full `chnero/chnrt/detach` process-parity migration closure scope. |
| `2026-05-27` | `40` | `Codex` | WSHEDIMPL17 amendment: ratified WS17 segment/hydraulic scaffold projection/guard closure (`ws10_channel_{id}_nslpts` + segment `x/slope/depa/depb/wida/widb` families) and narrowed `GAP-SYSTEM-008` to remaining full `chnero/chnrt/detach` process-family migration scope. |
| `2026-05-27` | `41` | `Codex` | WSHEDIMPL18 amendment: migrated baseline `shield`/`trncap` transport-capacity authority into WS10 channel sediment publication (`tc`) via class-aware contributor payload aggregation and removed surrogate `tc=qsed` identity coupling, while preserving non-promotable `GAP-SYSTEM-008` posture for unresolved segment-loop detachment/deposition routines (`case12/case34/detach/dcap/enddet`) and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `42` | `Codex` | WSHEDIMPL19 amendment: ratified fail-closed WS10 channel sediment branch payload export (`particle_class_count`, `particle_flow_fraction_{class:04}`, `particle_diameter_m_{class:04}`) and upstream channel-dependency payload ingress continuity for class-aware aggregation, while preserving non-promotable `GAP-SYSTEM-008` posture for unresolved segment-loop detachment/deposition routines (`case12/case34/detach/dcap/enddet`) and full `chnero/chnrt` inflow-partition parity closure. |
| `2026-05-27` | `43` | `Codex` | WSHEDIMPL20 amendment: added opt-in WS20 segment-loop `case12` routing scaffolding with unresolved-detachment diagnostics publication (`ws20_case1_segment_count`, `ws20_case2_segment_count`, `ws20_detachment_unmigrated_segment_count`) while preserving non-promotable `GAP-SYSTEM-008` posture for remaining baseline-authoritative detachment/deposition families (`case34/detach/dcap/enddet`) and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `44` | `Codex` | WSHEDIMPL21 amendment: added WS10 opt-in WS21 case34/enddet diagnostics scaffolding (`ws21_case3_segment_count`, `ws21_case4_segment_count`, `ws21_enddet_segment_count`, `ws21_detach_unmigrated_segment_count`) with explicit unresolved detach-capacity diagnostics publication while preserving non-promotable `GAP-SYSTEM-008` posture for remaining baseline-authoritative `detach/dcap` migration and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `45` | `Codex` | WSHEDIMPL22 amendment: replaced WS21 opt-in unresolved fallback with baseline-lineage `dcap` + `case34/enddet` execution and required fail-closed `crfrac` projection gating (`ws10_channel_{id}_crfrac_{class:04}`), while preserving non-promotable `GAP-SYSTEM-008` posture for residual WS21 `case4 -> detach` iterative closure (`nt < cnpart`) and remaining full `chnero/chnrt` parity closure. |
| `2026-05-27` | `46` | `Codex` | WSHEDIMPL23 amendment: migrated baseline-authoritative `detach.for` iterative closure behavior for WS21 `case4` rows (`nt < cnpart`) and removed residual WS21 unresolved-detachment fallback requirement for that branch, while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `47` | `Codex` | WSHEDIMPL24 amendment: migrated baseline-authoritative `case12.for` deposition-to-detachment transition continuation (`xdemax < x(i)` into `detach.for`) with explicit transition diagnostics publication (`ws24_case2_detach_segment_count`), while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `48` | `Codex` | WSHEDIMPL25 amendment: closed residual WS20 opt-in unresolved-detachment fallback behavior by auto-activating WS21 migration lanes under WS20 opt-in and enforcing fail-closed `crfrac` requirements for WS20-only opt-in lanes, while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `49` | `Codex` | WSHEDIMPL26 amendment: migrated baseline-authoritative `dcap(flagm=2)` max-detachment limiter semantics for WS23 iterative detach closure lanes and preserved non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `50` | `Codex` | WSHEDIMPL27 amendment: migrated baseline-authoritative `enddet.for` bracket progression semantics (`xdbig/xdsmal`) for WS21 case4 enddet closure lanes and preserved non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `51` | `Codex` | WSHEDIMPL28 amendment: migrated baseline-authoritative `chnrt.for` segment boundary-width semantics (`widb(i-1)` upper boundary, `wida(i)` lower boundary) in WS20 segment-loop routing lanes and preserved non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `52` | `Codex` | WSHEDIMPL29 amendment: migrated rectangular-channel width-mutation semantics by projecting `dcap` eroded-width outcomes (`werb`) into WS20 `widb(i-1)` updates and state-symbol writeback (`ws10_channel_{id}_widb_{point:04}`), while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `53` | `Codex` | WSHEDIMPL30 amendment: migrated erodible-lane shape-transition continuity by activating `ishape=3` routing pathways and applying `depa/depb`-driven rectangular fallback mapping for WS20/WS21 hydraulic and detach-capacity calls, while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `54` | `Codex` | WSHEDIMPL31 amendment: migrated baseline-authoritative lower-boundary width-mutation continuity (`flagc=2`, `wera>wfl`) by projecting detach eroded-width outcomes (`wera`) into WS20 rectangular-lane `wida(i)` updates and state-symbol writeback (`ws10_channel_{id}_wida_{point:04}`), while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `55` | `Codex` | WSHEDIMPL32 amendment: reconciled parser/runtime naturally eroded shape-class lineage by aligning watershed channel parser projection and WS10 runtime consumption on explicit `ishape=3` mapping semantics (strict domain `1..=3`, compatibility `ishape>3 -> 3`), while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `56` | `Codex` | WSHEDIMPL33 amendment: reconciled parser/runtime channel `ienslp` lineage by aligning watershed channel parser projection and WS10 runtime seed validation on explicit `ienslp` domain semantics (`1..=2`, fail-closed out-of-domain), while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `57` | `Codex` | WSHEDIMPL34 amendment: reconciled parser/runtime watershed-channel Manning relation lineage by aligning parser projection authority and WS10 runtime seed validation on explicit `chnn >= chnnbr` fail-closed semantics, while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `58` | `Codex` | WSHEDIMPL35 amendment: reconciled parser/runtime channel control lineage by projecting `icntrl`/`flgout` into WS10 runtime seed surfaces with explicit fail-closed domain semantics (`icntrl in [0,4]`, `flgout in [0,1]`), while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `59` | `Codex` | WSHEDIMPL36 amendment: reconciled parser/runtime rating-curve control lineage by projecting `ws10_channel_{id}_{rccoef,rcexp,rcoset}` for `icntrl==4` lanes into WS10 runtime seed surfaces with explicit fail-closed payload-shape/domain semantics (`rccoef>0`, `rcexp>0`, `rcoset>=0`), while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `60` | `Codex` | WSHEDIMPL37 amendment: added trace linkage for companion WS11 hydrology route-chain parity closure (`wshcqi/wshirs/wshrun`) and `GAP-ROUTE-008` disposition while preserving non-promotable `GAP-SYSTEM-008` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `61` | `Codex` | WSHEDIMPL38 amendment: closed `GAP-SYSTEM-008` by retiring unresolved-detachment diagnostics symbols and replacing residual WS20/WS21 invalid-segment fallback continuation with typed fail-closed domain guard behavior under canonical channel sediment migration authority. |
| `2026-05-28` | `62` | `Codex` | WSHEDIMPL39 amendment: dispositioned system out-of-scope follow-up blockers by re-baselining companion-dependency posture (`GAP-SYSTEM-001` -> promotable-with-risk) and ratifying concrete ARCH22 alias mappings for active watershed integration boundaries (`GAP-SYSTEM-002` -> closed). |
| `2026-05-28` | `63` | `Codex` | WSHEDIMPL40 amendment: ratified WS11 Muskingum-Cunge system-integration parity closure for prior wave-state memory ingress (`ws10_channel_{id}_{qin,q1}`), baseline-lineage single-segment lateral-term scaling continuity (`c4 = 2*qlat*dtchr*c0`), and signed MC coefficient publication semantics (`c1/c2/c3`) without non-physical non-negative clamp repair (`GAP-SYSTEM-009` closed); retained follow-on `ipeak=5` variable-parameter dynamic-coefficient integration gap (`GAP-SYSTEM-010`) as promotable-with-risk. |
| `2026-05-28` | `64` | `Codex` | WSHEDIMPL41 amendment: migrated WS11 `ipeak=5` MVPMC3 dynamic-coefficient refresh integration into the current single-segment WS10 lane via reduced segment-state dynamic reference-flow lineage and per-step `c0..c4` refresh publication semantics, dispositioning `GAP-SYSTEM-010` to `closed`. |
| `2026-07-02` | `100` | `Claude Code` | MOFEFID-B02 reconciliation: `INV-SYSTEM-031` anti-clone `QOFE == Q` rejection SUPERSEDED by `SC-RUNOFFPART-001#INV-RUNOFFPART-032` + MOFE04 canonicalized policy; per-OFE genuineness proven by distinctness/lineage evidence, not QOFE!=Q. |
