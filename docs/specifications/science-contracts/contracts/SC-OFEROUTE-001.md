---
contract_id: SC-OFEROUTE-001
title: Hillslope OFE-by-OFE Overland-Flow Routing Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 1
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

Status: `in_review`
Maturity: `draft`
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

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-OFEROUTE-001 | Friction-menu domain and additivity: `f_s, f_f, f_w, f_veg >= 0` and `f_eq = f_s + f_f + f_w + f_veg` (eq. (7)); `lambda in [0,1]`; degenerate operands (`Re<=0`, `h<=0`, `D_r<=0`) yield `0`, never NaN. Anchors the shadow-first `ofe_routing::friction` kernels. | hard-fail | REF-OFEROUTE-FRAMEWORK, REF-OFEROUTE-PHYS-BOUNDS | `[DIRECT][Static]` |
| INV-OFEROUTE-002 | Skin regime dispatch: `Re <= ~1000` uses Shen & Li `f_s = (3393 I^0.407 + k_o)/Re` (eq. 2), `Re > ~1000` uses Hirsch `f_s = 3.19/Re^0.45` (eq. 3); rainfall intensity `I` in `m s^-1`, `Re = q/nu`. The exact crossover and `nu`/`I` unit convention are confirmed empirically by D-val Case 1/2 reproduction (frozen-library posture — primaries cited secondary via R-63). | hard-fail | REF-OFEROUTE-SKIN-SHENLI, REF-OFEROUTE-SKIN-HIRSCH, REF-OFEROUTE-KO-WOOLHISER | `[DIRECT][Static] via R-63` |
| INV-OFEROUTE-003 | Form + wave submergence gating: `f_f = (16/pi) C_d (h/D_r) lambda` (eq. 4); when `h/D_r < 1` wave resistance applies with `f_w = 3.32 lambda / Fr^0.5` for `Fr > 0.5`, ramping linearly `0 -> f_w(Fr=0.5)` for `Fr <= 0.5` (eq. 5). Applicability bounds per Lawrence 1997 / the Abrahams 1998 discussion. | hard-fail | REF-OFEROUTE-FORM-LAWRENCE, REF-OFEROUTE-WAVE-HUABRAHAMS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-OFEROUTE-004 | Vegetation resistance: `beta = min(0.135 sqrt(LAI/h_c), 0.33)` (cap enforced), `L_c = (C_d LAI/h_c)^-1`, `f_veg` per eq. (6); zero for unsubmerged/absent canopy. | hard-fail | REF-OFEROUTE-VEG-KATUL | `[DIRECT][Static]` |
| INV-OFEROUTE-005 | KWE fidelity: continuity `dh/dt + dq/dx = v` with `q = alpha h^1.5`, `alpha = C S_o^0.5`, `v` the rainfall-excess rate; no negative depth/discharge published. | hard-fail | REF-OFEROUTE-KWE, REF-OFEROUTE-PHYS-BOUNDS | `[DIRECT][Static]` |
| INV-OFEROUTE-006 | Scheme conservation: the TVD-MacCormack step must conserve mass to numerical noise over each OFE (inflow + rainfall-excess - outflow = storage change within a named tolerance); the dissipative term must not create/destroy mass. Per-increment conservation is a hard stop during staged implementation. | hard-fail | REF-OFEROUTE-TVD-MACCORMACK, REF-OFEROUTE-FRAMEWORK | `[DIRECT][Static] via R-63` |
| INV-OFEROUTE-007 | CFL stability: `Cr = (Delta t/Delta x) c <= 1` at every cell, `c = 1.5 C S_o^0.5 h^0.5`; `Delta t` is selected to satisfy CFL, and a CFL violation is a hard-fail rather than a silently-unstable run. | hard-fail | REF-OFEROUTE-TVD-MACCORMACK | `[DIRECT][Static] via R-63` |
| INV-OFEROUTE-008 | Per-OFE hydrograph handoff: the upstream OFE outlet hydrograph is the downstream OFE upstream boundary condition, carried as an hourly profile over the `SC-RUNOFFPART-001#INV-RUNOFFPART-029` transfer seam. Aggregating the routed hydrograph to a daily transfer, or synthesizing the downstream boundary from a daily `Q`, is invalid when the subsystem is active. | hard-fail | REF-OFEROUTE-FRAMEWORK, SC-RUNOFFPART-001#INV-RUNOFFPART-029 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-OFEROUTE-009 | Runon re-infiltration coupling: routed inter-OFE excess re-infiltrates on the downstream OFE per `SC-RUNOFFPART-001#INV-RUNOFFPART-031` (DC01), hourly-faithful. When this subsystem is active it OWNS the hourly runon supply and must not double-count DC01's daily-lump runon admission; the reconciliation design is a named prerequisite (`GAP-OFEROUTE-003`) for D5. | hard-fail | SC-RUNOFFPART-001#INV-RUNOFFPART-031, REF-OFEROUTE-FRAMEWORK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-OFEROUTE-010 | Opt-in activation: with the subsystem OFF (default), all protected hillslope outputs are byte-identical to the pre-subsystem runtime; the subsystem produces no phase-span side effect on the default path. Default activation is a separate later gate (not authorized by this contract or by ADR-0033). | hard-fail | REF-OFEROUTE-FRAMEWORK, ADR-0033 | `[DIRECT][Static]` |
| INV-OFEROUTE-011 | Validation acceptance (D-val): the four Papanicolaou validation cases reproduce the published enhanced-WEPP Nash-Sutcliffe efficiencies within a named tolerance (`Ef` 0.91 bare / 0.75 roughness elements / 0.87 vegetation / 0.88 curvature-shock), and the Zone 1/Zone 2 stream-power taxonomy is reproduced qualitatively (thresholds within stated bounds of the paper's `k`/`l` fits). Case inputs and provenance: `docs/work-packages/20260702-mofefid-d01-ofe-routing-scaffold-001/artifacts/`. | hard-fail | REF-OFEROUTE-FRAMEWORK, REF-OFEROUTE-SHOCK-IWAGAKI | `[DIRECT][Static]` |

## Invariant Guard Map

| Invariant | Guard class | Enforcement path | Failure behavior | Acceptance | Evidence |
|---|---|---|---|---|---|
| `INV-OFEROUTE-001` | unit (pure kernel) | `ofe_routing::friction` non-negativity/additivity + degenerate-input tests | typed test failure | D3 kernel tests (landed) | `[DIRECT][Static]` |
| `INV-OFEROUTE-002..004` | unit (pure kernel) | friction regime/gating/cap tests + D-val Case 1/2/3 operand reproduction | typed test / D-val Ef | D-val fixtures | `[DIRECT][Static]` |
| `INV-OFEROUTE-005..007` | runtime (solver) | KWE/TVD/CFL solver per-increment conservation + CFL guard | hard error / per-increment stop | D4 solver stages | `[DIRECT][Static] via R-63` |
| `INV-OFEROUTE-008..009` | runtime (cascade) | per-OFE hydrograph handoff over the transfer seam; DC01 double-count reconciliation | hard error / `HOLD` | D5 cascade; `GAP-OFEROUTE-003` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-OFEROUTE-010` | runtime (activation) | default-path byte-identity gate with subsystem off | protected-output diff | every stage | `[DIRECT][Static]` |
| `INV-OFEROUTE-011` | validation | D-val Ef + Zone taxonomy vs the four cases | Ef out of tolerance -> investigate | D-val | `[DIRECT][Static]` |

## Gaps

| Gap ID | Statement | Impact | Disposition | Evidence |
|---|---|---|---|---|
| GAP-OFEROUTE-001 | TVD-MacCormack numerics primaries (Garcia-Navarro 1992, Mingham 2001) are un-acquired under the frozen reference library; the scheme (eqs. 8-14) is cited secondary via R-63. | Scheme constants/limiter form taken from R-63; re-derivable from open kinematic-wave/shallow-water sources already in-repo (USGS PP1302, KINEROS). | promotable-with-risk; unit/limiter fidelity confirmed by D-val Case 4 (Iwagaki shock). | `[DIRECT][Static] via R-63` |
| GAP-OFEROUTE-002 | Formulation primaries Shen & Li (1973), the Abrahams (1998) discussion of Lawrence, and Woolhiser (1975) k_o tables are un-acquired (frozen library); eqs. (2)-(3) constants and eq. (4) applicability bounds are cited secondary via R-63 / KINEROS. | Constant/regime-bound fidelity rests on R-63's statement + D-val empirical confirmation rather than a digit-by-digit primary read. | promotable-with-risk; `INV-OFEROUTE-002/003` confirmed by D-val Case 1/2/3 Ef. | `[DIRECT][Static] via R-63` |
| GAP-OFEROUTE-003 | The runon re-infiltration reconciliation (`INV-OFEROUTE-009`): when this subsystem is active it must own the hourly runon supply without double-counting DC01's daily-lump admission (`INV-RUNOFFPART-031`). The concrete design (supersede vs. compose) is unspecified. | Blocks D5 cascade activation alongside DC01. | non-promotable until D5 design; named prerequisite. | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| `OFEROUTE-KWE-TVD-SOLVER` | `SC-OFEROUTE-001.md#algorithm-specification` | `active` | `unpromoted-binding` | `INV-OFEROUTE-005, INV-OFEROUTE-006, INV-OFEROUTE-007` | `science-review-follow-on` | Prospective (D4): single-OFE KWE + TVD-MacCormack + CFL. No runtime binding until D4; validated on Cases 1/2/4 (Iwagaki shock). |
| `OFEROUTE-CASCADE-HANDOFF` | `SC-OFEROUTE-001.md#algorithm-specification` | `active` | `unpromoted-binding` | `INV-OFEROUTE-008, INV-OFEROUTE-009` | `science-review-follow-on` | Prospective (D5): per-OFE hydrograph handoff over the `INV-RUNOFFPART-029` seam + DC01 runon reconciliation (`GAP-OFEROUTE-003`). No runtime binding until D5. |
| `OFEROUTE-ACTIVATION-VALIDATION` | `SC-OFEROUTE-001.md#invariants` | `active` | `maps-to-existing-INV` | `INV-OFEROUTE-010, INV-OFEROUTE-011` | `none` | Opt-in default-byte-flat gate + D-val Ef/Zone acceptance; the byte-flat gate is enforceable now (subsystem-off), D-val at each stage. |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-02` | `1` | `Claude Code` | Initial authoring (MOFEFID Lane D / D-gate): friction menu (eqs. 2-7), KWE + TVD-MacCormack + CFL (eqs. 8-14, A1-A2), per-OFE hydrograph handoff, runon re-infiltration coupling, opt-in activation, and D-val acceptance. `INV-OFEROUTE-001` anchors the landed shadow-first friction kernels; solver/cascade invariants (005-009) gate D4/D5. Frozen-library citation posture recorded (`GAP-OFEROUTE-001/002`). Authored for ratification per ADR-0033 narrowed scope. |
