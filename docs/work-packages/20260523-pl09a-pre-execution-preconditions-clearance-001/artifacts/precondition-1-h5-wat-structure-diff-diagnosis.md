# PL09A Precondition 1: `H5.wat.dat` `structure_diff` Diagnosis

Status: `complete`
Evidence mode: `Static + Ran`
Disposition: `cleared`

Static:
- PL08 reported Tier-A `structure_diff` with line-count and numeric-arity
  mismatches.

Ran:
- Parsed persisted replay outputs from:
  - `/tmp/pl08_tiera_cmp_20260522/baseline/output/H5.wat.dat`
  - `/tmp/pl08_tiera_cmp_20260522/candidate/output/H5.wat.dat`
- Validated key-row overlap and first-20-column parity on `(OFE,J,Y)` rows.

## Diagnostic Result

1. Structural mismatch is primarily output schema/header shape, not shared
   keyed-row value drift:
   - line counts: baseline `1123`, candidate `1118` (delta `5`)
   - numeric keyed rows: baseline `1095`, candidate `1095`
   - shared keyed rows: `1095`
   - first 20 columns mismatches on shared keyed rows: `0`
2. Baseline numeric rows are 25-column; candidate numeric rows are 20-column.
   Baseline includes five trailing fields absent from candidate output.
3. First text divergence begins at line 18 with additional baseline explanatory
   header lines (e.g., `SoilWaterTotal=...`) and shifted separator/header
   placement.

## Precondition Closure Decision

`cleared`.

Interpretation:
- The reported Tier-A `structure_diff` is now diagnosed to a concrete
  schema/header divergence class.
- Queue execution can proceed with this explicit diagnosis in scope; full
  comparator closeout remains in PL14/PL15.

## Queue Impact

- No queue descope is authorized in this package.
- PL14/PL15 should treat `H5.wat.dat` as a known schema-alignment risk class
  in addition to process-kernel parity risk.

## Evidence Links

- `/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/h5_wat_comparator.json`
- `/tmp/pl08_tiera_cmp_20260522/baseline/output/H5.wat.dat`
- `/tmp/pl08_tiera_cmp_20260522/candidate/output/H5.wat.dat`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/claude-pl09-pre-execution-review.md:257`
