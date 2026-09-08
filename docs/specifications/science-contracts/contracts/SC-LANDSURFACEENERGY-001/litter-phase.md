[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| terminal-support.md#support | physical solve or receipt interval admission, including snow-free work | physical support, zero support and pre-Newton floor | section |
| [water-vapor](water-vapor.md#water-vapor), [soil-coupling](soil-coupling.md#soil-coupling), [surface-custody](surface-custody.md#surface-custody) | active litter phase, ingress or closure | physical and exact operand custody | whole chapter |
| ../SC-SURFACELIQUID-001.md#frozen-forest-litter-surface-owner-v2-amendment | litter state or closure | exclusive liquid/ice owner | whole external contract; frozen protocol scope |
| ../SC-SURFACELIQUID-001.md#exact-v3-litter-phase-capacity-spill-custody-amendment | phase spill custody or closure | once-only spill | whole external contract; frozen protocol scope |
| ../SC-SURFACELIQUID-001.md#exact-heterogeneous-v3-finalized-use-join-amendment | heterogeneous finalized resources | exact native/ordinary join | whole external contract; frozen protocol scope |
| ../SC-SURFACELIQUID-001.md#purpose-and-scientific-scope | cross-contract surface scope | surface scope | whole external contract; frozen protocol scope |

<a id="litter-phase"></a>
# Litter Phase

Accepted-primitive balance reconstruction applies the physical rules, accepted-solve admission checks and receipts below. Accepted-primitive admission uses the complete nonlinear-solve interface and its required definitions. Independent solver-algorithm, numerical-branch implementation or evaluator-equivalence review additionally requires full numerical-methods. Physical primitive capture follows this mechanism’s operand and real-consumer evidence requirements; replay-experiment qualification is separate.
Current V3 litter specialization, including conservative post-phase capacity spill and heterogeneous resource joins. The later spill rule extends the initial exact-surface operand enumeration with one named negative spill operand. It does not replace phase, vapor, retained-ingress or WB14 custody.

<a id="version-14-snow-free-frozen-forest-litter-successor-amendment"></a>
## Version 14 snow-free frozen forest-litter successor amendment

Version 14 admits `OPENWEPP_SNOW_FREE_LSE_V3` as an immutable successor for
the snow-free `forest_litter` surface only. It retains every V1 and V2 model,
configuration, state, receipt, restart, and serialized byte unchanged. V3
imports the complete V2 canopy, radiation, turbulence, soil-thermal,
water-authorization, numerical, closure, 60-second support-admission, event,
and rollback authority, then adds the phase-specific litter rules below. It
does not widen bare-mineral, ponded, soil-frozen, snow-present, or snow-terminal
authority.

<a id="retained-authority-and-adjudicated-constants"></a>
### Retained authority and adjudicated constants

The peer-reviewed R-156 authority is
`references/vendorable/gmd-10-1621-2017-isba-meb-litter.pdf`, Appendix A,
equations A1--A4 and A7--A14, SHA-256
`2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d`.
The retained official SURFEX v8 sources are
`references/vendorable/surfex-v8/isba_meb.F90.source.html`, generated source
lines 1992--2159, SHA-256
`0a300739b5dc660b61d29db144dd92f886e8fdf9934eac8facc022585992087a`;
`references/vendorable/surfex-v8/isba_fluxes_meb.F90.source.html`, generated
source lines 388--407, SHA-256
`e0378bc89ee0d52cffe14841aac56de1d8d379edf18ad29f24cfdb9ea0dfdbbc`;
and `references/vendorable/surfex-v8/ini_csts.F90.source.html`, generated
source lines 146--157, SHA-256
`f39840df4d851efc70044f9e3ad62822371ed743c3c3a8055a4c940e2f86d73a`.
The retained source is governed by the retained CeCILL-C v1 English license,
SHA-256
`7280115e43fa03917f2f23370519be8c9fb0b57f4c86f8da5f7ac10c070f6aa0`.

The selected constants are

```text
T_ref    = 273.15 K
rho_w    = 1000 kg m^-3
rho_i    = 920 kg m^-3
C_w      = 4218 J kg^-1 K^-1
C_i      = 2106 J kg^-1 K^-1
L_f      = 333700 J kg^-1
tau_ice  = 3300 s
W_i,max  = 0.85*rho_w*dz_l kg m^-2 tile-ground.
```

R-156 supplies `T_ref`, `rho_i`, and `C_i`; the named SURFEX instantiation
supplies `tau_ice`, `L_f`, executable ordering, and the `0.85 m3 m^-3`
capacity. The capacity is liquid-water-equivalent because generated lines
2080--2081 convert both liquid and litter ice with `rho_w`. Therefore
`0.85*rho_i*dz_l` is a rejected capacity. The R-156 A4 printed sign conflicts
with A1--A3 and its melt prose; generated lines 2089--2117 and exact
liquid/ice conservation select `signed_phase = m_freeze - m_melt`.

<a id="v3-state-phase-free-solve-and-signed-vapor"></a>
### V3 state, phase-free solve, and signed vapor

Hydrology's versioned surface owner exclusively owns finite, nonnegative
`W_l` and `W_i` in `kg m^-2 tile-ground`; `W_i` is liquid-water-equivalent
litter ice and is neither snow nor soil `frozwt`. LSE V3 owns the surface
sensible-energy coordinate

```text
U = (C_dry + W_l*C_w + W_i*C_i)*(T_l-T_ref)   [J m^-2 tile-ground].
```

The complete V2 nonlinear system is first solved phase-free: no freeze, melt,
fusion term, phase-updated capacity, or phase-adjusted temperature appears in
any Newton residual, Jacobian, active branch, water authorization, or
convergence witness. Its immutable beginning-phase availability is
`W_l,0,W_i,0`. Define

```text
p_i = 0                                  when W_l,0 + W_i,0 = 0
p_i = W_i,0/(W_l,0 + W_i,0)             otherwise
h_ul = 0.5*(1-cos(pi*W_l,0/W_l,max))
h_ui = 0.5*(1-cos(pi*W_i,0/W_i,max))
v_l,raw = (1-p_i)*rho_a*(h_ul*q_sat_liquid(T_l,p)-q_recipient)/r_l-c
v_i,raw = p_i*rho_a*(h_ui*q_sat_liquid(T_l,p)-q_recipient)/r_l-c.
```

Both phases deliberately use saturation over liquid water, as R-156 A8
states; saturation over ice is not admitted. Positive `v_l` is liquid
evaporation and positive `v_i` is ice sublimation; negative values are liquid
condensation and ice deposition. Hydrology authorizes the two signed
components separately. Each positive finalized mass is bounded by only its
named immutable beginning pool; a negative component has no availability cap
and credits only its named phase. A liquid request cannot debit ice, an ice
request cannot debit liquid, and neither can use current ingress.

The phase-specific energy fluxes leaving the litter are

```text
Q_v,l = v_l*[C_w*(T_l-T_ref) + L_v(T_l)]
Q_v,i = v_i*[C_i*(T_l-T_ref) + L_s(T_l)].
```

They remain separately signed through authorization, owner-candidate
construction, receipt sealing, and independent reconstruction. Only then may
the air-side consumer aggregate `v_l+v_i` or `Q_v,l+Q_v,i`. Latent-only
energy, absolute-value aggregation, one total-store availability cap, or a
second SC-EVAP debit is invalid.

After fixed-authorization phase-free acceptance, install the finalized vapor
mass and energy exactly once:

```text
W_l,* = W_l,0 - dt*v_l
W_i,* = W_i,0 - dt*v_i
U_*   = (C_dry + W_l,*C_w + W_i,*C_i)*(T_*-T_ref).
```

The signed equations include condensation/deposition because negative vapor
adds mass. The accepted phase-free ledger must independently reconstruct the
same `U_*`, `T_*`, phase masses, and phase-specific vapor energy before phase
change begins. No raw request, authorization-as-use, or producer residual may
be installed.

<a id="bounded-kinetic-phase-and-fusion-energy-closure"></a>
### Bounded kinetic phase and fusion-energy closure

Require `0 <= W_i,* <= W_i,max`, `W_l,* >= 0`, positive finite `dz_l`, and a
positive finite ending heat capacity. With all operands evaluated from the
accepted post-vapor/pre-ingress state, define

```text
M_warm   = rho_i*C_i*dz_l*max(T_*-T_ref,0)/L_f
M_cold   = rho_i*C_i*dz_l*max(T_ref-T_*,0)/L_f
m_melt   = min(W_i,*, (dt/tau_ice)*min(M_warm,W_i,*))
m_freeze = min(W_l,*, W_i,max-W_i,*,
               (dt/tau_ice)*min(M_cold,W_l,*))
m_phase  = m_freeze - m_melt.
```

The outer mass/capacity bounds are binding for every admitted `dt`, including
supports greater than `tau_ice`; the phase operator never creates a maximum
step-size requirement. At `T_*=T_ref`, both transfers are exact zero. The
atomic phase candidate is

```text
W_l,phase = W_l,* - m_freeze + m_melt
W_i,phase = W_i,* + m_freeze - m_melt
U_phase   = U_* + L_f*m_phase
C_phase   = C_dry + W_l,phase*C_w + W_i,phase*C_i
T_phase   = T_ref + U_phase/C_phase.
```

Thus liquid debit equals ice credit on freezing, ice debit equals liquid
credit on melting, and the phase-only total enthalpy coordinate
`H_phase = U - L_f*W_i` is invariant exactly. Deriving `T_phase` with the
pre-phase heat capacity or applying literal `T += L_f*m_phase/C_*` is rejected
because it leaves an unowned heat-capacity-change energy term. A phase result
never triggers a same-support flux, fixed-point, water-authorization, or
Newton re-solve; it is the ending thermal state and next-support warm start.

<a id="ingress-wb14-identity-restart-receipts-and-failure-posture"></a>
### Ingress, WB14, identity, restart, receipts, and failure posture

#### Liquid-only WB14 availability

Only after the vapor and phase candidates pass may current precipitation,
runon, throughfall, canopy drainage, stemflow, and litter overflow enter the
existing hydrology chronology. Every admitted current parcel is liquid and
carries the unchanged V2 liquid sensible enthalpy. The complete owner
candidate then executes the existing WB14 partition with liquid-only
availability. `W_i,phase` cannot infiltrate, run off, drain, satisfy WB14,
enter soil `frozwt`, or mutate soil. Ingress cannot retroactively donate to
the already accepted vapor or phase operation.

#### Identity, restart and receipts

The immutable model tag is `OPENWEPP_SNOW_FREE_LSE_V3`; the phase receipt tag
is `OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1`. The V3 definition binds its V2
parent digest, both terminal contract digests, all retained-source hashes,
selected constants, equations, ordering, error map, and explicit refusals.
Checked V2-to-V3 LSE migration copies every V2 scientific value bit-identically
and changes only successor identity/digest material while joining an explicit
surface-owner V2 state. Checked surface-owner V1-to-V2 migration initializes
litter ice to exact zero; a new V2 seed may instead carry explicit finite
nonnegative ice. Temperature never synthesizes initial ice. Production V3-to-
V2 LSE and surface-owner V2-to-V1 downgrades are prohibited.

Every restart/checkpoint carries explicit LSE V3, surface-owner V2, model-
definition, contract, configuration, state, phase-state, and receipt tags and
digests. Missing, duplicate, stale, mixed-version, unknown, or digest-mismatched
restart material rejects before evaluation. A sealed phase receipt binds the
half-open support and exact duration bits; OFE/tile/area/owner/transaction
identities; beginning `W_l,W_i,U,T`; both raw and finalized signed vapor
components and phase-specific energies; `W_l,*,W_i,*,U_*,T_*`; all phase
constants, capacities, transfer bounds, `m_melt,m_freeze,m_phase`; the
phase-ending state; current-ingress parcel identities; liquid-only WB14
handoff; beginning and candidate owner digests; and independent mass, energy,
and `H_phase` reconstructions. Receipt serialization uses repository canonical
framing and never hashes a producer residual as proof.

Any domain, identity, availability, closure, receipt, restart, WB14, later
owner, or publication failure rolls back the LSE, surface owner, hydrology,
soil thermal, vegetation, BGC, receipts, checkpoint, and enclosing transaction
to byte-identical beginnings. The typed families are `LSEB-E-045` for V3
identity/domain/migration, `LSEB-E-046` for phase-specific vapor custody or
enthalpy, `LSEB-E-047` for phase mass/fusion/ending-capacity closure, and
`LSEB-E-048` for chronology/receipt/restart/rollback joins.

SC-EVAP-001 remains the owner of daily WB17 soil/residue/canopy ET. It neither
owns nor repeats this subdaily pre-WB14 litter liquid/ice vapor transaction.
SC-SURFACELIQUID-001 v14 owns the matching surface state/custody specialization,
and SC-WATBAL-001 retains liquid-only WB14 ownership. The exact
`60000000000 ns` physical fallback floor, support receipt, closure tolerances,
event chronology, topology, custody, and fail-closed posture are unchanged;
stable supports must still accept steps substantially larger than 60 seconds.

<a id="v3-invariants-and-required-production-vectors"></a>
### V3 invariants and required production vectors

| ID | Binding V3 rule | Guard/failure |
|---|---|---|

Contract-derived vectors must cover exact empty, all-liquid, all-ice, mixed,
freezing, melting, exact `T_ref`, condensation, deposition, availability-capped
evaporation/sublimation, ice-capacity saturation, `dt<tau_ice`, `dt=tau_ice`,
`dt>tau_ice`, exact 60 seconds, and substantially larger stable support.
Independent poisons must distinguish wrong A4 sign, `273.16 K`, `rho_i=917`,
wrong `L_f`, wrong `tau_ice`, `rho_i` ice capacity, saturation over ice,
latent-only vapor, total-store capping, simultaneous double debit, instant
equilibrium, freeze-only logic, pre-phase capacity temperature, current-ingress
donation, ice-as-WB14 supply, same-support re-solve, implicit ice initialization,
production downgrade, `zertol` ice deletion, `xwgmin` regularization, soil
compensation, producer-residual closure, stale restart, and partial commit.

The unchanged canonical real-consumer obligations are
`tests/integration/erosion_single_ofe_p61_sediment.rs` and
`tests/integration/dff_ws1_native_forest_cli.rs`. Each must run through the
production selector, persist/reload successor state, and prove primitive-
operand liquid/ice mass, fusion-energy, vapor-energy, WB14, and whole-envelope
closure. Contract or source scanning alone is intentionally insufficient.

<a id="spill"></a>
<a id="exact-v3-litter-phase-capacity-spill-amendment"></a>
## Exact V3 Litter-Phase Capacity-Spill Amendment

`INV-LANDSURFACEENERGY-156` closes the already-required `litter overflow`
handoff between bounded phase and current ingress. The existing V3 phase
receipt remains the immutable raw bounded-phase image; it is not rewritten or
accepted as an over-capacity surface owner. For each forest-litter tile,
reconstruct the raw phase result from the accepted post-vapor state and phase
receipt in the existing order:

```text
W_raw = W_l,* - m_freeze + m_melt
W_i,end = W_i,* + m_freeze - m_melt
U_raw = U_* + L_f*(m_freeze-m_melt)
C_raw = C_dry + C_w*W_raw + C_i*W_i,end
T_raw = T_ref + U_raw/C_raw.
```

If `W_raw<=W_l,max`, the typed spill is exact positive zero and the raw state
is the retained state. If `W_raw>W_l,max`, construct one
`LitterPhaseCapacitySpillV1` by the canonical checked binary64 operations:

```text
m_spill,tile = W_raw - W_l,max
h_spill       = C_w*(T_raw-T_ref)
Q_spill,tile  = m_spill,tile*h_spill
W_retained    = W_raw - m_spill,tile
U_retained    = U_raw - Q_spill,tile
C_retained    = C_dry + C_w*W_retained + C_i*W_i,end
T_retained    = T_ref + U_retained/C_retained.
```

Every operation must be finite, `m_spill,tile>0`, `0<=W_retained<=W_l,max`,
and `C_retained>0`. The second subtraction is the authoritative remainder;
`min`, clamp, saturation normalization, a tolerance snap, or discard is not
the spill algorithm. Independent reconstruction must reproduce the raw phase
mass from retained mass plus spill and the raw sensible enthalpy from retained
enthalpy plus `Q_spill,tile`, using the declared operation order. The spill is
liquid only; litter ice and fusion energy are not reclassified as runoff.

The companion binds the original phase-receipt SHA-256, LSE configuration and
surface-owner identity, transaction, exact child support, OFE/tile/surface/
source key, `W_raw,U_raw,T_raw`, capacity, retained state, spill mass,
`h_spill`, and `Q_spill`. It cannot be caller synthesized or labeled as a
condensation credit. The phase-adjusted surface owner seals the retained state
and its closure debits `m_spill,tile` exactly once. The V16 exact-surface owner
adds exactly one named negative
`LitterPhaseCapacitySpillEnergy` operand `-Q_spill,tile`, independently
reconstructed from companion mass and temperature. This debit is in addition
to, and ordered after, the unchanged phase-free and fusion operands.

For current-ingress custody, checked area conversion produces
`m_spill,ofe=f_t*m_spill,tile`; its parcel energy is
`Q_spill,ofe=m_spill,ofe*h_spill`. One internally constructed
`LitterPhaseOverflow` parcel uses the same transaction and full accepted child
support `[0,dt)`, retains the exact source key and phase-receipt identity, and
enters the ordinary SurfaceLiquid/WB14 mixing, infiltration, excess, retention,
runoff, and topology routing path exactly once. The full-child support is the
existing aggregate LSE-child timing authority; it may not be reassigned to a
rainfall hour or another support. Any retained portion returns through the
ordinary named retained-ingress energy receipt, never by cancelling or
omitting the spill debit.

Missing/duplicate spill, wrong phase receipt, key, capacity, transaction,
support, area basis, sign, temperature, specific enthalpy, exact-surface
operand, or WB14 receipt rejects before installation. A failed split, ingress,
final replay, owner join, or publication preserves the complete LSE,
surface-liquid, exact-enthalpy, soil, WB14 parent, receipt, and runner beginning
bytes. The spill does not invoke a second vapor/phase evaluation, fixed point,
water authorization, or same-support LSE solve and changes no phase equation,
capacity, tolerance, temporal floor, or WB14 constitutive rule.

`OBL-LANDSURFACEENERGY-C-011` — Prove zero/below/at/above-capacity cases,
melt-created positive spill, exact raw-to-retained-plus-spill mass and sensible-
enthalpy reconstruction, named negative exact-surface operand, checked tile/OFE
basis conversion, full-child timing, one WB14 call/supply, retained/infiltrated/
routed dispositions, phase/key/transaction/support/enthalpy substitution
poisons, no same-support re-solve, and byte-exact rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Evaluate bounded phase once; split an over-capacity raw liquid ending into retained state plus typed spill; debit litter energy; admit the spill once to current ingress/WB14. |
| branch/guard | `W_raw<=W_l,max` has zero spill; `W_raw>W_l,max` requires the complete typed spill and exact closure. Missing custody rejects; no clamp or condensation alias exists. |
| invariant guard map | `INV-LANDSURFACEENERGY-156` -> pure spill splitter, phase companion, surface-owner closure, V16 exact debit, ingress receipt, WB14 and rollback joins. |
| test vector | `OBL-LANDSURFACEENERGY-C-011`: capacity boundary sides, melt spill, exact closures, one ingress/WB14, substitutions, no-resolve, rollback. |
| binding exposure | `LSE-V3-LITTER-PHASE-CAPACITY-SPILL`, active, `maps-to-existing-INV`, IDs `156/C-011`, dual review/verification. |
| change log | 2026-09-01, contract 21: conservative typed post-phase litter capacity spill into current ingress; unchanged phase/WB14 equations, tolerances, floor, and prior receipt bytes. |

<a id="resource-join"></a>
<a id="exact-heterogeneous-v3-surface-resource-join-amendment"></a>
## Exact Heterogeneous V3 Surface-Resource Join Amendment

`INV-LANDSURFACEENERGY-157` governs a V3 batch containing both accepted native
frozen-litter tiles and ordinary legacy open or covered surface withdrawals.
The unified water protocol remains the sole authority for requests,
authorizations, and finalized uses. The accepted native phase receipt remains
the sole consumer of its phase-specific litter-vapor row. Match that row by
the exact transaction, child support, OFE, tile, surface/source identity and
checked tile-fraction/support aggregation, and remove it from further resource
application. Its receipt-derived finalized amount `F` must satisfy the same
`0<=F<=A<=D` custody relation as the unified protocol; `A` remains an upper
authorization and need not be bit-identical to `F`. The finalized-use row is
bit-identical to the receipt-derived `F`. A missing, duplicate, foreign, or
out-of-bound native row rejects; it cannot be reclassified as ordinary.

The complete unmatched surface set is the ordinary set. Every ordinary row
must retain its original request/authorization/finalized-use identity and
`0<=F<=A<=D`. Aggregate those rows in complete `GroundWaterKey` order and
apply the existing checked `F/f_t` debit exactly once to the accepted
phase-adjusted V2 liquid owner. The join is typed
`SurfaceLiquidV2HeterogeneousResourceJoinV1`; it starts from that exact owner,
not from an independently reconstructed legacy beginning, and produces the
one resource candidate consumed by the existing one current-ingress call.
Zero ordinary rows are the identity join.

### Mass custody without additional energy

This is a mass-custody join, not a second LSE energy operation. The accepted
LSE tile ending and native phase/exact-surface receipts already own vapor,
fusion, and sensible-energy effects. The join retains native litter ice,
surface enthalpy high mirrors and exact carry/receipt bytes, phase closure, and
`LitterPhaseCapacitySpillV1` unchanged. The spill remains the separate internal
`LitterPhaseOverflow` ingress parcel and never enters `finalized_uses`.
Ordinary resource debit supplies no new parcel and no second enthalpy, latent,
fusion, or exact-surface operand. Wholesale substitution of
`accepted.surface_resource`, rebuilding from legacy owner bytes, phase-row
replay, spill reclassification, capacity repair, and a second ingress are
forbidden.

The join authenticates one transaction and accepted child support, complete
request/authorization/finalized-use cardinality, exact owner/configuration and
predecessor lineage, row/store keys, source and area basis, finite amount, and
canonical debit reconstruction. Every finalized row is accounted exactly once
as soil, accepted native phase, or ordinary surface resource. Failure of the
partition, debit, native receipt join, ingress, receiver closure, final owner
join, or publication preserves every surface-liquid V1/V2, LSE V3/V16, soil,
WB14 parent, spill, receipt, cursor, and runner beginning byte.

`OBL-LANDSURFACEENERGY-C-012` — Prove a heterogeneous native/ordinary batch,
zero and positive ordinary withdrawals, canonical order independence, exact
ordinary mass closure, native-row exclusion and replay refusal, retained
phase/spill/ice/enthalpy bytes, one resource candidate and ingress, rejection
of omitted/duplicate/foreign/wrong-amount/transaction/support/key rows, and
complete rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Partition finalized surface rows by accepted native phase receipt; apply the complete unmatched ordinary set once to the phase-adjusted V2 owner; execute the existing ingress once. |
| branch/guard | Native phase rows must match and are never replayed; every other row is an authenticated ordinary finalized use. No wholesale resource replacement or untyped remainder is admitted. |
| invariant guard map | `INV-LANDSURFACEENERGY-157` -> typed heterogeneous row partition, V2 resource join, canonical ordinary debit, one-ingress and rollback validators. |
| test vector | `OBL-LANDSURFACEENERGY-C-012`: heterogeneous/identity cases, mass closure, native replay poison, row identity/cardinality poisons, retained receipt bytes, one ingress, rollback. |
| binding exposure | `LSE-V3-HETEROGENEOUS-SURFACE-RESOURCE-JOIN`, active, `maps-to-existing-INV`, IDs `157/C-012`, dual review/verification. |
| change log | 2026-09-02, contract 22: exact once-only ordinary finalized-use debit on the accepted native phase-adjusted V2 owner; unchanged LSE/phase/WB14 equations, tolerances, support, and energy receipts. |


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-156"></a> `INV-LANDSURFACEENERGY-156` | After the bounded V3 litter phase operator, liquid above the configured litter-liquid capacity is one typed physical spill. The retained phase state and spill exactly partition the raw phase mass and sensible enthalpy at the raw ending temperature; the spill is debited from litter once, enters current-ingress/WB14 once with exact phase-receipt, transaction, support, key, area-basis, mass, and enthalpy custody, and contributes one named negative exact-surface-energy operand. It is neither clipping nor condensation and never causes a same-support phase or flux re-solve. | `INV-LANDSURFACEENERGY-144/151`, `SC-SURFACELIQUID-001#INV-SURFACELIQUID-028`, `SC-WATBAL-001#INV-WATBAL-103` | `[DIRECT][Static] + [INFERENCE][Static]` | phase split/receipt/current-ingress/exact-surface/rollback test | `LSEB-E-047/048/050` with complete rollback |
| <a id="INV-LANDSURFACEENERGY-157"></a> `INV-LANDSURFACEENERGY-157` | A heterogeneous V3 water-protocol finalization joins the accepted native frozen-litter phase owner to ordinary surface withdrawals without replacing either custody chain. Native litter vapor rows already consumed by the phase receipt are matched and excluded exactly; every remaining ordinary finalized-use row is authenticated against its request and authorization and debits the phase-adjusted V2 liquid owner exactly once, in canonical key order, before the one current-ingress call. Native phase/spill/ice/enthalpy custody is retained unchanged and no energy operand is replayed or synthesized. | `SC-SURFACELIQUID-001#INV-SURFACELIQUID-004/018/028/029` + transaction atomicity | `[DIRECT][Static] + [INFERENCE][Static]` | typed row partition/V2 resource join/one-ingress/rollback test | `LSEB-E-047/048/050` or typed surface protocol failure with complete rollback |
| <a id="INV-LANDSURFACEENERGY-140"></a> `INV-LANDSURFACEENERGY-140` | V3 is an immutable V2 successor; V1/V2 scientific and serialized bytes remain unchanged and mixed identities reject. | v31:L1638-L1638 | [INFERENCE][Static] | identity/migration/restart / `LSEB-E-045/048` | identity/migration/restart / `LSEB-E-045/048` |
| <a id="INV-LANDSURFACEENERGY-141"></a> `INV-LANDSURFACEENERGY-141` | The nonlinear solve is phase-free and publishes separate signed liquid and ice vapor under liquid-water saturation with phase-specific sensible-plus-latent enthalpy. | v31:L1639-L1639 | [INFERENCE][Static] | exact component/authority joins / `LSEB-E-046` | exact component/authority joins / `LSEB-E-046` |
| <a id="INV-LANDSURFACEENERGY-142"></a> `INV-LANDSURFACEENERGY-142` | Finalized vapor uses immutable beginning phase availability, installs each mass/energy component exactly once, and cannot consume current ingress. | v31:L1640-L1640 | [INFERENCE][Static] | D/A/F and owner-candidate reconstruction / `LSEB-E-046` | D/A/F and owner-candidate reconstruction / `LSEB-E-046` |
| <a id="INV-LANDSURFACEENERGY-143"></a> `INV-LANDSURFACEENERGY-143` | The `3300 s` kinetic operator is bounded by named liquid, ice, and liquid-water-equivalent ice-capacity operands and uses `freeze-melt`. | v31:L1641-L1641 | [INFERENCE][Static] | independent transfer reconstruction / `LSEB-E-047` | independent transfer reconstruction / `LSEB-E-047` |
| <a id="INV-LANDSURFACEENERGY-144"></a> `INV-LANDSURFACEENERGY-144` | Equal liquid/ice transfer and `L_f*m_phase` conserve `U-L_f*W_i`; ending temperature uses ending dry/liquid/ice heat capacity. | v31:L1642-L1642 | [INFERENCE][Static] | independent mass/fusion/temperature closure / `LSEB-E-047` | independent mass/fusion/temperature closure / `LSEB-E-047` |
| <a id="INV-LANDSURFACEENERGY-145"></a> `INV-LANDSURFACEENERGY-145` | Phase precedes current liquid ingress and liquid-only WB14; litter ice never enters soil, runoff, infiltration, or `frozwt`. | v31:L1643-L1643 | [INFERENCE][Static] | chronology and owner guard / `LSEB-E-048` | chronology and owner guard / `LSEB-E-048` |
| <a id="INV-LANDSURFACEENERGY-146"></a> `INV-LANDSURFACEENERGY-146` | Phase is post-solve and cannot trigger a same-support re-solve; exact 60-second fallback and larger-support obligation remain unchanged. | v31:L1644-L1644 | [INFERENCE][Static] | solve-count/support receipt / `LSEB-E-048` | solve-count/support receipt / `LSEB-E-048` |
| <a id="INV-LANDSURFACEENERGY-147"></a> `INV-LANDSURFACEENERGY-147` | Successor restart and phase receipts bind every primitive operand and all owner identities; any later failure rolls back the full envelope byte-exactly. | v31:L1645-L1645 | [INFERENCE][Static] | restart/receipt/rollback / `LSEB-E-048` | restart/receipt/rollback / `LSEB-E-048` |
| <a id="INV-LANDSURFACEENERGY-148"></a> `INV-LANDSURFACEENERGY-148` | SC-EVAP daily WB17 does not duplicate V3's subdaily pre-WB14 surface vapor debit or credit. | v31:L1646-L1646 | [INFERENCE][Static] | cross-owner exact-one guard / `LSEB-E-046/048` | cross-owner exact-one guard / `LSEB-E-046/048` |
| <a id="INV-LANDSURFACEENERGY-149"></a> `INV-LANDSURFACEENERGY-149` | Canonical `p61` and native-forest consumers must install and persist V3 phase state with independent mass/energy closure; producer-only or synthetic evidence cannot close runtime adoption. | v31:L1647-L1647 | [INFERENCE][Static] | real-consumer gate / hard `HOLD` until run | real-consumer gate / hard `HOLD` until run |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-011"></a> `OBL-LANDSURFACEENERGY-C-011` | prove the post-phase raw litter liquid and sensible enthalpy split into one within-capacity retained state and one typed spill, with exact phase-receipt/transaction/support/key custody, one negative exact-surface operand, one current-ingress/WB14 handoff, independent tile/OFE mass and enthalpy reconstruction, no re-solve, and complete rollback. | V3 litter post-phase capacity-spill consumers | v31:L350-L354 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#spill); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-012"></a> `OBL-LANDSURFACEENERGY-C-012` | prove a heterogeneous V3 batch classifies and consumes every finalized surface row exactly once: accepted native litter vapor rows only through their phase receipts and all remaining authenticated ordinary rows through one canonical debit of the phase-adjusted V2 owner before one ingress. Prove no phase/spill/ice/enthalpy replacement or energy replay, exact transaction/support/key/cardinality joins, and full rollback. | Heterogeneous finalized native/ordinary resource join consumers | v31:L355-L360 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#resource-join); named fixtures/tests/real consumers |
