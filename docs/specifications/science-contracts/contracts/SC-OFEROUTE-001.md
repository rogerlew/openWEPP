---
contract_id: SC-OFEROUTE-001
title: Hillslope OFE-by-OFE Overland-Flow Routing Process Contract
status: approved
maturity: active
owner: openWEPP maintainers + hydrology reviewer
contract_version: 2
producer_scope:
  - Space/time-variant overland-flow resistance (skin/form/wave/vegetation) surfaces
  - 1-D kinematic-wave routing state (depth, unit discharge) per OFE per sub-timestep
  - Per-OFE outlet hydrograph handed to the downstream OFE as an upstream boundary condition
consumer_scope:
  - Runoff-partition rainfall-excess coupling (SC-RUNOFFPART-001)
  - Inter-OFE water transfer seam (SC-RUNOFFPART-001 INV-RUNOFFPART-028/029)
  - Erosion continuity consumers that read peak/duration surfaces (SC-SED-001)
evidence_level: static
last_reviewed: 2026-07-02
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
- Coupling of routed rainfall-excess re-infiltration to the DC01 runon
  semantics (`SC-RUNOFFPART-001#INV-RUNOFFPART-031`).
- Opt-in activation posture (default hillslope path unchanged).

Out of scope:
- Rainfall-excess / infiltration internals owned by `SC-RUNOFFPART-001`.
- Subsurface lateral flow owned by `SC-SUBHYD-001`.
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
| REF-OFEROUTE-SKIN-SHENLI | Shen & Li (1973), JHD ASCE 99(HY5) — **secondary via R-63** (eq. (2) as stated in Papanicolaou §2.2) | Laminar skin resistance `f_s = (3393 I^0.407 + k_o)/Re`, `Re < ~1000`. Frozen library: primary un-acquired; unit convention confirmed by D-val. | `[DIRECT][Static] via R-63` |
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
| Skin-regime dispatch | `Re <= 1000` vs. `Re > 1000` after finite `Re = q/nu` construction | Use Shen & Li below/at the crossover and Hirsch above it; D-val confirms unit convention and crossover fidelity. | Hard fail for missing/non-finite operands or invalid `nu`; test failure for wrong branch. | `INV-OFEROUTE-002` |
| Roughness domain | `lambda`, `C_d`, `D_r`, `LAI`, `h_c`, `S_o`, `Delta x`, `Delta t` consumed by active routing | Enforce finite values and stated domains before equation evaluation. | Typed runtime failure in D4/D5; unit-test failure for pure friction kernels. | `INV-OFEROUTE-001..007` |
| Form/wave submergence | `h/D_r < 1`, `Fr <= 0.5`, `Fr > 0.5` | Apply form resistance per eq. (4); apply wave resistance only under the stated submergence/Froude policy with the low-`Fr` linear ramp. | Hard fail for invalid finite operands; branch-test failure for wrong ramp. | `INV-OFEROUTE-003` |
| Vegetation branch | `LAI > 0` and `h_c > 0` vs. absent/unsubmerged canopy | Apply Katul beta cap and length scale when active; return zero vegetation resistance when canopy is absent. | Hard fail for invalid finite operands; bounded zero only for absent canopy. | `INV-OFEROUTE-004` |
| KWE/TVD conservation | Each solver sub-timestep | Preserve non-negative `h`/`q` and close inflow + rainfall excess - outflow - storage change within the named tolerance. | Typed runtime hard fail / staged implementation stop. | `INV-OFEROUTE-005`, `INV-OFEROUTE-006` |
| CFL stability | `Cr > 1` candidate sub-timestep | Reduce `Delta t` until `Cr <= 1`; if no finite positive sub-timestep can satisfy CFL, fail closed. | Typed runtime hard fail. | `INV-OFEROUTE-007` |
| Cascade handoff | Active multi-OFE routing | Publish upstream OFE outlet hydrograph as downstream upstream-boundary profile; do not synthesize from daily `Q`. | D5 closure requires seam evidence; activation requires typed runtime hard fail for malformed handoffs. | `INV-OFEROUTE-008` |
| DC01 runon ownership | Active routing plus downstream re-infiltration | The routed subsystem must own the hourly runon supply without double-counting the DC01 daily-lump path. | Design resolved by `GAP-OFEROUTE-003`; runtime integration must hard-fail if the routed hourly runon and DC01 daily-lump paths both feed the same active lane. | `INV-OFEROUTE-009` |
| Default activation | Subsystem off | Preserve byte identity of protected outputs and publish no phase-span routed side effects. | Protected-output diff blocks promotion. | `INV-OFEROUTE-010` |

## Invariants

| Invariant ID | Statement | Guard | Failure posture | Authority | Evidence |
|---|---|---|---|---|---|
| INV-OFEROUTE-001 | Friction-menu domain and additivity: `f_s, f_f, f_w, f_veg >= 0` and `f_eq = f_s + f_f + f_w + f_veg` (eq. (7)); `lambda in [0,1]`; physically degenerate no-flow/component-absent operands (`q<=0`, `h<=0`, `D_r<=0`, absent canopy) yield the relevant component `0`, never NaN. Anchors the shadow-first `ofe_routing::friction` kernels. | pure-kernel tests + D4 runtime domain guard | Hard fail for invalid/non-finite/out-of-domain operands; approved bounded zero only for physically absent flow/component states. | REF-OFEROUTE-FRAMEWORK, REF-OFEROUTE-PHYS-BOUNDS | `[DIRECT][Static]` |
| INV-OFEROUTE-002 | Skin regime dispatch: `Re <= ~1000` uses Shen & Li `f_s = (3393 I^0.407 + k_o)/Re` (eq. 2), `Re > ~1000` uses Hirsch `f_s = 3.19/Re^0.45` (eq. 3); rainfall intensity `I` in `m s^-1`, `Re = q/nu`. The exact crossover and `nu`/`I` unit convention are confirmed empirically by D-val Case 1/2 reproduction (frozen-library posture - primaries cited secondary via R-63). | pure-kernel branch tests + D-val Case 1/2 operand reconstruction | Test failure or D-val failure blocks promotion; invalid active operands hard fail. | REF-OFEROUTE-SKIN-SHENLI, REF-OFEROUTE-SKIN-HIRSCH, REF-OFEROUTE-KO-WOOLHISER | `[DIRECT][Static] via R-63` |
| INV-OFEROUTE-003 | Form + wave submergence gating: `f_f = (16/pi) C_d (h/D_r) lambda` (eq. 4); when `h/D_r < 1` wave resistance applies with `f_w = 3.32 lambda / Fr^0.5` for `Fr > 0.5`, ramping linearly `0 -> f_w(Fr=0.5)` for `Fr <= 0.5` (eq. 5). Applicability bounds per Lawrence 1997 / the Abrahams 1998 discussion. | pure-kernel branch tests + D-val Case 2/4 | Test failure or D-val failure blocks promotion; invalid active operands hard fail. | REF-OFEROUTE-FORM-LAWRENCE, REF-OFEROUTE-WAVE-HUABRAHAMS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-OFEROUTE-004 | Vegetation resistance: `beta = min(0.135 sqrt(LAI/h_c), 0.33)` (cap enforced), `L_c = (C_d LAI/h_c)^-1`, `f_veg` per eq. (6); zero for unsubmerged/absent canopy. | pure-kernel branch tests + D-val Case 3 | Test failure or D-val failure blocks promotion; invalid active operands hard fail. | REF-OFEROUTE-VEG-KATUL | `[DIRECT][Static]` |
| INV-OFEROUTE-005 | KWE fidelity: continuity `dh/dt + dq/dx = v` with `q = alpha h^1.5`, `alpha = C S_o^0.5`, `v` the rainfall-excess rate; no negative depth/discharge published. | D4 solver tests + runtime non-negativity/domain guard | Typed runtime hard fail; staged solver cannot promote. | REF-OFEROUTE-KWE, REF-OFEROUTE-PHYS-BOUNDS | `[DIRECT][Static]` |
| INV-OFEROUTE-006 | Scheme conservation: the TVD-MacCormack step must conserve mass to numerical noise over each OFE (inflow + rainfall-excess - outflow = storage change within a named tolerance); the dissipative term must not create/destroy mass. Per-increment conservation is a hard stop during staged implementation. | D4 per-increment closure guard + solver tests | Typed runtime hard fail / implementation `HOLD` until named tolerance evidence exists. | REF-OFEROUTE-TVD-MACCORMACK, REF-OFEROUTE-FRAMEWORK | `[DIRECT][Static] via R-63` |
| INV-OFEROUTE-007 | CFL stability: `Cr = (Delta t/Delta x) c <= 1` at every cell, `c = 1.5 C S_o^0.5 h^0.5`; `Delta t` is selected to satisfy CFL, and a CFL violation is a hard-fail rather than a silently-unstable run. | D4 CFL guard + timestep-selection tests | Typed runtime hard fail if finite positive sub-timestep cannot satisfy CFL. | REF-OFEROUTE-TVD-MACCORMACK | `[DIRECT][Static] via R-63` |
| INV-OFEROUTE-008 | Per-OFE hydrograph handoff: the upstream OFE outlet hydrograph is the downstream OFE upstream boundary condition, carried as an hourly profile over the `SC-RUNOFFPART-001#INV-RUNOFFPART-029` transfer seam. Aggregating the routed hydrograph to a daily transfer, or synthesizing the downstream boundary from a daily `Q`, is invalid when the subsystem is active. | D5 cascade seam tests + runtime transfer guard | D5 closure requires cascade seam proof; activation/integration requires typed runtime hard fail for malformed handoffs. | REF-OFEROUTE-FRAMEWORK, SC-RUNOFFPART-001#INV-RUNOFFPART-029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-OFEROUTE-009 | Runon re-infiltration coupling: routed inter-OFE excess re-infiltrates on the downstream OFE per `SC-RUNOFFPART-001#INV-RUNOFFPART-031` (DC01), hourly-faithful. When this subsystem is active it OWNS the hourly runon supply and must not double-count DC01's daily-lump runon admission; the reconciliation design is RESOLVED (`GAP-OFEROUTE-003`, SUPERSEDE): per-OFE Green-Ampt infiltration acts on the RAINFALL rate to form the routed excess, and the upstream hydrograph is a surface boundary condition that is NOT re-infiltrated - routing supersedes DC01's daily-lump runon re-infiltration. D6 landed the infiltration kernel + rainfall-to-runoff coupling (shadow-first). | D5 cascade + D6 infiltration coupling (landed, shadow-first) + activation DC01-disable guard | Design resolved and routing mechanism landed; production wiring + DC01-disable guard is the activation gate; typed runtime hard fail after that implementation. | SC-RUNOFFPART-001#INV-RUNOFFPART-031, REF-OFEROUTE-FRAMEWORK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-OFEROUTE-010 | Opt-in activation: with the subsystem OFF (default), all protected hillslope outputs are byte-identical to the pre-subsystem runtime; the subsystem produces no phase-span side effect on the default path. Default activation is a separate later gate (not authorized by this contract or by ADR-0033). | default-path identity gate | Protected-output byte diff blocks promotion. | REF-OFEROUTE-FRAMEWORK, ADR-0033 | `[DIRECT][Static]` |
| INV-OFEROUTE-011 | Validation acceptance (D-val): the four Papanicolaou validation cases reproduce the published enhanced-WEPP Nash-Sutcliffe efficiencies within a named tolerance (`Ef` 0.91 bare / 0.75 roughness elements / 0.87 vegetation / 0.88 curvature-shock), and the Zone 1/Zone 2 stream-power taxonomy is reproduced qualitatively (thresholds within stated bounds of the paper's `k`/`l` fits). Case inputs and provenance: `docs/work-packages/20260702-mofefid-d01-ofe-routing-scaffold-001/artifacts/`. **Stage assignment:** this Ef acceptance is the D-val stage executed at D5/integration, NOT the D4 single-OFE routing solver: Cases 1-3 require Green-Ampt rainfall->rainfall-excess infiltration owned by `SC-RUNOFFPART-001` (the routing solver consumes rainfall excess and cannot reproduce the rainfall->runoff hydrograph alone), and Case 4 requires the digitized observed series (the in-repo supplemental carries the paper's model series, not cleanly-labeled observed data). The Nash-Sutcliffe harness is implemented and unit-tested at D4; the Ef run is D-val. | D-val harness + source-manifest checks | D-val failure blocks activation/default promotion and requires investigation. | REF-OFEROUTE-FRAMEWORK, REF-OFEROUTE-SHOCK-IWAGAKI | `[DIRECT][Static]` |

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
| OBL-OFEROUTE-P-004 | Runon coupling | Apply the resolved `GAP-OFEROUTE-003` supersede-then-compose design before active multi-OFE routing composes with DC01. | D5 design disposition plus integration runtime double-count guard. |
| OBL-OFEROUTE-P-005 | Activation gate | Keep subsystem default-off byte-flat and require a separate default-activation gate. | Protected-output identity evidence. |

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
| `nu` | D-val convention, commonly `1.14e-6` | m^2 s^-1 | REF-OFEROUTE-SKIN-SHENLI via R-63; D-val confirmation | Exact value is a D4/D-val binding, not assumed solely from secondary text. |
| `m` | `1.5` | dimensionless | REF-OFEROUTE-KWE | Kinematic depth-discharge exponent. |
| Skin crossover | `Re <= 1000` Shen & Li; `Re > 1000` Hirsch | dimensionless | REF-OFEROUTE-SKIN-SHENLI, REF-OFEROUTE-SKIN-HIRSCH | Approximate crossover in R-63; D-val confirms. |
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
| Single-OFE solver (D4) | Physics vectors: mass conservation (discretization-only, resolution-convergent, positivity-clamp = 0), CFL stability, Case-1 steady-state, Case-4 shock structure, fail-closed forcing/parameter/timestep domains. | D4 KWE/TVD/CFL closure + non-negativity + fail-closed tests. Formal `Ef`-vs-observed is NOT a D4 vector; see the D-val row. |
| Multi-OFE cascade (D5) | Cascade conservation (width-aware), handoff volume continuity (incl. width change), downslope accumulation, Case-3 vegetated-strip steady-storage signature, degenerate fail-closed. | D5 `ofe_routing::cascade` tests (landed, shadow-first); `GAP-OFEROUTE-003` design resolved (supersede-then-compose), runtime composition integration-scope. |
| D-val Ef acceptance (INV-OFEROUTE-011) | Papanicolaou Cases 1-4 Nash-Sutcliffe reproduction + Zone taxonomy. | D5/integration: infiltration-coupled forcing (Cases 1-3, SC-RUNOFFPART) + digitized observed series; the D4 Nash-Sutcliffe harness executed against those. NOT a D4 vector. |
| Default path | H2637 or equivalent protected-output run with subsystem off. | Byte-identical protected outputs and no routed side effects. |
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
| `OFEROUTE-KWE-TVD-SOLVER` | `SC-OFEROUTE-001.md#algorithm-specification` | `active` | `unpromoted-binding` | `INV-OFEROUTE-005, INV-OFEROUTE-006, INV-OFEROUTE-007` | `science-review-follow-on` | D4 landed the single-OFE KWE + TVD-MacCormack + CFL solver (shadow-first), validated on the physics it owns: mass conservation (discretization-only, resolution-convergent, positivity-clamp = 0 across all clamp sites), CFL stability, Case-1 steady-state, Case-4 shock structure. The formal Ef-vs-observed acceptance (`INV-OFEROUTE-011`) is NOT a D4 evidence surface — it is the D-val stage below (D5/integration). |
| `OFEROUTE-CASCADE-HANDOFF` | `SC-OFEROUTE-001.md#algorithm-specification` | `active` | `unpromoted-binding` | `INV-OFEROUTE-008, INV-OFEROUTE-009` | `science-review-follow-on` | D5 landed `ofe_routing::cascade` (shadow-first): OFE-by-OFE routing with width-aware outlet-hydrograph -> upstream-boundary handoff (INV-008), validated on cascade conservation, handoff volume continuity, downslope accumulation, and the Case-3 vegetated-strip storage signature. `GAP-OFEROUTE-003` (INV-009) design resolved (SUPERSEDE, faithful Papanicolaou); D6 landed `ofe_routing::infiltration` (Green-Ampt) + `run_infiltrated_cascade` (rainfall -> excess -> route). Production wiring + DC01-disable is the activation gate, so this stays routed to science-review-follow-on until integrated. |
| `OFEROUTE-ACTIVATION-VALIDATION` | `SC-OFEROUTE-001.md#invariants` | `active` | `maps-to-existing-INV` | `INV-OFEROUTE-010, INV-OFEROUTE-011` | `none` | Opt-in default-byte-flat gate + D-val Ef/Zone acceptance; the byte-flat gate is enforceable now (subsystem-off), D-val at each stage. |

## Gaps

| Gap ID | Statement | Impact | Disposition | Evidence |
|---|---|---|---|---|
| GAP-OFEROUTE-001 | TVD-MacCormack numerics primaries (Garcia-Navarro 1992, Mingham 2001) are un-acquired under the frozen reference library; the scheme (eqs. 8-14) is cited secondary via R-63. | Scheme constants/limiter form taken from R-63; re-derivable from open kinematic-wave/shallow-water sources already in-repo (USGS PP1302, KINEROS). | promotable-with-risk; unit/limiter fidelity confirmed by D-val Case 4 (Iwagaki shock). | `[DIRECT][Static] via R-63` |
| GAP-OFEROUTE-002 | Formulation primaries Shen & Li (1973), the Abrahams (1998) discussion of Lawrence, and Woolhiser (1975) k_o tables are un-acquired (frozen library); eqs. (2)-(3) constants and eq. (4) applicability bounds are cited secondary via R-63 / KINEROS. | Constant/regime-bound fidelity rests on R-63's statement + D-val empirical confirmation rather than a digit-by-digit primary read. | promotable-with-risk; `INV-OFEROUTE-002/003` confirmed by D-val Case 1/2/3 Ef. | `[DIRECT][Static] via R-63` |
| GAP-OFEROUTE-003 | The runon re-infiltration reconciliation (`INV-OFEROUTE-009`) vs DC01's daily-lump admission (`INV-RUNOFFPART-031`). **Design resolved (D5/D6, 2026-07-02): SUPERSEDE (faithful Papanicolaou assumption 2).** When the routing subsystem is active, per-OFE Green-Ampt-Mein-Larsen infiltration acts on the **RAINFALL** rate to produce the rainfall-excess the cascade routes; the upstream OFE's outlet hydrograph is added as a downstream **surface boundary condition** (D5 handoff) and is **NOT re-infiltrated**. The routing therefore SUPERSEDES DC01's daily-lump runon re-infiltration with hydraulic surface routing — it does NOT compose a second re-infiltration of the routed runon (a D5-wording correction: the earlier 'compose with re-infiltration of the routed hydrograph' misread; infiltration composes on rainfall, not on runon). D6 landed `ofe_routing::infiltration` (Green-Ampt kernel) + `run_infiltrated_cascade` (rainfall -> per-OFE infiltration -> excess -> route), shadow-first. Remaining INTEGRATION scope: wire this into production and DISABLE DC01's daily-lump admission when the subsystem is active (production activation gate). | Rainfall-to-runoff routing mechanism landed (D6); runtime production wiring + DC01-disable is the activation gate. | design-resolved; production-wiring-integration-scope. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-02` | `1` | `Claude Code` | Initial authoring (MOFEFID Lane D / D-gate): friction menu (eqs. 2-7), KWE + TVD-MacCormack + CFL (eqs. 8-14, A1-A2), per-OFE hydrograph handoff, runon re-infiltration coupling, opt-in activation, and D-val acceptance. `INV-OFEROUTE-001` anchors the landed shadow-first friction kernels; solver/cascade invariants (005-009) gate D4/D5. Frozen-library citation posture recorded (`GAP-OFEROUTE-001/002`). Authored for ratification per ADR-0033 narrowed scope. |
| `2026-07-02` | `2` | `Codex` | Ratification review amendment: added missing kernel-process profile sections, guard/failure-posture invariant fields, symbol alias and unit-governance maps, producer/consumer obligations, constants, tolerance notes, and test-vector obligations; moved gaps after the Binding Exposure Index; ratified the contract as the D4 prerequisite while preserving `GAP-OFEROUTE-003` as a D5 `HOLD`. |
| `2026-07-02` | `3` | `Claude Code` | MOFEFID-D4 review disposition: clarified that the D4 single-OFE solver evidence surface is PHYSICS validation (conservation/CFL/steady-state/shock), NOT formal Ef-vs-observed; `INV-OFEROUTE-011` Ef acceptance is explicitly the D-val stage at D5/integration (needs SC-RUNOFFPART infiltration coupling for Cases 1-3 and digitized observed series for Case 4). Corrects the `OFEROUTE-KWE-TVD-SOLVER` BEI note that implied D4 did case-Ef validation. No invariant/physics change. |
| `2026-07-02` | `4` | `Claude Code` | MOFEFID-D6: landed `ofe_routing::infiltration` (Green-Ampt-Mein-Larsen per-OFE infiltration) + `run_infiltrated_cascade` (rainfall -> per-OFE excess -> routed cascade), shadow-first, completing the rainfall-to-runoff routing mechanism. CORRECTED the `GAP-OFEROUTE-003` resolution from the D5 'supersede-then-compose (re-infiltrate the routed hydrograph)' misread to faithful-Papanicolaou SUPERSEDE: infiltration acts on RAINFALL (assumption 2), runon is surface-routed (assumption 1) and NOT re-infiltrated; routing supersedes DC01's daily-lump runon re-infiltration. Production wiring + DC01-disable remains the activation gate. |
| `2026-07-02` | `4` | `Codex` | D4 merge re-check cleanup: made revision-history numbering unique and confirmed the Test-Vector Obligations row separates D4 physics evidence from D-val/D5 `Ef` acceptance. No invariant/physics change. |
| `2026-07-02` | `5` | `Claude Code` | MOFEFID-D5: landed the OFE-by-OFE cascade (`ofe_routing::cascade`, shadow-first) - width-aware outlet-hydrograph handoff (INV-OFEROUTE-008) validated on cascade conservation, handoff continuity, downslope accumulation, and the Case-3 vegetated-strip storage signature. Resolved `GAP-OFEROUTE-003` design as supersede-then-compose (routing owns hourly runon, supersedes DC01 daily-lump, composes with downstream hourly infiltration); the runtime infiltration composition + DC01-disable guard remains integration scope. |
