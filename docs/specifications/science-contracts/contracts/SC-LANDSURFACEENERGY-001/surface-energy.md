[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | every task | universal scope, owners, failure and qualification | whole mechanism chapter |
| terminal-support.md#support | physical solve or receipt interval admission, including snow-free work | physical support, zero support and pre-Newton floor | section (entry extent) |
| soil-coupling.md#soil-coupling | surface temperature/humidity or ground transfer | thermal state and CN lower boundary | whole mechanism chapter |
| nonlinear-solve.md#solve | physical rule selection or accepted-primitive closure | ordered solve and error precedence; INV108-110 | section (entry extent) |
| nonlinear-solve.md#nonlinear-solve | solver implementation or full evaluator correctness | all active numerical branches | whole mechanism chapter |
| water-vapor.md#vapor | represented-snow evaluator/error-order requirements review | signed enthalpy, immutable water/ingress and canonical errors | section (entry extent) |
| water-vapor.md#water | represented-snow evaluator/error-order requirements review | signed enthalpy, immutable water/ingress and canonical errors | section (entry extent) |
| water-vapor.md#errors | represented-snow evaluator/error-order requirements review | signed enthalpy, immutable water/ingress and canonical errors | section (entry extent) |
| water-vapor.md#water-vapor | snow-free physical rules, active water/ingress, energy closure or water-owner audit | complete accepted enthalpy and immutable-water duties | whole mechanism chapter |
| ../SC-VEGETATION-001.md#openwepp_c3_woody_v8-coupled-ground-energy-amendment | covered-canopy physics or closure | V8 canopy owner | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |
| ../SC-VEGETATION-001.md#purpose | cross-contract scope | vegetation entry/scope constraints | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |

<a id="surface-energy"></a>
# Surface Energy
Current V1 snow-free radiation and neutral turbulent physics. V2 imports this physics; V3 imports it for its admitted litter successor. The represented-snow lower boundary has its separate terminal/soil interface authority. No current-temperature, recipient or domain qualifier below is optional.

<a id="openwepp_snow_free_lse_v1-constitutive-authority"></a>
## `OPENWEPP_SNOW_FREE_LSE_V1` Constitutive Authority

Version 3 prospectively supersedes the version-2 missing-authority posture only
for this named model and supported domain. Historical conservation and custody
requirements remain binding.

<a id="selected-sources-and-domain"></a>
### Selected sources and domain

The selected reference-model stack is CLM5 equations 2.5.72--2.5.81,
2.5.86--2.5.153 and 2.6.1--2.6.91; Boone et al. (2017) ISBA-MEB Part 1
equations 4--28 and Appendix I; Napoly et al. (2017) Appendix A equations
A1--A14; FSM2.1.1 equations 25--34; and the ORCHIDEE arbitrary-level longwave
matrix S2.16--S2.24. The exact retained bytes and rights are recorded as
R-153 and R-155--R-158 in `references/annotated_bibliography.md`.

The executable domain requires snow absent at both endpoints, no terminal snow
payload, positive finite neutral-domain wind, positive finite interval and
area, liquid/unfrozen water and soil, complete forcing/configuration/state, one
ground class per tile, and exact owner identity. `bare_mineral_soil` and
`forest_litter` are admitted. Calm wind, nonneutral stability, snow, terminal
snow, frozen or thawing material, multiple ground classes per tile and missing
liquid-temperature lineage return typed unsupported errors before calculation.

<a id="exact-ownership-and-state"></a>
### Exact ownership and state

Hydrology exclusively owns ponded, litter-held and soil-layer water mass.
More completely, it owns every water mass: ponded/depression storage,
litter-held water, and every soil-layer liquid/frozen store. LSE owns one
surface thermal node per tile and no water amount. The surface dry body and any
positive hydrology-owned surface store are isothermal at that node. The
soil-thermal owner, not LSE, owns all `N` soil temperatures and enthalpies.
Vegetation retains all canopy processes. No model state contains two mutable
representations of the same mass, temperature, or enthalpy.

Strict configuration, state, forcing, water-protocol, diagnostics, and coupled
owner-envelope machine surfaces are the six package artifacts named
`lse_v1_*_schema.json`; every frozen digest is part of the model definition.
Configuration supplies OFE/tile
topology, one surface class per
tile, VIS/NIR albedo, neutral aerodynamic geometry, surface dry heat capacity,
surface thickness/conductivity, soil-layer geometry and thermal properties,
and litter properties where applicable. LSE persistent state supplies one
physical `surface_enthalpy_j_m2_tile` per tile plus numerical warm starts,
transaction lineage, and digests. `surface_temperature_k` is retained only as
a solver warm start/diagnostic and is reconstructed from enthalpy, immutable
hydrology-owned `W`, and `C_dry` before use; it is not a second physical state.
Hydrology supplies all beginning water amounts, and soil thermal supplies its
`N` temperatures/enthalpies, through immutable owner snapshots with exact
digest lineage. No executable default supplies a scientific value. Unknown,
missing, extra, duplicate, stale, nonfinite, wrong-owner, or out-of-domain
fields reject.

For one OFE, tile fractions are finite, positive, unique by tile ID, and sum
to one under the topology tolerance. `stand-ground` in this contract means
that one OFE's horizontal ground area; it never means an entire routed
hillslope. Tile-local amounts become OFE-ground amounts by multiplication by
`f_t` exactly once. Cross-OFE runon retains source and destination OFE IDs.

<a id="shortwave-and-reciprocal-longwave"></a>
### Shortwave and reciprocal longwave

V8 performs the unchanged V7 two-stream shortwave solve using ground
surface-class VIS/NIR albedos as the full-column lower boundary. Ground receives
terminal direct/diffuse VIS/NIR exactly once; reflected radiation traverses
the overlying column and is never sent directly to the atmosphere.

V1 selects unit longwave emissivity and no longwave reflection. For top-to-
bottom occupancies `i=0..n-1`, `P_i=LAI_i+SAI_i` on tile-ground basis and

```text
tau_i = exp[-0.8*Omega_i*P_i].
```

The coefficient `0.8` has units `m2 plant area m-2 ground`; clumping
`0<Omega_i<=1` is applied exactly once. With atmospheric boundary `Ldn_0` and
current trial surface temperature `T_s`,

```text
Ldn_(i+1) = tau_i*Ldn_i + (1-tau_i)*E_i
Lup_n     = sigma*T_s^4
Lup_i     = tau_i*Lup_(i+1) + (1-tau_i)*E_i.
```

For component `j` in the ordered set `{sun_leaf, shade_leaf, wet_surface,
dry_stem}`, let `a_j>=0` be its exact tile-ground emissive area and
`w_j=a_j/sum(a_j)`. If `sum(a_j)=0`, `tau_i=1` and every component longwave
term is exact zero. Otherwise the component net is

```text
R_lw,i,j = w_j*(1-tau_i)*(Ldn_i+Lup_(i+1))
             - 2*w_j*(1-tau_i)*sigma*T_i,j^4.
```

Thus `sum_j R_lw,i,j` equals the layer net and each physical temperature enters
its own emission. The ground term is `R_lw,s=Ldn_n-sigma*T_s^4`. All terms are
recomputed from current nonlinear trial temperatures. A bulk canopy
temperature, prescribed upward ground longwave, stale previous-step ground
temperature, or direct ground-to-atmosphere bypass is noncanonical.

<a id="neutral-turbulent-heat-and-vapor-network"></a>
### Neutral turbulent heat and vapor network

Flux signs in this section are positive away from the surface/component and
toward its air recipient. Let `rho_a` and `c_p` be the V8 forcing-derived moist
air density and heat capacity and let `q_s` be the admitted surface humidity.
For any surface-to-air resistance `r_h,r_v>0`,

```text
H_s = rho_a*c_p*(T_s-T_recipient)/r_h
v_s = rho_a*(q_s-q_recipient)/r_v.
```

The signed vapor flux `v_s>0` is evaporation and `v_s<0` is condensation. It
is never clipped to zero.

An open tile has `d=0` and uses the exact neutral log-law

```text
r_h = ln(z_ref/z0m)*ln(z_ref/z0h)/(kappa^2*u_ref)
r_v = ln(z_ref/z0m)*ln(z_ref/z0q)/(kappa^2*u_ref),
```

where `kappa=0.4`, `u_ref>0`, and
`z_ref>max(z0m,z0h,z0q)>0`. No wind floor, stability correction, convective
velocity, or alternate roughness is admitted.

A covered tile owns one zero-storage canopy-air node `(T_c,q_c)`. Every V8
component uses its unchanged component boundary/stomatal conductance to that
shared node. Ground/litter exchange uses ISBA-MEB equations 54--63 specialized
exactly to `psi_H=1` and `f_hv=1`:

```text
Re       = u_l*l_w/nu
c_d      = 1.328*(2/sqrt(Re)) + 0.45*((1-chi_L)/pi)^1.6
d        = 1.1*z_hv*ln[1+(c_d*LAI)^0.25]
u_hv     = u_ref
u_star   = kappa*u_hv/ln[(z_hv-d)/z0v]
K_hv     = kappa*u_star*(z_hv-d)
r_gn     = z_hv/(phi_v*K_hv)
           * {exp[phi_v*(1-z0g/z_hv)]
              - exp[phi_v*(1-(d+z0v)/z_hv)]}
r_g-c    = r_gn/psi_H = r_gn
z_r      = z_ref-d
phi_z    = (z_hv-d)/z_r <= 1.
```

The frozen constants are `phi_v=2`, `z0g=0.007 m`, `chi_L=0.12`,
`u_l=1 m s-1`, `l_w=0.02 m`, and `nu=1.5e-5 m2 s-1`. Configuration supplies
`z_hv,z0v,z_ref,LAI`; it must satisfy `LAI>0`, `z_hv>d+z0v>z0g>0`,
`z_ref-d>=z_hv-d>0`, and every logarithm/exponential/resistance must be finite
and positive. Heat and vapor use this same neutral ground-to-canopy path as
distinct semantic operands. The canopy-to-reference paths use the open neutral
log-law above with configured canopy `d,z0m,z0h,z0q` and
`z_ref>d+max(z0m,z0h,z0q)`.

The exact shared zero-storage residuals are

```text
R_Tc = sum_j H_j + H_s - H_c->atm
R_qc = sum_j v_j + v_s - v_c->atm.
```

Ground terms occur exactly once per tile, not once per occupancy. Reference-air
ground transfer beneath a canopy, a ground term omitted from the canopy-air
node, or a producer-aggregated canopy flux is invalid.

<a id="scope"></a>
### Scope

This version authorizes contract-derived fixtures and later default-off shadow
implementation. It authorizes no production selector/default/output, snow
handoff, calibration, empirical validation or transferability claim, or
cutover.


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-101"></a> `INV-LANDSURFACEENERGY-101` | Component longwave uses current component temperature and `tau=exp[-0.8*Omega*(LAI+SAI)]`; no bulk-canopy or stale-ground operand is accepted. | v31:L1090-L1090 | [INFERENCE][Static] | [Ordered domain/closure guards](nonlinear-solve.md#solve) and [typed errors](water-vapor.md#errors) | Typed domain/convergence/closure: [errors](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-102"></a> `INV-LANDSURFACEENERGY-102` | Neutral open and covered turbulent paths use only the exact equations/constants/domains above; no wind floor or stability substitute exists. | v31:L1091-L1091 | [INFERENCE][Static] | [Ordered domain/closure guards](nonlinear-solve.md#solve) and [typed errors](water-vapor.md#errors) | Typed domain/convergence/closure: [errors](water-vapor.md#errors); no partial state or promotion |
