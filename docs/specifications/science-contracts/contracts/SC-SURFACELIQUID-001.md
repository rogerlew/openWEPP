---
contract_id: SC-SURFACELIQUID-001
title: Persistent Snow-Free Surface-Liquid Hydrology Custody Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology/land-surface-energy reviewer
contract_version: 8
producer_scope:
  - Persistent snow-free bare-surface and forest-litter liquid hydrology state
  - Same-snapshot withdrawal authorization and finalized debit
  - Signed condensation credit and actual WB14 post-solve ingress routing
consumer_scope:
  - OPENWEPP_SNOW_FREE_LSE_V1 runtime shadow
  - Production WB14 infiltration/runoff and routed-runon owners
  - Restart and atomic shadow-state consumers
evidence_level: static+contract_vectors
last_reviewed: pending
supersedes: []
superseded_by: []
---

# SC-SURFACELIQUID-001 Persistent Snow-Free Surface-Liquid Hydrology Custody Contract

Status: `in_review` (version 7 remains the released runtime baseline)
Maturity: `draft`
Evidence mode: `Static + contract vectors`

## Purpose And Scientific Scope

Define the exact persistent hydrology owner required by
`OPENWEPP_SNOW_FREE_LSE_V1` for snow-free bare-surface and forest-litter
liquid. This contract admits ownership, state, transaction, conservative
routing, and the exact adapter to the existing WB14 Green-Ampt producer. It
does not introduce an alternative evaporation, infiltration, or runoff law.

In scope are strict per-OFE/tile/surface/source state, restart identity, one
immutable beginning snapshot, proportional maximum authorization, finalized
debit, signed condensation credit, one chronological WB14 call per OFE per
accepted child inside one immutable half-hour parent,
post-infiltration capacity retention, overflow/runoff routing, parcel
mass/enthalpy joins, candidate isolation, and rollback.

Out of scope are snow and terminal snow, frozen/thawing surface liquid,
production selection or default changes, legacy PMET/ET replacement, output
publication, calibration, deployment, and cutover.

## Authority Anchors

| Anchor | Authority | Use | Evidence |
|---|---|---|---|
| `REF-SURFACELIQUID-LSE-OWNER` | `SC-LANDSURFACEENERGY-001` version 3 | Hydrology-only liquid-mass ownership; exact LSE ground-water identity; signed condensation; immutable-beginning transaction; enthalpy-bearing ingress. | `[DIRECT][Static]` |
| `REF-SURFACELIQUID-WATBAL-STAGE-B` | `SC-WATBAL-001#INV-WATBAL-101` | Hydrology-only candidate mutation and bounded Stage-B resource use. | `[DIRECT][Static]` |
| `REF-SURFACELIQUID-WB14` | `compute_wb14_infiltration_depression_with_profile`, `DirectWb14InfiltrationProducerInputs`, `SC-RUNOFFPART-001#INV-RUNOFFPART-031`, and `SC-WATBAL-001#INV-WATBAL-103` | One actual chronological nonlinear infiltration partition, timed local precipitation and additional supply, routed carry, and runoff custody. | `[DIRECT][Static + Ran]` |
| `REF-SURFACELIQUID-BINARY64` | Rust `f64` primitive semantics and IEEE-754 binary64 nonnegative bit ordering | Round-to-nearest proportional-row arithmetic and the bounded common-scale representability selection; no scientific tolerance or physics change. | `[DIRECT][Static + contract vectors]` |
| `REF-SURFACELIQUID-PHYSICAL` | Conservation of mass and energy at an owner boundary | Exact debit/credit, capacity overflow, proportional parcel splits, and cross-owner identities. | `[INFERENCE][Static + contract vectors]` |
| `REF-SURFACELIQUID-COUPLED-TIME` | `SC-COUPLEDTIME-001#INV-COUPLEDTIME-002/005/006/007/008/009/013/017/019` and the `openwepp-coupled-time` accepted parent/slab API | Exact half-open parent/child support, hard-boundary reduction, immutable accepted-slab identity, event chronology, owner adjacency, and atomic candidate publication. | `[DIRECT][Static + Ran]` |
| `REF-SURFACELIQUID-STAGE3-CADENCE` | `SC-SNOWENERGY-001#INV-SNOWENERGY-023/042` | Latest-state Stage-3 `1800/900/60 s` maximum-step selection and per-lane OFE-ground basis. | `[DIRECT][Static + Ran]` |

Package artifacts summarize implementation evidence but do not replace these
canonical authorities.

## Variables And Units

| Symbol | Units | Meaning |
|---|---|---|
| `k` | typed identity | `(run_id, ofe_id, tile_id, surface_id, surface_class, source_type, source_id)` |
| `f_t` | dimensionless | tile fraction of the owning OFE ground area |
| `W_0,k`, `W_1,k` | `kg H2O m^-2 tile-ground` | beginning and ending persistent liquid mass |
| `W_max,k` | `kg H2O m^-2 tile-ground` | finite store capacity |
| `D_i`, `A_i`, `F_i` | `kg H2O m^-2 OFE-ground interval` | request, maximum authorization, and finalized use |
| `R_i` | `kg H2O m^-2 OFE-ground interval` | raw binary64 proportional authorization before a joint representability correction |
| `c_k` | dimensionless binary64 | one common downward authorization scale for all requests sharing source key `k` |
| `C_i` | `kg H2O m^-2 OFE-ground interval` | accepted condensation credit |
| `A_o` | `m^2` | horizontal plan area of one OFE |
| `m_p` | `kg H2O m^-2 basis-OFE-ground` | one timed parcel amount keyed by `basis_ofe_id` |
| `T_p` | `K` | parcel temperature |
| `Q_p` | `J m^-2 basis-OFE-ground` | parcel sensible enthalpy relative to `T_ref` |
| `h_l(T)` | `J kg^-1` | `C_w*(T-T_ref)` |
| `Delta t_parent` | `s` | immutable `1800 s` WB14 parent interval and persistent cursor unit |
| `Delta t_proposed` | `s` | Stage-3 maximum-step proposal, exactly one of `1800`, `900`, or `60 s` |
| `Delta t_child` | `s` | exact positive coupled-time-selected child support, no greater than `Delta t_proposed` or the parent remainder |
| `surface_liquid.store_mass` | `kg H2O m^-2 tile-ground` | machine-readable registry symbol for `W` |
| `surface_liquid.store_capacity` | `kg H2O m^-2 tile-ground` | machine-readable registry symbol for `W_max` |
| `surface_liquid.resource_amount` | `kg H2O m^-2 OFE-ground interval` | machine-readable registry symbol for `D/A/F` |
| `surface_liquid.condensation_credit` | `kg H2O m^-2 OFE-ground interval` | machine-readable registry symbol for `C` |
| `surface_liquid.parcel_mass` | `kg H2O m^-2 basis-OFE-ground` | machine-readable registry symbol for `m_p` |
| `surface_liquid.parcel_temperature` | `K` | machine-readable registry symbol for `T_p` |
| `surface_liquid.parcel_enthalpy` | `J m^-2 basis-OFE-ground` | machine-readable registry symbol for `Q_p` |
| `surface_liquid.ofe_area` | `m^2` | machine-readable registry symbol for `A_o` |
| `surface_liquid.interval` | `s` | machine-readable registry symbol for `dt` |

`OFE-ground`, named `stand-ground` by the LSE DTO, is one OFE's horizontal
ground area and never the whole hillslope. Storage remains tile-ground.
`tile_to_ofe_ground_amount(x)=f_t*x` and
`ofe_ground_to_tile_amount(x)=x/f_t` are the only conversions, each applied
exactly once at the owner boundary.

## Algorithm State Surfaces

### Strict configuration

Configuration contains owner ID, configuration digest, run identity, ordered
OFE topology, an exact ordered production-owner binding for every OFE, and an
exact ordered set of records:

```text
SurfaceLiquidOfeBinding {
    ofe_id,
    production_lane_index,
    production_lane_id,
    ordered_soil_layer_ids,
    infiltration_soil_thermal_layer_id
}
```

```text
SurfaceLiquidConfigurationRecord {
    ofe_id,
    tile_id,
    surface_id,
    surface_class,
    source_type,
    source_id,
    tile_fraction,
    capacity_kg_m2_tile,
    ofe_area_m2,
    ground_ingress_mode,
    runon_destination_ofe_id,
    runon_destination_tile_id
}
```

Each OFE additionally has exactly one immutable WB14 identity, serialized in
OFE topology order:

```text
DirectWb14ImmutableIdentityV1 {
    schema_sha256,
    ofe_id_sha256,
    production_lane_id,
    surface_liquid_configuration_sha256,
    wb14_configuration_sha256,
    wb14_model_definition_sha256,
    effective_conductivity_m_s_bits,
    matric_potential_m_bits,
    storage_capacity_m_bits,
}
```

All digests are nonzero. Conductivity is finite and positive; matric potential
and storage capacity are finite and nonnegative. The canonical parent identity
uses `framed_sha256` domain `openwepp-wb14-parent-interval-v1`. Its tagged
fields, in order, are coupled-parent digest, day (`u64` big endian), interval
(`u8`), support endpoints (`u128` big endian), parent-beginning owner digest,
beginning-cursor digest, then every field of
`DirectWb14ImmutableIdentityV1` above. Digest fields are exactly 32 bytes,
`production_lane_id` is `u32` big endian, and parameter bits are `u64` big
endian. `ofe_id_sha256=SHA256(UTF-8 ofe_id)`;
`wb14_configuration_sha256=SHA256(serde_json(input.wb14_parameters))` using the
same topology-ordered DTO consumed by execution; `schema_sha256` hashes
`OPENWEPP_DIRECT_WB14_PARENT_WORKING_STATE_V2`; and the model digest hashes the
named unchanged Green--Ampt implementation identity. Every child repeats and
bitwise verifies this authority before physics; final reconstruction repeats
the same topology order.

The exact admitted pairs are:

| `surface_class` | `source_type` | Meaning |
|---|---|---|
| `bare_mineral_soil` | `surface_liquid` | Hydrology-owned ponded/depression liquid over bare soil. |
| `forest_litter` | `litter_liquid` | Hydrology-owned liquid held by forest litter. |

`soil_layer_liquid` remains the soil-layer owner and is not duplicated here.
The OFE binding at topology rank `r` must name production lane index `r`, its
exact lane ID, and the exact ordered soil-layer identities carried by the real
hydrology adapter. Its configured `A_o` must be bitwise equal to the production
lane area. `infiltration_soil_thermal_layer_id` names the actual top receiving
layer in the LSE soil-thermal candidate. Missing, extra, reordered, duplicate,
or wrong-lane bindings reject before snapshot construction.
`ground_ingress_mode` is exactly `open_raw_precipitation` or
`covered_canopy_release`. A bare or litter surface may use either mode according
to canopy topology; surface class does not infer exposure. The exact mode enters
configuration bytes/digest and the runtime must receive the matching ingress
variant for every tile exactly once.
Every `surface_id`, `source_id`, and `tile_id` must equal the corresponding
`GroundWaterKey` fields used by LSE. IDs are nonempty typed values. Keys are
unique. `0 < f_t <= 1`; `W_max > 0`; `A_o > 0`; all are finite. Every record
for one OFE carries the same bitwise-identical area. For each OFE, tile
fractions close to one when
`abs(sum(f_t)-1) <= 64*epsilon*max(abs(sum(f_t)),1)`.

The OFE topology is a strict acyclic upstream-to-downstream order. A routed
destination, when present, has a greater topology index. The final OFE has no
destination. A routed destination names one configured receiving tile. No
cycle, backward edge, missing destination, fan-out, or unresolved parcel is
admitted. No executable `Default` supplies a record or scientific value.

Canonical configuration bytes are deterministic JSON of a
`deny_unknown_fields` structure with records sorted by the complete key and
OFE topology order. Strings are UTF-8 JSON strings; enums use the exact
snake-case tokens above; integers use canonical decimal form; finite `f64`
values are encoded by their 16-character lowercase big-endian IEEE-754 hex
bits. `configuration_sha256` is lowercase SHA-256 over the UTF-8 bytes after
replacing only its own value with 64 zeroes. Owner, run, topology,
destinations, all identities, production lane and layer bindings, fractions,
capacities, OFE areas, and ingress modes enter the
digest.

### Persistent state and restart

```text
SurfaceLiquidStateRecord {
    key,
    liquid_kg_m2_tile,
    last_accepted_transaction_id
}
SurfaceLiquidOwnedState {
    owner_id,
    configuration_sha256,
    state_sha256,
    records,
    continuations
}
```

State has one record for every configured key and no other record. Every `W`
is finite and `0 <= W <= W_max`. Initial state has explicit caller-supplied
mass and `last_accepted_transaction_id=None` on every record. An accepted
state has the same `Some(predecessor_transaction_id)` on every record. A new
transaction is nonzero, differs from its predecessor, and supplies the exact
expected predecessor; initial execution requires `expected_predecessor=None`.
The candidate sets every record to `Some(new_transaction_id)`.

Canonical state bytes use the configuration encoding rules, sort records by
complete key, and include owner/configuration identity, every key, mass, and
transaction lineage. Only `state_sha256` is replaced by 64 zeroes for its own
digest. Unknown, missing, extra, duplicate, stale, mixed-lineage, nonfinite,
or wrong-capacity states reject. Parse, serialize, digest, and parse round-trip
exactly.

`continuations` is an exact map keyed by `ofe_id`, with one entry for every
configured OFE and no other entry:

```text
DirectWb14ContinuationState {
    ofe_id,
    day_index,
    next_interval_index,
    cumulative_supply_m,
    cumulative_infiltration_m,
    last_accepted_transaction_id
}
```

Continuation records serialize after store records in ascending OFE topology
order. Every field enters `state_sha256`. All entries carry the same accepted
transaction lineage as store records. Initial state uses a caller-supplied
`day_index`, exact interval zero, zero cumulatives, and null lineage. Accepted
state uses interval `1..=48` and nonnull lineage. These combinations are strict
state-validation rules, not deferred execution checks. At a new-day
transaction, only a beginning entry
at interval 48 may increment the day and reset candidate interval/cumulatives
before advancing interval zero. Any failure preserves the prior boundary bytes.

Residue interception inputs, `residue_interception_after_m`, WB14
`depression_storage_delta_m`, WAT5 retention, snow liquid, soil-layer liquid,
and LSE surface enthalpy are not aliases of this state.

### Transaction inputs and outputs

Inputs are one immutable owner state, exact parent transaction/predecessor and
support, typed LSE requests, authorizations, finalized uses, condensation
credits, timed current-ingress parcels, the accepted coupled child support and
receipt, the latest Stage-3 upper-bound proposal, and the actual immutable WB14
production inputs for each OFE.

Outputs are the one authorization batch, one uncommitted ending owner
candidate, actual production soil-liquid candidate credit, typed soil-thermal
infiltration candidate receipt, retained LSE tile-state candidate credit,
retained/infiltration/routed/outlet receipts, independent ledger operands,
state digest, and rollback hashes. No function in this contract independently
commits production state.

The runtime day is exactly 48 consecutive `Delta t_parent=1800 s` parent
transactions. Each parent contains one or more contiguous accepted children.
`Delta t_proposed` is selected from `{1800,900,60} s` using the latest accepted
Stage-3 state; coupled time selects exact `Delta t_child`, which may be shorter
at an event, restart, output, or parent endpoint and must not exceed the
proposal. Zero duration is an event transaction, never a physics child.
The hydrology owner carries the day-local continuation above. It initializes
to exact zero only at the start of a new direct-runtime day,
advances exactly once per finalized parent interval, and is included in
restart bytes, state digest, transaction lineage, rollback, and ending state.
Children advance only parent-local slab lineage and WB14 cumulative working
state. The next parent interval must equal the retained index. No parent or
child may be replayed, skipped, reordered, or evaluated with a reset cumulative
infiltration state.

### Exact relationship to legacy depression retention

The persistent `bare_mineral_soil/surface_liquid` record is the only
depression/ponded mass in the native default-off shadow. It replaces, rather
than augments, WB14's same-pass `depression_storage_delta_m`. The shadow calls
WB14 with `depression_storage_capacity_m=0`; the returned depression delta and
every WAT5 depression-retention bin must be exact zero. Persistent retention
is applied only to WB14's post-infiltration excess as specified below.

Production dispatch remains unchanged and may continue using its legacy
nonpersistent depression branch. No production state is migrated or mutated
by this shadow. A future cutover must separately migrate or initialize the
persistent store and suppress the legacy branch; this contract does not
authorize that cutover.

## Algorithm Specification

### 1. Validate identity and freeze the beginning snapshot

Validate strict configuration/state bytes and digests, exact key set,
owner/run/OFE/tile/surface/source identity, topology, persistent parent lineage,
the exact `Delta t_parent=1800 s` support and daily continuation index, selected
upper bound, exact accepted child support/receipt, child ordinal and receipt
predecessor, finite domains, units, and admitted branch before calculation.
The effective beginning owner is the latest accepted parent-local complete
candidate, or the persistent beginning owner for child zero. Freeze its `W_0`.
The snapshot precedes current precipitation, runon, throughfall,
drainage, stemflow, infiltration, and runoff. Those masses cannot satisfy
same-interval withdrawal.

### 2. Authorize beginning-store withdrawals once

For one source key:

```text
S_k = f_t * W_0,k
D_sum,k = checked_sum_in_complete_key_order(D_i)
A_i = D_i                         when D_sum,k <= S_k
R_i = fl(fl(D_i * S_k) / D_sum,k) otherwise
```

The multiplication, division, and sum above are finite checked IEEE-754
binary64 operations. In the oversubscribed branch, raw rows are evaluated and
summed in complete request-key order. If `R_sum,k=sum_i(R_i) <= S_k`, then
`A_i=R_i` bit-for-bit.

Binary64 rounding can instead produce a finite `R_sum,k>S_k` even though every
row is the admitted proportional formula. That representability-only case is
admitted only when the excess satisfies the mass-closure envelope:

```text
R_sum,k - S_k
    <= 1e-14 kg m^-2
       + 64*epsilon*(abs(R_sum,k)+abs(S_k)).
```

Then compute `c_0=fl(S_k/R_sum,k)`. If the checked canonical sum of
`fl(R_i*c_0)` does not exceed `S_k`, select `c_k=c_0`. Otherwise select the
greatest positive finite binary64 `c_k<=c_0` whose checked canonical sum of
`fl(R_i*c_k)` does not exceed `S_k`. Selection is a monotone bisection over the
ordered nonnegative binary64 bit interval from exact zero through `c_0` and
terminates after at most 64 bit decisions. Final authorization is:

```text
A_i = fl(R_i*c_k) for every row sharing k.
```

Every positive `R_i` must remain positive after the common scale. Failure of
any finite operation, an overshoot outside the stated mass envelope, absence
of a positive jointly safe scale, or failure to establish the bound within 64
bit decisions is `SURFACELIQUID-E-003`. This is one symmetric common scaling,
not a per-key priority: no canonical-last remainder, largest-row repair,
request-order repair, or row-specific next-down operation is admitted. Caller
order therefore cannot change any request-key authorization bits. The rule is
a bounded binary64 representability rule only; it does not loosen `A<=D`,
`F<=A`, owner closure, or any physical acceptance tolerance.

Requests preserve the complete `GroundWaterKey` and group only by exact
`(OFE,source_tile,source_type,source_id)`. Zero demand or supply yields exact
zero. No request inflation, priority, borrowing, current-ingress credit,
second authorization, or fallback is admitted.

### 3. Validate finalized use and signed condensation credit

Every finalized row has the exact request/authorization identity and satisfies
`0 <= F_i <= A_i <= D_i`. Debit only finalized use. A condensation credit is
a positive amount produced only by an accepted negative LSE vapor flux and
uses the existing DTO basis `kg_h2o_m-2_stand_ground_interval`, meaning
OFE-ground here. It carries exact transaction, hydrology owner, OFE, tile,
surface, temperature, and specific-liquid-enthalpy identity. The configuration
maps it uniquely to the tile's admitted source key.

For each store, finalized uses are summed with finite checked binary64
addition in complete `GroundWaterKey` order before the one tile-basis debit.
Caller slice order cannot change the ending store bits. Candidate construction
and independent reconstruction each derive this canonical sum from the typed
finalized-use rows; neither consumes a producer-supplied debit total.

```text
W_pre,raw,k = W_0,k - sum_i(F_i/f_t) + sum_i(C_i/f_t)
W_pre,k = min(W_max,k, W_pre,raw,k)
m_cond_overflow,k = f_t * max(0, W_pre,raw,k-W_max,k)
```

Unused authorization remains in storage. Condensation overflow is credited
before emission, never clipped or discarded, and becomes an OFE-ground timed
additional-supply parcel with `Q=m*h_l(T)`. It cannot re-enter same-interval
authorization.

### 4. Construct exact tile-weighted ground ingress

For each accepted child support, precipitation has exactly one ground recipient:

```text
P_ground,o = sum_open_tiles(f_t * P_raw,t)
             + sum_covered_tiles(f_t * R_canopy,t)
```

An open tile has no vegetation occupancy and contributes its tile-local raw
precipitation. A covered tile contributes only the accepted V8 sum of
throughfall, initial drainage, second drainage, and stemflow. It contributes
no raw precipitation because those releases already derive from that forcing.
Each tile is exactly open or covered. Tile fractions enter once. The original
unmodified precipitation remains available separately to the erosion-rainfall
owner; it is not a second infiltration supply.

Add upstream runon and condensation overflow to this ground-ingress set.
Every parcel retains exact `[start_s,end_s)` support within the current child.
Canopy and condensation amounts are uniform on that support because the
accepted LSE/V8 outputs are child amounts. Upstream
runon preserves the producing support.

### 5. Advance one stateful production WB14 continuation per OFE

Process OFEs once in strict topology order within each accepted child.
An OFE is eligible only after every upstream OFE has inserted its routed
parcel for that child. For exactly one continuation call per OFE per child:

1. slice all tile-weighted ground-ingress parcels to the current exact support;
2. convert `kg m^-2 OFE-ground` to metres once with
   `ofe_ground_water_mass_to_depth_m(x)=x/rho_w`;
3. partition the interval at every source boundary and apply the existing
   `compute_green_ampt_interval_infiltration` transition chronologically;
4. enter the transition with retained cumulative supply and infiltration;
5. preserve production conductivity, matric potential, and infiltration
   storage capacity;
6. return interval infiltration/excess plus the advanced continuation;
7. apply interval infiltration once with the same production
   `apply_same_pass_infiltration` transition and exact production tillage depth
   to the bound candidate lane, never directly to the surface store; and
8. independently reconstruct the resulting ordered soil-layer mass deltas.
9. retain no legacy depression amount: the equivalent
   `depression_storage_capacity_m` is exact zero.

The shared production transition is extracted from
`compute_wb14_infiltration_depression_with_profile`; the existing daily WB14
wrapper and the shadow continuation must call that same function. Extraction
requires parity vectors proving the legacy daily wrapper is unchanged. The
runtime calls it once per OFE per accepted child, never once per source
parcel and never once with a replayed full-day hyetograph. At interval 48 the
continuation is the direct day's accepted cumulative result; only the next
day may reset it to zero. No proportional infiltration proxy, scalar daily
partition, or copied Green-Ampt equation is admitted.

### 6. Conservatively mix and partition the actual WB14 result

For each exact chronological subinterval `b`, mix all source parcels before
partition. Let `x_p,b` and `q_p,b` be their OFE-ground mass and enthalpy,
`X_b=sum_p(x_p,b)`, and `Q_b=sum_p(q_p,b)`. The actual stateful WB14 transition
supplies infiltration `I_b` and post-infiltration excess `E_b` with
`I_b+E_b=X_b`.

```text
h_mix,b = 0                        when X_b = 0
h_mix,b = Q_b / X_b                otherwise
Q_infiltration,b = I_b * h_mix,b
Q_excess,b = Q_b - Q_infiltration,b
```

This is an exact conservative well-mixed OFE supply selection. Source
provenance is retained: `I_p=(x_p/X_b)*I_b` and `E_p=x_p-I_p`; the final source
in canonical identity order receives floating remainders. Every attributed
parcel uses `h_mix,b`, so source kind cannot select which temperature
infiltrates or remains. Each `E_p` retains its exact destination tile/source
custody. The rule does not alter cumulative infiltration or rerun WB14.

For a split fraction `r=m_child/m_parent`, with `m_parent>0`:

```text
Q_child = r * Q_parent
Q_remainder = Q_parent - Q_child
```

Both children retain the parent's temperature and `T_ref`. Zero-mass parcels
carry exact zero enthalpy. Temperature and enthalpy are never independently
reconstructed from a different source.

### 7. Retain mixed post-infiltration excess and route overflow

For one exact tile/source key `k` and subinterval, sum only attributed excess
whose exact destination is `k`: `E_k=sum_p(E_p,k)`. Compute remaining
OFE-ground capacity `R_k=f_t*(W_max-W)`. Retention is:

```text
m_retained,k = min(E_k, R_k)
m_runoff,k = E_k - m_retained,k
```

Within a key, provenance is split proportionally and the final parcel receives
floating remainders. Every receipt uses `h_mix,b`; source order cannot select
temperature. No excess crosses tile or source identity and no lateral
redistribution is admitted. `W_k` increases by `m_retained,k/f_t`. The retained
LSE energy receipt is
`Q_retained,k,tile = (m_retained,k*h_mix,b)/f_t` and is added exactly once to
the same tile's `surface_enthalpy_j_m2_tile`. The runoff enthalpy is
`Q_runoff,k=m_runoff,k*h_mix,b`. No tile exceeds capacity and no arbitrary
source-kind priority changes surface temperature.

Every parcel carries a `basis_ofe_id` distinct from immutable origin/source
lineage. Before routing it equals source OFE `u`. Routing to destination `d`
creates a recipient parcel with `basis_ofe_id=d` while preserving origin/source
IDs. The named conversion is:

```text
m_runon,d = m_runoff,u * A_u/A_d
Q_runon,d = Q_runoff,u * A_u/A_d.
```

`route_ofe_ground_amount` applies this area ratio exactly once to mass and
enthalpy; its inverse is never applied in the same route. This preserves
absolute mass and energy for
unequal OFE areas. The parcel is inserted before downstream eligibility.
At each topology rank the child receipt seals canonical ordered
`pending_routed_parcels_before_sha256` and
`pending_routed_parcels_after_sha256`. Their preimages contain the ordered OFE
queue keys and, for each parcel, parcel ID, origin and recipient OFE/tile,
basis OFE, kind, support bits, mass bits, and enthalpy bits. The enclosing
scalar child receipt seals both queue digests together with its predecessor
receipt. Reconstruction requires the prior rank's `after` digest to equal the
next rank's `before` digest and proves that every newly routed parcel is
present before downstream WB14 eligibility.
Final-OFE runoff is outlet runoff. Strictly increasing topology indices make
the queue finite; every OFE executes once per child and every
routed parcel terminates downstream or at the outlet.

### 8. Validate ending state and owner joins

For every key independently reconstruct:

```text
W_1 = W_0 - sum(F/f_t) + sum(C/f_t)
      - condensation_overflow/f_t + retained_excess/f_t.
```

For every source parcel independently reconstruct mass and enthalpy across
infiltration, retention, routed runoff, and outlet runoff. Infiltration credits
the exact bound production lane through the shared same-pass transition. Its
enthalpy credits the exact named soil-thermal layer candidate. Retained
enthalpy credits the exact LSE tile state after one OFE-to-tile conversion.
Recompute the strict state, parent-local WB14 continuation, ordered per-OFE
child-receipt-set digest, and complete beginning/ending owner-set digests. Join
surface, hydrology, soil, soil-thermal, LSE, V11, Stage 3, and coupled-clock
candidate identities. Intermediate children update only the complete
parent-local candidate and child receipt chain. The final child additionally
seals the ordered per-OFE parent-receipt set and projects one persistent
transaction lineage and cursor advance. All operations use clones. No fallible
operation may follow the later all-owner atomic replacement.

The child owner manifest is ordered exactly as `surface`, `hydrology`, `soil`,
`soil_thermal`, `lse`, `vegetation_v11`, `snow_stage3`, and `coupled_clock`.
For each entry the receipt contains length-prefixed canonical beginning and
ending bytes and their SHA-256 digests; an inactive owner requires byte-equal
beginning and ending values. Length-prefixed provider cursor, GSI cursor,
pending publication, event ordinal, and child-chain bytes follow the owner
manifest. Thus:

```text
child[n].beginning_complete_owner_set_sha256
    = child[n-1].ending_complete_owner_set_sha256
child[n].predecessor_receipt_sha256 = child[n-1].receipt_sha256
```

Restart serializes the parent authority, accepted child cursor, next ordinal,
ordered per-OFE scalar authorities, current complete candidate bytes, auxiliary
cursor/event bytes, and ordered receipts. Finalization validates this wire,
constructs the complete persistent ending candidate, then performs one
infallible all-owner/cursor replacement. Failure before that replacement,
including after final owner join, preserves every persistent and parent-local
beginning byte.

## Branch And Guard Table

| Order | Trigger | Guard class | Required behavior | Typed error |
|---|---|---|---|---|
| 1 | malformed or unknown/missing field | schema | Reject before identity projection. | `SURFACELIQUID-E-001` |
| 2 | owner/configuration/state/transaction/key mismatch | identity | Reject exact identity or lineage. | `SURFACELIQUID-E-002` |
| 3 | nonfinite/out-of-domain capacity, fraction, mass, interval, temperature, topology, or unsafe proportional representability | domain | Reject without normalization except the exact symmetric joint-authorization rule in section 2. | `SURFACELIQUID-E-003` |
| 4 | snow, terminal snow, frozen, or thawing surface branch | unsupported domain | Reject before candidate work, except the exact typed terminal parcel into an already-authorized actual frozen/thawing receiver under INV-010/011. | `SURFACELIQUID-E-004` |
| 5 | duplicate/missing request, authorization, use, credit, or parcel | protocol cardinality | Reject complete protocol. | `SURFACELIQUID-E-005` |
| 6 | `F>A`, `A>D`, negative amount, or wrong basis | resource bound | Reject; no tolerance repairs it. | `SURFACELIQUID-E-006` |
| 7 | legacy depression retention nonzero in native shadow | exact-one owner | Reject duplicate storage custody. | `SURFACELIQUID-E-007` |
| 8 | wrong parent support, inadmissible proposal, zero/oversized/noncontiguous child support, coupled-receipt mismatch, continuation carry, or more/fewer than one WB14 call per OFE/child | production-producer binding | Reject reset, replay, proxy, incomplete partition, or early persistent-cursor advance. A tagged INV-011 terminal segment and an INV-012 child each consume their exact accepted support. | `SURFACELIQUID-E-008` |
| 9 | capacity, attribution, routing, or parcel enthalpy mismatch | candidate closure | Reject candidate. | `SURFACELIQUID-E-009` |
| 10 | local/owner/soil join closure failure | independent closure | Reject candidate. | `SURFACELIQUID-E-010` |
| 11 | rollback or complete-owner mismatch | atomic envelope | Reject envelope. | `SURFACELIQUID-E-011` |
| 12 | OFE/lane/configuration/model/parameter/ordinal/support/predecessor or scalar-receipt substitution | parent/child receipt authority | Reject before accepting the child; discard the complete candidate. | `SURFACELIQUID-E-002` / `SURFACELIQUID-E-011` |
| 13 | incomplete parent, child-receipt reconstruction failure, parent-receipt mismatch, or persistent cursor advancement before finalization | parent finalization | Reject publication and preserve all persistent owners and cursors byte-for-byte. | `SURFACELIQUID-E-011` |

Errors use this precedence. Every public failure exposes its exact
`SURFACELIQUID-E-001..011` code, phase, transaction when available, OFE, tile,
surface/source and parcel when applicable, plus beginning and attempted owner
hashes. An unavailable identity is represented by typed absence, not an empty
string. A generic category plus prose detail is not the canonical payload.

## Invariants And Invariant Guard Map

| Invariant | Binding rule | Authority | Enforcement path | Guard/failure | Evidence |
|---|---|---|---|---|---|
| `INV-SURFACELIQUID-001` | One persistent mass for every exact LSE bare-surface/litter source key; no adjacent value aliases it. | LSE ownership + physical conservation | strict config/state validator | identity/domain; `E-001..004` | `[DIRECT][Static + Ran]` schema/digest vectors + alias poisons |
| `INV-SURFACELIQUID-002` | Restart bytes, digest, key set, predecessor lineage, and WB14 day continuation round-trip exactly. | correctness authority model | parser/serializer/restart | schema/identity; `E-001..003` | `[DIRECT][Static + Ran]` field mutation, cadence, and restart vectors |
| `INV-SURFACELIQUID-003` | One immutable beginning snapshot supplies one proportional authorization; a representational aggregate overshoot may use only the common, symmetric, bounded binary64 scale in section 2. | LSE transaction + WATBAL Stage B + IEEE-754 representability under physical conservation | resource arbiter | arithmetic/cardinality/bound; `E-003,E-005..006` | `[DIRECT][Static + Ran] + [INFERENCE][Static]` zero/full/partial/competition, joint-supply and order-reversal vectors |
| `INV-SURFACELIQUID-004` | Exact identity and `0<=F<=A<=D`; aggregate finalized use in complete key order, debit it once, and credit condensation once. | LSE water protocol | candidate protocol validator | arithmetic/identity/bound; `E-003,E-005..006` | `[DIRECT][Static + Ran]` D/A/F, caller-order and condensation vectors |
| `INV-SURFACELIQUID-005` | Persistent ponding replaces the native shadow's legacy depression retention. | exact-one ownership | WB14 input/profile validator | duplicate owner; `E-007` | `[DIRECT][Static + Ran]` zero-capacity and nonzero-delta poison |
| `INV-SURFACELIQUID-006` | Each OFE/accepted child uses one actual stateful shared WB14 transition; open raw rain and covered canopy release are mutually exclusive ground supplies. | WB14 production path + V8 canopy ownership | receipt-owned parent coordinator | producer binding; `E-008` | `[DIRECT][Static + Ran]` parent/child cadence-state, no-duplication, 48-parent daily parity, and short-child attachment vectors |
| `INV-SURFACELIQUID-007` | Mixed post-infiltration excess retains exact tile/source custody; remainder routes once with basis re-keying and OFE-area conversion. | runoff/routing authority + conservative mixing | retention/routing candidate | closure/topology; `E-009` | `[DIRECT][Static + Ran] + [INFERENCE][Static]` multi-temperature, multi-tile, unequal-area multi-OFE vectors |
| `INV-SURFACELIQUID-008` | Mass, enthalpy, infiltration, storage, and runoff are independently reconstructed without producer residuals. | physical conservation | external ledger validators | closure; `E-009..010` | `[INFERENCE][Static + Ran]` independent numerical vectors |
| `INV-SURFACELIQUID-009` | All work is candidate-only and every failure preserves complete beginning and production bytes. | transaction atomicity | shadow owner envelope | rollback; `E-011` | `[DIRECT][Static + Ran]` phase-injection hashes |
| `INV-SURFACELIQUID-010` | One fingerprinted 0 C parcel equals retained snow liquid plus snow-support rain plus melt less refreeze; atomic snow debit, surface credit, and consumed marker prevent replay. | snow/physical conservation | terminal receipt validator | identity/cardinality/closure; `E-003,E-005,E-011` | `[DIRECT][Static + Ran] + [INFERENCE][Static]` numeric equation, replay, alias, rollback vectors |
| `INV-SURFACELIQUID-011` | A tagged remaining segment calls the actual shared Green-Ampt/Mein-Larsen transition over exact half-open wall support and advances base-bin continuation only at its endpoint. | WB14 production path | direct-runtime adapter | cadence/support; `E-008` | `[DIRECT][Static + Ran]` nonlinear segment, endpoint, ponding, restart vectors |
| `INV-SURFACELIQUID-012` | One immutable `1800 s` WB14 parent contains contiguous positive children accepted at or below latest-state proposals of `1800/900/60 s`; children do not advance the persistent cursor and complete finalization advances it exactly once. | `REF-SURFACELIQUID-COUPLED-TIME`, `REF-SURFACELIQUID-STAGE3-CADENCE`, WB14 chronology | receipt-owned multi-OFE parent coordinator and finalizer | cadence/support/finalization; `E-008,E-011` | `[DIRECT][Static + Ran]` 1x1800, 2x900, 30x60, dynamic proposal, truncation, one-cursor vectors |
| `INV-SURFACELIQUID-013` | Every child binds coupled support, immutable OFE/lane/configuration/model/parameter identity, exact Green-Ampt inputs and working progression, complete beginning/ending owner sets, and a reconstructable predecessor receipt chain. | `REF-SURFACELIQUID-COUPLED-TIME` + correctness authority model | child and parent receipt `validate()` replay | identity/cardinality/replay; `E-002,E-005,E-011` | `[DIRECT][Static + Ran]` substitution, omission, reorder, replay, restart, and receipt-byte poisons |
| `INV-SURFACELIQUID-014` | The complete parent candidate processes all OFEs in topology order and atomically stages surface storage, attributed liquid/enthalpy, routing, soil, soil thermal, LSE, V11, Stage 3, clock, provider/GSI, event, and receipt owners. | physical conservation + `REF-SURFACELIQUID-COUPLED-TIME` | complete-owner coordinator and covered owner join | closure/rollback; `E-009..011` | `[DIRECT][Static + Ran] + [INFERENCE][Static]` complete parity, short-child two-OFE routing, child/final-join rollback vectors |

## Producer Obligations

| Producer | Required operands and behavior | Prohibited substitution |
|---|---|---|
| LSE | Exact `GroundWaterKey` request/use and `CondensationCredit` with OFE-ground basis, surface/source identity, accepted temperature, and enthalpy. | Negative request, authorization-as-use, tile-basis credit, clipped condensation. |
| Vegetation/forcing/upstream OFE | Timed, typed ingress parcels with exact OFE/tile/source and mass/enthalpy identity. | Untimed daily scalar, wrong destination, air-temperature enthalpy fallback. |
| Hydrology configuration/state | Strict complete persistent store, capacities, topology, predecessor lineage, and digest. | Residue, WAT5, snow, soil-layer, or legacy depression-delta alias. |
| Coupled Stage-3 controller | Select `Delta t_proposed` from the latest accepted Stage-3 state and supply the exact coupled accepted-slab receipt/support. | Infer proposal from elapsed duration, reuse beginning Stage-3 state, or treat a zero-time event as physics. |
| WB14 parent coordinator | One sealed scalar authority per configured OFE, one actual shared-kernel call per OFE/child in topology order, immutable parameter/model identity, ordered child receipts, final parent receipts, retained day carry, zero legacy depression capacity, and exact ground-ingress records. | Independent per-OFE publication, full-day replay, per-parcel Green-Ampt, copied formula, proportional infiltration proxy, raw-rain plus canopy duplication. |

## Consumer Obligations

| Consumer | Required validation/use | Prohibited behavior |
|---|---|---|
| LSE fixed-cap solve | Consume exact authorization once and emit exact finalized use/credit. | Reauthorize or treat ingress as available supply. |
| Soil liquid/thermal owners | Independently receive attributed infiltration mass and enthalpy. | Accept producer residual or unmatched energy. |
| Routed hydrology | Preserve timing/source/destination and insert only before a later topology lane. | Cycle, backward route, scalar carry, duplicate debit. |
| Shadow orchestrator/restart | Validate the complete candidate and replace the whole shadow state only after all joins pass. | Partial owner commit, production mutation, synthesized state. |
| Coupled owner join | Bind exact beginning/ending complete-owner sets, coupled slab, ordered WB14 child receipts, and final-only parent receipts/cursor transition. | Digest-only trust, inactive-owner mutation, omitted receipt, or child-local persistent lineage. |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Invalid aliases |
|---|---|---|---|---|
| `k` | persistent key projected from `GroundWaterKey` | owner identity | exact field equality | soil layer, snow layer, residue ET scalar |
| `W` | `surface_liquid.liquid_kg_m2_tile` | persistent state | `kg H2O m^-2 tile-ground` | residue interception, WB14 depression delta, WAT5 retention |
| `W_max` | `surface_liquid.capacity_kg_m2_tile` | configuration | `kg H2O m^-2 tile-ground` | soil porosity, depression delta |
| `D/A/F` | `surface_liquid.amount_kg_m2_ofe_ground` | LSE protocol | `kg H2O m^-2 OFE-ground interval` | authorization-as-use, daily ET scalar |
| `C` | `surface_liquid.condensation_kg_m2_ofe_ground` | LSE protocol | `kg H2O m^-2 OFE-ground interval` | negative request, tile-ground credit |
| `A_o` | `surface_liquid.ofe_area_m2` | routing identity | `m^2` positive finite | lane index or unscaled depth |
| `m_p` | `surface_liquid.parcel_mass_kg_m2_basis_ofe` | WB14/routing | `kg H2O m^-2 basis-OFE-ground` | scalar runoff or un-rekeyed destination depth |
| `T_p` | `surface_liquid.parcel_temperature_k` | WB14/routing | `K` | Celsius or untyped temperature |
| `Q_p` | `surface_liquid.parcel_enthalpy_j_m2_basis_ofe` | WB14/routing | `J m^-2 basis-OFE-ground` | power rate or un-rekeyed destination energy |
| `Delta t_parent` | `DirectWb14ParentAuthorityV1.parent_support` | persistent cadence | exact `1800 s` | child duration or daily scalar |
| `Delta t_proposed` | `selected_upper_bound_s_bits` | Stage-3 proposal | exact `s` bits in `{1800,900,60}` | accepted duration inferred as proposal |
| `Delta t_child` | coupled slab `TimeSupport` / `accepted_duration_s_bits` | child physics | exact positive `s` support | zero-time event or proposed maximum |
| `K_eff` | `DirectOfeWb14Parameters.effective_conductivity_m_s` / identity bits | WB14 parent and child | `m s^-1`, positive finite, bitwise frozen | another OFE or child value |
| `psi` | `DirectOfeWb14Parameters.matric_potential_m` / identity bits | WB14 parent and child | `m`, finite nonnegative, bitwise frozen | pressure head in another unit |
| `S_f` | `DirectOfeWb14Parameters.infiltration_storage_capacity_m` / identity bits | WB14 parent and child | `m`, finite nonnegative, bitwise frozen | surface store capacity |
| WB14 configuration/model identity | `wb14_configuration_sha256` / `wb14_model_definition_sha256` | parent authority and receipts | exact nonzero digest equality | Rust type/domain-tag inference |
| OFE/lane identity | `ofe_id` / `production_lane_id` | topology-ranked authority | exact configured binding | scalar broadcast or lane index substitution |
| `wall_t*,wall_end,d` | terminal receiver absolute support | wall-time identity | `s` plus calendar/bin identity | transaction ID, full-bin duration, proportional scale |
| `m_terminal_liquid` | terminal receipt mass | exact-one ingress | `kg H2O m^-2 OFE-ground` | store level/change, runoff, rain-only, CoE melt |

## Constants And Parameters

| Symbol | Value | Authority |
|---|---|---|
| `T_ref` | `273.15 K` | `SC-LANDSURFACEENERGY-001` version 3 |
| `C_w` | `4218 J kg^-1 K^-1` | `SC-LANDSURFACEENERGY-001` version 3 |
| `rho_w` | `1000 kg m^-3` | `SC-LANDSURFACEENERGY-001` version 3 water-mass conversion |
| WB14 parent interval | `1800 s` | admitted direct-runtime cadence and persistent cursor unit |
| Stage-3 upper-bound proposals | `{1800,900,60} s` | coupled Stage-3 cadence authority; maximum step, not guaranteed accepted duration |
| intervals per direct day | `48` | `86400 s / 1800 s` |
| topology tolerance | `64*epsilon*max(abs(sum(f_t)),1)` | Exact LSE runtime configuration rule |
| mass closure absolute term | `1e-14 kg m^-2` | Existing vegetation/owner closure convention |
| enthalpy closure absolute term | `1e-9 J m^-2` | LSE component-ledger convention |
| joint authorization search bound | `64 binary64 bit decisions` | Complete monotone bisection over one nonnegative IEEE-754 binary64 scale; representability only |

Capacity and tile fraction are explicit site configuration. No universal
capacity, inferred capacity, or executable default is admitted.

## Unit-Governance Map

| Boundary symbol | Canonical unit/basis | Registry posture | Runtime representation | Conversion/output rule |
|---|---|---|---|---|
| `W,W_max` | `kg H2O m^-2 tile-ground` | registry symbols `surface_liquid.store_mass` and `surface_liquid.store_capacity` | typed field, scalar only inside owner module | never published; no raw metre alias |
| `D,A,F,C` | `kg H2O m^-2 OFE-ground interval` | registry symbols `surface_liquid.resource_amount` and `surface_liquid.condensation_credit` plus LSE `StandGroundWaterAmountBasis` | `WaterAmount`/`WaterAuthorization`/`CondensationCredit` | `stand-ground` means OFE; tile conversion named and once |
| `m_p` | `kg H2O m^-2 basis-OFE-ground` | registry symbol `surface_liquid.parcel_mass` | typed parcel field with `basis_ofe_id` | `ofe_ground_water_mass_to_depth_m(x)=x/rho_w` |
| `T_p` | `K` | registry symbol `surface_liquid.parcel_temperature` | typed parcel field; finite domain | no Celsius/raw temperature substitution |
| `Q_p` | `J m^-2 basis-OFE-ground` | registry symbol `surface_liquid.parcel_enthalpy` | typed parcel field with `basis_ofe_id` | amount, never W m^-2 rate |
| `Delta t_parent,Delta t_proposed,Delta t_child,start_s,end_s` | `s` | registry symbol `surface_liquid.interval` | typed coupled support plus exact binary64 duration bits | parent exact 1800; proposal in 1800/900/60; child exact positive accepted support |
| `K_eff` | `m s^-1` | WB14 production configuration authority | positive finite binary64 plus canonical bits | no conversion; immutable per parent |
| `psi,S_f` | `m` | WB14 production configuration authority | finite nonnegative binary64 plus canonical bits | no conversion; immutable per parent |
| WB14/OFE/lane/configuration identities | typed ID or SHA-256 | configuration and coupled-owner registries | typed IDs and `Digest32` | exact canonical-preimage equality; never dimensional conversion |
| `f_t` | dimensionless | topology semantic entry | strict config scalar | only named tile/OFE helpers consume it |
| `A_o` | `m2` | registry symbol `surface_liquid.ofe_area` | typed configuration field | only `route_ofe_ground_amount` consumes area ratio |

Raw dimensional literals are limited to frozen `T_ref`, `C_w`, and declared
closure absolute terms. This package publishes no output metadata.

## Tolerance And Numeric Notes

Identity, unit, basis, key set, transaction, cardinality, `F<=A<=D`, capacity,
topology direction, WB14 call count, and parcel ownership are exact and never
tolerance-repaired. Mass closure accepts only:

```text
abs(residual) <= 1e-14 kg m^-2 + 64*epsilon*sum_abs_mass_operands.
```

Parcel enthalpy uses the same scale rule with `1e-9 J m^-2`. These tolerances
cannot repair missing/duplicate operands or wrong identity. Canonical
serialization compares floating bit patterns exactly.

The section-2 authorization scale may consult the mass envelope only after
each raw proportional row and its canonical aggregate are finite. It may only
move every row downward through one common representable factor and must prove
the resulting aggregate is `<=S_k` exactly. It is not a generic approximate
comparison and cannot admit an overdraw.

## Calibration And Identifiability Posture

`science_implementation_status=IMPLEMENTED`; lifecycle and gap status retain
the pending verification/promotion posture. `calibration_evidence_status=NOT_APPLICABLE`
and `identifiability_status=NOT_APPLICABLE`.

Custody arithmetic is non-calibratable. `capacity_kg_m2_tile` is a required
external site/surface parameter that controls retention and overflow; this
package neither estimates nor recommends it. Future observation operators are
beginning/ending ponded or litter liquid, infiltration, and routed/outlet
runoff. No synthetic recovery, empirical validation, parameter quality, or
transferability claim is made. Missing capacity is a hard configuration error,
not a calibration fallback.

| Readiness obligation | Status | Evidence/disposition |
|---|---|---|
| typed/enumerable parameter surface | `PASS` | Capacity and immutable WB14 parameters are explicit external configuration; custody introduces no fitted parameter. |
| observation operator with units and scale | `PASS` | Beginning/ending store, infiltration, and routed/outlet runoff are defined above with tile/OFE bases. |
| deterministic candidate execution | `PASS` | Canonical configuration, state, parent, child, and receipt reconstruction are required. |
| objective reconstruction | `NOT_APPLICABLE` | This contract defines no calibration objective and authorizes no fitting. |
| sensitivity analysis | `NOT_APPLICABLE` | Custody arithmetic is non-calibratable; external parameter sensitivity belongs to its process/calibration package. |
| identifiability/confounding analysis | `NOT_APPLICABLE` | No parameter inference is authorized. |
| boundary, saturation, and failure reporting | `PASS` | Capacity saturation and every invalid domain map to typed guards and closure receipts. |
| equifinality/uncertainty retention | `NOT_APPLICABLE` | No candidate ensemble or inferred parameter is produced. |
| synthetic recovery | `NOT_APPLICABLE` | No fitted custody coefficient or observation objective exists. |
| additional-data inventory | `NOT_APPLICABLE` | Capacity provenance is a required external configuration obligation, not an empirical claim in this contract. |

## Test-Vector Obligations

Independent positive vectors must cover strict zero/positive restart state;
every configuration/state field affecting digests; exact initial and accepted
lineage; zero/full/partial supply; competition; raw-share aggregate overshoot
with a jointly safe common scale; three equal demands; request-order reversal;
exact `F=A` ending-state debit after common scaling; three distinct finalized
uses with caller-order reversal and bit-identical ending state; unused authorization; debit;
OFE-basis condensation and capacity overflow; bare/litter identity; exact
tile/OFE conversion; precipitation, runon, and every canopy release class;
48 stateful timed nonlinear WB14 continuations with daily-wrapper parity;
no-duplication open/covered precipitation; mixed-temperature partition;
post-infiltration proportional-capacity retention; infiltration; unequal-area
routed multi-OFE and outlet runoff; two unequal-area OFEs across two accepted
900-second children with same-child runon in both children, ordered per-OFE
child receipts, one final parent-receipt set, and one persistent cursor advance;
mass/enthalpy split; restart equivalence; and byte-identical rollback.

Poison vectors must reject incompatible LSE identity; residue/depression/WAT5/
snow aliases; duplicate store; nonzero legacy depression retention; missing or
extra keys; scalar broadcast; stale/mixed transaction; current-ingress supply;
request inflation; second authorization; authorization-as-use; wrong
OFE/tile/surface/source/basis; omitted/doubled `f_t`; clipped condensation;
canonical-last remainder or any row-specific authorization correction;
missing temperature/enthalpy; per-parcel or copied Green-Ampt; proportional
infiltration proxy; reset/replayed continuation; wrong parent/proposal/accepted-
child cadence or coupled receipt; raw rain plus canopy release; multiple calls
per OFE/child; untimed daily scalar;
omitted/doubled OFE area ratio; cycle/backward route; producer residual;
partial mutation; OFE/lane/K/matric-potential/storage/model/ordinal/support/
predecessor substitution; early parent cursor advancement; and production
selector/default reachability.

Expected values are independently reconstructed from frozen operands rather
than generated by production Rust.

## Terminal Meltout Receipt And Partial-WB14 Amendment

`INV-SURFACELIQUID-010` admits one `terminal_receiver_v1` ingress parcel keyed
by snow event fingerprint, transaction/OFE/tile, `t*`, and remaining support.
Its mass is exactly retained snow liquid plus newly generated terminal liquid,
its temperature is `273.15 K`, and its sensible enthalpy relative to `T_ref` is
zero. It is accepted exactly once before the actual receiver's remaining-time
WB14 solve and is never aliased to rain, runon, store level/change,
infiltration, ponding, overflow, runoff, or snow drainage. Duplicate, missing,
wrong-support, wrong-basis, nonzero-enthalpy, or CoE-plus-Stage-3 receipts fail
typed and leave all owners unchanged.

`INV-SURFACELIQUID-011` extends the existing WB14 continuation without changing
production cadence: the default-off transaction may execute a partial WB14
interval over exactly `dt_remaining`, retaining interval/day identity, elapsed
support, cumulative supply/infiltration, ponding, overflow, runon, and runoff.
Pre-event support is not replayed and remaining support is not spread across a
full bin. A cross-midnight endpoint advances continuation once. Restart
before/after meltout or at an accepted terminal substep restores the same
partial continuation and produces byte-identical final owner state.

The fixed production base remains exactly 48 parent wall bins of 1800 seconds.
Ordinary calls are accepted only as sealed children of the INV-012 parent;
unsealed variable-duration calls remain `SURFACELIQUID-E-008`. The terminal
exception is a tagged receiver segment inside one identified base wall bin.
Let `D=1800 s`, `d=wall_end-wall_t*`,
`0<=d<=D`. Time-varying forcing is partitioned on the half-open absolute
support `[wall_t*,wall_end)`; it is not scaled from a full-bin aggregate and
does not replay `[wall_start,wall_t*)`. A parcel exactly at `wall_t*` belongs
to the receiver. The adapter passes beginning cumulative infiltration,
beginning ponded water, soil Green-Ampt parameters, the source-boundary
rain/additional-supply partition, and `d` to the actual shared
`compute_green_ampt_interval_infiltration` Mein-Larsen transition. Its returned
infiltration and ending cumulative state feed the existing WB14 depression-
storage/overflow/runoff chronology. No copied equation, per-parcel solve,
invented integral, or scaled full-bin proxy is authorized.

Continuation adds absolute `wall_bin_start`, `wall_bin_end`,
`consumed_support_end=wall_t*`, and `base_bin_complete`. The segment advances
support monotonically to `wall_end`; only then does interval index advance. At
midnight the old bin/day closes before new-day interval zero opens, each once.
`d=0` executes no WB14 physics. Untagged variable duration, overlap/gap,
full-bin scaling, endpoint duplication, or early index advance is
`SURFACELIQUID-E-008`.

Under the v8 parent transaction, `d` denotes the union of all post-event child
supports, not permission for one oversized child. The child containing meltout
ends exactly at `t*`; the zero-duration event transaction consumes no WB14
physics, increments the event ordinal once, and appends its receipt to the same
parent chain. The coupled controller then projects the admitted snow-free
successor state, selects a fresh `Delta t_proposed` from `{1800,900,60} s`, and
accepts one or more contiguous positive post-event children, each no longer
than its own proposal, until `wall_end`. The terminal receiver parcel enters
the first post-event child's ordered ingress queue exactly once. Child ordinals
remain monotonic across the event, restart retains the event and chain head,
and the persistent WB14 interval cursor advances only after the union of pre-
event and post-event supports reaches the parent endpoint. The released v7
single tagged `d` transition remains only for execution outside a v8 child-slab
parent.

Both are candidate-only and join the atomic all-owner commit. They authorize
no production/default/output change, CoE retirement, carrier or efficacy
claim, qualification, or cutover.

| Canonical surface | Binding |
|---|---|
| Algorithm | validate receipt/support; split endpoint forcing; run existing WB14 equations with `Delta t=d`; close/route; advance wall continuation |
| Branch/guard | `d=0` no-op; tagged `0<d<=1800` terminal support and sealed positive INV-012 children are allowed; every other variable-duration call is `E-008`; receipt replay `E-003/E-011` |
| Alias/unit | wall support is absolute date/seconds; transaction ID orders commits only; receipt is `kg m^-2 OFE-ground`, never storage/runoff |
| Tolerance | existing WB14/mass bounds apply to independently integrated segment operands; none repairs support/cardinality/identity |
| Tests | endpoint rain/runon, nonlinear unequal full-bin/segment forcing, zero/full remainder, midnight, restart/replay, debit-credit marker, rollback |

`GAP-SURFACELIQUID-004` is narrowly superseded only for receipt of this typed
0 C parcel into an already-authorized actual frozen or thawing receiver.
Frozen-liquid constitutive physics beyond that receipt remains
`AUTHORITY_MISSING` and non-promotable.

## WB14 Parent/Child Receipt Schema Details

The following details define the canonical version-8 receipt wire for one
default-off transaction over exactly one existing half-hour WB14 parent. Every
child calls the unchanged shared Green-Ampt transition
with its exact duration and current accepted cumulative state. It changes no
equation, parameter, clamp, tolerance, forcing selector, output, or restart
wire.

The immutable parent authority binds the enclosing coupled-parent identity and
exact 1800-second half-open support, schema and WB14 model-definition identity,
the complete parent-beginning owner-set digest, the surface-liquid
configuration digest and ordered OFE topology, the persistent
day/next-interval cursor digest, and for every OFE the exact `ofe_id`,
production-lane identity, WB14 configuration digest, effective-conductivity
bits, matric-potential bits, and storage-capacity bits. Zero identity digests,
nonfinite or negative beginning cumulatives, and cumulative infiltration above
cumulative supply reject before physics.
The parent-local working state contains accepted support end, next child
ordinal, cumulative supply, cumulative infiltration, and receipt-chain head.
It is a candidate and is never a second persistent cursor.

Stage 3 proposes an upper bound of exactly 1800, 900, or 60 seconds from the
latest accepted Stage-3 state. Coupled time selects the actual positive child
support, which may be shorter because of an event, restart, output, parent
endpoint, or another hard boundary and must not exceed the proposal. The child
consumes the exact accepted-duration bits. A zero-duration event is a separate
event transaction. Child support must begin at the latest accepted endpoint,
remain within the parent, and carry the next ordinal and current chain head.
Its receipt binds parent identity, parent-beginning owner and cursor, selected
upper bound, accepted coupled-time receipt, immutable OFE/WB14 identities,
complete Green-Ampt input bits, beginning and ending per-OFE working states,
predecessor chain, complete beginning and ending owner-set digests, staged
surface/hydrology/soil/soil-thermal/LSE/V11/Stage-3/clock digests, and its own
digest. Only accepted child
candidates become the next parent-local beginning. Omission, duplication,
reorder, overlap, gap, altered beginning, or replay rejects exactly.

The parent candidate owns the real topology-ordered surface transaction:
surface stores, timed source parcels and enthalpy, per-OFE WB14 working states,
pending routed parcels, production-soil and soil-thermal candidates, LSE, V11,
Stage 3, clock, provider/GSI cursor, event, and receipt-chain candidates. Every
child processes all OFEs in topology order so upstream runoff becomes
downstream runon within that child. Inactive owners are byte-identical carries,
and child `n` beginning complete-owner identity equals child `n-1` ending.

Finalization is legal only after one or more accepted children exactly cover
the parent. It copies the parent-local cumulative state and advances the
persistent interval cursor once. Children never advance that cursor. A failed
child or finalization discards the complete candidate, including staged
surface, soil, runoff, V11, Stage 3, clock, provider, GSI, receipt, and event
owners; no partial candidate is publishable.

Cadence choice belongs to the coupled Stage 3 controller and is reevaluated
from the latest accepted Stage 3 state. WB14 validates the selected child's
closed cadence and chronology but does not infer snow mass or select cadence.
One 1800-second child must return the historical interval outcome bitwise.

Independent `validate()` reconstruction begins at the canonical parent chain,
replays every immutable identity, ordinal, support, working-state transition,
complete-owner join, closure operand, and receipt digest, and derives the final
cursor. A stored digest or producer-only self-check is insufficient.

The runtime installs the accepted parent-local
surface state before hydrology snapshot construction and every LSE resource
request, authorization, debit, condensation credit, and ingress operation. It
uses one sealed scalar parent authority per OFE and records the exact ordered
Green--Ampt transition windows used by timed parcel partitioning. Every
complete-owner child join binds the ordered per-OFE child-receipt-set digest;
the final child alone additionally binds the ordered parent-receipt-set digest
and projects the persistent interval cursor. The scalar authority consumes the
actual coupled parent transaction, accepted
slab, complete beginning owner set, and selected Stage 3 upper bound; none is
inferred from accepted duration or a local counter. Final evidence retains and
seals replayable ordered scalar-authority/finalization bytes, and parent-local
physical plus scalar custody has a validated restart encoding. These are
binding runtime requirements and are independently reconstructed.

Required vectors are one 1800-second child bit-identical complete production-
owner parity; two 900-second children;
thirty 60-second children; a latest-state-selected mixed cadence; zero-supply
no-op children; an event-truncated child; two unequal-area OFEs with same-child
routing/runon, parcel attribution, and enthalpy closure; positive cumulative
closure independently reconstructed; rollback after child 1/2 and 17/30;
final-owner-join rollback; OFE/lane/configuration/K/psi/storage/model
substitution; zero-digest and invalid-cumulative poisons; omission,
duplication, reorder, overlap, gap, and replay poisons; and exactly one
persistent continuation advance.

| Canonical rule | Binding |
|---|---|
| `INV-SURFACELIQUID-012` | One WB14 parent covers exactly one existing 1800-second day/interval continuation; persistent cursor is immutable during children accepted at or below a selected 1800/900/60-second upper bound and advances once at complete finalization. |
| `INV-SURFACELIQUID-013` | Every child binds coupled support, immutable OFE/lane/configuration/model/parameter identity, complete Green-Ampt inputs, per-OFE working progression, complete beginning/ending owner sets, and canonical reconstructable receipt chain. |
| `INV-SURFACELIQUID-014` | The complete parent candidate processes all OFEs in topology order and atomically stages surface storage, attributed liquid/enthalpy, routing, production soil, soil thermal, LSE, V11, Stage 3, clock, provider/GSI, event, and receipt owners. |

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `SURFACELIQUID-V7-TERMINAL` | Terminal Meltout Receipt And Partial-WB14 details above | `active` | `maps-to-existing-INV` | `INV-SURFACELIQUID-010, INV-SURFACELIQUID-011` | `none` | Released terminal receiver and partial-bin rules retained unchanged. |
| `SURFACELIQUID-V8-CHILD-SLAB` | WB14 Parent/Child Receipt Schema Details above | `active` | `maps-to-existing-INV` | `INV-SURFACELIQUID-006, INV-SURFACELIQUID-012, INV-SURFACELIQUID-013, INV-SURFACELIQUID-014` | `flagged-binding-addition` | Version-8 rules are exposed in the core algorithm, guards, obligations, invariants, aliases, units, vectors, and gaps and undergo this cycle's dual review. |

## Gap Register And Promotability

| Gap | Status | Disposition |
|---|---|---|
| `GAP-SURFACELIQUID-001` persistent owner absent | `CLOSED` | Persistent owner, canonical state/restart, and complete-owner projection are implemented and tested. |
| `GAP-SURFACELIQUID-002` runtime owner implementation | `IMPLEMENTED_AWAITING_PROMOTION_VERIFICATION` | Exact state/candidate operations, receipt-owned parent coordination, reconstruction, and rollback are implementation candidates; closure awaits this cycle's disposition and verification. |
| `GAP-SURFACELIQUID-003` production selector/cutover | `NOT_AUTHORIZED` | Later separately governed package required. |
| `GAP-SURFACELIQUID-004` snow/frozen/thawing custody | `AUTHORITY_MISSING`, `NON_PROMOTABLE` | Typed unsupported; snow contracts own that domain. |
| `GAP-SURFACELIQUID-005` multi-production-lane covered Stage-3 parent execution | `NOT_AUTHORIZED` | The parent ledger representation is lane-keyed and contains no unweighted cross-OFE scalar, but production rejects more than one active covered Stage-3 lane with a typed support error. Genuine multi-lane covered execution requires its own fixture and review cycle. Snow-free multi-OFE topology remains admitted. |

This contract authorizes the default-off child-slab attachment after v8
promotion. It does not authorize output publication, selector cutover, or
multi-lane covered Stage-3 execution.

## Change Log

| Date | Version | Author | Change |
|---|---|---|---|
| 2026-08-19 | 7 | Codex | Added exact-one 0 C terminal receipt and partial-WB14 continuation/restart authority (`INV-SURFACELIQUID-010/011`) for the default-off terminal receiver transaction. |
| 2026-08-23 | 8 (in review) | Codex | Formalized child slabs as exact coupled supports beneath Stage-3 cadence proposals; bound OFE/lane/configuration/model/parameter and complete-owner identity; required topology-ordered complete transaction staging, receipt reconstruction, final-only cursor publication, parity, truncation, routing, poison, and rollback gates. Version 7 remains released until promotion. |
| 2026-08-23 | 8 (integration candidate) | Codex | Installed parent-local surface state before child resource physics; connected live topology-ordered ingress to per-OFE reconstructable scalar receipt authorities; bound child and final parent receipt sets into complete-owner joins; separated slab and persistent-parent counters; added actual 30x60 and rollback evidence. Promotion remains review-gated. |
| 2026-08-23 | prospective | Codex | Recorded the unreleased WB14 parent/child transaction design and contract vectors. Independent reviews held release on complete-owner integration, dynamic Stage 3 cadence, and rollback evidence; v7 authority and the production guard remain unchanged. |
| 2026-08-14 | 1 | Codex | Initial contract-first draft. |
| 2026-08-14 | 2 | Codex | Align exact LSE surface/source identities and OFE condensation basis; bind one actual timed aggregate WB14 call per OFE, zero legacy depression retention, post-infiltration persistent retention, routed topology, canonical digests, profile sections, unit governance, and independent vector obligations. |
| 2026-08-14 | 3 | Codex | Bind the exact 1800-second/48-step stateful WB14 continuation, mutually exclusive open-rain/covered-canopy supply, conservative mixed enthalpy, exact tile/source retention, retained LSE energy receipt, water density, machine-readable registry seams, continuation restart schema, and basis-rekeyed unequal-area OFE routing. |
| 2026-08-14 | 4 | Codex | Add the strict per-tile `ground_ingress_mode` discriminator required to validate mutually exclusive open-precipitation and covered-canopy ingress without caller-driven branch inference. |
| 2026-08-14 | 5 | Codex | Bind every surface OFE to the actual production lane and ordered soil layers; require strict restart combinations; require shared production same-pass infiltration credit, typed soil-thermal and retained-LSE receipts, independent full-equation closure, and canonical contextual failure payloads. |
| 2026-08-15 | 6 | Codex | Admit the symmetric binary64 joint-supply representability rule for a raw proportional aggregate overshoot: one common downward scale, exact no-overdraw proof, 64-decision bound, contextual E003 failure, canonical request/finalized-use aggregation, caller-order-invariant ending state, and no canonical-last remainder. |
