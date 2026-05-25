# MOFE04 Output Publication Implementation Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Implemented MOFE04 publication closure in:
- `crates/openwepp-runner/src/hillslope/mod.rs`

Key behavior implemented:
- Added explicit publication provenance fields to WB13 publication manifest payload:
  - `publication_ofe_policy`
  - `contributor_ofe_count`
  - `area_policy`
  - `publication_area_m2`
- Added canonical MOFE04 policy constants:
  - `single-row-canonicalized-hillslope-aggregate`
  - `sum-ofe-geometry-area`
- Replaced primary-OFE area derivation with aggregate area derivation over all OFEs:
  - `publication_area_m2 = Σ(fwidth_i * slplen_i)`
- Enforced typed hard-fail guards for malformed publication domains:
  - zero contributor count,
  - non-finite/non-positive publication area,
  - non-canonicalized WB13 OFE keys.
- Updated WB13 row assembly and scheduler lifecycle plumbing to use aggregate publication area.
- Updated runner unit coverage for aggregate OFE area semantics.

## Ran
- `cargo test -p openwepp-runner simimpl11_area_derives_from_aggregate_ofe_geometry -- --nocapture`
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests mofe04 -- --nocapture`
