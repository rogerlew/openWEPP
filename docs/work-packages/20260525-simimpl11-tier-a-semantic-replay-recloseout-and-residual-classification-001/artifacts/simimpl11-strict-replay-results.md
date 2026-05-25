# simimpl11-strict-replay-results

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- Strict lane provenance:
  - `artifacts/replay-run-20260525T001432Z/suite_dat/investigation/pl14s_provenance_manifest.json`
- Strict comparator JSON:
  - `artifacts/replay-run-20260525T001432Z/suite_dat/investigation/h5_wat_strict_comparator.json`

## Ran
- Strict branch execution status:
  - `strict_skipped=false`
  - `strict_required=true`
  - strict comparator return code: `0`
- Comparator outcome:
  - `strict_pass=false`
  - `status_counts={"structure_diff": 1}`
  - compared file: `H5.wat.dat`
  - baseline lines: `1123`
  - candidate lines: `1`
  - numeric values compared: `0`
- SHA256 fingerprints:
  - strict JSON: `d26050ac9cbf2d552e83826dd329c611cfbafcaffced4731e4f96a38e363c9c9`
  - candidate dat: `912f60703bb7fc265089a4be322faf432535b46554d4b19f157207aa4914c935`
