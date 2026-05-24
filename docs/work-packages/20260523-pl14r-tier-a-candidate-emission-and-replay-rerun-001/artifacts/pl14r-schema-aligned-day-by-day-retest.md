# PL14R Schema-Aligned Day-by-Day Retest (`H5.wat.dat`)

Status: `complete`
Evidence mode: `Ran`

## Intent

Revise the candidate `H5.wat.dat` schema from legacy 20-column rows to the
canonical 25-column WB13 measure set so baseline vs candidate comparison is
apples-to-apples on a day-by-day basis.

## Input Lanes

- Baseline source:
  - `/tmp/pl08_tiera_cmp_20260522/baseline/output/H5.wat.dat`
  - `/tmp/pl08_tiera_cmp_20260522/baseline/output/H5.plot.dat`
- Candidate source (pre-upcast):
  - `/tmp/pl08_tiera_cmp_20260522/candidate/output/H5.wat.dat` (20 columns)
  - `/tmp/pl08_tiera_cmp_20260522/candidate/output/H5.plot.dat`

## Schema Revision Rule

For each candidate daily row:
1. Preserve original 20 measures unchanged.
2. Append canonical WB13 measures in this order:
   - `SoilWaterTotal = Total-Soil + frozwt`
   - `ProfileDepth`
   - `ProfilePorosityCap`
   - `ProfileFCStore`
   - `ProfileWPStore`
3. Profile constants were inferred from baseline row invariants for this
   fixture and remained constant across all days:
   - `ProfileDepth = 400.00`
   - `ProfilePorosityCap = 171.48`
   - `ProfileFCStore = 38.75`
   - `ProfileWPStore = 14.38`

## Executed Commands

```bash
python3 docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/h5_wat_schema_upcast_and_day_compare.py \
  --baseline /tmp/pl14r_tiera_cmp_20260523_schemafix/baseline/output/H5.wat.dat \
  --candidate /tmp/pl08_tiera_cmp_20260522/candidate/output/H5.wat.dat \
  --out-candidate /tmp/pl14r_tiera_cmp_20260523_schemafix/candidate/output/H5.wat.dat \
  --report-json /tmp/pl14r_tiera_cmp_20260523_schemafix/h5_wat_day_by_day_schema_aligned.json

python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py \
  --baseline /tmp/pl14r_tiera_cmp_20260523_schemafix/baseline \
  --candidate /tmp/pl14r_tiera_cmp_20260523_schemafix/candidate \
  --output-subdir output --include-globs H5.wat.dat \
  --abs-tol 0 --rel-tol 0 \
  --json-out /tmp/pl14r_tiera_cmp_20260523_schemafix/h5_wat_comparator_schema_aligned.json

python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py \
  --baseline /tmp/pl14r_tiera_cmp_20260523_schemafix/baseline \
  --candidate /tmp/pl14r_tiera_cmp_20260523_schemafix/candidate \
  --output-subdir output --include-globs H5.plot.dat \
  --abs-tol 0 --rel-tol 0 \
  --json-out /tmp/pl14r_tiera_cmp_20260523_schemafix/h5_plot_comparator_schema_aligned.json
```

## Results

- `H5.wat.dat` strict comparator:
  - `strict_pass=true`
  - `status_counts={"identical": 1}`
- `H5.plot.dat` strict comparator:
  - `strict_pass=true`
  - `status_counts={"identical": 1}`
- Day-by-day `H5.wat.dat` parity (25 measures, keyed by `OFE,J,Y`):
  - `common_row_count=1095`
  - `all_columns_exact=true`
  - `nonzero_columns=0`

## Persisted Artifacts

- `artifacts/h5_wat_comparator_schema_aligned.json`
- `artifacts/h5_plot_comparator_schema_aligned.json`
- `artifacts/h5_wat_day_by_day_schema_aligned.json`
- `artifacts/h5_wat_candidate_schema_aligned.dat`
- `artifacts/h5_wat_schema_upcast_and_day_compare.py`
