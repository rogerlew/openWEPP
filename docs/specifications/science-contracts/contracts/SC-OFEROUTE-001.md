---
contract_id: SC-OFEROUTE-001
title: Hillslope OFE-by-OFE Overland-Flow Routing Process Contract
status: approved
maturity: active
owner: openWEPP maintainers + hydrology reviewer
contract_version: 3
producer_scope:
  - Space/time-variant overland-flow resistance (skin/form/wave/vegetation) surfaces
  - 1-D kinematic-wave routing state (depth, unit discharge) per OFE per sub-timestep
  - Per-OFE outlet hydrograph handed to the downstream OFE as an upstream boundary condition
consumer_scope:
  - Runoff-partition rainfall-excess coupling (SC-RUNOFFPART-001)
  - Inter-OFE water transfer seam (SC-RUNOFFPART-001 INV-RUNOFFPART-028/029)
  - Erosion continuity consumers that read peak/duration surfaces (SC-SED-001)
evidence_level: static
last_reviewed: 2026-07-04
supersedes: []
superseded_by: []
---

# SC-OFEROUTE-001 Hillslope OFE-by-OFE Overland-Flow Routing Process Contract

Status: `approved`
Maturity: `active`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for **opt-in** hillslope overland-flow
routing that replaces the legacy equivalent-plane / equilibrium-storage
representation with **OFE-by-OFE kinematic-wave routing under space/time-variant
flow resistance** (Papanicolaou et al. 2018). This contract governs the
friction-factor menu, the 1-D kinematic-wave solution and its TVD-MacCormack
shock-capturing scheme, the CFL stability policy, and the per-OFE hydrograph
handoff. It is the canonical authority that the MOFEFID Lane D solver
(`ADR-0033`, ratified in narrowed scope) requires **before** solver
implementation, and it anchors the already-landed shadow-first friction
kernels (`ofe_routing::friction`).

## Scientific Scope

In scope:
- Additive friction menu: skin (grain + raindrop), form (isolated roughness
  elements), wave, and vegetation resistance, and the equivalent
  Darcy-Weisbach `f_eq`.
- 1-D kinematic-wave approximation (continuity + `q = alpha h^m`) solved per
  OFE with the TVD-MacCormack predictor/corrector and dissipative flux limiter.
- CFL sub-timestep policy within the hillslope day loop.
- Per-OFE outlet-hydrograph -> downstream-OFE upstream-boundary handoff over
  the `INV-RUNOFFPART-029` transfer seam.
- Per-OFE Green-Ampt infiltration on rainfall producing the routed excess,
  with active routing SUPERSEDING DC01's daily-lump runon re-infiltration
  (`SC-RUNOFFPART-001#INV-RUNOFFPART-031`); the routed runon is surface-routed,
  not re-infiltrated.
- Opt-in activation posture (default hillslope path unchanged).
- Subsurface-to-surface **exfiltration coupling** when the subsystem is active:
  subsurface excess (return flow / saturation-excess exfiltration) entering the
  OFE surface routing as a source term, and conservation/export of **baseflow**
  (subsurface outflow not exfiltrated), so an active router closes the full
  hillslope water balance. The subsurface *flow physics* stays `SC-SUBHYD-001`'s;
  the coupling *seam* is an activation requirement here (`GAP-OFEROUTE-006`,
  `INV-OFEROUTE-012`).

Out of scope:
- Rainfall-excess / infiltration internals owned by `SC-RUNOFFPART-001`.
- Subsurface lateral flow **physics** owned by `SC-SUBHYD-001` (Darcy `latqcc`,
  percolation). NOTE: the **coupling** of subsurface excess into the active
  surface router (return flow / saturation excess) and baseflow accounting is
  IN scope as an activation seam — `GAP-OFEROUTE-006` / `INV-OFEROUTE-012`.
- Watershed **channel** routing owned by `SC-ROUTE-001` (this contract is the
  **hillslope overland** analog, deliberately distinct).
- Sediment detachment/transport owned by `SC-SED-001` (this contract produces
  the hydraulic surfaces a later sediment coupling consumes; it does not close
  sediment).
- Default activation of the subsystem (a separate later gate with its own
  no-regression, endpoint-timing, and magnitude re-adjudication).
- Kernel Rust API naming.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-OFEROUTE-FRAMEWORK | Papanicolaou et al. 2018, WRR 54 (R-63); local `references/copyrighted/Papanicolaou2018.md` + supplemental | Enhanced-WEPP OFE-by-OFE framework: friction menu eqs. (2)-(7), TVD-MacCormack eqs. (8)-(14), KWE Appendix A, Zone 1/2 stream-power taxonomy. Primary. | `[DIRECT][Static]` |
| REF-OFEROUTE-SKIN-SHENLI | Shen & Li (1973), JHD ASCE 99(HY5) — **secondary via R-63** (eq. (2) as stated in Papanicolaou §2.2) | Laminar skin resistance `f_s = (3393 I^0.407 + k_o)/Re`, `Re < ~1000`; R-63 explicitly states rainfall intensity `I` in `m/s`. Frozen library: primary un-acquired; D8 confirms the local SI `I` convention with low-`k_o` regression, not a primary-source coefficient audit. | `[DIRECT][Static] via R-63` |
| REF-OFEROUTE-SKIN-HIRSCH | Hirsch (1996) — **secondary via R-63** (eq. (3)) | Turbulent skin resistance `f_s = 3.19/Re^0.45`, `Re > ~1000`. | `[DIRECT][Static] via R-63` |
| REF-OFEROUTE-FORM-LAWRENCE | Lawrence (1997), ESPL 22(4) (R-77, primary in hand); Abrahams (1998) discussion **secondary via R-63** | Form resistance `f_f = (16/pi) C_d (h/D_r) lambda` (eq. (4)) and its inundation-ratio applicability bounds. | `[DIRECT][Static]` + `[INFERENCE][Static]` |
| REF-OFEROUTE-WAVE-HUABRAHAMS | Hu & Abrahams (2006), ESPL 31(10) (R-72, primary in hand) | Wave resistance `f_w = 3.32 lambda / Fr^0.5` (`Fr > 0.5`; linear ramp below), eq. (5). | `[DIRECT][Static]` |
| REF-OFEROUTE-VEG-KATUL | Katul, Poggi & Ridolfi (2011), WRR 47 (R-78, primary); Thompson et al. (2011) (R-80) | Vegetation resistance `f_veg` with `beta = min(0.135 sqrt(LAI/h_c), 0.33)`, `L_c = (C_d LAI/h_c)^-1`, eq. (6). | `[DIRECT][Static]` |
| REF-OFEROUTE-KO-WOOLHISER | Woolhiser (1975) k_o tables — **secondary via KINEROS** (Smith 1990, in-repo) | Tabulated laminar friction coefficients `k_o` by surface type. Frozen library: KINEROS reproduces the coefficients. | `[DIRECT][Static] via KINEROS` |
| REF-OFEROUTE-KWE | Lighthill & Whitham (1955) (R-01); USGS PP1302 (R-04), in-repo | 1-D kinematic-wave continuity + `q = alpha h^m` (m = 1.5) and celerity relations (Papanicolaou Appendix A). Primary. | `[DIRECT][Static]` |
| REF-OFEROUTE-TVD-MACCORMACK | Garcia-Navarro et al. (1992), JHE 118(10); Mingham et al. (2001) — **secondary via R-63** (scheme eqs. (8)-(14) as stated in Papanicolaou §2.3) | TVD-corrected MacCormack predictor/corrector, flux limiter `phi`, dissipative term. Frozen library: numerics primaries un-acquired; scheme re-derivable from open sources. | `[DIRECT][Static] via R-63` |
| REF-OFEROUTE-SHOCK-IWAGAKI | Iwagaki (1955), DPRI Bulletin 10 (R-74, primary in hand) | Method-of-characteristics shock dataset; the concave three-section flume (Case 4) tests TVD shock capture. | `[DIRECT][Static]` |
| REF-OFEROUTE-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative depth/discharge/friction magnitudes; bounded roughness concentration `lambda in [0,1]`; regime-threshold branch handling. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Meaning | Units |
|---|---|---|
| `h` | overland-flow depth | m |
| `q` | unit-width discharge | m^2 s^-1 |
| `v` | rainfall-excess rate `r - i_f` | m s^-1 |
| `alpha` | kinematic depth-discharge coefficient `C S_o^0.5` | m^(1/2) s^-1 |
| `m` | depth-discharge exponent | 1.5 (dimensionless) |
| `C` | Chezy coefficient `sqrt(8 g / f_eq)` | m^(1/2) s^-1 |
| `f_eq`, `f_s`, `f_f`, `f_w`, `f_veg` | equivalent + component friction factors | dimensionless |
| `Re` | Reynolds number `q / nu` | dimensionless |
| `Fr` | Froude number `q / sqrt(g h^3)` | dimensionless |
| `S_o` | OFE gradient | m m^-1 |
| `k_o` | laminar friction coefficient (surface type) | dimensionless |
| `C_d`, `D_r`, `lambda` | drag coefficient, element tip height, roughness concentration | -, m, - |
| `LAI`, `h_c`, `L_c`, `beta` | leaf area index, canopy height, adjustment length, momentum-absorption coeff. | m^2 m^-2, m, m, - |
| `Cr`, `Delta t`, `Delta x` | Courant number, sub-timestep, mesh size | -, s, m |

## Algorithm State Surfaces

| Surface class | Required surfaces | Owner / boundary | Notes |
|---|---|---|---|
| Required inputs | Rainfall-excess rate `v`, upstream boundary unit discharge, OFE geometry (`Delta x`, OFE length, `S_o`), mesh depth/discharge initial state, and friction operands (`I`, `k_o`, `C_d`, `D_r`, `lambda`, `LAI`, `h_c`, `nu`, `g`) | `SC-RUNOFFPART-001` for rainfall excess and inter-OFE transfer; SC-OFEROUTE for routed state and friction operands | D4/D5 must name exact Rust/API boundary fields before runtime binding. |
| Required outputs | Cell/sub-timestep `h`, `q`, `f_s`, `f_f`, `f_w`, `f_veg`, `f_eq`, `C`, `alpha`, `Cr`, OFE outlet hydrograph, conservation residual, and validation diagnostics | SC-OFEROUTE | Published output metadata is required before any user-visible or retained artifact publication. |
| Mutated state | Transient OFE routing mesh state and, when active, the downstream OFE hourly runon supply carried over `INV-RUNOFFPART-029` | SC-OFEROUTE + `SC-RUNOFFPART-001` seam | With subsystem off, no routed state is allocated into the default hillslope phase path and no protected output may change. |

## Algorithm Specification

1. **Friction menu (eqs. 2-7).** Per OFE per sub-timestep compute `f_s`
   (regime-dispatched by `Re`), `f_f`, `f_w` (`Fr`-gated), `f_veg`, and the
   additive `f_eq = f_s + f_f + f_w + f_veg`; derive `C = sqrt(8 g / f_eq)` and
   `alpha = C S_o^0.5`.
2. **KWE (eqs. A1-A2).** Continuity `dh/dt + dq/dx = v` with `q = alpha h^m`,
   `m = 1.5`.
3. **TVD-MacCormack (eqs. 8-14).** Predictor/corrector over the OFE mesh with
   the dissipative flux-limiter term (`phi`, `Gr`, `Cf(Cr)`); final depth is
   the predictor/corrector average plus the TVD term; final discharge
   `q = alpha h^1.5`.
4. **CFL (eq. 12).** `Cr = (Delta t / Delta x) c <= 1`, celerity
   `c = 1.5 C S_o^0.5 h^0.5`; `Delta t` chosen to satisfy CFL at every cell.
5. **Per-OFE handoff.** The OFE outlet hydrograph becomes the downstream OFE's
   upstream boundary condition, carried over the `INV-RUNOFFPART-029` seam as an
   hourly profile (not a daily aggregate).

## Branch and Guard Table

| Branch / guard | Trigger | Required behavior | Failure posture | Invariants |
|---|---|---|---|---|
| No-flow friction degeneracy | Physically absent flow or component (`q <= 0`, `h <= 0`, `D_r <= 0`, absent canopy) | Return the corresponding resistance component as `0` and keep `f_eq` finite; this is a bounded no-flow normalization, not an invalid-input fallback. | Approved bounded normalization; non-finite inputs remain hard failures. | `INV-OFEROUTE-001`, `INV-OFEROUTE-003`, `INV-OFEROUTE-004` |
| Skin-regime dispatch | `Re <= 1000` vs. `Re > 1000` after finite `Re = q/nu` construction | Use Shen & Li below/at the crossover and Hirsch above it; D8 low-`k_o` regression confirms the local SI rainfall-intensity convention stated by R-63. | Hard fail for missing/non-finite operands or invalid `nu`; test failure for wrong branch. | `INV-OFEROUTE-002` |
| Roughness domain | `lambda`, `C_d`, `D_r`, `LAI`, `h_c`, `S_o`, `Delta x`, `Delta t` consumed by active routing | Enforce finite values and stated domains before equation evaluation. | Typed runtime failure in D4/D5; unit-test failure for pure friction kernels. | `INV-OFEROUTE-001..007` |
| Form/wave submergence | `h/D_r < 1`, `Fr <= 0.5`, `Fr > 0.5` | Apply form resistance per eq. (4); apply wave resistance only under the stated submergence/Froude policy with the low-`Fr` linear ramp. | Hard fail for invalid finite operands; branch-test failure for wrong ramp. | `INV-OFEROUTE-003` |
| Vegetation branch | `LAI > 0` and `h_c > 0` vs. absent/unsubmerged canopy | Apply Katul beta cap and length scale when active; return zero vegetation resistance when canopy is absent. | Hard fail for invalid finite operands; bounded zero only for absent canopy. | `INV-OFEROUTE-004` |
| KWE/TVD conservation | Each solver sub-timestep | Preserve non-negative `h`/`q` and close inflow + rainfall excess - outflow - storage change within the named tolerance. | Typed runtime hard fail / staged implementation stop. | `INV-OFEROUTE-005`, `INV-OFEROUTE-006` |
| CFL stability | `Cr > 1` candidate sub-timestep | Reduce `Delta t` until `Cr <= 1`; if no finite positive sub-timestep can satisfy CFL, fail closed. | Typed runtime hard fail. | `INV-OFEROUTE-007` |
| Cascade handoff | Active multi-OFE routing | Publish upstream OFE outlet hydrograph as downstream upstream-boundary profile; do not synthesize from daily `Q`. | D5 closure requires seam evidence; activation requires typed runtime hard fail for malformed handoffs. | `INV-OFEROUTE-008` |
| DC01 runon ownership | Active routing SUPERSEDES DC01 runon re-infiltration | The routed subsystem owns the hourly runon supply (surface-routed, not re-infiltrated) and supersedes DC01's daily-lump admission; per-OFE infiltration acts on rainfall only. | Design resolved by `GAP-OFEROUTE-003` (SUPERSEDE); runtime integration must hard-fail if the routed hourly runon and DC01 daily-lump paths both feed the same active lane. | `INV-OFEROUTE-009` |
| Subsurface coupling & baseflow | Active routing on a hillslope with subsurface excess / baseflow | Subsurface exfiltration (return flow / saturation excess) enters the OFE surface routing as a source term; baseflow (non-exfiltrated subsurface outflow) is conserved and exported; the full hillslope water balance closes with the subsurface terms present. | Production activation BLOCKED until implemented + fixture-verified; typed runtime hard fail on water-balance non-closure when active. | `INV-OFEROUTE-012` |
| Default activation | Subsystem off | Preserve byte identity of protected outputs and publish no phase-span routed side effects. | Protected-output diff blocks promotion. | `INV-OFEROUTE-010` |

## Invariants

| Invariant ID | Statement | Guard | Failure posture | Authority | Evidence |
|---|---|---|---|---|---|
| INV-OFEROUTE-001 | Friction-menu domain and additivity: `f_s, f_f, f_w, f_veg >= 0` and `f_eq = f_s + f_f + f_w + f_veg` (eq. (7)); `lambda in [0,1]`; physically degenerate no-flow/component-absent operands (`q<=0`, `h<=0`, `D_r<=0`, absent canopy) yield the relevant component `0`, never NaN. Anchors the shadow-first `ofe_routing::friction` kernels. | pure-kernel tests + D4 runtime domain guard | Hard fail for invalid/non-finite/out-of-domain operands; approved bounded zero only for physically absent flow/component states. | REF-OFEROUTE-FRAMEWORK, REF-OFEROUTE-PHYS-BOUNDS | `[DIRECT][Static]` |
| INV-OFEROUTE-002 | Skin regime dispatch: `Re <= ~1000` uses Shen & Li `f_s = (3393 I^0.407 + k_o)/Re` (eq. 2), `Re > ~1000` uses Hirsch `f_s = 3.19/Re^0.45` (eq. 3); rainfall intensity `I` in `m s^-1`, `Re = q/nu`. **D8 status:** the local SI `I` convention is confirmed against R-63's explicit "rainfall intensity (m/s)" statement by a low-`k_o`, rain-driven regression where the `I` term dominates `k_o`; active callers must fail closed before negative/non-finite intensity reaches the pure equation helper, and the helper no longer silently normalizes negative `I` to zero. Exact Shen & Li / Hirsch / Woolhiser primary-source coefficient provenance remains frozen-library (`GAP-OFEROUTE-002`). | pure-kernel branch tests + D8 low-`k_o` unit regression + D-val operand reconstruction | Test failure or D-val failure blocks promotion; invalid active operands hard fail. | REF-OFEROUTE-SKIN-SHENLI, REF-OFEROUTE-SKIN-HIRSCH, REF-OFEROUTE-KO-WOOLHISER | `[DIRECT][Static] via R-63` |
| INV-OFEROUTE-003 | Form + wave submergence gating: `f_f = (16/pi) C_d (h/D_r) lambda` (eq. 4); when `h/D_r < 1` wave resistance applies with `f_w = 3.32 lambda / Fr^0.5` for `Fr > 0.5`, ramping linearly `0 -> f_w(Fr=0.5)` for `Fr <= 0.5` (eq. 5). Applicability bounds per Lawrence 1997 / the Abrahams 1998 discussion. | pure-kernel branch tests + D-val Case 2/4 | Test failure or D-val failure blocks promotion; invalid active operands hard fail. | REF-OFEROUTE-FORM-LAWRENCE, REF-OFEROUTE-WAVE-HUABRAHAMS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-OFEROUTE-004 | Vegetation resistance: `beta = min(0.135 sqrt(LAI/h_c), 0.33)` (cap enforced), `L_c = (C_d LAI/h_c)^-1`, `f_veg` per eq. (6); zero for unsubmerged/absent canopy. | pure-kernel branch tests + D-val Case 3 | Test failure or D-val failure blocks promotion; invalid active operands hard fail. | REF-OFEROUTE-VEG-KATUL | `[DIRECT][Static]` |
| INV-OFEROUTE-005 | KWE fidelity: continuity `dh/dt + dq/dx = v` with `q = alpha h^1.5`, `alpha = C S_o^0.5`, `v` the rainfall-excess rate; no negative depth/discharge published. | D4 solver tests + runtime non-negativity/domain guard | Typed runtime hard fail; staged solver cannot promote. | REF-OFEROUTE-KWE, REF-OFEROUTE-PHYS-BOUNDS | `[DIRECT][Static]` |
| INV-OFEROUTE-006 | Scheme conservation: the TVD-MacCormack step must conserve mass to numerical noise over each OFE (inflow + rainfall-excess - outflow = storage change within a named tolerance); the dissipative term must not create/destroy mass. Per-increment conservation is a hard stop during staged implementation. | D4 per-increment closure guard + solver tests | Typed runtime hard fail / implementation `HOLD` until named tolerance evidence exists. | REF-OFEROUTE-TVD-MACCORMACK, REF-OFEROUTE-FRAMEWORK | `[DIRECT][Static] via R-63` |
| INV-OFEROUTE-007 | CFL stability: `Cr = (Delta t/Delta x) c <= 1` at every cell, `c = 1.5 C S_o^0.5 h^0.5`; `Delta t` is selected to satisfy CFL, and a CFL violation is a hard-fail rather than a silently-unstable run. | D4 CFL guard + timestep-selection tests | Typed runtime hard fail if finite positive sub-timestep cannot satisfy CFL. | REF-OFEROUTE-TVD-MACCORMACK | `[DIRECT][Static] via R-63` |
| INV-OFEROUTE-008 | Per-OFE hydrograph handoff: the upstream OFE outlet hydrograph is the downstream OFE upstream boundary condition, carried as an hourly profile over the `SC-RUNOFFPART-001#INV-RUNOFFPART-029` transfer seam. Aggregating the routed hydrograph to a daily transfer, or synthesizing the downstream boundary from a daily `Q`, is invalid when the subsystem is active. | D5 cascade seam tests + runtime transfer guard | D5 closure requires cascade seam proof; activation/integration requires typed runtime hard fail for malformed handoffs. | REF-OFEROUTE-FRAMEWORK, SC-RUNOFFPART-001#INV-RUNOFFPART-029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-OFEROUTE-009 | Runon ownership (SUPERSEDE, faithful Papanicolaou): when this subsystem is active, per-OFE Green-Ampt-Mein-Larsen infiltration acts on the RAINFALL rate (assumption 2) to form each OFE's routed rainfall-excess, and the upstream OFE's outlet hydrograph is added as a downstream SURFACE boundary condition (assumption 1) that is NOT re-infiltrated. The routing therefore OWNS the hourly inter-OFE runon supply and SUPERSEDES DC01's daily-lump runon re-infiltration (`SC-RUNOFFPART-001#INV-RUNOFFPART-031`) with hydraulic surface routing - there is no second re-infiltration of the routed runon, and the two paths must never both feed the same active lane (double-count). Design RESOLVED (`GAP-OFEROUTE-003`, SUPERSEDE); D6 landed the infiltration kernel + rainfall-to-runoff coupling (shadow-first). | D5 cascade + D6 infiltration coupling (landed, shadow-first) + activation DC01-disable guard | Design resolved and routing mechanism landed; production wiring + DC01-disable guard is the activation gate; typed runtime hard fail after that implementation. | SC-RUNOFFPART-001#INV-RUNOFFPART-031, REF-OFEROUTE-FRAMEWORK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-OFEROUTE-010 | Opt-in activation: with the subsystem OFF (default), all protected hillslope outputs are byte-identical to the pre-subsystem runtime; the subsystem produces no phase-span side effect on the default path. Default activation is a separate later gate (not authorized by this contract or by ADR-0033). | default-path identity gate | Protected-output byte diff blocks promotion. | REF-OFEROUTE-FRAMEWORK, ADR-0033 | `[DIRECT][Static]` |
| INV-OFEROUTE-011 | Validation acceptance (D-val): the four Papanicolaou validation cases reproduce the published enhanced-WEPP Nash-Sutcliffe efficiencies within a named tolerance (`Ef` 0.91 bare / 0.75 roughness elements / 0.87 vegetation / 0.88 curvature-shock), and the Zone 1/Zone 2 stream-power taxonomy is reproduced qualitatively (thresholds within stated bounds of the paper's `k`/`l` fits). Case inputs and provenance: `docs/work-packages/20260702-mofefid-d01-ofe-routing-scaffold-001/artifacts/`. **Stage assignment:** this Ef acceptance is the D-val stage executed at D5/integration, NOT the D4 single-OFE routing solver: Cases 1-3 require Green-Ampt rainfall->rainfall-excess infiltration owned by `SC-RUNOFFPART-001` (the routing solver consumes rainfall excess and cannot reproduce the rainfall->runoff hydrograph alone), and Case 4 requires the digitized observed series (the in-repo supplemental carries the paper's model series, not cleanly-labeled observed data). The D-val computes `NS_trace` (openWEPP vs digitized enhanced-WEPP MODEL trace = method fidelity); the paper's `Ef_obs` (enhanced-vs-observed) stays a citation and is NOT recomputed as openWEPP validation against nature. **D8 status (2026-07-02, PARTIAL — invariant NOT closed):** Case 1 (bare) remains PARTIAL: steady magnitude reproduces (`NS_trace` 0.868) but 10-90% rise remains about 5000 s vs 3580 s; D8 attributes the lag to Green-Ampt operand sensitivity, not routing celerity. Case 2 (isolated) is operand-limited: default texture-derived `Ks=20 mm/h` gives `NS_trace` 0.454 / peak ratio 0.747, while plausible `Ks=10 mm/h` gives `NS_trace` 0.961 / peak ratio 0.922. Case 3 remains comparator-surface/operand boundary: the enhanced trace peak exceeds the recorded rainfall-length ceiling under the D01 operands. Case 4 (curvature shock) is shock-capture numerics boundary (`GAP-OFEROUTE-005`): D8 fixed sampled-hydrograph attribution, after which `k_o=200` is `NS_trace` 0.263, peak ratio 0.837, sampled `t_peak` 37 s vs 25.98 s, and the peak/timing remain resolution-sensitive. **ZERO cases cleanly reproduce.** Zone 1/Zone 2 stream-power taxonomy is NOT YET RUN and remains explicitly DEFERRED. Evidence: D7 artifacts plus `docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/artifacts/`. | D-val harness (`tools/dval/`) + `ofe_routing::dval` cited-scalar tests + source-manifest checks | D-val failure blocks activation/default promotion and requires investigation; INV-011 stays open until all cases reproduce or their gaps are dispositioned. | REF-OFEROUTE-FRAMEWORK, REF-OFEROUTE-SHOCK-IWAGAKI | `[DIRECT][Static]` |
| INV-OFEROUTE-012 | Subsurface coupling & baseflow (activation gate). When the subsystem is active, the OFE surface router MUST be coupled to the subsurface (`SC-SUBHYD-001`): (a) subsurface excess that exfiltrates — return flow / saturation excess, where subsurface lateral inflow plus local water exceeds soil storage/transmission capacity — MUST enter the kinematic-wave routing as a surface source term, not be dropped; (b) **baseflow** (subsurface outflow leaving the hillslope that is not exfiltrated) MUST be conserved and exported, not silently lost when the router owns the surface path; (c) the full hillslope water balance (rainfall + upstream inflow = surface outflow + subsurface outflow + ET + storage change) MUST close with the subsurface terms present. Rationale: the target steep-wet-forest hillslopes are subsurface-dominated — H2637 routes ~99% as lateral flow (`SC-SUBHYD-001#INV-SUBHYD-033`, MAGPARITY01) — so a surface-only active router drops the dominant pathway. **Until this coupling is implemented AND functional — with a subsurface-excess-to-runoff test fixture and a subsurface-dominated (H2637-class) full-hillslope closure vector — production activation of the subsystem is BLOCKED** (composes with `INV-OFEROUTE-010`/`011`). Coupling-seam design is RESOLVED (`GAP-OFEROUTE-006`, 2026-07-05): (a) binds to `ui_SCrunf` per hour (+ `wb14_hourly_excess`) as the routed source series; the subsurface inter-OFE carry `ui_LfCrf` stays subsurface; (b) binds to the outlet `latqcc` export bypassing the router; activation additionally REQUIRES the hourly lane (daily-lane hillslopes fail closed — the seam consumes the `INV-SUBHYD-023` hourly carries). | subsurface-excess-to-runoff fixture + subsurface-dominated closure vector + activation gate | Production activation BLOCKED until implemented + fixture-verified; typed runtime hard fail on water-balance non-closure when active. | SC-SUBHYD-001, REF-OFEROUTE-FRAMEWORK, REF-OFEROUTE-PHYS-BOUNDS | `[INFERENCE][Static]` (operator-directed gate; coupling design not yet resolved) |

## Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact | Evidence |
|---|---|---|---|---|---|
| `INV-OFEROUTE-001` | `ofe_routing::friction` non-negativity/additivity + degenerate-input tests; D4 active-domain guard | test + runtime | typed test failure; typed runtime failure for invalid active operands | D3 kernel tests (landed); D4 domain tests pending | `[DIRECT][Static]` |
| `INV-OFEROUTE-002..004` | friction regime/gating/cap tests + D-val Case 1/2/3 operand reproduction | test + validation | typed test / D-val Ef failure | D-val fixtures | `[DIRECT][Static]` |
| `INV-OFEROUTE-005..007` | KWE/TVD/CFL solver per-increment conservation + CFL guard | runtime + test | hard error / per-increment stop | D4 solver stages | `[DIRECT][Static] via R-63` |
| `INV-OFEROUTE-008..009` | per-OFE hydrograph handoff over the transfer seam; DC01 double-count reconciliation | runtime + governance | hard error for malformed handoff; integration hard error if routed hourly runon double-counts DC01 daily-lump admission | D5 cascade; `GAP-OFEROUTE-003` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-OFEROUTE-010` | default-path byte-identity gate with subsystem off | runtime + promotion | protected-output diff | every stage | `[DIRECT][Static]` |
| `INV-OFEROUTE-011` | D-val Ef + Zone taxonomy vs the four cases | validation | Ef out of tolerance -> investigate | D-val | `[DIRECT][Static]` |

## Producer Obligations

| Obligation ID | Producer | Obligation | Guard / evidence |
|---|---|---|---|
| OBL-OFEROUTE-P-001 | Friction kernels | Preserve equation-specific operands, domains, additivity, and bounded no-flow normalization exactly as `INV-OFEROUTE-001..004` state. | D3/D4 unit tests and D-val operand reconstruction. |
| OBL-OFEROUTE-P-002 | Single-OFE solver | Publish finite non-negative `h`/`q`, enforce CFL, and emit per-increment conservation diagnostics before any activation. | D4 solver tests and closure evidence. |
| OBL-OFEROUTE-P-003 | Cascade runtime | Carry upstream OFE outlet hydrograph as the downstream boundary profile without daily aggregation. | D5 downstream-consumer proof. |
| OBL-OFEROUTE-P-004 | Runon ownership | Apply the resolved `GAP-OFEROUTE-003` SUPERSEDE design (per-OFE infiltration on rainfall; routed runon surface-routed, not re-infiltrated; DC01 daily-lump disabled when active) before active multi-OFE routing. | D5/D6 design disposition plus integration runtime double-count guard. |
| OBL-OFEROUTE-P-005 | Activation gate | Keep subsystem default-off byte-flat and require a separate default-activation gate. | Protected-output identity evidence. |
| OBL-OFEROUTE-P-006 | Subsurface coupling | Before active multi-OFE routing, couple subsurface excess (return flow / saturation excess) into the OFE surface router as a source term and conserve/export baseflow, closing the full hillslope water balance; resolve the `GAP-OFEROUTE-006` seam ownership. | `INV-OFEROUTE-012` gate: subsurface-excess-to-runoff fixture + subsurface-dominated closure vector + activation block until functional. |

## Consumer Obligations

| Obligation ID | Consumer | Obligation | Guard / evidence |
|---|---|---|---|
| OBL-OFEROUTE-C-001 | `SC-RUNOFFPART-001` rainfall-excess seam | Provide rainfall-excess forcing to the solver in `m s^-1` and receive routed runon only through the named seam when active. | D4/D5 seam tests. |
| OBL-OFEROUTE-C-002 | Downstream OFE routing | Consume the upstream OFE outlet hydrograph as an hourly boundary profile, not a daily scalar. | D5 cascade test and trace evidence. |
| OBL-OFEROUTE-C-003 | Default hillslope runtime | Ignore all routed state when the subsystem is off. | Default-path byte identity gate. |
| OBL-OFEROUTE-C-004 | Erosion consumers (`SC-SED-001`) | Treat routed hydraulics as prospective until sediment coupling authorizes exact peak/duration surfaces. | Follow-on sediment contract gate. |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `h` | `ofe_route.depth_m[cell]` (prospective) | D4 solver state | Same unit, `m`; boundary registry pending D4. | SC-OFEROUTE-001 |
| `q` | `ofe_route.unit_discharge_m2_s[cell]` (prospective) | D4 solver state and D5 outlet hydrograph | Same unit, `m^2 s^-1`; publication metadata pending D4/D5. | SC-OFEROUTE-001 |
| `v` | rainfall-excess forcing field, exact runtime name pending D4 | SC-RUNOFFPART -> SC-OFEROUTE input seam | Same unit, `m s^-1`; conversion helper or scalar exception must be recorded before binding. | SC-RUNOFFPART-001 / SC-OFEROUTE-001 |
| `f_s`, `f_f`, `f_w`, `f_veg`, `f_eq` | `ofe_routing::friction::*` return values | D3 friction kernels / D4 solver operands | Dimensionless; pure scalar internal values. | SC-OFEROUTE-001 |
| `Re`, `Fr`, `Cr` | solver diagnostic scalars, exact runtime names pending D4 | D4 guard diagnostics | Dimensionless; scalar exception acceptable until publication. | SC-OFEROUTE-001 |
| `C`, `alpha` | solver coefficient fields, exact runtime names pending D4 | D4 solver operands | Same units as declared; typed wrapper or scalar exception required before runtime boundary publication. | SC-OFEROUTE-001 |
| `Delta t`, `Delta x`, `S_o` | sub-timestep, mesh spacing, OFE slope fields, exact runtime names pending D4 | D4 solver configuration | `s`, `m`, and `m m^-1`; boundary registry pending D4. | SC-OFEROUTE-001 |
| outlet hydrograph | `ofe_route.outlet_unit_discharge_m2_s[t]` (prospective) | D5 inter-OFE transfer seam | Same unit, `m^2 s^-1`; seam metadata required before D5 closure. | SC-OFEROUTE-001 / SC-RUNOFFPART-001 |

## Constants and Parameters

| Name | Value / domain | Units | Provenance | Notes |
|---|---|---|---|---|
| `g` | `9.81` | m s^-2 | REF-OFEROUTE-KWE / standard gravity convention | D4 may use the repository standard gravity constant if already canonicalized. |
| `nu` | local representative convention `1.14e-6` | m^2 s^-1 | REF-OFEROUTE-SKIN-SHENLI via R-63 (`Re = q/nu`) + local D8 unit audit | R-63 confirms the unit relation; exact temperature/value provenance remains secondary/frozen-library, not primary-read. |
| `m` | `1.5` | dimensionless | REF-OFEROUTE-KWE | Kinematic depth-discharge exponent. |
| Skin crossover | `Re <= 1000` Shen & Li; `Re > 1000` Hirsch | dimensionless | REF-OFEROUTE-SKIN-SHENLI, REF-OFEROUTE-SKIN-HIRSCH | Approximate crossover in R-63; primary coefficient/crossover provenance remains `GAP-OFEROUTE-002`. |
| Shen & Li constants | `3393`, `0.407`, `k_o` | mixed / dimensionless as in eq. (2) | REF-OFEROUTE-SKIN-SHENLI, REF-OFEROUTE-KO-WOOLHISER | Frozen-library posture via R-63/KINEROS. |
| Hirsch constants | `3.19`, `0.45` | dimensionless exponents/coefficient | REF-OFEROUTE-SKIN-HIRSCH | Frozen-library posture via R-63. |
| Form coefficient | `16/pi` multiplier with `C_d`, `h/D_r`, `lambda` | dimensionless | REF-OFEROUTE-FORM-LAWRENCE | Applicability bounds confirmed by D-val. |
| Wave coefficient | `3.32`, `Fr` exponent `-0.5`, low-`Fr` ramp at `0.5` | dimensionless | REF-OFEROUTE-WAVE-HUABRAHAMS | Low-`Fr` linear ramp is part of the local accepted formulation. |
| Vegetation beta | `min(0.135 sqrt(LAI/h_c), 0.33)` | dimensionless | REF-OFEROUTE-VEG-KATUL | Cap is mandatory. |
| D-val `Ef` targets | `0.91`, `0.75`, `0.87`, `0.88` | dimensionless | REF-OFEROUTE-FRAMEWORK, REF-OFEROUTE-SHOCK-IWAGAKI | Case-specific acceptance targets; tolerance must be named by D-val package. |

## Unit-Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `h` | m | pending D4 | none if stored in SI | not allowed once runtime-bound | required before publication |
| `q` | m^2 s^-1 | pending D4/D5 | none if stored in SI | not allowed once runtime-bound | required for outlet hydrograph publication |
| `v` | m s^-1 | pending D4 seam registration or `SC-RUNOFFPART-001` reference | required if rainfall-excess source is depth-per-hour/day | not allowed for active seam without recorded helper | required only if published |
| `C`, `alpha` | m^(1/2) s^-1 | pending D4 if exposed | none if internal SI | allowed only for internal non-public scalar | required before publication |
| `Re`, `Fr`, `Cr`, `f_*` | dimensionless | none for internal pure kernels; pending if published | none | allowed for internal scalar diagnostics | required before publication |
| `Delta t`, `Delta x`, `S_o` | s, m, m m^-1 | pending D4 | required if sourced from hourly/day or length sidecar units | not allowed for active boundary without recorded helper | not published by default |

## Tolerance and Numeric Notes

- Default-off identity is byte-exact for protected outputs.
- D3 friction-kernel unit tests may use roundoff-level tolerances only where
  equation evaluation is floating-point sensitive; branch/domain outcomes are
  exact.
- D4 must name the per-increment conservation tolerance before solver promotion;
  `INV-OFEROUTE-006` cannot close with an unnamed "numerical noise" claim.
- CFL is not tolerance-based: every accepted sub-timestep must satisfy
  `Cr <= 1` after finite celerity construction.
- D-val acceptance tolerances for the four `Ef` targets and Zone 1/2 taxonomy
  must be declared in the D-val package before default activation is considered.

## Test-Vector Obligations

| Obligation | Minimum vectors | Expected evidence |
|---|---|---|
| Friction menu | Unit vectors for skin laminar/turbulent dispatch, form resistance, wave gating/ramp, vegetation beta cap, additivity, and no-flow degeneracy. | D3/D4 unit tests grounded in eqs. (2)-(7). |
| Single-OFE solver (D4) | Physics vectors: mass conservation (discretization-only, resolution-convergent, positivity-clamp = 0), CFL stability, Case-1 steady-state, Case-4 positive-front shock smoke coverage, fail-closed forcing/parameter/timestep domains. | D4 KWE/TVD/CFL closure + non-negativity + fail-closed tests. Formal shock/D-val fidelity is NOT a D4 vector; see `INV-OFEROUTE-011` and `GAP-OFEROUTE-005`. |
| Multi-OFE cascade (D5) | Cascade conservation (width-aware), handoff volume continuity (incl. width change), downslope accumulation, Case-3 vegetated-strip steady-storage signature, degenerate fail-closed. | D5 `ofe_routing::cascade` tests (landed, shadow-first); `GAP-OFEROUTE-003` design resolved (SUPERSEDE, faithful Papanicolaou - see D6), production wiring + DC01-disable integration-scope. |
| D-val Ef acceptance (INV-OFEROUTE-011) | Papanicolaou Cases 1-4 Nash-Sutcliffe reproduction + Zone taxonomy. | D5/integration: infiltration-coupled forcing (Cases 1-3, SC-RUNOFFPART) + digitized observed series; the D4 Nash-Sutcliffe harness executed against those. NOT a D4 vector. |
| Default path | H2637 or equivalent protected-output run with subsystem off. | Byte-identical protected outputs and no routed side effects. |
| Subsurface coupling & baseflow (`INV-OFEROUTE-012`) | A **subsurface-excess-to-runoff fixture** exercising return flow / saturation-excess exfiltration through the active router (subsurface lateral inflow exceeding soil capacity → surface source term → routed), plus a subsurface-dominated (H2637-class) full-hillslope water-balance closure vector including baseflow export. | Fixture-verified subsurface→surface coupling + closed water balance with subsurface terms present; required before production activation. |
| Source provenance | D01 source manifest plus any newly acquired primary references. | Hash/source checks before D-val acceptance. |

## Binding Exposure Index

Status: `lane-d-prospective`
Evidence mode: `Static`

This index maps SC-OFEROUTE-001 invariants to their runtime binding surfaces.
The friction-menu row is already bound (the shadow-first `ofe_routing::friction`
kernels landed in D01). The solver and cascade rows are **prospective** — their
bindings do not exist until D4/D5 implement them under this ratified contract —
and are routed to `science-review-follow-on` so the completion gate is not
satisfied by an un-implemented subsystem.

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `OFEROUTE-FRICTION-MENU` | `SC-OFEROUTE-001.md#algorithm-specification` | `active` | `maps-to-existing-INV` | `INV-OFEROUTE-001, INV-OFEROUTE-002, INV-OFEROUTE-003, INV-OFEROUTE-004` | `none` | Landed: `ofe_routing::friction` pure kernels (eqs. 2-7), shadow-first, unit-tested. Contract-anchored retroactively per ADR-0033. |
| `OFEROUTE-KWE-TVD-SOLVER` | `SC-OFEROUTE-001.md#algorithm-specification` | `active` | `unpromoted-binding` | `INV-OFEROUTE-005, INV-OFEROUTE-006, INV-OFEROUTE-007` | `science-review-follow-on` | D4 landed the single-OFE KWE + TVD-MacCormack + CFL solver (shadow-first), validated on the physics it owns: mass conservation (discretization-only, resolution-convergent, positivity-clamp = 0 across all clamp sites), CFL stability, Case-1 steady-state, and Case-4 positive-front smoke coverage. Formal shock/D-val fidelity (`INV-OFEROUTE-011`) is NOT a D4 evidence surface and remains partial with `GAP-OFEROUTE-005`. |
| `OFEROUTE-CASCADE-HANDOFF` | `SC-OFEROUTE-001.md#algorithm-specification` | `active` | `unpromoted-binding` | `INV-OFEROUTE-008, INV-OFEROUTE-009` | `science-review-follow-on` | D5 landed `ofe_routing::cascade` (shadow-first): OFE-by-OFE routing with width-aware outlet-hydrograph -> upstream-boundary handoff (INV-008), validated on cascade conservation, handoff volume continuity, downslope accumulation, and the Case-3 vegetated-strip storage signature. `GAP-OFEROUTE-003` (INV-009) design resolved (SUPERSEDE, faithful Papanicolaou); D6 landed `ofe_routing::infiltration` (Green-Ampt) + `run_infiltrated_cascade` (rainfall -> excess -> route). Production wiring + DC01-disable is the activation gate, so this stays routed to science-review-follow-on until integrated. |
| `OFEROUTE-ACTIVATION-VALIDATION` | `SC-OFEROUTE-001.md#invariants` | `active` | `maps-to-existing-INV` | `INV-OFEROUTE-010, INV-OFEROUTE-011, INV-OFEROUTE-012` | `science-review-follow-on` | Opt-in default-byte-flat gate + D-val Ef/Zone acceptance + the subsurface-coupling/baseflow activation gate (`INV-OFEROUTE-012`, `GAP-OFEROUTE-006`, design-open). The byte-flat gate is enforceable now (subsystem-off); D-val at each stage; the subsurface-coupling gate blocks production activation until subsurface-excess-to-runoff is implemented + fixture-verified. |

## Gaps

| Gap ID | Statement | Impact | Disposition | Evidence |
|---|---|---|---|---|
| GAP-OFEROUTE-001 | TVD-MacCormack numerics primaries (Garcia-Navarro 1992, Mingham 2001) are un-acquired under the frozen reference library; the scheme (eqs. 8-14) is cited secondary via R-63. | Scheme constants/limiter form taken from R-63; re-derivable from open kinematic-wave/shallow-water sources already in-repo (USGS PP1302, KINEROS). | promotable-with-risk for shadow physics tests only; D8 shows Case 4 peak/timing remain resolution-sensitive after sampled-hydrograph correction, so limiter/shock fidelity is NOT confirmed by D-val (`GAP-OFEROUTE-005`). | `[DIRECT][Static] via R-63` |
| GAP-OFEROUTE-002 | Formulation primaries Shen & Li (1973), the Abrahams (1998) discussion of Lawrence, and Woolhiser (1975) k_o tables are un-acquired (frozen library); eqs. (2)-(3) constants and eq. (4) applicability bounds are cited secondary via R-63 / KINEROS. | Constant/regime-bound fidelity rests on R-63's statement plus local regression rather than a digit-by-digit primary read. | promotable-with-risk; D8 closes the local SI `I` unit convention against R-63 with a low-`k_o` regression, but primary coefficient/crossover and Woolhiser `k_o` provenance remain frozen-library. | `[DIRECT][Static] via R-63` |
| GAP-OFEROUTE-003 | The runon re-infiltration reconciliation (`INV-OFEROUTE-009`) vs DC01's daily-lump admission (`INV-RUNOFFPART-031`). **Design resolved (D5/D6, 2026-07-02): SUPERSEDE (faithful Papanicolaou assumption 2).** When the routing subsystem is active, per-OFE Green-Ampt-Mein-Larsen infiltration acts on the **RAINFALL** rate to produce the rainfall-excess the cascade routes; the upstream OFE's outlet hydrograph is added as a downstream **surface boundary condition** (D5 handoff) and is **NOT re-infiltrated**. The routing therefore SUPERSEDES DC01's daily-lump runon re-infiltration with hydraulic surface routing — it does NOT compose a second re-infiltration of the routed runon (a D5-wording correction: the earlier 'compose with re-infiltration of the routed hydrograph' misread; infiltration composes on rainfall, not on runon). D6 landed `ofe_routing::infiltration` (Green-Ampt kernel) + `run_infiltrated_cascade` (rainfall -> per-OFE infiltration -> excess -> route), shadow-first. Remaining INTEGRATION scope: wire this into production and DISABLE DC01's daily-lump admission when the subsystem is active (production activation gate). | Rainfall-to-runoff routing mechanism landed (D6); runtime production wiring + DC01-disable is the activation gate. | design-resolved; production-wiring-integration-scope. | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-OFEROUTE-004 (WITHDRAWN 2026-07-02) | **Withdrawn on correction.** The D7 execution first attributed a ~5-6 s Iwagaki shock-timing lag to the solver, but that rested on a forcing bug: `run_iwagaki` fed the lateral-supply rate into the skin-term rainfall intensity `I`, although the Iwagaki experiment has NO rain (`I = 0`). The D7 forcing-bug attribution remains withdrawn. | n/a (withdrawn) | withdrawn — the D7 attribution was an artifact of a corrected forcing bug. | `[DIRECT][Static]` |
| GAP-OFEROUTE-006 | **Subsurface-excess-to-runoff (return flow / saturation excess) and baseflow are NOT coupled into the active surface router.** The Lane D solver forcing is rainfall-excess (`v = r - i_f`) + the upstream OFE **surface** hydrograph only (`ofe_routing::kinematic_wave::Forcing`); there is no subsurface exfiltration source term and no baseflow export, and the contract originally scoped subsurface entirely out (owned by `SC-SUBHYD-001`). On subsurface-dominated steep-wet-forest hillslopes — openWEPP's target class, where H2637 routes ~99% as lateral flow (`SC-SUBHYD-001#INV-SUBHYD-033`, MAGPARITY01) — a surface-only active router would drop the dominant water pathway. | **Blocks production activation** (`INV-OFEROUTE-012`): an active surface-only router would silently omit the majority of the water on the hillslopes that matter most. | **design-RESOLVED (2026-07-05, WP `20260705-mofefid-laned-gap006-subsurface-seam-design-001`): the exfiltration source term IS `ui_SCrunf`** (the `SC-SUBHYD-001#INV-SUBHYD-023` hourly top-layer saturation-excess clip — WEPP's only subsurface→surface pathway; return flow surfaces through it), consumed per-hour alongside `wb14_hourly_excess` (the same two limbs the DC01 weights already unify, as a rate series instead of a day-shape); the inter-OFE subsurface carry `ui_LfCrf` STAYS subsurface (the router supersedes SURFACE runon only — never subsurface transfer); outlet `latqcc` baseflow export bypasses the router; hourly lane is an activation precondition (daily lanes fail closed). Seam ownership: `SC-OFEROUTE-001` owns the coupling rule; `SC-SUBHYD-001` keeps the physics and the carry surfaces unchanged. IMPLEMENTATION remains the activation work (`INV-OFEROUTE-012` fixtures as specified in the WP's `artifacts/seam-design.md`). Analogous to `GAP-OFEROUTE-003` (opened → design-resolved → implemented). | `[INFERENCE][Static] + [DIRECT][Static]` (seam surfaces verified in SC-SUBHYD-001 + the DC01 lineage) |
| GAP-OFEROUTE-005 | Iwagaki shock sampled-hydrograph / resolution sensitivity. D8 corrected the solver sampler so hydrograph samples are interpolated to the actual sample time rather than stamped with the step-end value. After that correction, Case 4 no longer supports the D7 "timing/rise reproduce" claim: at `k_o=200`, sampled `t_peak` is about 37 s vs reference 25.98 s, `NS_trace` is about 0.263, and peak/timing are materially sensitive to cell count and max sub-step. A simple in-envelope increase of the implicit `alpha` iteration count was considered and rejected because it changed steady/cascade conservation tests, so no contract-backed numerical correction is landed here. | Blocks using Case 4 as D-val shock-capture confirmation or default-activation evidence; formal closure needs a separate shock numerics package with TVD primary/source authority, convergence criteria, and Iwagaki operand bounds. | declared boundary; D8 corrects the sampled metric and records the unresolved shock-capture numerical boundary. | `[DIRECT][Ran]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-05` | `3` | `Claude Code` | GAP-OFEROUTE-006 design-RESOLVED (Lane D contract-first continuation): the subsurface-coupling seam binds to existing contract-governed surfaces — exfiltration = the `ui_SCrunf` hourly saturation-excess clip (return flow surfaces through it; no new physics), `ui_LfCrf` stays subsurface, outlet `latqcc` baseflow bypasses the router, hourly-lane activation precondition; `INV-OFEROUTE-012` rewritten with the concrete bindings; gate-fixture specifications in the WP seam-design artifact. Implementation remains the activation work. |
| `2026-07-02` | `1` | `Claude Code` | Initial authoring (MOFEFID Lane D / D-gate): friction menu (eqs. 2-7), KWE + TVD-MacCormack + CFL (eqs. 8-14, A1-A2), per-OFE hydrograph handoff, runon re-infiltration coupling, opt-in activation, and D-val acceptance. `INV-OFEROUTE-001` anchors the landed shadow-first friction kernels; solver/cascade invariants (005-009) gate D4/D5. Frozen-library citation posture recorded (`GAP-OFEROUTE-001/002`). Authored for ratification per ADR-0033 narrowed scope. |
| `2026-07-02` | `2` | `Codex` | Ratification review amendment: added missing kernel-process profile sections, guard/failure-posture invariant fields, symbol alias and unit-governance maps, producer/consumer obligations, constants, tolerance notes, and test-vector obligations; moved gaps after the Binding Exposure Index; ratified the contract as the D4 prerequisite while preserving `GAP-OFEROUTE-003` as a D5 `HOLD`. |
| `2026-07-02` | `3` | `Claude Code` | MOFEFID-D4 review disposition: clarified that the D4 single-OFE solver evidence surface is PHYSICS validation (conservation/CFL/steady-state/shock), NOT formal Ef-vs-observed; `INV-OFEROUTE-011` Ef acceptance is explicitly the D-val stage at D5/integration (needs SC-RUNOFFPART infiltration coupling for Cases 1-3 and digitized observed series for Case 4). Corrects the `OFEROUTE-KWE-TVD-SOLVER` BEI note that implied D4 did case-Ef validation. No invariant/physics change. |
| `2026-07-02` | `4` | `Codex` | D4 merge re-check cleanup: made revision-history numbering unique and confirmed the Test-Vector Obligations row separates D4 physics evidence from D-val/D5 `Ef` acceptance. No invariant/physics change. |
| `2026-07-02` | `5` | `Claude Code` | MOFEFID-D5: landed the OFE-by-OFE cascade (`ofe_routing::cascade`, shadow-first) - width-aware outlet-hydrograph handoff (INV-OFEROUTE-008) validated on cascade conservation, handoff continuity, downslope accumulation, and the Case-3 vegetated-strip storage signature. Resolved `GAP-OFEROUTE-003` design as supersede-then-compose (routing owns hourly runon, supersedes DC01 daily-lump, composes with downstream hourly infiltration); the runtime infiltration composition + DC01-disable guard remains integration scope. |
| `2026-07-02` | `6` | `Claude Code` | MOFEFID-D6: landed `ofe_routing::infiltration` (Green-Ampt-Mein-Larsen per-OFE infiltration) + `run_infiltrated_cascade` (rainfall -> per-OFE excess -> routed cascade), shadow-first, completing the rainfall-to-runoff routing mechanism. CORRECTED the D5 rev-5 `GAP-OFEROUTE-003` resolution (`supersede-then-compose`, which implied re-infiltrating the routed hydrograph) to faithful-Papanicolaou SUPERSEDE: infiltration acts on RAINFALL (assumption 2), the upstream hydrograph is a surface boundary condition (assumption 1) that is NOT re-infiltrated, and routing supersedes DC01's daily-lump runon re-infiltration. Purged the now-contradictory re-infiltration/compose wording from the scope, kernel-process, `INV-OFEROUTE-009`, `OBL-OFEROUTE-P-004`, and `GAP-OFEROUTE-003` rows. Production wiring + DC01-disable is the activation gate. |
| `2026-07-02` | `7` | `Claude Code` | MOFEFID-D7 D-val execution: added `GAP-OFEROUTE-004` (shock-capture / ~5-6 s Iwagaki phase lag, solver-side, attributed); updated `INV-OFEROUTE-011` with the PARTIAL D-val status (Case 1 reproduces qualified, Cases 2-3 operand-limited/caveated, Case 4 gap), made the `NS_trace` (method fidelity) vs paper `Ef_obs` (citation) split explicit, and recorded the Zone 1/2 taxonomy as explicitly deferred (not closed). Invariant remains open. Evidence in the D7 work-package. No production wiring; routing stays shadow-first. |
| `2026-07-02` | `8` | `Claude Code` | MOFEFID-D7 review disposition (Codex execution review): WITHDREW `GAP-OFEROUTE-004` — the solver-side shock-lag attribution was an artifact of a forcing bug (`run_iwagaki` fed the lateral rate into the skin-term rainfall intensity `I`; Iwagaki has no rain). With `I=0`, Case-4 timing/rise reproduce at k_o~200 and the residual is operand-limited on the unspecified k_o. Corrected `INV-OFEROUTE-011` (Case 1 downgraded to PARTIAL — steady magnitude reproduces but rising-limb shape fails ~40 %; zero cases cleanly reproduce). Reconciled `INV-OFEROUTE-002` / `GAP-OFEROUTE-002` — the skin `I`/`nu` convention is NOT confirmed by D-val (k_o-dominated; audit open). Invariant remains open. |
| `2026-07-02` | `9` | `Codex` | MOFEFID-D8 defect closure: confirmed the local Shen & Li rainfall-intensity unit convention against R-63 with a low-`k_o` regression and removed silent negative-`I` normalization from the pure skin helper; corrected solver hydrograph sampling to interpolate within solver steps; reclassified Case 2 as Ks-operand-limited, Case 3 as comparator-surface/operand boundary, Case 1 rising-limb lag as Green-Ampt operand-limited, and opened `GAP-OFEROUTE-005` for Case 4 shock peak/timing resolution sensitivity after sampled metric correction. `INV-OFEROUTE-011` remains partial; no production wiring. |
| `2026-07-04` | `10` | `Claude Code` | **Operator-directed activation gate.** Opened `GAP-OFEROUTE-006` and `INV-OFEROUTE-012` requiring that, before production activation, the active surface router be COUPLED to the subsurface — subsurface excess (return flow / saturation excess) entering the routing as a source term, and baseflow conserved/exported — so an active router closes the full hillslope water balance on the subsurface-dominated steep-wet-forest hillslopes that are openWEPP's target (H2637 routes ~99% as lateral flow). The Lane D solver forcing is currently rainfall-excess + upstream SURFACE hydrograph only (`kinematic_wave::Forcing`); the contract had scoped subsurface entirely out. Added `OBL-OFEROUTE-P-006`, a Test-Vector Obligation naming the required **subsurface-excess-to-runoff fixture** + a subsurface-dominated closure vector, a Branch/Guard gate row, and refined the scope boundary (subsurface flow PHYSICS stays `SC-SUBHYD-001`; the exfiltration/baseflow COUPLING seam is an activation requirement here). Coupling-seam ownership/design is `GAP-OFEROUTE-006` (design-open). **Scope-expanding amendment — Codex ratification recommended.** |
