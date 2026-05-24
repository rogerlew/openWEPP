# PL14R Comparator JSON Artifact Index

Status: `complete`
Evidence mode: `Ran`

## Persisted Artifacts

1. `artifacts/h5_wat_comparator.json`
- Source run path: `/tmp/pl14r_tiera_cmp_20260523/h5_wat_comparator.json`
- SHA256: `31086f2dffc4d8790436b24f4e0b40d4982fbcf0e397890abcba2410f09b0abc`
- Summary: `strict_pass=false`, `raw.status_counts.structure_diff=1`

2. `artifacts/h5_plot_comparator.json`
- Source run path: `/tmp/pl14r_tiera_cmp_20260523/h5_plot_comparator.json`
- SHA256: `2f8fe012e027403f63e35d746166de652cc6f4c36626f358d2ce3175bcbf9de7`
- Summary: `strict_pass=false`, `raw.only_baseline_count=1`,
  `raw.only_baseline_examples=["H5.plot.dat"]`

3. `artifacts/h5_wat_comparator_schema_aligned.json`
- Source run path:
  `/tmp/pl14r_tiera_cmp_20260523_schemafix/h5_wat_comparator_schema_aligned.json`
- SHA256: `abb3cc0eaaae6a87c98120a0666006550d00a75345d8bef6da3b67ab3ea53dd7`
- Summary: `strict_pass=true`, `raw.status_counts.identical=1`

4. `artifacts/h5_plot_comparator_schema_aligned.json`
- Source run path:
  `/tmp/pl14r_tiera_cmp_20260523_schemafix/h5_plot_comparator_schema_aligned.json`
- SHA256: `ba528b3d2259e0c3d6c148b22885abf7da68d3e1bada6b9908f3b667c387a00c`
- Summary: `strict_pass=true`, `raw.status_counts.identical=1`

5. `artifacts/h5_wat_day_by_day_schema_aligned.json`
- Source run path:
  `/tmp/pl14r_tiera_cmp_20260523_schemafix/h5_wat_day_by_day_schema_aligned.json`
- SHA256: `1643f5e834a3a1d918aab33915c2d2bc0817bf4c33b522c82deb0e05a1cf31b0`
- Summary: `day_compare.all_columns_exact=true`, `day_compare.common_row_count=1095`

## Comparator JSON Schema Provenance

- Comparator tool: `/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py`
- Comparator tool SHA256:
  `c9c5f2eac59cdd4c6b8f7bc8423577e679effd68554b87ff62abf76371af91c8`
