# EROD21 Route Semantic Parity Evidence Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- EROD21 is the ROUTEPLAN01 final closure gate for sediment-routing hold-lift
  disposition after EROD20.
- Admissible evidence target for this package is route branch-family rerun
  evidence from contract-derived route vectors and runner continuity lanes.

## Ran
Replay bundle: `artifacts/replay-run-20260526T210606Z/`

1. Full route contract suite rerun
- Command: `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract -- --nocapture`
- Result: pass (`route_contract_suite_full.exit_code=0`)
- Evidence: `14 passed; 0 failed`, including all EROD17 route branch vectors and
  EROD18 route topology guard vectors.

2. Focused route branch rerun
- Command: `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract erod17_contract_ -- --nocapture`
- Result: pass (`route_contract_suite_erod17_focus.exit_code=0`)
- Evidence: `5 passed; 0 failed` across `mshear`, deposition-end, `ndep`
  follow-up, `qostar` threshold, and core seam publication vectors.

3. MOFE03 runner continuity rerun
- Command: `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`
- Result: pass (`runner_mofe03_lane.exit_code=0`)
- Evidence: `2 passed; 0 failed` for multi-OFE Wave-2-enabled and single-OFE
  Wave-2-disabled policy lanes.

4. Route-window literal regression scan
- Command: `sed -n '6100,6660p' crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs | rg -n "0\.005|1\.0e-8|Vec::with_capacity\(5 \+ \(class_count \* 6\)\)|\(1\.\.=4\)"`
- Result: no matches (`magic_number_route_window_scan.exit_code=1` with
  expected ripgrep no-match semantics).

## Residual classification
- No blocking rerun residuals observed for route branch-family closure scope.
- Comparator infrastructure in `tools/legacy_comparison_suite` remains WAT-only;
  route-branch closure evidence in EROD21 therefore relies on route
  contract-derived vectors plus runner continuity reruns.
