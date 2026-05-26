# MOFE11 H324 Parity Rerun Report

Status: complete (lane rerun executed)
Evidence mode: Ran

Execution intent:
- Re-run carved-letter `H324` candidate lane after closing legacy
  `oratea/orater=0` runtime/decomposition guard compatibility.

Command:
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe11 --policy compat`

Result:
- Prior blocker (`HS-RUNTIME-E-050` on `oratea=0`) is closed.
- Lane completed successfully; manifest recorded in:
  - `/tmp/openwepp_mofe324_semantic_parity/output_mofe11/openwepp_hillslope_run_manifest.json`
- Candidate outputs emitted (resolved by runfile output path):
  - `/tmp/openwepp_mofe324_semantic_parity/output/H324.hbp`
  - `/tmp/openwepp_mofe324_semantic_parity/output/H324.loss.json`
  - `/tmp/openwepp_mofe324_semantic_parity/output/H324.plot.parquet`
  - `/tmp/openwepp_mofe324_semantic_parity/output/H324.wat.parquet`

Manifest highlights:
- `wb13_publication.publication_ofe_policy`:
  `single-row-canonicalized-hillslope-aggregate`
- `wb13_publication.contributor_ofe_count`: `2`
- `wb13_publication.row_count`: `2192`
- `first_row_key`: `year=1, julian_day=1, ofe=1`
- `last_row_key`: `year=6, julian_day=365, ofe=1`

Semantic comparator status:
1. Direct baseline dat comparator attempt:
- failed with `no baseline rows parsed` because baseline `H324.wat.dat` is
  26-column and comparator currently supports 20/25-column dat parsing.

2. Investigation-only normalized baseline comparator attempt:
- Baseline normalized by dropping trailing `InterceptionStorage` column into:
  `/tmp/openwepp_mofe324_semantic_parity/baseline_mofe11/H324.wat.25col.dat`
- Comparator run completed and produced:
  `/tmp/openwepp_mofe324_semantic_parity/output_mofe11/h324_semantic_report.json`
- Report summary:
  - `semantic_pass=false`
  - `common_row_count=0`
  - `only_baseline_count=4384`
  - `only_candidate_count=2192`

Interpretation:
- MOFE11 objective to close `oratea/orater=0` runtime/decomposition blocker is
  satisfied.
- Semantic parity closure remains blocked by comparator/baseline row-identity
  mismatch (`Y=2020..2025` and multi-OFE baseline rows versus
  `Y=1..6` single-row canonicalized candidate keys).
