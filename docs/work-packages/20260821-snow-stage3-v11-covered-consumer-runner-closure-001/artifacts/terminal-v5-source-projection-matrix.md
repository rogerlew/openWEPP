# TerminalCarrierPhaseProjectionV5 source projection matrix

Source: `openwepp_hillslope_orchestrator::v11_covered::carrier_phase::CoveredCarrierPhaseResultV1`. No whole-type serialization is permitted.

| order | live field | resolved source type | disposition | exact nested projection / reason |
|---:|---|---|---|---|
| 0 | `carrier_envelope` | `UncommittedCoveredV8OwnerEnvelope` | `project` | exact provider owner-envelope identity only; `transaction_id()`; `covered_lse_state identities`; `soil-thermal candidate identity` |
| 1 | `carrier_source_receipts` | `BTreeMap < (OfeId , TileId) , CoveredCarrierInitialGuessV1 >` | `project` | ascending destination and explicit source-receipt adapter; `destination.ofe_id`; `destination.tile_id`; `source receipt exact schema` |
| 2 | `complete_lower_boundaries` | `BTreeMap < (OfeId , TileId) , Stage3SnowCoveredLowerBoundary >` | `project` | ascending destination and exact lower-boundary fields; `destination.ofe_id`; `destination.tile_id`; `snow_temperature_k`; `soil_temperature_k`; `thermal_conductivity_w_m_k`; `path_length_m` |
| 3 | `covered_lse_states` | `BTreeMap < (OfeId , TileId) , CoveredLseIterationState >` | `project` | ascending destination and explicit LSE-state adapter; `destination.ofe_id`; `destination.tile_id`; `LSE iteration exact schema` |
| 4 | `ending_candidates` | `CoveredCarrierEphemeralCandidatesV1` | `project` | retain ending joint digest and exact trial-receipt projection only; `joint.receipt_sha256()`; `terminal_snow_soil_trial_receipt()` |
| 5 | `precipitation_sets` | `BTreeMap < u32 , Stage3PrecipitationPhaseParcelSetV1 >` | `project` | ordered provider-generated parcel-set digests; `key:u32`; `value.receipt_sha256` |
| 6 | `soil_candidate` | `SoilThermalSnapshot` | `project` | explicit soil snapshot adapter; `configuration_id`; `state_id`; `ordered OFE/layer snapshots` |
| 7 | `soil_top_boundary_credit` | `SoilThermalTopBoundaryCreditV1` | `project` | explicit top-boundary credit adapter; `configuration/state identities`; `signed support`; `begin/end energy` |
| 8 | `transition` | `CoveredTerminalTrialTransitionV1` | `exclude` | terminal transition outcomes are not provider-carrier evidence |
| 9 | `wb14_child_receipt_set_sha256` | `String` | `include` | UTF-8 digest identity |
| 10 | `wb14_child_replay_bytes` | `Vec < u8 >` | `include` | explicit byte projection; not a native wire |
| 11 | `wb14_parent_receipt_set_sha256` | `Option < String >` | `exclude` | parent replay is outside rejected child evidence |
| 12 | `wb14_parent_replay_bytes` | `Option < Vec < u8 > >` | `exclude` | parent replay is outside rejected child evidence |
