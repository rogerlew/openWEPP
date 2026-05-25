# simimpl13-implementation-and-test-evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- SIMIMPL13 produced assessment artifacts only.
- No production Rust/kernel source files were modified in SIMIMPL13 scope.
- No replay rerun was executed in this package; SIMIMPL11 replay artifacts were
  consumed as authority evidence.

## Ran
- Evidence extraction and audit commands executed:
  - `sed -n` across package, contract, and upstream SIMIMPL11 artifacts.
  - `rg -n` across runner/comparator/test surfaces.
  - `awk` numeric-row probes for baseline/candidate `H5.wat.dat` surfaces.
  - `duckdb` row-count/schema probes for candidate parquet surface.
  - `python` JSON probes for strict/semantic comparator metrics and provenance.
- Key observed outputs used in disposition:
  - baseline keyed rows: `1095`
  - candidate keyed rows: `1`
  - semantic overlap: `common_row_count=0`
