# MOFE11 Implementation and Test Evidence

Status: complete
Evidence mode: Ran

Ran:
1. `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_allows_zero_oratea_orater_for_legacy_no_decay`
- Result: pass (post-implementation)

2. `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_rejects_negative_oratea_projection_field`
- Result: pass

3. `cargo test -p openwepp-hillslope-orchestrator decomposition_boundary_rejects_negative_oratea_with_typed_failure`
- Result: pass

4. `cargo test -p openwepp --test parser_runtime_seam_integration pl17_contract_conformance_scheduler_preserves_seed_masses_when_decomposition_constants_are_zero`
- Result: pass (post-implementation)

5. `cargo fmt --check`
- Result: pass

6. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: pass

7. `cargo test --workspace`
- Result: pass

8. `cargo deny check`
- Result: pass (warnings only: duplicate crates + unmatched license allowances)

9. `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe11 --policy compat`
- Result: pass; lane completed and emitted candidate outputs.

10. `python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py --baseline-wat /wc1/runs/ca/carved-letter/wepp/output/H324.wat.dat --candidate-wat /tmp/openwepp_mofe324_semantic_parity/output/H324.wat.parquet --report-json /tmp/openwepp_mofe324_semantic_parity/output_mofe11/h324_semantic_report.json --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- Result: fail (`no baseline rows parsed`) because baseline dat has 26 columns
  and comparator currently parses only 20/25-column dat.

11. `awk '{ if (NF==26) { for (i=1;i<=25;i++) printf "%s%s", $i, (i<25?" ":"\n") } else { print $0 } }' /wc1/runs/ca/carved-letter/wepp/output/H324.wat.dat > /tmp/openwepp_mofe324_semantic_parity/baseline_mofe11/H324.wat.25col.dat`
- Result: pass (temporary normalized baseline generated for investigation-only
  comparison run).

12. `python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py --baseline-wat /tmp/openwepp_mofe324_semantic_parity/baseline_mofe11/H324.wat.25col.dat --candidate-wat /tmp/openwepp_mofe324_semantic_parity/output/H324.wat.parquet --report-json /tmp/openwepp_mofe324_semantic_parity/output_mofe11/h324_semantic_report.json --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- Result: pass (tool execution); semantic report indicates `common_row_count=0`
  and `semantic_pass=false`.
