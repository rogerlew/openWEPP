# MOFE13 Implementation and Test Evidence

Status: complete
Evidence mode: Ran

Ran:
1. `cargo test -p openwepp --test wb14_infiltration_hyetograph_kernel_contract ksatadj`
- Result: initial fail during test-authoring refinement; pass after runtime-state
  aligned expected-`Ke` calculation update.

2. `cargo test -p openwepp --test wb14_infiltration_hyetograph_kernel_contract wb14_contract_conformance_rejects_active_9001_zero_ksatrec`
- Result: pass.

3. `cargo test -p openwepp-hillslope-orchestrator ksatadj`
- Result: pass.

4. `cargo fmt --check`
- Result: fail (formatting only).

5. `cargo fmt`
- Result: pass.

6. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: fail (strict lints), then pass after lint-conformant refactor.

7. `cargo test --workspace`
- Result: pass.

8. `cargo deny check`
- Result: pass (warnings only: duplicate crates + unmatched license allowances).

9. `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe13 --policy compat`
- Result: pass; lane completed with candidate output emission.

10. `python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py --baseline-wat /wc1/runs/ca/carved-letter/wepp/output/H324.wat.dat --candidate-wat /tmp/openwepp_mofe324_semantic_parity/output/H324.wat.parquet --report-json /tmp/openwepp_mofe324_semantic_parity/output_mofe13/h324_semantic_report.json --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- Result: fail (environment): system `python` missing `pyarrow`.

11. `/workdir/wepppy/.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py --baseline-wat /wc1/runs/ca/carved-letter/wepp/output/H324.wat.dat --candidate-wat /tmp/openwepp_mofe324_semantic_parity/output/H324.wat.parquet --report-json /tmp/openwepp_mofe324_semantic_parity/output_mofe13/h324_semantic_report.json --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- Result: fail (`no baseline rows parsed` due 26-column baseline).

12. `awk '{ if (NF==26) { for (i=1;i<=25;i++) printf "%s%s", $i, (i<25?" ":"\\n") } else { print $0 } }' /wc1/runs/ca/carved-letter/wepp/output/H324.wat.dat > /tmp/openwepp_mofe324_semantic_parity/baseline_mofe13/H324.wat.25col.dat`
- Result: pass (investigation-only normalized baseline).

13. `/workdir/wepppy/.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py --baseline-wat /tmp/openwepp_mofe324_semantic_parity/baseline_mofe13/H324.wat.25col.dat --candidate-wat /tmp/openwepp_mofe324_semantic_parity/output/H324.wat.parquet --report-json /tmp/openwepp_mofe324_semantic_parity/output_mofe13/h324_semantic_report.json --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- Result: pass (tool execution); semantic report remains `semantic_pass=false`
  with `common_row_count=0`.
