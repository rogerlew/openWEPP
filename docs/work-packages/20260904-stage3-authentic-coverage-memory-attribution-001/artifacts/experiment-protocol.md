# Experiment protocol

Status: `PHASE 3 COMPLETE — R1 CONTROL ADMITTED`

Evidence mode: `Static + Ran`

Initial freeze time: `2026-09-04 America/Los_Angeles`, after the exploratory fallback
probe and before the paired control loop. The exploratory one-run release probe
(`raw/current_baseline_profile_run1.log`) and the first 12-run observer loop
(`raw/exploratory_memory_results_12.jsonl`) remain explicitly non-frozen and are
not used for causal attribution.

Workload identity is the retained, real runner test
`hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile` from
`crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs`.
Build once before measurement; run fresh processes from the exact binary named
in `baseline-and-recovery-manifest.md`, pinned with `taskset -c 0`,
`RUST_MIN_STACK=67108864`, release profile, `CARGO_PROFILE_RELEASE_LTO=false`,
and one test thread. No allocator preload or production treatment variable is
used. The paired script records the binary hash, argv, CPU affinity, sample
period, timeout, and result/sample hashes in
`raw/current_memory_manifest.jsonl`.

Protocol revision R1 was recorded before the admitted rerun: the observed arm
must first resolve `/proc/<pid>/exe` to the exact hashed binary; a pre-readiness
disappearance is `not_ready`, while a field omission after readiness is a hard
`field_gap` failure. R1 is implemented by
`raw/run_memory_observer.sh` (SHA-256 `70134949b52ca1a0024b6a53d2db1a8a0510e52723d6718fbe690677d1a2a4ec`)
and readiness is persisted in each observed result. The earlier completed set
without persisted readiness is retained under
`raw/frozen_control_attempt_without_persisted_readiness_20260904/`; the failed
field-gap set is retained under
`raw/aborted_frozen_attempt_20260904_block5_field_gap/` and neither is the
admitted result set.

The R1 admitted control is a balanced twelve-block AB/BA design (24 fresh processes,
after two unobserved warmups). Arm A runs the workload with no external memory
observer and retains only the runner's own endpoint `rss_kib`. Arm B runs the
same workload while an external observer samples `/proc/<pid>/status` and
`/proc/<pid>/smaps_rollup` every 20 ms. Odd blocks use AB and even blocks BA.
The observer requests a 20 ms inter-sample sleep (actual timestamps include
read/parse overhead and are not treated as an achieved cadence). It reports
`VmRSS`, `VmHWM`, smaps `Rss`, `Private_Clean`,
`Private_Dirty`, and their explicit sum; missing fields are counted and fail the
arm (never converted to zero). A per-process timeout is 120 s; timeout,
nonzero exit, missing probe JSON, missing samples, or field gaps reject that
arm. Build/fixture setup occurs inside the test process and is therefore
included in the runner's endpoint but not treated as a separate memory claim.

The runner endpoint is interpreted narrowly: its `rss_kib` is the first numeric
field of `VmRSS` in KiB, sampled during JSON construction while report,
telemetry, and output locals remain live, before fixture teardown. It is not a
peak, heap, or retained-allocation measure. External `VmHWM` and smaps values
are attribution diagnostics only; the observer's sampled HWM is a lower bound
if the process disappears before a final sample.

All successful arms must preserve the exact source/outlet/storage/clamp values,
parent-support and trial counters, accepted publication count, complete
qualification telemetry, output parsability, and teardown exit status. Offline
reconstruction checks pair ordering, count completeness, endpoint-vs-observer
field identity, and exact output closure. There is no production replay
activation, solver/equation/tolerance change, acceptance-rule change, or source
patch in this package.

Historical revision-31 and revision-61 outputs remain archival. Revision-31
candidate bytes are recoverable in `/tmp/v31_recover` plus ordered session
patches; revision-61 implementation patches are recoverable only as ordered
session payloads. No historical candidate is copied into the retained checkout
or treated as a matched re-run. Any reconstructed candidate must use an
isolated source/build tree and cannot qualify a production change.
