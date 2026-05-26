# MOFE13 H324 Parity Rerun Report

Status: complete (lane rerun executed)
Evidence mode: Ran

Execution intent:
- Re-run carved-letter `H324` lane after WB14 `ksatadj` regime alignment and
  record semantic comparator posture.

Command:
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe13 --policy compat`

Lane result:
- Command completed successfully.
- Manifest:
  - `/tmp/openwepp_mofe324_semantic_parity/output_mofe13/openwepp_hillslope_run_manifest.json`
- Candidate outputs (resolved runfile output path):
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
1. Canonical baseline comparator run:
- `/workdir/wepppy/.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py --baseline-wat /wc1/runs/ca/carved-letter/wepp/output/H324.wat.dat --candidate-wat /tmp/openwepp_mofe324_semantic_parity/output/H324.wat.parquet --report-json /tmp/openwepp_mofe324_semantic_parity/output_mofe13/h324_semantic_report.json --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- Result: fail (`no baseline rows parsed`), same 26-column baseline parser-width
  limitation observed previously.

2. Investigation-only normalized baseline run:
- `awk '{ if (NF==26) { for (i=1;i<=25;i++) printf "%s%s", $i, (i<25?" ":"\\n") } else { print $0 } }' /wc1/runs/ca/carved-letter/wepp/output/H324.wat.dat > /tmp/openwepp_mofe324_semantic_parity/baseline_mofe13/H324.wat.25col.dat`
- `/workdir/wepppy/.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py --baseline-wat /tmp/openwepp_mofe324_semantic_parity/baseline_mofe13/H324.wat.25col.dat --candidate-wat /tmp/openwepp_mofe324_semantic_parity/output/H324.wat.parquet --report-json /tmp/openwepp_mofe324_semantic_parity/output_mofe13/h324_semantic_report.json --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- Result: pass (tool execution), report indicates:
  - `semantic_pass=false`
  - `common_row_count=0`
  - `only_baseline_count=4384`
  - `only_candidate_count=2192`

Interpretation:
- MOFE13 objective (WB14 `ksatadj` regime migration + lane rerun) is complete.
- Semantic parity closure remains blocked by row-key identity mismatch
  (`Y=2020..2025` + multi-OFE baseline rows versus canonicalized
  `Y=1..6, OFE=1` candidate rows), not by lane execution failure.
