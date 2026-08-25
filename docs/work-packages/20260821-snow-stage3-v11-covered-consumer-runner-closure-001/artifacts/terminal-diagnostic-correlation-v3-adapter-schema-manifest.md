# Terminal diagnostic correlation V3 adapter-schema manifest

Status: `FROZEN WITH V3 AUTHORITY / NO IMPLEMENTATION AUTHORITY`

Base: `8b2a7fe1789fb06386110fb5d6e3bc5fd2f7d962`

This manifest closes every diagnostic-only wire adapter required by
`TerminalCarrierPhaseRecordV3`. No adapter field list, tag, order or encoding
choice is deferred to implementation intent. These adapters observe existing
typed values; they neither replace nor change any production receipt.

## Common canonical encoding

All records use the repository's established `framed_sha256` convention. The
domain string is the schema tag. Each ordered field is encoded as:
`tag_len:u64`, UTF-8 tag bytes, `value_len:u64`, value bytes. All integer and
length values are unsigned big-endian. `u8/u32/u64/u128` occupy exactly
`1/4/8/16` bytes. `ModelTimeNs` is its inner `u128`, not `u64`. `TimeSupport`
is `start_ns:u128`, then `end_ns:u128`. `Digest32` is exactly 32 bytes.

`f64` is `to_bits():u64` big-endian. All recorded physical floats must be
finite because capture occurs only after existing admission. Positive and
negative zero remain distinct raw bits; no signed-zero normalization is
allowed. Boolean is `u8(0|1)`. An option is `present:u8`, followed by the
payload when present. A byte string is `length:u64`, then bytes. A sequence is
`count:u64`, then each item as `item_len:u64`, item bytes. A map is a sequence
in the stated canonical key order; duplicates are invalid. Every adapter has
first field `schema_version=u32(1)` and final field `record_sha256`, computed
over the domain and all preceding framed fields.

Nested production receipts with an existing canonical digest and replay bytes
are encoded as `type_tag`, `canonical_digest`, `replay_len`, `replay_bytes`.
The digest must independently reconstruct from those exact replay bytes under
the production receipt's existing domain. A digest without replay bytes is
never a complete nested receipt. Opaque seven-owner state is intentionally
encoded as owner ID plus exact bytes, not deserialized.

## Closed enum and collection tags

- `LiveProviderRole`: `FULL=0`, `HALF_1=1`, `HALF_2=2`, `RETRY=3`,
  `BRACKET_LOWER=4`, `BRACKET_UPPER=5`, `ROOT=6`.
- `PairPosition`: `COARSE=0`, `FINE_1=1`, `FINE_2=2`.
- `PairDecision`: `ACCEPT=0`, `REJECT_RETRY=1`.
- `AdmissionDecision`: `ADMIT=0`, `BELOW_CARRIER_DOMAIN=1`,
  `DOMAIN_OR_NONFINITE=2`.
- `LseAdmission`: `ADMITTED=0`, `TYPED_REJECTION=1`; rejection payload is the
  exact existing error discriminant `u32` plus UTF-8 static variant name.
- owner order: `vegetation`, `snow`, `land_surface_energy`, `hydrology`,
  `bgc`, `soil_thermal`, `surface_liquid`.
- lane maps: ascending `lane_id:u32`; OFE maps: ascending `OfeId:u32`;
  tile maps: ascending tuple `(OfeId:u32, TileId:u32)`; soil layers and
  vegetation strata: stored typed ordinal ascending; provider/coupling calls:
  actual call order, then ordinal equality check.
- receipt-class order: prescribed amount, boundary series/rate, generated
  amount, snow--soil, `q_ss`, hydrology, WB14 child, WB14 parent.

## A1 — trial request adapter

Domain: `openwepp-terminal-trial-request-adapter-v1`.

Fields, in exact order: `schema_version`, `lane_id:u32`, `support`,
`live_provider_role:u8`, `attempt_ordinal:u32`,
`coupling_iteration:u32`, `ice_kg_m2:f64`, `liquid_kg_m2:f64`,
`cold_content_j_m2:f64`, `surface_temperature_c:f64`, `snow_depth_m:f64`,
`snow_density_kg_m3:f64`, `ending_hint_present:u8`, and, when present,
`hint_ice_kg_m2`, `hint_liquid_kg_m2`, `hint_cold_content_j_m2`,
`hint_surface_temperature_c`, followed by `beginning_joint` as A2 and
`record_sha256`.

## A2 — complete joint adapter

Domain: `openwepp-terminal-joint-adapter-v1`.

Fields: `schema_version`, `source_owner_set_sha256`, `lane_id:u32`,
`source_snow_owner_sha256`, `interval_index:u64`, `state_support`,
`accepted_predecessor_count:u64`, ordered predecessor `Digest32` values,
then exactly seven `(owner_id byte string, owner_state byte string)` pairs in
the owner order above, `production_joint_receipt_sha256`, `record_sha256`.

## A3 — probe-child/forcing/topology adapter

Domain: `openwepp-terminal-probe-child-adapter-v1`.

Fields: `schema_version`, `parent_transaction_sha256`,
`enclosing_parent_support`, `trial_support`, `physical_child_ordinal:u32`,
`attempt_ordinal:u32`, `live_provider_role:u8`, `beginning_joint_sha256`,
`beginning_owner_set_sha256`, `complete_forcing_sha256`, `topology_sha256`,
`production_probe_child_receipt_sha256`, `record_sha256`.

The forcing companion follows immediately in the carrier-phase record as
`sealed_support_forcing`: exact canonical replay bytes of
`DirectSnowStage3V11PreparedSupport::coupled_subslab` for the trial support,
and as `lse_forcing`: exact canonical replay bytes of the projected
`covered_v11_interval.lse_forcing`, each preceded by type tag, byte length and
SHA-256. The topology companion is the ordered complete
`DirectSurfaceLiquidOfeBinding` set with fields `ofe_id:u32`,
`production_lane_id:u32`, exact destination/tile IDs in declaration order,
followed by the production topology digest. No summarized topology digest may
replace this ordered payload.

## A4 — precipitation/prescribed-amount adapter

Domain: `openwepp-terminal-prescribed-amount-set-adapter-v1`.

Fields: `schema_version`, `lane_count:u64`, then each ascending lane's
`lane_id:u32` and `Stage3PrecipitationPhaseParcelSetV1` in declaration order:
its schema/version and parent/support identities; ordered source parcels with
source/provider identity, phase tag, mass `kg m^-2`, temperature `K`, specific
enthalpy `J kg^-1`, advected energy `J m^-2`, exact support and source digest;
ordered interception results; snow-ground and liquid-ground parcel totals;
set digest. It then contains the sealed incident radiation/longwave amount
receipts, each as exact type tag, support, value `J m^-2`, source digest and
canonical receipt digest. Final field: `record_sha256`.

Zero-length source/interception/amount sequences are encoded explicitly with
count zero; omission is invalid.

## A5 — rate/component and carrier-envelope adapter

Domain: `openwepp-terminal-rate-component-set-adapter-v1`.

Fields: `schema_version`; ordered ascending destination count; for each
destination `(ofe_id:u32,tile_id:u32)`, exact
`Stage3SnowCoveredLowerBoundary` declaration-order fields: support, incident
and absorbed shortwave, atmospheric/canopy/emitted/net longwave, sensible,
latent/vapor, precipitation advection, snow--soil heat, surface temperature,
active-set/source digests and boundary receipt digest; then matching
`CoveredCarrierInitialGuessV1` declaration-order thermodynamic/source fields
and digest; matching `CoveredLseIterationState` declaration-order temperature,
humidity, component-rate, residual, active-set and convergence fields and
digest. Next are the `UncommittedCoveredV8OwnerEnvelope` exact ordered owner
candidates and its forcing, topology, prescribed, generated, rate/component,
resource-use and LSE admission receipt payloads in their declaration order.
Final field: `record_sha256`.

Every numeric field is encoded even when zero. `LseAdmission` and exact active
set are explicit; a complete-envelope digest without these payloads is invalid.

## A6 — generated-amount adapter

Domain: `openwepp-terminal-generated-amount-set-adapter-v1`.

Fields: `schema_version`; destination count and, per canonical destination,
typed source/destination IDs followed by generated throughfall, initial
drainage, second drainage, stemflow, canopy condensation, canopy evaporation,
snow deposition, snow sublimation, melt, refreeze, retained liquid, routed
runoff, runon, infiltration, overflow and BGC debit. Each amount entry is
`kind_tag:u8`, `mass_kg_m2:f64`, `energy_j_m2:f64`, support, producer receipt
digest and consumer receipt digest. Kind tags follow the order in the previous
sentence starting at zero. Final fields are complete generated-set digest and
`record_sha256`.

Terminal ProducedUnconsumed liquid is not in this adapter. The carrier-phase
record separately encodes `terminal_parcel_absent=true`, terminal hydrology
ingress `0.0`, terminal WB14 credit `0.0`, and terminal surface-liquid ingress
`0.0`, preserving signed-zero bits from the observed values.

## A7 — snow--soil and q_ss adapter

Domain: `openwepp-terminal-snow-soil-adapter-v1`.

Fields: `schema_version`, lane count, then per ascending lane the
`TerminalSnowSoilTrialReceiptV1` fields in declaration order:
`schema_version:u32`, `lane_id:u32`, `support`, beginning snow-state digest,
beginning soil-state digest, snow temperature `K`, soil-interface temperature
`K`, conductivity `W m^-1 K^-1`, path length `m`, `q_ss_w_m2:f64`, snow heat
`J m^-2`, soil heat `J m^-2`, soil candidate digest, top-boundary-credit
digest and production receipt digest. Then `TerminalSnowSoilHeatReceiptV1`
contains lane/support, snow debit, soil credit, residual, input/output owner
digests and production receipt digest. Final field: `record_sha256`.

If a declaration uses equivalent names, the values still occupy this fixed
semantic order. Both equal-and-opposite values are mandatory; `q_ss` cannot be
reconstructed only from the total energy.

## A8 — soil candidate/top-boundary adapter

Domain: `openwepp-terminal-soil-candidate-adapter-v1`.

Fields: `schema_version`; exact `SoilThermalSnapshot` ordered OFE/layer fields
in production declaration order, including layer identity, temperature,
enthalpy/storage and snapshot digest; then `SoilThermalTopBoundaryCreditV1`
ordered fields including OFE, layer, support, heat `J m^-2`, beginning/ending
soil digests and credit digest; `record_sha256`.

## A9 — WB14/hydrology replay adapter

Domain: `openwepp-terminal-wb14-hydrology-adapter-v1`.

Fields: `schema_version`, WB14 child receipt-set UTF-8 digest string,
`child_replay_len:u64`, exact `wb14_child_replay_bytes`, parent-present flag
and, when present, parent receipt-set UTF-8 digest string,
`parent_replay_len:u64`, exact parent replay bytes. The child/parent replay is
decoded under its existing schema and re-encoded byte-identically; its ordered
OFE transitions bind beginning/end surface and hydrology owners, supply,
infiltration, retention, overflow, runoff, runon, enthalpy, cursor and receipt
digests. Then follow the exact hydrology-complete ending joint as A2 and
`record_sha256`.

For terminal trial carrier-phase records the parent replay must be absent
because the parent interval is not finalized. Terminal-liquid ingress values
must be explicit zero as required by A6.

## A10 — physical state, flux, preview and ledger adapters

Domain: `openwepp-terminal-physical-adapter-v1`.

`TerminalState` order is `ice_kg_m2`, `liquid_kg_m2`,
`cold_content_j_m2`. `TerminalFluxIntegral` order is `complete_energy_j_m2`,
`vapor_mass_exchange_kg_m2`, `shortwave_energy_j_m2`,
`longwave_energy_j_m2`, `sensible_energy_j_m2`, `latent_energy_j_m2`,
`advected_energy_j_m2`, `snow_soil_heat_energy_j_m2`,
`external_liquid_kg_m2`. `CoveredTerminalEndingSnowHintV1` order is ice,
liquid, cold content, surface temperature. `TerminalLedger` order is complete
energy, cold-energy change, refrozen mass, deposition mass, sublimation mass,
melt mass, unallocated energy, shortwave, longwave, sensible, latent,
advected, snow--soil heat, external liquid. All are `f64` raw bits.

Each payload begins with its closed subtype tag (`STATE=0`, `FLUX=1`,
`PREVIEW=2`, `LEDGER=3`) and ends with its own digest. The enclosing adapter
ends with `record_sha256`.

## Completeness rule

V3 implementation must construct A1--A10 from the actual typed values named
above. If source inspection discovers an additional field in one of those
production types that participates in forcing, topology, prescribed/rate/
generated amounts, `q_ss`, hydrology, physical state/ledger or identity, the
authority is incomplete and implementation stops for a reviewed manifest
successor. It is forbidden to omit the field, substitute a summary digest, or
choose an encoding during implementation.
