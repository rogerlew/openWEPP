# simimpl16-alias-and-provenance-test-coverage-evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Alias continuity coverage:
- `Total-Soil` required canonical investigation marker is explicitly asserted.
- replay semantic marker checks retain lineage for both
  `Total-Soil` and `Total-Soil Water` mapping authority.
- Provenance coverage:
- required script markers now include `common_row_count`,
  `conversion_source_row_consistency_ready`, and
  `conversion_source_row_consistency_blockers`.
- Harness enforces conversion-derived dat row-consistency blockers before
  closeout readiness is emitted.

## Ran
- `pl14_contract_conformance_requires_total_soil_in_investigation_columns` passed.
- `pl14s_contract_conformance_declares_semantic_report_and_provenance_schema_markers` passed.
- `pl14s_contract_conformance_requires_conversion_dat_row_consistency_for_evidence_readiness` passed.
