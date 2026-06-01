---
suite_id: cas_l4_subhyd_watyld_fcwp_consistency_001
title: WB19 FC/WP + COCA Water-Yield Consistency Suite
status: active
authority_level: 4
domain: subhyd
process_family: lateral_drain
sc_invariant_refs:
  - SC-SUBHYD-001#INV-SUBHYD-019
  - SC-WATBAL-001#INV-WATBAL-009
external_citations:
  - citation_id: EXT-SUBHYD-WB19-CH6-001
    source_type: book
    title: WEPP User Summary Chapter 6 Subsurface Hydrology and Drainage
    locator: references/50201000/chap6.pdf
    version_or_edition: NSERL Report No. 11 (August 1995)
    retrieved_utc: 2026-06-01
    notes: "WB19 coupling computes avfca from theta_fc and couples watyld into fcdep mutation."
fixtures:
  - fixture_id: FX-SUBHYD-WB19-FCWP-COCA-WATYLD-001
    path: tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/wb19_fcwp_coca_watyld_cases.json
    fixture_class: component
    units_basis: SI
    hash: 42dbffae3d690c680bda3124935cc25e511266bb42511cbe97649b7144a68948
    source_repo: /workdir/openWEPP
    source_commit: 236ecee254b7c1672cade901a39cce4352c907b1
    source_path: tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/wb19_fcwp_coca_watyld_cases.json
    source_sha256: 42dbffae3d690c680bda3124935cc25e511266bb42511cbe97649b7144a68948
    transform_note: "Repository-authored paired FC/WP theta lineage fixture for WB19 avfca/watyld authority."
tolerances:
  mode: abs
  abs:
    value: 1.0e-12
    comparator: "<="
  units: m_and_dimensionless
  notes: "q/watyld/fcdep/unsdep comparisons use strict absolute tolerance."
gate_lane: required
failure_class: hard-fail
runtime_cost_class: component
owner: openWEPP maintainers
provenance:
  authored_by: Codex
  authored_utc: 2026-06-01
  last_updated_utc: 2026-06-01
notes: ""
---

# cas_l4_subhyd_watyld_fcwp_consistency_001 WB19 FC/WP + COCA Water-Yield Consistency Suite

## Purpose

Validate WB19 water-yield and saturated-depth coupling authority with explicit
FC/WP theta lineage:

1. `avfca` follows `thetfc_####` (not `wb18_perc_fc_####/dg_####` surrogate).
2. `watyld = avpora - (avfca + (1-avcoca))`.
3. For `solwpv < 2006`, `fcdep` mutation follows `q/watyld`.
4. Under fixed `drfc` lineage and fixed forcing, realized `q` remains stable
   while FC/WP theta lineage perturbs `watyld` and downstream `fcdep`.

## Authority Links

- Contract invariants:
  - `SC-SUBHYD-001#INV-SUBHYD-019`
  - `SC-WATBAL-001#INV-WATBAL-009`
- External citation:
  - `EXT-SUBHYD-WB19-CH6-001`

## Expected Behavior

1. Case pair executes nominal WB19 lateral status.
2. Case pair preserves near-identical realized `q` under fixed `drfc`/forcing.
3. Higher `thetfc` produces lower `watyld` under fixed `por`/`coca`.
4. Lower `watyld` increases `q/watyld` drawdown and reduces post-phase `fcdep`.

## Fixture Coverage

1. `low_fc_theta`: baseline FC/WP theta lineage.
2. `high_fc_theta`: increased `thetfc` and paired `thetdr` maintaining fixed
   `wb18_perc_fc`/`drfc`.

## Gate and Failure Semantics

- Lane: `required`
- Failure class: `hard-fail`
- Failure action: block acceptance until WB19 FC/WP + COCA coupling authority
  is restored.

## Implementation Notes

- Executed by
  `tests/integration/hphys0227_wb19_fcwp_coca_watyld_authority_contract.rs`.
