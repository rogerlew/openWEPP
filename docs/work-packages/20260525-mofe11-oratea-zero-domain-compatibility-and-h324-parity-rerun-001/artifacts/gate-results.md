# Gate Results

Status: complete
Evidence mode: Ran

## Contract-first gates
- `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_allows_zero_oratea_orater_for_legacy_no_decay` -> fail (expected pre-implementation gate)
- `cargo test -p openwepp --test parser_runtime_seam_integration pl17_contract_conformance_scheduler_preserves_seed_masses_when_decomposition_constants_are_zero` -> fail (expected pre-implementation gate)
- `cargo test -p openwepp-hillslope-orchestrator decomposition_boundary_rejects_negative_oratea_with_typed_failure` -> pass (typed negative guard retained)

## Post-implementation validation
- `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_allows_zero_oratea_orater_for_legacy_no_decay` -> pass
- `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_rejects_negative_oratea_projection_field` -> pass
- `cargo test -p openwepp-hillslope-orchestrator decomposition_boundary_rejects_negative_oratea_with_typed_failure` -> pass
- `cargo test -p openwepp --test parser_runtime_seam_integration pl17_contract_conformance_scheduler_preserves_seed_masses_when_decomposition_constants_are_zero` -> pass
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only: duplicate crate versions + unmatched license allowances)

## Parity lane execution
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe11 --policy compat` -> pass (candidate outputs emitted)

## Semantic comparator execution
- Direct comparator against `/wc1/runs/ca/carved-letter/wepp/output/H324.wat.dat` -> fail (`no baseline rows parsed`, 26-column baseline dat unsupported by current 20/25 parser).
- Investigation-only comparator against normalized baseline `/tmp/openwepp_mofe324_semantic_parity/baseline_mofe11/H324.wat.25col.dat` -> executed, `semantic_pass=false`, `common_row_count=0`.
