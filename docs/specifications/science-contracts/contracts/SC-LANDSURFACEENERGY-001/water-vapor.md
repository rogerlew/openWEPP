[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| terminal-support.md#support | physical solve or receipt interval admission, including snow-free work | physical support, zero support and pre-Newton floor | section |
| soil-coupling.md#soil-coupling | ground transfer or energy closure | opposite ground transfer and state | whole chapter |
| surface-energy.md#surface-energy | physical evaluator or energy closure | radiative/turbulent operands | whole chapter |
| litter-phase.md#litter-phase | active frozen-litter vapor/phase or closure | signed phase-specific vapor and spill | whole chapter |
| ../SC-SNOWFREEZE-001.md#invariants | rain-temperature provider-rule review (including the retained provider) or provider-derived ingress-enthalpy requirements/closure | INV-SNOWFREEZE-075 provider; accepted authoritative parcel reconstruction alone does not rederive it | whole external contract |
| ../SC-WATBAL-001.md#wb14-infiltration-and-hyetograph-coupling-addendum | ingress partition or closure | sole WB14 owner | whole external contract; frozen protocol scope |
| ../SC-WATBAL-001.md#purpose | cross-contract water scope | water scope | whole external contract; frozen protocol scope |
| ../SC-SURFACELIQUID-001.md#algorithm-specification | surface-water custody or closure | accepted water protocol | whole external contract; frozen protocol scope |
| ../SC-SURFACELIQUID-001.md#purpose-and-scientific-scope | cross-contract surface scope | surface owner scope | whole external contract; frozen protocol scope |

<a id="water-vapor"></a>
# Water Vapor
Current signed water/vapor enthalpy and immutable-beginning/current-ingress chronology. The exact one-ULP liquid publication rule applies only at its named boundaries. Litter phase-specific vapor specializes these rules only as specified in litter-phase.

<a id="vapor"></a>
<a id="signed-vapor-and-liquid-enthalpy"></a>
### Signed vapor and liquid enthalpy

The liquid reference state is `T_ref=273.15 K`; `C_w=4218 J kg^-1 K^-1`.
Each parcel carries `h_l(T)=C_w*(T-T_ref)` and `Q=m*h_l(T)`. Positive-mass
mixing conserves enthalpy exactly:
`T_mix=T_ref+sum(m_i*h_i)/(C_w*sum(m_i))`. A zero-mass crossing has neither
temperature nor energy. Rain temperature is the exact retained output
`hydrometeor_temperature_c+273.15` from
`openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity`
on the active `harder_pomeroy_hourly` provider required by
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-075`; LSE neither recomputes nor partially
transcribes that solver. Runon must carry the typed temperature and enthalpy of
the accepted upstream outlet parcel. Missing runon temperature rejects; air,
soil, freezing, or downstream surface temperature is never substituted.
Infiltration and runoff carry the exact accepted source-parcel or conservative
mixture temperature selected by hydrology.

The selected liquid vaporization enthalpy is
`L_v(T)=2.501e6-2369*(T-T_ref) J kg^-1`. The signed vapor-energy flux leaving
the surface is

```text
Q_v = v_s*[h_l(T_s)+L_v(T_s)]                 [W m^-2].
```

Thus evaporation (`v_s>0`) removes both liquid sensible enthalpy and latent
energy, while condensation (`v_s<0`) credits both. Evaporation produces a
positive water request `D=max(v_s,0)*dt`; only finalized positive use is
debited. Condensation produces no withdrawal request and hydrology credits the
exact amount `-v_s*dt` to the typed surface store. Any clipping, absolute
value, latent-only record, authorization-as-use, or missing condensation mass
credit fails.

The interval surface equation before current-ingress advection is

```text
(U_pre-U_0)/dt = R_sw + R_lw - H_s - Q_v - G_s1.
```

`U_pre` uses the hydrology-owned pre-ingress mass after finalized evaporation
debit or condensation credit. Every term is evaluated at the accepted trial
surface state and occurs exactly once.

<a id="water"></a>
<a id="immutable-beginning-water-transaction-and-current-ingress"></a>
### Immutable-beginning water transaction and current ingress

The water snapshot precedes all current-interval rain, runon, and canopy liquid
release. Only immutable beginning surface/litter and soil-layer liquid stores
are available to root uptake and ground evaporation. Current precipitation,
runon, throughfall, both canopy drainage terms, stemflow, and litter overflow
cannot satisfy or reduce a same-interval withdrawal request.

From immutable beginning vegetation/LSE/soil-thermal state and that immutable
hydrology snapshot, solve the complete current-temperature canopy--ground
system without owner caps. Publish root, surface/litter, and soil-layer
requests with transaction/OFE/tile/occupancy/surface/source/layer/basis
identity. Hydrology authorizes all same-snapshot requests exactly once, before
current ingress. Therefore a changed final canopy release cannot shrink or
enlarge authorized supply.

Rebuild the complete system from the original beginning state with fixed
source-specific caps. The cap is active iff `cap_rate<=q_law`; equality selects
the cap branch and zero generalized derivative. Gas, canopy energy, surface
energy, shared air, hydraulics and soil thermal state re-solve together. Final
use is `F=f_tile*q*dt`, independently checked as `0<=F<=A<=D`. No second
authorization, donation, scalar stress or potential-state continuation exists.

After the capped solve accepts, hydrology applies finalized beginning-store
debits and the explicit condensation credit, then accepts final current
precipitation, runon, and canopy release. Hydrology partitions that ingress
exactly once into retained surface/litter store, infiltration, routed runoff,
and outlet runoff. This is owner candidate construction, not a second
authorization. Ingress parcels retain their individual enthalpies. Retained
ingress updates surface `U`; infiltration energy credits soil thermal node 1;
routed runoff transfers the same accepted parcel mass and enthalpy to the
downstream OFE; outlet runoff transfers them out. The ending surface
temperature is obtained only from the authoritative ending `U_s,W,C_dry`
identity. Current ingress does not feed the already accepted same-interval
H/LE/G flux evaluation.

```text
U_s,1       = U_pre + sum(Q_retained_ingress)
E_soil,1,1  = E_soil,1,pre + sum(Q_infiltration)
Q_runon,dst = Q_routed_runoff,src
Q_outlet    = sum(Q_outlet_runoff).
```

Each equality uses the identical accepted mass/enthalpy parcel; zero mass has
zero energy and no temperature. The soil owner reconstructs its first-layer
temperature from the credited enthalpy. Hydrology may combine parcels only by
the conservative mixing equation above.

Potential and final passes use identical operator ordering. Only the final
pass constructs owner candidates and applies the once-only ingress partition.

<a id="errors"></a>
<a id="independent-closure-and-errors"></a>
### Independent closure and errors

Independent validators reconstruct local then weighted OFE-ground shortwave,
longwave, sensible, signed vapor enthalpy, surface storage, soil storage,
ground and advected energy; hydrology mass;
latent mass/energy; and equal/opposite ground heat. They consume primitive
operands, never producer residuals. Required new error families are:

- `LSEB-E-030` unsupported domain;
- `LSEB-E-031` strict configuration/state/identity;
- `LSEB-E-032` radiation or turbulent ownership;
- `LSEB-E-033` source-water D/A/F;
- `LSEB-E-034` numerical convergence;
- `LSEB-E-035` component/control-volume closure;
- `LSEB-E-036` liquid enthalpy/latent join; and
- `LSEB-E-037` ground-heat or atomic-owner join.

Additional typed failures are `LSEB-E-038` current-ingress ordering or
same-interval availability, `LSEB-E-039` condensation mass/energy credit, and
`LSEB-E-040` soil-thermal owner/state/enthalpy mismatch.

Any error preserves vegetation, hydrology, LSE, BGC, soil-thermal and envelope
bytes. `GAP-LANDSURFACEENERGY-001..003` and the authority portion of 006 are
`AUTHORITY_ADMITTED`; gap 004 remains `IMPLEMENTATION_MISSING`; gap 005 remains
outside the snow-free model.

<a id="v1-invariants-and-independent-fixtures"></a>
### V1 invariants and independent fixtures

| ID | Binding V1 rule |
|---|---|

The digest-bound independent fixture family must include open bare-soil day and
night, dry and wet litter, covered and open tiles, two heterogeneous columns,
zero shortwave, longwave and ground-heat sign reversal, ground feedback to
canopy air, evaporation and condensation, full and partial surface/top-layer
authorization, concurrent root/ground scarcity, dry source, rain/runon/
infiltration/runoff advection, equilibrium-zero and finite heat storage,
alternate warm starts, singular/backtracking/iteration failures, and exact
rollback. Frozen, snow, terminal-snow, calm, and nonneutral inputs are typed
rejection vectors.

Required poisons independently distinguish bulk/repartitioned longwave,
prescribed upward ground longwave, reference-air ground exchange beneath
canopy, omitted ground canopy-air H/v, agricultural PMET donation, current
ingress counted as available water, final canopy release shrinking
authorization, missing/doubled `f_t`, authorization as finalized use, vapor
zero-clipping, condensation sign reversal or missing mass credit, latent-only
vapor energy, omitted/swapped liquid advection, duplicated `G`, an LSE-owned
soil temperature, hidden wind floor, and producer-supplied residual.

<a id="version-9-exact-liquid-reference-state-representation-amendment"></a>
## Version 9 exact liquid reference-state representation amendment

The liquid enthalpy datum remains exactly `T_ref = 273.15 K` and
`h_l(T)=4218*(T-T_ref) J kg^-1`. In the covered-canopy solver, the existing
inactive wet-surface coordinate anchor can represent its physically exact
phase-reference solution as the immediately adjacent binary64 value above
`T_ref`. At a 60-second admitted adaptive support, multiplying that representational
offset by a positive parcel mass produces a nonzero energy far below the
binary64 spacing of an otherwise valid persistent receiver ledger. Publishing
that artifact would require the receiver either to lose a positive credit or
to reject an exact-reference physical state.

Before a covered-canopy liquid ledger or release is materialized, and nowhere
else, define

```text
T_ref_bits       = 0x4071126666666666
T_ref_next_up    = 0x4071126666666667 = 273.15000000000003 K
Delta_T_ref_up   = T_ref_next_up - T_ref
                 = 2^-44 K = 5.684341886080802e-14 K.
```

If `T_wet`, or the mass-weighted Stage 3 terminal-liquid publication
temperature, is bit-identical to `T_ref_next_up`, set it to `T_ref` before
computing `h_l` and persist the canonical temperature and exact zero specific
enthalpy together. Exact `T_ref` is unchanged. A value below `T_ref` retains
the existing covered-canopy-snow rejection; a value at or above the second
upward binary64 neighbor is not normalized. This is a reference-state
representation rule, not a tolerance relaxation: it does not change liquid
mass, wet fraction, solver residuals, accepted support, storage arithmetic,
phase ownership, or any non-reference temperature.

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-130` | Only the exact first upward binary64 neighbor of `273.15 K` is canonicalized to the exact liquid enthalpy reference before covered-canopy ledger/release or Stage 3 terminal-liquid publication. | exact-bit runtime guard plus below/at/above boundary vectors; existing typed closure/domain failure otherwise |


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-130"></a> `INV-LANDSURFACEENERGY-130` | A covered-canopy or Stage 3 terminal-liquid temperature represented as exactly one upward binary64 spacing from `T_ref` is canonicalized to exact `T_ref` at its named publication boundary; every other temperature remains unchanged or fails its existing domain guard. | exact reference-state representation authority | `[INFERENCE][Static]` | runtime/test | typed closure/domain failure |
| <a id="INV-LANDSURFACEENERGY-104"></a> `INV-LANDSURFACEENERGY-104` | Signed vapor mass and `v*[h_l+L_v]` energy preserve sign; condensation has one explicit hydrology credit. | v31:L1093-L1093 | [INFERENCE][Static] | [Solve guards](solve-boundary.md#solve) and [typed errors](#errors) | [Typed failures](#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-105"></a> `INV-LANDSURFACEENERGY-105` | Root and ground requests share beginning stores before ingress; accepted current ingress is partitioned exactly once after the capped solve. | v31:L1094-L1094 | [INFERENCE][Static] | [Solve guards](solve-boundary.md#solve) and [typed errors](#errors) | [Typed failures](#errors); no partial state or promotion |
| <a id="INV-LANDSURFACEENERGY-107"></a> `INV-LANDSURFACEENERGY-107` | Local tile closure precedes one `f_t` weighting to OFE ground; no cross-OFE aggregate is called stand ground. | v31:L1096-L1096 | [INFERENCE][Static] | [Solve guards](solve-boundary.md#solve) and [typed errors](#errors) | [Typed failures](#errors); no partial state or promotion |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-P-001"></a> `OBL-LANDSURFACEENERGY-P-001` | produce immutable, source-identified records for all water and energy operands and a sealed start-to-end ledger. | All LSE producer paths; retain named model/regime and reviewed terminal-cutover limits | v31:L311-L312 | [Guards/errors](#errors) | [Tests](common-details.md#tests); [Detail](common-details.md#tests); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-P-002"></a> `OBL-LANDSURFACEENERGY-P-002` | validate before mutation and commit energy and water state atomically. | All LSE producer paths; retain named model/regime and reviewed terminal-cutover limits | v31:L313-L314 | [Guards/errors](#errors) | [Tests](common-details.md#tests); [Detail](common-details.md#tests); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-P-003"></a> `OBL-LANDSURFACEENERGY-P-003` | expose branch, interval, tolerances, residuals, and exact component lineage without silent defaults or clamps. | All LSE producer paths; retain named model/regime and reviewed terminal-cutover limits | v31:L315-L316 | [Guards/errors](#errors) | [Tests](common-details.md#tests); [Detail](common-details.md#tests); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-P-004"></a> `OBL-LANDSURFACEENERGY-P-004` | reject all schema-v8 terminal payloads until a reviewed atomic cutover revises both snow and receiving-surface authority. | All LSE producer paths; retain named model/regime and reviewed terminal-cutover limits | v31:L317-L318 | [Guards/errors](#errors) | [Tests](common-details.md#tests); [Detail](common-details.md#tests); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-001"></a> `OBL-LANDSURFACEENERGY-C-001` | ET supplies one actual evaporation debit and consumes no second latent debit. | ET consumers of actual evaporation and latent energy | v31:L326-L327 | [Guards/errors](#errors) | [Tests](common-details.md#tests); [Detail](common-details.md#tests); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-002"></a> `OBL-LANDSURFACEENERGY-C-002` | infiltration/runoff consumes one water offer, returns sealed partition terms, and remains sole water-partition owner. | Infiltration/runoff consumers of the LSE water offer | v31:L328-L329 | [Guards/errors](#errors) | [Tests](common-details.md#tests); [Detail](common-details.md#tests); named fixtures/tests/real consumers |


<a id="isolated-b01-transparent-canopy-liquid"></a>
## Isolated B01 transparent-canopy liquid energy

The owner-authorized B01 cycle integration experiment may correct the reached
zero-area canopy liquid-temperature identity under policy14. This is an
explicit experimental amendment to source-bound liquid release temperature, not a
new canopy heat-capacity model, freezing model, or production activation.
Provenance is the first law applied to the existing liquid reference and
SC-VEGETATION-001 INV-075/114, with the 2026-09-10 owner authorization sections
4 and7. Evidence class: [INFERENCE][Static]; independent accepted-primitive
reconstruction of the reached rain transfer motivates implementation, not a
calibrated parameter or changed forcing.

The admitted transparent column is identified from immutable beginning state
before iteration: every occupancy has exactly zero LAI, SAI, sun/shade/stem
component area and beginning canopy liquid. Zero wet fraction alone is not
sufficient. In this regime interception capacity, canopy storage, radiation,
sensible exchange and vapor exchange are zero under the existing equations.
Retain the existing top-to-bottom E04 water splits, including the separate
stemflow bypass. The column does not exchange heat with the passing water.
For positive incoming mass, its actual release temperature therefore represents

```text
M_in = sum(m_p)                    [kg m^-2 tile ground]
H_in = sum(m_p*h_p)                [J m^-2 tile ground]
T_release = 273.15 + H_in/(4218*M_in)   [K].
```

Use actual authenticated precipitation parcels and their retained provider,
source state, destination, transaction and support, never air or ground
substitution. Each occupancy receives its actual upstream release mass and
enthalpy. A wholly transparent column retains the same mixture temperature
through every rank while stemflow bypasses lower ranks. The sum of terminal
throughfall/drainage and all stemflow enthalpies must independently equal the
top incoming enthalpy within the prospectively declared arithmetic allowance.
Downstream OFE mixing remains a separate conservative transfer and may change
source-tagged receipt temperatures; those postmix tags are not enthalpy tracers.

For this transfer's binary64 arithmetic, use the prospective forward-error
allowance `gamma_n*A` in J m^-2 tile ground, `gamma_n=n*u/(1-n*u)`,
`u=2^-53`, `n=64*(N_parcels+4*N_occupancies+8)`. This conservatively counts
parcel products/sums, the mixture division/reference addition, unchanged mass
splits and final enthalpy products/sums. `A` is the sum of absolute incoming
and outgoing `m*h` products plus `4218*sum(m*(abs(T)+273.15))` over those same
inputs/outputs, explicitly retaining subtraction/reference-rounding scale.
Add only the separately authorized one-upward-ULP reference publication term
`4218*M*ulp(273.15)` when that exact existing rule actually applies. This is
an accounting allowance, not the nonlinear temperature tolerance; any missing
source or larger discrepancy remains failed evidence. Report individual
supports before cumulative sums. Compute the mixture once from the complete
top parcel set, then retain unchanged specific enthalpy along each transparent
rank's linear mass splits; `N_parcels` counts every top product/sum entry once
and `4*N_occupancies` bounds the four release branches per rank. An
implementation that instead re-sums tagged parcel lists at each rank must count
all those entries explicitly. Integer counts and all allowance operands must
be finite/checked, with `n*u<1`. This relative-error model admits only normal
nonzero evaluated masses, products, sums and divisions; exact mathematical
zero remains allowed. A subnormal or underflowed nonzero calculation, or
arithmetic overflow, yields typed arithmetic-domain refusal in this isolated
identity/closure path. It is never normalized to zero or charged an invented
underflow reservoir. The separately authorized ULP term applies only to the
actual mass whose release temperature undergoes that exact named adjustment.

When incoming mass and beginning canopy liquid are both exactly absent, keep
the existing inactive coordinate anchor. Zero mass carries no temperature or
energy. A positive-mass missing/foreign/corrupt thermal source rejects before
iteration. No guessed temperature, zero-filled heat, extra residual reservoir,
postsolve clipping or dropped parcel is permitted. The downstream liquid
material domain remains 200..350 K. Authenticated passage may carry signed
liquid sensible enthalpy below the reference temperature; it does not freeze,
warm, clip or repartition the already supplied liquid phase. The existing
one-upward-ULP publication rule remains exact and separate from the raw
source-mixture temperature, with its mass-specific allowance as above.

This paragraph prospectively supersedes the earlier transparent numerical
anchor amendment. The zero-area numerical wet coordinate retains its canonical
inactive target `max(T_canopy,273.15 K)` in the residual and both ordinary/native
Jacobians. Existing liquid-vapor trial domains remain unchanged. This numerical
coordinate carries no component energy and does not set the passage temperature.
Publish its numerical component value into occupancy state and the next warm
start; retain the distinct source-bound release temperature in the E04 ledger.
For this passage only, that ledger temperature need not equal the numerical wet
coordinate, and its below-reference value does not invoke canopy freezing.
Ordinary/nontransparent below-freezing refusal and release-coordinate equality
remain unchanged. Interception capacity/intercepted water, beginning/ending
canopy stores, vapor exchange and both drainage terms must be exactly zero.

The private typed boundary retains complete validated forcing and destination,
transaction/support/configuration joins. Each positive-input transparent ledger
must carry explicit typed source proof bound to that immutable boundary and its
actual occupancy/rank. Required proof presence is rederived even for warm liquid;
omission cannot downgrade to ordinary admission. Dry columns carry no mixture
proof. The proof describes top input M/H/raw mixture T; each rank's actual mass
must independently match predecessor throughfall after unchanged stemflow splits.
Potential and fixed-final projection validate this same source context and their
respective pass. A global policy flag, zero ending store or self-sealed proof is
insufficient. Persisted replay rebinds proof to original beginning eligibility,
forcing/configuration/support and preserves both temperature roles exactly.
No nonlinear tolerance, component closure or custody guard is loosened.

Required checks are zero-area rain with a temperature distinct from canopy
and ground, unequal-temperature positive parcel mixing, two occupancy ranks
with nonzero stemflow, no-rain absence, positive beginning-store/area exclusion,
missing/foreign/source/support/enthalpy/rank proof poisons, ordinary cold refusal,
numerical-anchor/Jacobian parity, warm/cold and mixed-temperature passage,
authentic rain-plus-snow provider reconstruction, two-support cold continuation
and restored second-support parity for both temperature roles,
actual accepted pre-mix donor and postmix recipient closure, and repeated
workflow/rollback/restart evidence. Acceptance observations must continue to
report canopy area and liquid inventory on every admitted support; a reached
nontransparent canopy is outside this limited repair and must receive its own
energy assessment before whole-cycle closure is claimed. Production,
integration and release remain HOLD.

Binding exposure: EXP-B01-TRANSPARENT-CANOPY-LIQUID-20260910, active only in the
isolated authorized experiment; maps to existing INV-LANDSURFACEENERGY-105/107
liquid custody and SC-VEGETATION-001 INV-075/114. Typed LSE domain, identity,
component-closure and complete-owner rollback guards enforce this amendment.
Change log: 2026-09-10, prospective experimental transparent-column liquid
enthalpy identity; no new material constant or active-canopy process.

Change log: 2026-09-10, supersede transparent wet-coordinate/source-temperature
identity with independently source-bound passage and unchanged numerical anchor.
Actual090 cold provider reconstruction establishes residual mixed-phase liquid;
this exception adds no canopy freezing or phase-repartition process.
