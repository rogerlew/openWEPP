# Phase A Baseline Profile: GAP-OFEHYB-002 / H2637

## Workspace and artifact info
- Command: `cd /home/workdir/openWEPP && git rev-parse --abbrev-ref HEAD`
  - Exit: `0`
  - Value: `main`
- Command: `cd /home/workdir/openWEPP && git rev-parse HEAD`
  - Exit: `0`
  - Value: `f7bafb4044ac8eb75ea834243279443a743e32fc`
- Command: `cd /home/workdir/openWEPP && git status --short`
  - Exit: `0`
  - Result at capture time: `?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-active-hybrid-time.log`, `?? .../artifacts/h2637-active-hybrid.log`, `?? .../artifacts/h2637-scratch/`, `?? .../artifacts/runner-build.log`

## Build command
- Command: `cargo build --release -p openwepp-runner --bins`
- Exit: `0`
- Log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/runner-build.log`

## Binary provenance
- Path: `/home/workdir/openWEPP/target/release/openwepp-cli-hill`
- Size (bytes): `9897672`
- Mtime: `2026-07-07 00:26:34.961399117 -0700`
- SHA256: `628486b358b94bf87f09880c0e3b687a924b33502967e08fba5145b0a8e72f51`

## H2637 scratch preparation
- Scratch target: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch`
- Source used: `.../20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/h2637-scratch`
- Copy method: `rsync -a --delete --exclude='output'` to avoid stale run logs

## Baseline run command(s)
### Attempt 1 (failed)
- Command: `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1 OPENWEPP_LANED_SHADOW_PROFILE=1 /usr/bin/time -v target/release/openwepp-cli-hill --run-dir <scratch> --run-file p2637.run.toml --output-dir <scratch>/output`
- Exit: `127`
- Log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-active-hybrid.log`
- Failure: `/usr/bin/time: cannot run OPENWEPP_LANED_ACTIVE=1: No such file or directory`

### Attempt 2 (successful)
- Command: `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1 OPENWEPP_LANED_SHADOW_PROFILE=1 /usr/bin/time -v target/release/openwepp-cli-hill --run-dir /home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch --run-file p2637.run.toml --output-dir /home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch/output`
- Exit: `0`
- Log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-active-hybrid-time.log`

## Timing slots
- User time (s): `38.39`
- System time (s): `0.01`
- Elapsed wall time (s): `38.41` (from `0:38.41`)
- Exit status line in log: `0`

## Laned active profile counters (from log)
- `solver_runs`: `17898`
- `solver_steps`: `7381407`
- `solver_steps_homogeneous`: `381501`
- `solver_steps_source_free`: `1739149`
- `solver_steps_implicit`: `980804`
- `implicit_equilibrium_map_evaluations`: `151435969`
- `implicit_branch_evaluations`: `20110816`
- `alpha_evaluations`: `119746485`
- `solver_cfl_ns`: `19860432575`
- `solver_step_ns`: `7519343099`
- `solver_sample_ns`: `414248311`

## Output artifacts and hashes
- `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch/output/H2637.hbp`
  - SHA256: `939e37a7352c0f7a75c4004829a7a3886ee0f1b91820164e36fe7d734cde5fa5`
- `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch/output/H2637.loss.json`
  - SHA256: `725f57233fd60df097a824a2c20f26992a58b3a457594245a9ac91d2278f3cfb`
- `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch/output/H2637.pass.parquet`
  - SHA256: `a26ddd09729b960d8fbed6bbb351d37f5307b21eb8cdb3c0003500f59d4fec04`
- `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch/output/openwepp_hillslope_run_manifest.json`
  - SHA256: `0e72834b73bf2c06036452e80ba83ff5f60b109f0fa0566d58b0858bcef43ce0`

## Comparison to GAP001 final baseline
Reference baseline (GAP001 final): user `37.96s`, implicit steps `980804`, map evals `151435969`, branch evals `20110816`
- User time: `38.39s` (delta `+0.43s`, `+1.13%`)
- Implicit steps: `980804` (delta `0`, `0%`)
- Map evals: `151435969` (delta `0`, `0%`)
- Branch evals: `20110816` (delta `0`, `0%`)
