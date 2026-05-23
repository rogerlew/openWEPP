# CLIM07 Confidence-Tier Reporting Evidence

Status: `completed`
Evidence mode: `Ran`

## Routing Policy Evidence
Verified deterministic routing metadata from
`openwepp-comparator-metadata`:
- `single_ofe_daily_water_balance` ->
  `ComparatorConfidenceTier::HigherConfidence` with message id
  `COMPMETA-HC-SINGLE-OFE-DAILY-001`.
- `hourly_water_balance` ->
  `ComparatorConfidenceTier::Investigation` with message id
  `COMPMETA-I-HOURLY-001`.
- `watershed_water_balance` ->
  `ComparatorConfidenceTier::Investigation` with message id
  `COMPMETA-I-WATERSHED-001`.
- Missing required OFE metadata for single-OFE daily route hard-fails with
  `ComparatorTierRoutingError::MissingRequiredMetadata` and message id
  `COMPMETA-E-MISSING-OFE-COUNT`.

## Run Evidence
1. `cargo test --test comparator_tier_routing_metadata`
- result: pass (`5 passed`).

2. `cargo test --test clim07_climate_comparator_and_closure_contract`
- result: pass (`4 passed`), including
  `clim07_confidence_tier_routing_vectors_match_governance_policy`.
