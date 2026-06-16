# PERFHO02 Worker Handoff

Status: HANDOFF READY 2026-06-16
Evidence mode: **Static**

## Summary

PERFHO02 characterized the post-PERFOPT01 H2637 residual. The next optimization should not target output writers. It should target repeated hydrology symbol creation/lookup and secondary writeback application sort/allocation/insertion.

## Important Paths

- Raw GDB log: `/tmp/perfho02/gdb-h2637-post-perfopt.txt`
- Perf data: `/tmp/perfho02/perf-h2637-post-perfopt.data`
- Perf reports: `/tmp/perfho02/perf-h2637-report-nochildren.txt`, `/tmp/perfho02/perf-h2637-report-children.txt`
- Runfile: `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/runfiles/perfho02-h2637.run`
- Profiler evidence: `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/perfho02-profiler-evidence.md`
- Verdict: `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/perfho02-residual-verdict.md`

## First Actionable Follow-On

Scaffold `PERFOPT02-symbol-access-and-writeback-application`.

Recommended first targets:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/00_pl_slot_resolution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs`

Hard constraints for PERFOPT02:

- bit-identical H2637 + OFE ladder outputs against a pre-optimization baseline;
- no FP reduction-order changes;
- no fail-closed guard weakening;
- no science-contract or output-schema change unless a later package explicitly changes authority first.
