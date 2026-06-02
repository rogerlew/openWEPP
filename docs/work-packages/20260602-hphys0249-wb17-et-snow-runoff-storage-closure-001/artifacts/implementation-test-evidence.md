# Implementation Test Evidence

Status: complete

Evidence mode: static + ran

Ran:

- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`
  - Pre-implementation log:
    `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/artifacts/gate-logs/pre_impl_wb17_contract_test.log`
  - Result before production edits: failed expected HPHYS0249 vectors (`5`
    passed, `2` failed).
  - Post-implementation log:
    `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/artifacts/gate-logs/post_impl_wb17_contract_test.log`
  - Result after first production edits: passed `7/7`.
  - Final log after review fixes:
    `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/artifacts/gate-logs/final_wb17_contract_test.log`
  - Final result: passed `9/9`.
- Full `H1..H39` runtime and semantic suite:
  - Root: `/tmp/hphys0249_20260602T161254Z_postreview`
  - Runtime success: `39/39`.
  - Semantic report success: `39/39`.
  - Semantic pass: `0/39`.
  - Common rows: `1461..1461`.
  - Summary:
    `/tmp/hphys0249_20260602T161254Z_postreview/reports/hillslope_semantic_summary.md`
- Final broad gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
  - `git diff --check`
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo test --test auth11_required_suite_obligation_guards_contract`

Static:

- HPHYS0249 corrected the targeted WB17 soil-evaporation layer-storage defect.
- Review fixes split root uptake into a post-WB19 scheduler phase and restored
  baseline residue add-back behavior.
- HPHYS0249 did not close `Ep`, snow/runoff timing, or aggregate storage;
  disposition remains `HOLD`.
