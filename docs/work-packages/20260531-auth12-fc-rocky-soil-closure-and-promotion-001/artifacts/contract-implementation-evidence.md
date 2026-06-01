# AUTH12 Contract Implementation Evidence

Status: complete  
Evidence mode: Static

## Contract Amendments Applied

1. `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
   - Added `AUTH12 FC Rocky-Soil Closure and Promotion Addendum`.
   - Ratified measured-theta FC/WP authority for datver families
     `7777/7778/9002/9003/9005`: measured `fc_measured/wp_measured` are
     authoritative producer-basis payloads and runtime must apply paired `cpm`
     correction (legacy `scon.for` basis).
   - Ratified direct-theta suite promotion posture to
     `gate_lane=required`, `failure_class=hard-fail` after closure.
2. `docs/specifications/science-contracts/index.md`
   - Updated SC-SOIL summary to include AUTH12 closure semantics.
3. `docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md`
   - Updated lane/failure posture to required/hard-fail.
   - Updated fixture provenance hashes and anchored-case coverage.
4. `docs/specifications/external-authority/registry.yaml`
   - Promoted direct-theta suite posture to required/hard-fail.
5. `docs/specifications/external-authority/promotion-protocol.md`
   - Added AUTH12 posture-change log entry.
