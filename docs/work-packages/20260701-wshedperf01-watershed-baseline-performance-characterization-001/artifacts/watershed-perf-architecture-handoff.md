# Watershed Performance Architecture Handoff

Status: `UPDATED`

## Required Conclusions

- Measured baseline is split by scope:
  - `full-legacy-watershed` (pinned baseline binary): `0:07.86` wall on `/tmp/wshedperf01_20260701_081511/timing/legacy_pw0_timed_1.time`.
  - `routed-stage-from-existing-hbp` (openWEPP): `~0:00.07` wall on `/tmp/wshedperf01_20260701_081511/timing/openwepp_watershed_from_hbp_timed_1.time` and repeat set in `/tmp/wshedperf01_20260701_083200/timing/repeat_openwepp_routed/`.
- Ratio by strict scope is **not directly comparable**:
  - legacy sample executes full watershed pipeline with legacy executable behavior.
  - openWEPP routed sample starts from existing HBP pass/manifests and bypasses full hillslope execution.

- Practical full openWEPP end-to-end timing is now **validated** (single run):
  - `1:02.27` wall, user `61.74`, system `0.53`, max RSS `16,896 KB`.
  - evidence: `/tmp/wshedperf01_20260701_102200/timing/openwepp_watershed_end2end_full_validated.time`.
- Historical blockers remain documented in the log, but this command path is no longer an active blocker:
  - `CLIHILL-E-010`, `CLIWAT-E-021`, and `pipeline_output/H1.hbp` pathing from earlier variants.

- Dominant openWEPP end-to-end cost signal (current package phase):
  - Routed-stage CLI remains short (~0.07–0.11s, 8 MiB RSS), so the end-to-end command cost is dominated by hillslope orchestration and execution.
  - The validated full path includes 36 hillslope CLI invocations plus watershed routing and produces non-empty routed outputs.
  - This confirms hillslope-runfile materialization and concurrency are the first-order optimization target before routed-stage tuning.

- CPU-scalable decomposition candidates
  - Parallelize/robustify hillslope dispatch and runfile/materialization first.
  - Validate deterministic runfile path contracts:
    - `inputs.soil/man/man/run` relative path roots for `runfiles/end2end*` variants.
    - `use_existing_pass_file` and manifest/pass path topology.
  - After successful hill orchestration, measure watershed routing with 1..N concurrency profiles and split between generation vs routing.

- Hillslope performance lessons to carry forward
  - Existing perf work already shows hillslope command surfaces are materially longer than routed-stage; end-to-end work should not optimize routed-only path before hill-stage correctness and input topology are fixed.
  - Sidecar warnings and fallback behavior (`legacy-sidecar-discovery`) appear repeatedly and should be removed from release characterization once paths stabilize.

## Next work package recommendation

- Boundary: make full openWEPP end-to-end artifact surface valid and repeatable.
- Suggested package boundary (next package):
  - rerun a clean 3+ repeat full openWEPP pipeline with the validated `runfiles/p${i}_end2end3.run` + `wshedperf01_openwepp_watershed_end2end_final.run` contract to capture variance and stability;
  - collect `perf` attribution on both hillslope dispatch and routed stage for the full chain;
  - keep legacy scope as canonical non-comparable reference and continue to report strict scope labels.
- Write set: package-local artifacts only (no production source changes).
