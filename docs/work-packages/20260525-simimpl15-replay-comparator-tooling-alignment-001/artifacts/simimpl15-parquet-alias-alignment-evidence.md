# simimpl15-parquet-alias-alignment-evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- `semantic_hillslope_wat_compare.py` alias map includes:
- `Total-Soil -> Total-Soil`.
- `Total-Soil Water -> Total-Soil`.
- Semantic inputs now emit alias-source diagnostics in report metadata:
- `baseline_column_alias_sources`.
- `candidate_column_alias_sources`.
- Width diagnostics for parquet are based on observed row field counts, not placeholder sentinel widths.

## Ran
- `pl14_contract_conformance_requires_total_soil_in_investigation_columns` passed.
- `pl14s_contract_conformance_declares_semantic_report_and_provenance_schema_markers` passed.
- Targeted SIMIMPL15 integration set passed.
