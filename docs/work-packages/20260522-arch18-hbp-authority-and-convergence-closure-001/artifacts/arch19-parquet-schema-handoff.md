# ARCH19 Parquet Schema Handoff

Static: handoff prepared from ARCH18 HBP authority/convergence outputs.
Ran: reference-source pin evidence captured.
Status: handoff-ready.

## Scope Boundary

ARCH18 does not author parquet boundary contracts; ARCH19 owns that closure.
This handoff provides concrete inputs only.

## Required ARCH19 Inputs from ARCH18

1. HBP parser authority output fields available for parquet mapping:
- schema/profile fields
- dimension fields (`hillslope_id`, `nyear`, `npart`, `nofe`, `max_layers`)
- deterministic day-directory entries
- schema2 payload block metadata
- path-resolution + warning metadata

2. Compatibility/governance constraints to preserve in parquet boundary specs:
- no text-pass fallback semantics
- strict vs compat observability
- shared warning ID `HBP-W-001`

3. Provenance pins to carry into ARCH19 contract text:
- `/workdir/wepp-forest` HEAD: `028feb2317a35a9ad3e578c0e5798631fc0e61bd`
- `/workdir/wepppyo3` HEAD: `6c92e3fa70e45838e2a4778ee70ceae88db8e42b`

## Reference Surfaces for ARCH19

- HBP reference reader model:
  - `/workdir/wepppyo3/wepp_interchange/src/hill_hbp.rs`
- Parquet writer mechanics:
  - `/workdir/wepppyo3/wepp_interchange/src/parquet.rs`
- openWEPP HBP parser output surface:
  - `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/hbp.rs`

## Suggested ARCH19 Acceptance Checks

1. Parquet schema fields are traceable to canonical HBP symbols/aliases.
2. Schema1/Schema2 branch data remains distinguishable in parquet export shape.
3. Compatibility warnings/path-resolution are not dropped during export.
4. Provenance SHA references are recorded in ARCH19 artifacts.
