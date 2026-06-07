# Gate Results

Status: closed-with-follow-up-postreview

Evidence mode: Ran

| Gate | Status | Evidence |
|---|---|---|
| Reproduce failure | pass | p7 pre-fix `HKERNEL-WB14-RUNOFF-E-003` at J-95 |
| Localize mechanism | pass | separate SWE debit overdraw in signed-melt redistribution |
| Contract authority | pass | `SC-SNOWFREEZE-001` v52, `SC-WATBAL-001` v145 |
| Red regression | pass | focused test failed pre-fix |
| Production correction | pass | single-source routed/storage scalar |
| Focused regression | pass | focused test passed |
| Package tests | pass | `cargo test -p openwepp-hillslope-orchestrator` |
| Lint | pass | `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` |
| J-95 validation | pass | `p7`, `p11`, `p18`, `p20` published |
| Full workspace tests | pass | `cargo test --workspace` passed on 2026-06-07 |
| Workspace clippy | pass | `cargo clippy --workspace --all-targets -- -D warnings` passed |
| Cargo deny | pass | `cargo deny check` passed; existing duplicate-crate and unmatched-license warnings only |
| H1..H39 semantic gate | pass-with-open-parity | Runtime `39/39` rc=0; semantic comparator `0/39` pass, `0` structural failures; summary at `/tmp/snowsci_stage1_full_release_20260607T021210Z/reports/hillslope_semantic_summary.md` |
| WBVAL06 common-cause measurement | measured-follow-up | Before max annual R `94.433070 mm`; after max annual R `26.790809 mm` on the 18 WBVAL04 status-valid emitters; all remain above `1.0 mm/year`; summary at `/tmp/snowsci_stage1_wbval06_after_20260607T021725Z/reports/wbval06_before_after_residual_summary.md` |
| Dual independent review | limited | Claude review recorded and dispositioned; truly independent dual sub-agent review remains unmet |
