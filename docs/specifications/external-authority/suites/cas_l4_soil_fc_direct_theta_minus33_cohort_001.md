---
suite_id: cas_l4_soil_fc_direct_theta_minus33_cohort_001
title: Soil Profile FC Direct Theta(-33kPa) Constitutive Gate Suite
status: active
authority_level: 4
domain: soil
process_family: fc_wp
sc_invariant_refs:
  - SC-SOIL-001#INV-SOIL-014
external_citations:
  - citation_id: EXT-SOIL-FC-CH7-001
    source_type: book
    title: WEPP User Summary Chapter 7 Soil Component
    locator: references/50201000/chap7.pdf
    version_or_edition: NSERL Report No. 11 (August 1995)
    retrieved_utc: 2026-05-31
    notes: "Field-capacity constitutive definition at -33 kPa."
fixtures:
  - fixture_id: FX-FC-COHORT-CONFIG-001
    path: tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/cohort_case.json
    fixture_class: integration
    units_basis: m3_m3_and_mm
    hash: 3857193317f5f7112e46572bb7f5bb1004368b782821fa23d7728ab260ef2205
    source_repo: /workdir/openWEPP
    source_commit: f9be6a294083c17044a5b822470710f3bb017e98
    source_path: tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/cohort_case.json
    source_sha256: 3857193317f5f7112e46572bb7f5bb1004368b782821fa23d7728ab260ef2205
    transform_note: "Repository-authored required-case cohort configuration for direct theta(-33kPa) constitutive gate checks."
  - fixture_id: FX-FC-COHORT-SOIL-7778-001
    path: tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/valid_7778.sol
    fixture_class: integration
    units_basis: infile_soil
    hash: 6a08fb7859a2d31925109b929f22c8b4b9528025e7152ebe82f0f60a2838ec88
    source_repo: /workdir/openWEPP
    source_commit: f9be6a294083c17044a5b822470710f3bb017e98
    source_path: tests/fixtures/infile/soil/valid_7778.sol
    source_sha256: 6a08fb7859a2d31925109b929f22c8b4b9528025e7152ebe82f0f60a2838ec88
    transform_note: "Direct copy for reproducible cohort coverage."
  - fixture_id: FX-FC-COHORT-SOIL-H1-001
    path: tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/h1_high_rock_fc_authority.sol
    fixture_class: integration
    units_basis: infile_soil
    hash: 2202b14c684fefb19787375180f756e1f0deb06a41387a6111d65c42a263b1aa
    source_repo: /workdir/openWEPP
    source_commit: f9be6a294083c17044a5b822470710f3bb017e98
    source_path: tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/h1_worked_example_source.md
    source_sha256: cf8ca3c22124c279fefc8896dcaddb32b506afec6deaf0c0da32e8fa12e3c521
    transform_note: "Synthetic high-rock soil fixture encoded from the tracked worked-example source note."
  - fixture_id: FX-FC-COHORT-SOURCE-NOTE-001
    path: tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/h1_worked_example_source.md
    fixture_class: integration
    units_basis: markdown
    hash: cf8ca3c22124c279fefc8896dcaddb32b506afec6deaf0c0da32e8fa12e3c521
    source_repo: /workdir/openWEPP
    source_commit: f9be6a294083c17044a5b822470710f3bb017e98
    source_path: tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/h1_worked_example_source.md
    source_sha256: cf8ca3c22124c279fefc8896dcaddb32b506afec6deaf0c0da32e8fa12e3c521
    transform_note: "Tracked source note for H1 worked-example provenance."
tolerances:
  mode: rel
  rel:
    value: 0.35
    comparator: "<="
  units: relative_ratio
  notes: "Cohort classifies model-vs-direct authority residuals by explicit relative threshold."
gate_lane: required
failure_class: hard-fail
runtime_cost_class: integration
owner: openWEPP maintainers
provenance:
  authored_by: Codex
  authored_utc: 2026-05-31
  last_updated_utc: 2026-05-31
notes: "AUTH10 promotion: Level-4 required/hard-fail direct-theta constitutive gate over required-case cohort."
---

# cas_l4_soil_fc_direct_theta_minus33_cohort_001 Soil Profile FC Direct Theta(-33kPa) Constitutive Gate Suite

## Purpose

Provide an independent (legacy-free) constitutive gate for profile field-capacity
storage by comparing model-published `ProfileFCStore` against direct
`Σ(theta_fc(-33kPa)_i * dg_i)` authority from soil inputs.

## Authority Links

- Contract invariants:
  - `SC-SOIL-001#INV-SOIL-014`
- External citations:
  - `EXT-SOIL-FC-CH7-001`

## Expected Behavior

1. Parse each cohort soil in strict mode.
2. Compute direct profile FC authority from layer theta_fc and depth intervals.
3. Compare runtime `wb13_profile_fc_store_mm` to direct authority and require
   per-case relative residual `<= 0.35`.
4. Report and classify residuals by rock-fragment bucket (`low`, `medium`,
   `high`) using weighted profile rock percentage.

## Fixture Coverage

1. 7778 measured-theta reference soil.
2. High-rock H1-like worked-example soil encoded from documented layer table.
3. Cohort configuration + source worked-example note for provenance continuity.

## Tolerance Policy

- Mode: `rel`
- Relative threshold: `0.35`
- Units: `relative_ratio`

## Gate and Failure Semantics

- Lane: `required`
- Failure class: `hard-fail`
- Failure action: block acceptance on direct-theta FC residual violations.

## Implementation Notes

- Executed by `tests/integration/auth07_fc_authority_cohort_contract.rs`.
- Fixture lock/provenance sidecars:
  - `tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/fixtures.sha256`
  - `tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/fixtures.provenance.yaml`
