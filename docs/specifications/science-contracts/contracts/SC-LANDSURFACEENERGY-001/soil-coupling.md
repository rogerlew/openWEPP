[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| terminal-support.md#support | physical solve or receipt interval admission, including snow-free work | physical support, zero support and pre-Newton floor | section |
| water-vapor.md#water-vapor | liquid/energy closure or infiltration | accepted ingress enthalpy | whole chapter |
| soil-custody.md#soil-custody | exact soil energy storage or reconstruction | receiver-owned high/carry | whole chapter |
| surface-custody.md#surface-custody | exact surface storage or reconstruction | surface high/carry owner | whole chapter |
| terminal-support.md#terminal-support | represented-snow boundary or receiver transition | regime and support receipts | whole chapter |
| ../SC-SNOWENERGY-001.md#child-2c-shared-snow--canopy-turbulent-carrier-amendment | represented-snow boundary | snow carrier/soil receipt owner | whole external contract; frozen protocol scope |
| ../SC-SNOWENERGY-001.md#purpose | crossing represented-snow authority | snow scope and qualification | whole external contract; frozen protocol scope |

<a id="soil-coupling"></a>
# Soil Coupling
Current surface humidity/thermal-state and soil transfer rules, plus the separate persistent represented-snow bottom-volume/first-OFE-node boundary. Select the actual regime; tile and OFE/lane bases never alias. Exact storage representation is owned by soil-custody and surface-custody.

## Endpoint reseal and physical closure

At the represented-snow soil boundary, TOL-SNOWENERGY-005 retains the exact
equal/opposite heat ALREADY consumed by snow and soil. Reconstructed installed
endpoints must agree within 1e-9 J m^-2 and 1e-8 K before the consumed receipt
is resealed to those exact installed candidate identities. Resealing changes
neither soil enthalpy nor applied credit. The independent physical-ledger closure
threshold remains 1e-6 J m^-2; endpoint consistency is not a substitute for it.
Reconstruct both from primitive operands. Larger/nonfinite endpoint residuals
retain the canonical retry/iteration-limit and fail-closed response, never heat
repair. The complete original boundary and validation rules below remain required.

<a id="surface-humidity-surface-enthalpy-litter-and-soil-heat"></a>
### Surface humidity, surface enthalpy, litter, and soil heat

One surface node is used for both admitted classes. With hydrology-owned
surface water `W>=0`, configured dry areal heat capacity
`C_dry>=0 J m^-2 K^-1`, and `T_ref=273.15 K`, its authoritative enthalpy is

```text
U_s = (C_dry + W*C_w)*(T_s-T_ref).
```

`W` is an immutable hydrology operand during a solve, never an LSE state field.
The configured `finite_capacity` branch requires `C_dry+W*C_w>0`; its accepted
ending enthalpy is the sole physical LSE state. Temperature is derived using
the hydrology candidate's ending `W`; a retained temperature warm start must be
bit-identical to that derived candidate value and cannot be independently
adjusted. The
configured `equilibrium_zero` branch requires `C_dry=0`, `W=0`, and `U_s=0`
exactly and replaces the storage difference by exact zero while retaining
`T_s` as the algebraic surface-energy unknown. No other zero-capacity branch is
admitted.

For bare mineral soil without positive surface water, V1 transcribes CLM5
equations 5.72--5.81. With top-layer hydrology operands
`W_liq,1,W_ice,1,dz_1,theta_sat,psi_sat,B,Phi`:

```text
s_1       = min(1,max(0.01,(W_liq,1/rho_w + W_ice,1/rho_i)
                          /(dz_1*theta_sat)))
theta_1   = W_liq,1/(rho_w*dz_1)
psi_1     = max(-1e8, psi_sat*s_1^(-B))                 [mm]
alpha     = exp(psi_1*g/(1000*R_wv*T_s))
theta_air = Phi*(psi_sat/psi_air)^(1/B)
DSL       = D_max*(theta_init-theta_1)/(theta_init-theta_air)
            when theta_1<theta_init, otherwise 0
Phi_air   = Phi-theta_air
tau_pore  = Phi_air^2*(Phi_air/Phi)^(3/B)
D_v       = 2.12e-5*(T_s/T_ref)^1.75
r_soil    = DSL/(D_v*tau_pore)
q_soil    = alpha*q_sat(T_s,p).
```

The constants are `D_max=0.015 m`, `psi_air=-1e7 mm`,
`R_wv=461.5 J kg^-1 K^-1`,
`rho_w=1000 kg m^-3`, `rho_i=917 kg m^-3`, and `g=9.80665 m s^-2`.
If `q_sat(T_s)>q_recipient>q_soil`, V1 selects the CLM branch
`q_soil=q_recipient`, yielding exact zero vapor flux. Otherwise
`v_s=rho_a*(q_soil-q_recipient)/(r_v+r_soil)`. All denominators and powers
must be defined; this bounded CLM branch is the only admitted humidity
normalization. Positive surface water instead selects
`q_s=q_sat(T_s,p)`, `r_soil=0`, and its request source is that exact store.
Snow-free frozen/thawing states reject before these equations.

Forest litter is the Napoly/ISBA single layer. With hydrology-owned liquid
`W_l`, configured capacity `W_l,max`, thickness `dz_l`, dry density `rho_ld`,
and dry specific heat `c_ld`,

```text
h_ul     = 0.5*(1-cos(pi*W_l/W_l,max))
q_l      = h_ul*q_sat(T_s,p) + (1-h_ul)*q_recipient
v_l      = rho_a*(q_l-q_recipient)/r_l-c
lambda_l = 0.1 + 0.03*W_l/(rho_w*dz_l)
C_dry    = dz_l*rho_ld*c_ld.
```

`W_l,max>0` and `0<=W_l<=W_l,max`. Litter blocks direct mineral-soil
evaporation and upward capillary supply during this interval. `r_l-c` is the
covered neutral `r_g-c` or open neutral `r_v` as topology requires. Overflow
remains hydrology-owned ingress.

The soil-thermal owner has exact ordered nodes `k=1..N`, each with
`dz_k>0`, `lambda_k>0`, and areal heat capacity `C_k>0`. The surface has
`dz_s>0,lambda_s>0`; for litter these are `dz_l,lambda_l`, and bare soil has
its own configured mineral skin. Define

```text
g_s1     = 2/(dz_s/lambda_s + dz_1/lambda_1)
g_k,k+1  = 2/(dz_k/lambda_k + dz_(k+1)/lambda_(k+1))
G_s1     = g_s1*(T_s-T_1)
G_k,k+1  = g_k,k+1*(T_k-T_(k+1)).
```

All `G` here are positive downward. The surface residual contains `-G_s1`;
the soil-thermal candidate contains `+G_s1` exactly once. For Crank--Nicolson,
`bar(G)=0.5*(G_begin+G_end)` and

```text
C_1*(T_1,1-T_1,0)/dt = bar(G_s1)-bar(G_1,2)
C_k*(T_k,1-T_k,0)/dt = bar(G_k-1,k)-bar(G_k,k+1)
C_N*(T_N,1-T_N,0)/dt = bar(G_N-1,N),
```

with exact zero lower flux. `N>=1`; for `N=1` this reduces to
`C_1*(T_1,1-T_1,0)/dt=bar(G_s1)`. Phase change, frozen or thawing soil, an
LSE-owned duplicate soil temperature, and a second independently calculated
surface/soil flux are unsupported.

For `finite_capacity`, `G_s1,begin` uses the beginning surface temperature
derived from authoritative beginning `U_s,W,C_dry`, and `G_s1,end` uses the
trial ending surface temperature. For `equilibrium_zero`, no physical
beginning surface temperature exists: the current algebraic trial `T_s` is the
surface-side operand at both Crank--Nicolson endpoints,
`G_s1,begin=g_s1*(T_s-T_1,0)` and
`G_s1,end=g_s1*(T_s-T_1,1)`. The caller temperature warm start is numerical
only and must never enter `G_s1,begin` as a physical state operand.

<a id="version-8-persistent-snow--soil-boundary-amendment"></a>
## Version 8 persistent snow--soil boundary amendment

This amendment admits one lower boundary for a persistent Stage 3 column. The
boundary is OFE/lane-level because the snow owner and the existing
soil-thermal snapshot are each OFE-ground owners. It couples the bottom
represented Stage 3 thermal volume directly to ordered soil node `k=1`. Tile
surface temperatures and tile LSE thermal nodes do not participate. Selecting
one tile, averaging any tile subset, weighting the OFE soil node by tile
fractions, or applying one lane flux separately to every tile is invalid.

Pinned `frostn.for` lines 476--607 and `tmpadj.for` lines 266--353 establish
additive layer resistance and harmonic snow/soil conduction. They do not
define the current node-owner interface and their silent zero-flux fallbacks,
calibrated conductivity factors, frost-front paths, and prescribed surface
temperature are not imported. The current LSE node-centered authority supplies
the exact interface specialization. Let the bottom snow volume and top OFE
soil node have positive finite `(dz_sb,lambda_sb)` and `(dz_1,lambda_1)`:

```text
R_ss       = dz_sb/(2*lambda_sb) + dz_1/(2*lambda_1)
g_ss       = 1/R_ss = 2/(dz_sb/lambda_sb + dz_1/lambda_1)
G_ss,0     = g_ss*(T_sb,0-T_1,0)
G_ss,1     = g_ss*(T_sb,1-T_1,1)
bar(G_ss)  = 0.5*(G_ss,0+G_ss,1).
```

`G_ss` is positive downward from snow to soil and has units `W m^-2
OFE-ground`. The beginning endpoint uses only the sealed beginning Stage 3 and
soil-thermal owners. The ending endpoint uses the current candidate bottom
snow and first-soil-node temperatures. Consequently `bar(G_ss)` participates
inside the existing covered fixed point and cannot be calculated after it or
held fixed from a stale trial. The Stage 3 energy candidate contains
`-bar(G_ss)` exactly once; the first soil-node Crank--Nicolson equation and
candidate enthalpy contain `+bar(G_ss)` exactly once. Deeper node conduction
and the zero lower boundary retain the existing LSE equations unchanged.

One sealed `SnowSoilHeatReceiptV1` binds schema/model identity, exact half-open
support and duration, lane/OFE and OFE-ground basis, ordered topology and
configuration digests, beginning Stage 3 and soil-thermal owner IDs/digests,
bottom-snow and first-soil layer IDs, all four positive resistance operands,
both endpoint temperature pairs, both endpoint fluxes, accepted
`bar(G_ss)`, and both candidate-ending owner IDs/digests. Its digest uses the
repository canonical framed encoding and is reconstructed from those semantic
fields; receipt-hash order is not operand order.

The final nonlinear termination rule is `SC-SNOWENERGY-001@25` /
`TOL-SNOWENERGY-005`: retain exactly the equal/opposite heat that both solvers
consumed, require the receipt reconstructed from installed endpoints to differ
by no more than `1e-9 J m^-2` and `1e-8 K`, then reseal the consumed receipt to
the exact installed snow and soil candidate identities. A larger or nonfinite
residual retries within the fixed-point cap and fails closed on exhaustion.
This rule does not change soil enthalpy, applied credit, or the existing
`1e-6 J m^-2` physical-ledger closure threshold.

Independent validation recomputes the resistance, endpoint fluxes,
Crank--Nicolson flux, Stage 3 debit, first-soil-node credit/storage equation,
receipt digest, and beginning/candidate owner joins from primitive operands.
Producer residuals are diagnostic only. Any missing, duplicate, stale,
nonfinite, nonpositive-resistance, wrong-node, wrong-sign, wrong-basis,
substituted-receipt, convergence, reconstruction, or later transaction failure
returns `LSEB-E-044` / `SNOWENERGY-E-SOIL-HEAT-001` and leaves Stage 3,
soil-thermal, transaction, and receipt owners byte-identical.

| ID | Binding rule | Guard/failure |
|---|---|---|


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-100"></a> `INV-LANDSURFACEENERGY-100` | Hydrology owns every water mass; LSE owns one surface thermal node per tile; soil thermal owns all `N` soil temperatures and enthalpies. | v31:L1089-L1089 | [INFERENCE][Static] | [Solve guards](solve-boundary.md#solve) and [typed errors](water-vapor.md#errors) | [Typed failures](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-103"></a> `INV-LANDSURFACEENERGY-103` | Surface enthalpy is `(C_dry+W*C_w)*(T_s-T_ref)` using hydrology's exact mass, while every soil enthalpy remains independently owned. | v31:L1092-L1092 | [INFERENCE][Static] | [Solve guards](solve-boundary.md#solve) and [typed errors](water-vapor.md#errors) | [Typed failures](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-106"></a> `INV-LANDSURFACEENERGY-106` | Surface `-G_s1` and soil `+G_s1`, infiltration energy, and routed runoff enthalpy join once by exact OFE/tile/interval identity. | v31:L1095-L1095 | [INFERENCE][Static] | [Solve guards](solve-boundary.md#solve) and [typed errors](water-vapor.md#errors) | [Typed failures](water-vapor.md#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-124"></a> `INV-LANDSURFACEENERGY-124` | The persistent snow boundary joins the bottom Stage 3 volume to first ordered OFE soil node only; no tile selection, aggregation, weighting, or duplication is allowed. | v31:L1257-L1257 | [INFERENCE][Static] | topology/node/area guard / `LSEB-E-044` | topology/node/area guard / `LSEB-E-044` |
| <a id="INV-LANDSURFACEENERGY-125"></a> `INV-LANDSURFACEENERGY-125` | Half-snow plus half-soil series resistance and beginning/ending Crank--Nicolson evaluation produce one positive-downward `bar(G_ss)` inside the covered fixed point. | v31:L1258-L1258 | [INFERENCE][Static] | operand/endpoint/convergence guard / `LSEB-E-044` | operand/endpoint/convergence guard / `LSEB-E-044` |
| <a id="INV-LANDSURFACEENERGY-126"></a> `INV-LANDSURFACEENERGY-126` | Exact `-bar(G_ss)` snow custody and `+bar(G_ss)` first-soil-node custody share one reconstructable receipt and commit or roll back together. | v31:L1259-L1259 | [INFERENCE][Static] | independent receipt/owner transaction guard / `LSEB-E-044` | independent receipt/owner transaction guard / `LSEB-E-044` |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-003"></a> `OBL-LANDSURFACEENERGY-C-003` | soil/frost consumes exactly `-G` once and is sole subsurface conduction/phase-state mutator. | Soil/frost consumers of the surface ground-heat transfer | v31:L330-L331 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](common-details.md#tests); named fixtures/tests/real consumers |


## Isolated bulk trace boundary (2026-09-10)

Only `EXP-SNOW-ACCURACY-20260910-B` replaces the old iterative snow boundary
and snow/top-soil Crank--Nicolson constitutive comparison with its explicitly
lagged atmospheric boundary and piecewise implicit bulk exchange. Its canonical
method is specified in SC-SNOWENERGY-001. The same q is debited/credited once;
actual soil candidate ownership, exact high/carry conservation, deeper transfers,
OFE-ground basis and rollback remain binding. The reduced law's predicted top
soil temperature is distinct from the complete soil candidate's ending state.
New method-specific receipts must bind both, without asserting R0 endpoint
constitutive agreement. Existing R0/P receipts and physical-ledger tolerances
are unchanged. This is isolated experimental authority, not production activation.
