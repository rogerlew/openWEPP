# State, Configuration, And Forcing Schema

Evidence class: `Static`. This artifact is a human-readable index of the
six normative draft-2020-12 `lse_v1_*_schema.json` artifacts. Those JSON
schemas, not this summary, freeze exact field spelling, JSON type,
required-field set, enum values, cardinality, numerical domain, and
`additionalProperties: false` behavior. Their terminal digests are bound by
the model definition, exact fixture, authority tests, and dual terminal
reviews. Runtime implementation remains separately governed by Child 3.

## Root Records

| Record | Required fields | Ordering and cardinality |
|---|---|---|
| `configuration` | `model_version`, `model_definition_sha256`, `configuration_sha256`, `ofes` | at least one OFE; OFEs serialize in ascending `ofe_id` |
| `state` | `model_definition_sha256`, `state_sha256`, `last_accepted_transaction_id`, `ofes` | exactly the configuration OFE set and order |
| `forcing` | interval, air state, wind/stability, snow guards, four shortwave components, atmospheric longwave, precipitation parcels, runon parcels | one complete interval; parcel arrays use source-key order |

`model_version` is exactly `OPENWEPP_SNOW_FREE_LSE_V1`; all SHA-256 fields are
lower-case 64-character hex. Configuration and state reject unknown, missing,
extra, duplicate, stale, nonfinite, or out-of-domain values. Arrays are ordered
identity collections, never unordered bags. Duplicate identity is rejected
before digest comparison.

## Configuration Fields

| Scope | Field family | Type, unit, and presence rule |
|---|---|---|
| root | model/configuration identity | exact strings; always required |
| OFE | `ofe_id`, local tiles, configured soil layers | unique nonempty ID; at least one tile and soil layer |
| tile | `tile_id`, `fraction`, `surface_class` | unique within OFE; `0 < fraction <= 1`; fractions sum to exactly one within each OFE under the representation-only sum rule |
| tile optics | surface VIS/NIR albedo and emissivity | finite dimensionless values in `[0,1]`; V1 additionally requires emissivity exactly one |
| tile canopy | ordered canopy layers and four physical component records | exact top-to-bottom layer order; exact component identity/cardinality; tile-ground area basis |
| surface class | `bare_mineral_soil` or `forest_litter` | exactly one class per tile; class-specific fields are required and mutually exclusive |
| neutral aerodynamics | reference/canopy heights, displacement, momentum/scalar roughness, leaf dimension, wind-attenuation operands | finite SI values satisfying the admitted positive-wind geometry; no default or wind floor |
| soil layer | layer ID, thickness/depth, texture/porosity/organic and thermal operands | exact hydrology layer order and cardinality; positive geometry/conductivity/capacity |
| litter-only | thickness, dry density, dry heat capacity, water capacity, optics and exchange geometry | required for `forest_litter`, prohibited for bare mineral soil |
| numerics | complete warm-start shape, unit scales, frozen iteration/tolerance identity | values are the admitted constants, not caller-tunable acceptance rules |

The machine-readable schema must expose every field in these families rather
than hiding a required value behind prose or an executable default. The final
schema digests are frozen, model-definition-bound, fixture-bound, and confirmed
by the authority tests and dual terminal reviews.

## LSE Persistent State Fields

| Scope | Required field | Ownership and domain |
|---|---|---|
| root | `model_definition_sha256`, `state_sha256`, `last_accepted_transaction_id` | immutable identity and accepted lineage |
| OFE | `ofe_id` | exact configuration identity |
| tile | `tile_id`, `surface_enthalpy_j_m2_tile`, optional `surface_temperature_warm_start_k` | enthalpy is the sole mutable physical surface thermal state; temperature is derived from accepted enthalpy/mass/capacity, and any retained temperature is numerical warm start only |
| tile canopy air | `canopy_air_temperature_k`, `canopy_air_specific_humidity_kg_kg` | one shared V8 node per covered tile; open-tile value follows the schema's explicit absence/identity branch |
| tile lineage | `last_accepted_transaction_id` | null only for initial state; otherwise equals root lineage |
| numerical state | every authority-required joint-solve warm start | persistent and digest-bound; no synthesized `Default` |

LSE persistent state contains no hydrology mass and no soil-owner temperature
array. Its digest binds the exact hydrology-snapshot digest and
soil-thermal-snapshot digest it was constructed against, but does not copy or
mutate their physical fields.

## Immutable Adjacent-Owner Snapshot DTO

| Owner snapshot | Required fields | LSE posture |
|---|---|---|
| hydrology | owner/snapshot digest and lineage; ordered source records with source key and stand-ground mass | immutable input to D/A/F and enthalpy capacity; no LSE mass or temperature state |
| soil thermal | owner/snapshot digest and lineage; ordered positive-K layer temperatures matching configured layers | immutable lower boundary; receives a separately constructed equal/opposite candidate |

V1 has no persistent per-store liquid temperature and no second physical
surface-temperature state. Every hydrology-owned surface store within a tile is
isothermal with the tile surface node and contributes `C_w*W` to its sole
physical enthalpy. Accepted temperature is derived from that enthalpy, mass,
and configured capacity. A newly appearing store therefore uses the existing
surface enthalpy plus source-resolved incoming enthalpy; no liquid temperature
is synthesized. Snapshot digest/lineage, numerical warm start, owner state,
and class-specific presence changes are all independently digest-visible.

## Forcing Fields

| Field | Unit/domain | Provider and rule |
|---|---|---|
| `interval_s` | finite `s`, `>0` | scheduler interval |
| `air_temperature_k` | finite `K`, `>0` | climate projection |
| `air_specific_humidity_kg_kg` | finite `kg kg^-1`, `>=0` | climate projection |
| `air_pressure_pa` | finite `Pa`, `>0` | climate projection |
| `reference_wind_m_s` | finite `m s^-1`, `>0` | climate projection; no floor |
| `neutral_stability` | exactly `true` | admitted-domain guard |
| `snow_present`, `snow_terminal_present` | exactly `false` | admitted-domain guards |
| direct/diffuse VIS/NIR | finite `W m^-2`, `>=0` | V8 full-column boundary forcing |
| `atmospheric_longwave_w_m2` | finite `W m^-2`, `>=0` | top longwave boundary only; no prescribed upward-ground flux |
| precipitation parcel | source ID, `kg m^-2 tile`, positive-K `hydrometeor_temperature_c+273.15` | `openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity`; amount may be zero but zero amount creates no energy crossing |
| runon parcel | source ID, `kg m^-2 tile`, positive-K accepted upstream temperature | upstream hydrology/LSE routing receipt; missing thermal lineage rejects |

Forcing never substitutes air, soil, or freezing temperature for a missing
liquid temperature. Canopy throughfall, both drainage releases, and stemflow
are candidate crossings, not caller forcing; each carries the accepted source
occupancy's `wet_surface_temperature_k` under
`advected-energy-convention.md`.

## Transaction DTOs

`water_key` is the exact tuple `(transaction_id, owner_id, ofe_id, tile_id,
resource_type, source_id, amount_basis)`. The only amount basis is
`kg_h2o_m-2_stand_ground_interval`. Request, authorization, and finalized-use
records preserve that key byte-for-byte; authorization additionally carries one
typed reason. Diagnostics preserve solve identity, ordered residual records,
counts, active caps, optional step, typed failure, and before/after rollback
hashes. No producer residual is a candidate-acceptance operand.

## Canonical Serialization And Digest

Serialization uses UTF-8 canonical JSON with lexicographically ordered object
keys, identity-sorted arrays, no insignificant whitespace, and no Unicode
normalization. Finite numbers use the frozen oracle lexical form: lowercase
`e`, an explicit exponent sign, and at least two exponent digits (for example,
`1e-07`); integral-valued floating operands retain `.0`. This is a
representation rule only and does not change their binary64 values. The digest input
includes model identity, configuration/topology identity, every scientific
field, every class discriminator, exact array order, and transaction lineage.
Only the digest field being computed is replaced by the empty string. NaN,
infinity, negative zero in a non-signed field, duplicate keys, and alternate
unit/basis spellings reject before hashing.
