# Pre-Implementation Contract Gate

Status: complete

Evidence mode: ran

Ran:

- `cargo test -p openwepp-runner hphys0250_scheduler_lifecycle_preserves_pl_runtime_sentinel_for_ep_lineage hphys0250_wb13_ep_publication_consumes_final_root_uptake_flux -- --nocapture`
  was used as the pre-implementation HPHYS0250 contract gate before production
  edits. Log: `gate-logs/pre_impl_hphys0250_contract_tests.log`.

Result:

- Expected failure observed: `hphys0250_scheduler_lifecycle_preserves_pl_runtime_sentinel_for_ep_lineage`
  failed because pre-implementation runner code stripped `pl_schedule_slot_count`,
  suppressing PL scheduler activation needed for root-depth and final `Ep`
  lineage.
- Control test passed: `hphys0250_wb13_ep_publication_consumes_final_root_uptake_flux`
  passed, showing WB13 flux preference was already covered or independently
  correct at that pre-implementation point.
- The gate established that HPHYS0250 required production work on PL scheduler
  activation; additional tests were then added for zero-date perennial dispatch,
  initial live-canopy assimilation, growth/decomposition writeback, and WB15
  near-zero publication.

Truthfulness note:

- This artifact was corrected after QA review. The referenced pre-implementation
  log records one expected failing runner HPHYS0250 test and one passing runner
  HPHYS0250 test, not a passing `wb17_et_physics_kernel_contract` gate.
