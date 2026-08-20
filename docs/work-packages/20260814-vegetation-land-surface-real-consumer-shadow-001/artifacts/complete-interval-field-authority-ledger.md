# Complete interval field-authority ledger

Evidence class: `Static`. This ledger is the pre-implementation gate. No V10
live-owner correction was started.

Abbreviations: `met` = sealed half-hour meteorological receipt; `gsi` = accepted
daily GSI receipt; `V10cfg` = immutable V10 configuration; `LSEcfg` = immutable
LSE-V2 configuration; `Hcfg/Hstate` = immutable direct-hydrology configuration /
current staged hydrology; `Sstate` = staged surface-liquid state; `Tstate` =
staged soil thermal; `U` = accepted upstream parcel owner. All vectors are in
configured OFE/tile/layer order; every interval projection binds current V10
transaction and `1800 s` cadence. `digest` means the named owner's canonical
configuration/state/receipt digest, included in the sealed projection digest.

The columns below record every requested attribute: source/mutation point,
required owner and field, static/live status, conversion, digest/lineage/order,
domain, poison, omission consequence, and final disposition.

## Wrapper fields

| Field path | Units | Current source / mutation | Required owner + field | Static/live; equation/conversion | Digest, lineage, ordering | Domain / poison | Omission consequence | Final disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `DirectV9ShadowIntervalInput.lse_forcing` | composite | caller template; provider overwrites subset | internally sealed projection | interval-live; exhaustive projection | all owner digests; current transaction; configured tile order | poison any nested field | caller controls LSE | remove from closure API |
| `.vegetation_forcing` | composite | caller template; provider and partial live projection overwrite subset | internally sealed projection | interval-live | all owner digests; current transaction; configured layer order | poison any nested field | caller controls vegetation | remove from closure API |
| `.wb14_parameters` | composite | caller template | `Hcfg` per-OFE WB14 owner | static values joined each interval | hydrology config digest; current transaction; OFE order | missing/reorder/value poison | infiltration changes | reconstruct internally |
| `DirectV10ShadowDayInput.day_index` | day | caller | scheduler/current cursor | live; accepted scheduler quotient | cursor/config digest; exact day | wrong-day poison | replay/skip | remove from closure API |
| `.intervals` | 48 entries | caller | internal interval projector | rebuild once per interval, never once/day hydraulics | projection digest per item; index 0..47 | cardinality/order poison | stale state/caller physics | cache expectation only |

## `LandSurfaceForcing`

| Field path | Units | Current source / mutation | Required owner + field | Static/live; equation/conversion | Digest, lineage, ordering | Domain / poison | Omission consequence | Final disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `forcing_sha256` | hex | recomputed after template overwrite | internal projector | interval-live canonical digest | binds all fields/owners/transaction | stale/resealed poison | unsealed forcing | recompute internally |
| `transaction_id` | id | caller template | current V10 lineage | live; exact next transaction | V10 state digest; exact interval | wrong transaction | custody break | derive internally |
| `interval_s` | s | caller template | fixed cadence | static exact `1800` | projection digest | bit mutation | rate/time error | derive internally |
| `air_temperature_k` | K | provider overwrite | `met.air_temperature_c` | interval-live; `C + 273.15` | receipt digest; destination/interval order | nonfinite/range/mutation | atmospheric mismatch | sealed receipt |
| `air_specific_humidity_kg_kg` | kg kg-1 | provider overwrite | `met.specific_humidity_kg_kg` | interval-live identity | receipt digest | negative/mutation | humidity mismatch | sealed receipt |
| `air_pressure_pa` | Pa | provider overwrite | `met.pressure_kpa` | interval-live; `kPa * 1000` | receipt digest | nonpositive/mutation | pressure mismatch | sealed receipt |
| `reference_wind_m_s` | m s-1 | provider overwrite | `met.wind_m_s` | interval-live identity | receipt digest | nonpositive/calm typed unsupported | aerodynamic mismatch | sealed receipt |
| `neutral_stability` | bool | caller template | supported-domain scheduler rule | live/domain; exact admitted neutral branch | projection digest/current interval | flip/nonneutral poison | wrong solver branch | internally derived or typed unsupported |
| `snow_present_at_beginning` | bool | caller template | `Hstate` winter/snow predicate | interval-live exact staged posture | hydrology state digest | flip/snow poison | wrong domain | derive; snow typed unsupported |
| `snow_present_at_end` | bool | caller template | staged child outcome | interval-live | ending hydrology digest | flip poison | hidden terminal snow | derive; non-snow required |
| `snow_terminal_payload_present` | bool | caller template | staged child terminal owner | interval-live | ending hydrology digest | flip poison | payload loss | derive; terminal typed unsupported |
| `direct_vis_w_m2` | W m-2 | provider overwrite | `met.direct_visible_w_m2` | interval-live identity | receipt digest | negative/mutation | radiation mismatch | sealed receipt |
| `diffuse_vis_w_m2` | W m-2 | provider overwrite | `met.diffuse_visible_w_m2` | interval-live identity | receipt digest | negative/mutation | radiation mismatch | sealed receipt |
| `direct_nir_w_m2` | W m-2 | provider overwrite | `met.direct_nir_w_m2` | interval-live identity | receipt digest | negative/mutation | radiation mismatch | sealed receipt |
| `diffuse_nir_w_m2` | W m-2 | provider overwrite | `met.diffuse_nir_w_m2` | interval-live identity | receipt digest | negative/mutation | radiation mismatch | sealed receipt |
| `atmospheric_downward_longwave_w_m2` | W m-2 | provider overwrite | `met.downward_longwave_w_m2` | interval-live identity | receipt digest | negative/mutation | longwave mismatch | sealed receipt |
| `precipitation_parcels` | parcel vector | provider overwrite | sealed precipitation receipts | interval-live exact parcel projection | receipt/carry digests; parcel identity/order/support | duplicate/lineage/enthalpy poison | mass/energy loss | sealed receipt |
| `runon_parcels` | parcel vector | caller template | `Sstate` internal routing or `U` boundary | interval-live; exact parcel projection; exact empty only from topology | surface/upstream digest; source/destination/parcel order | all lineage/thermal/support poisons | double/missing ingress | reconstruct; external missing typed unsupported |

## `SnowFreeForcing`

| Field path | Units | Current source / mutation | Required owner + field | Static/live; equation/conversion | Digest, lineage, ordering | Domain / poison | Omission consequence | Final disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `air_temperature_k` | K | provider overwrite | `met.air_temperature_c` | live; `C + 273.15` | receipt digest/index | mutation | atmosphere mismatch | sealed receipt |
| `pressure_pa` | Pa | provider overwrite | `met.pressure_kpa` | live; `*1000` | receipt digest/index | mutation | atmosphere mismatch | sealed receipt |
| `co2_pa` | Pa | provider overwrite | static forcing `co2_pa` joined to receipt | static identity | static config digest | bit mutation | physiology mismatch | immutable forcing config |
| `vapor_pressure_deficit_kpa` | kPa | provider overwrite | accepted GSI/provider daily projection | live identity | receipt/GSI digest | negative/mutation | physiology mismatch | sealed receipt |
| `wind_m_s` | m s-1 | provider overwrite | `met.wind_m_s` | live identity | receipt digest | calm/domain mutation | conductance mismatch | sealed receipt |
| `rain_kg_m2` | kg m-2 | provider sums parcels | precipitation receipt parcels | live exact ordered sum | receipt/carry digest | parcel/mass mutation | water mismatch | sealed receipt derived sum |
| `direct_par_w_m2` | W m-2 | provider overwrite | direct VIS receipt | live identity | receipt digest | mutation | FvCB mismatch | sealed receipt |
| `diffuse_par_w_m2` | W m-2 | provider overwrite | diffuse VIS receipt | live identity | receipt digest | mutation | FvCB mismatch | sealed receipt |
| `direct_nir_w_m2` | W m-2 | provider overwrite | direct NIR receipt | live identity | receipt digest | mutation | energy mismatch | sealed receipt |
| `diffuse_nir_w_m2` | W m-2 | provider overwrite | diffuse NIR receipt | live identity | receipt digest | mutation | energy mismatch | sealed receipt |
| `solar_zenith_cosine` | 1 | provider overwrite | meteorological receipt | live identity | receipt digest | outside [0,1]/mutation | radiation geometry mismatch | sealed receipt |
| `ground_albedo_vis` | 1 | canonical zero compatibility field | per-tile `LSEcfg.tiles.surface_vis_albedo` | static tile-local identity; global scalar is dead in V10 | LSE config digest; tile order | heterogeneous/caller mutation | cross-tile optics alias | strict endpoint projects per tile |
| `ground_albedo_nir` | 1 | canonical zero compatibility field | per-tile `LSEcfg.tiles.surface_nir_albedo` | static tile-local identity; global scalar is dead in V10 | LSE config digest; tile order | heterogeneous/caller mutation | cross-tile optics alias | strict endpoint projects per tile |
| `longwave_down_w_m2` | W m-2 | provider overwrite | atmospheric receipt | live identity | receipt digest | mutation | longwave mismatch | sealed receipt |
| `longwave_up_w_m2` | W m-2 | canonical zero compatibility field | coupled LSE ground state/solver | solved from the current ground trial per tile | LSE state/config digest; tile order | template mutation must be irrelevant | duplicate ground boundary | strict endpoint ignores scalar |
| `specific_humidity` | kg kg-1 | provider overwrite | meteorological receipt | live identity | receipt digest | negative/mutation | humidity mismatch | sealed receipt |
| `reference_height_m` | m | provider overwrite | static forcing config | static identity | static config digest | nonpositive/mutation | aerodynamic mismatch | immutable forcing config |
| `soil_layers` | vector | live owner projection | Hstate/Tstate/subsurface plus digest-bound root configuration | interval-live, per exact OFE/layer/root | state/config/root receipt digests; configured rooted-layer order | nested and template poisons | stale/caller root physics | derived opaque root receipts |
| `gsi` | 0..1 | provider overwrite | accepted CP-GSI01 receipt | daily accepted identity | GSI receipt/state digest | mismatch/mutation | phenology mismatch | accepted daily receipt |

## `SoilLayerForcing`

| Field path | Units | Current source / mutation | Required owner + field | Static/live; equation/conversion | Digest, lineage, ordering | Domain / poison | Omission consequence | Final disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `layer_id` | id | compatibility template only | internal V10 root membership keyed by OFE/lane/occupancy/stratum/layer | static identity | root-config and hydrology snapshot digests; canonical key order | wrong/reorder/duplicate | wrong soil layer | internally derived receipt |
| `water_beginning_kg_m2` | kg m-2 OFE ground | compatibility template only | exact staged hydrology layer amount | interval-live exact amount/basis | hydrology snapshot/current transaction/OFE/lane key | differing-OFE/value poison | wrong demand/custody | internally projected per qualified receipt |
| `matric_potential_mm` | mm water head | compatibility template ignored | Brooks-Corey retention from live liquid/thickness/porosity plus immutable psi/B | interval-live exact libm authority | root-config + hydrology snapshot + transaction + qualified receipt digest | source/domain/pore/template poison | hydraulic equation impossible | internally derived opaque receipt |
| `hydraulic_conductivity_mm_s` | mm s-1 | compatibility template ignored | Brooks-Corey conductivity from live hydrology plus immutable B/Ksat | interval-live exact libm authority and `*1000` | root-config + hydrology snapshot + transaction + qualified receipt digest | source/unit/exponent/template poison | root conductance wrong | internally derived opaque receipt |
| `root_path_length_mm` | mm | compatibility template ignored | ordered layer-top sum + half thickness + stratum tissue path | static/live geometry projection | root-config + hydrology snapshot + OFE/lane/stratum key | predecessor/path/template poison | q3 denominator undefined | internally derived opaque receipt |
| `gravity_root_mm` | mm water head | compatibility template ignored | signed `-1000 * layer_node_depth_m` | interval-live exact geometry | root-config + hydrology snapshot + qualified receipt digest | sign/alias/template poison | q3 driving head wrong | internally derived opaque receipt |
| `temperature_k` | K | compatibility-only in root solve | exact OFE soil-thermal node remains owned by LSE ground projection | interval-live per OFE/layer | soil-thermal state/config digest; OFE/layer | differing-OFE/value poison | thermal boundary mismatch | not a root-hydraulic caller authority |
| `accessible` | bool | compatibility template ignored | configured positive root membership | static predicate | root-config + qualified receipt key | missing/unrooted poison | unauthorized uptake | receipt exists only for admitted membership |
| `frozen` | bool | compatibility template ignored | staged live hydrology posture | interval-live predicate | hydrology snapshot/OFE/lane/layer | frozen poison | unsupported domain bypass | typed `FrozenRootedLayerUnsupported` before receipt minting |

## `DirectOfeWb14Parameters`

| Field path | Units | Current source / mutation | Required owner + field | Static/live; equation/conversion | Digest, lineage, ordering | Domain / poison | Omission consequence | Final disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `ofe_id` | id | caller template | immutable Hcfg OFE | static identity | Hcfg digest; OFE order | wrong/reorder/duplicate | wrong infiltration owner | retain/reconstruct in V10 static context |
| `effective_conductivity_m_s` | m s-1 | caller template | immutable WB14 Hcfg | static identity | Hcfg + provider WB14 join digest | value poison | infiltration changes | immutable owner |
| `matric_potential_m` | m | caller template | immutable WB14 Hcfg | static nonnegative Green-Ampt parameter | Hcfg + provider WB14 join digest | value/sign poison | infiltration changes | immutable owner; never root potential |
| `infiltration_storage_capacity_m` | m | caller template | immutable WB14 Hcfg | static nonnegative identity | Hcfg digest | value poison | storage changes | immutable owner |

## `LiquidParcel`

| Field path | Units | Current source / mutation | Required owner + field | Static/live; equation/conversion | Digest, lineage, ordering | Domain / poison | Omission consequence | Final disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `parcel_kind` | enum | precipitation provider or caller runon | sealed receipt / `Sstate` / `U` | live exact kind | source digest/parcel order | caller-kind poison | ingress class alias | owner-projected |
| `parcel_id` | id | provider or caller | source owner | live unique identity | source digest/order | duplicate poison | double custody | owner-projected |
| `source_owner_id` | id | provider or caller | source owner | live exact identity | source digest | wrong-owner poison | forged custody | owner-projected |
| `source_ofe_id` | id | provider or caller | source topology | live exact identity | source digest/OFE order | wrong-source poison | wrong routing | owner-projected |
| `source_tile_id` | id | provider or caller | source topology | live exact identity | source digest/tile order | wrong-source poison | wrong routing | owner-projected |
| `destination_ofe_id` | id | provider or caller | configured routing destination | live exact identity | config/source digest | wrong-destination poison | wrong area/route | owner-projected |
| `destination_tile_id` | id | provider or caller | configured routing destination | live exact identity | config/source digest | wrong-destination poison | wrong area/route | owner-projected |
| `start_s` | s | provider or caller | source receipt support | live half-open support | receipt/source digest; chronological order | support poison | timing mismatch | owner-projected |
| `end_s` | s | provider or caller | source receipt support | live half-open support | receipt/source digest | support poison | timing mismatch | owner-projected |
| `amount_kg_m2_destination_tile_ground` | kg m-2 tile ground | provider or caller | source parcel with exact area conversion | live exact conversion | source digest/area basis | mass/area poison | water nonclosure | owner-projected |
| `temperature_provider` | enum | provider or caller | source owner class | live exact provider enum | source digest | provider poison | thermal provenance loss | owner-projected |
| `temperature_k` | K optional | provider or caller | source parcel thermal owner | live; absent iff exact zero | source digest | temperature/zero-mass poison | energy mismatch | owner-projected |
| `specific_liquid_enthalpy_j_kg` | J kg-1 optional | provider or caller | source parcel thermal owner | live exact `4218*(T-273.15)` in admitted liquid domain | source digest | enthalpy poison | energy mismatch | owner-projected |
| `source_state_sha256` | hex optional | provider or caller | source owner canonical state | live; required for positive mass | nested source digest | missing/wrong digest poison | unverifiable lineage | owner-projected |

## Exhaustive-destructuring gate disposition

The production closure projector cannot yet be written truthfully because three
required fields have no owner. When authority is admitted, its internal
projection and contract test must destructure `DirectV9ShadowIntervalInput`,
`LandSurfaceForcing`, `SnowFreeForcing`, `SoilLayerForcing`,
`DirectOfeWb14Parameters`, and `LiquidParcel` without `..`. Until then this
ledger is exhaustive against the exact definitions at intake; any source-field
addition requires updating this ledger before implementation resumes.
