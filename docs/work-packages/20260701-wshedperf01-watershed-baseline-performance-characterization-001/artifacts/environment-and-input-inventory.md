# Environment and Input Inventory

Status: `UPDATED`

Record `Static:` and `Ran:` fields here before canonical timing.

## Repository

Ran:
- `git rev-parse HEAD` = `44b1658e0d026d4642f0360e175da1462eb0c3a0`.
- `git status --short` at start: `M docs/work-packages/README.md` and untracked package directory `docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/`.

Static: repository-local changes are limited to work-package artifacts.

## Legacy baseline

Ran:
- Baseline path: `/workdir/wepp-forest_260430_baseline/release/wepp_260430`.
- SHA256: `d1447a202a7e32f3bc892a1011787249764d57f61370f004a3d9ccb20aeb8a17`.
- Timed command used: `/tmp/wshedperf01_20260701_081511/timing/legacy_pw0_timed_1.time` (exit `0`).

## openWEPP binaries

Ran:
- Release watershed CLI: `/home/workdir/openWEPP/target/release/openwepp-cli-watershed`.
- SHA256: `209de0a054332c33d9c865c1cf2d898a1cdfef92f3988514fd8fc2ab2aaccc20`.
- Release hillslope CLI: `/home/workdir/openWEPP/target/release/openwepp-cli-hill`.
- SHA256: `ba2366d9251b2db9209bc4c3ae9ca43f3bfafb10449b633fbeffc1bbf2baa504`.

## Timing tools

Static:
- `/usr/bin/time` = GNU Time (`time (GNU Time) UNKNOWN`).
- `perf` = `perf version 6.8.12`.
- `openwepp-cli-watershed --version` not rerun for this phase because canonical timing used only command-level command lines from timed runs.

## CPU / runtime host

Ran:
- `nproc` = `48`.
- `lscpu` summary:
  - `Model name: Intel(R) Xeon(R) CPU E5-2697 v2 @ 2.70GHz`
  - `Socket(s): 2`
  - `Core(s) per socket: 12`
  - `Thread(s) per core: 2`
  - `L1d cache: 768 KiB (24 instances)`
  - `L2 cache: 6 MiB (24 instances)`
  - `L3 cache: 60 MiB (2 instances)`

## Inputs and run roots

Ran:
- Canonical arboreal-dendrite input root (as required): `/wc1/runs/ar/arboreal-dendrite/wepp`.
- Timing-run stage roots:
  - `/tmp/wshedperf01_20260701_081511/stage/arboreal-dendrite`
  - `/tmp/wshedperf01_20260701_083200/stage/arboreal-dendrite`
  - `/tmp/wshedperf01_20260701_102200/stage/arboreal-dendrite`
- Run files used by timed commands:
  - `wshedperf01_openwepp_watershed_hbp_manifest_cond.run`
  - `wshedperf01_openwepp_watershed_end2end.run`
  - `wshedperf01_openwepp_watershed_end2end_fixed*.run`
  - `wshedperf01_openwepp_watershed_hbp.run`
  - `wshedperf01_openwepp_watershed_hbp_manifest.run`
  - `wshedperf01_openwepp_watershed_end2end_final.run`
- Hillslope run file families observed in timed loops:
  - `runfiles/p{1..36}.run`
  - `runfiles/p{1..36}_end2end2.run`
  - `runfiles/p{1..36}_end2end3.run`
  - `runfiles/end2end/p{1..36}.run`
- Successful full pipeline run used in this phase:
  - `/tmp/wshedperf01_20260701_102200/timing/openwepp_watershed_end2end_full_validated.time`
  - `/tmp/wshedperf01_20260701_102200/outs/openwepp_end_to_end_validated/interchange`

## Expected output surfaces

Ran:
- Expected routed/partial watershed interchange parquet set (14 files), as emitted by successful routed-stage runs:
  - `chan.out.parquet`
  - `chanwb.parquet`
  - `chnwb.parquet`
  - `ebe_pw0.parquet`
  - `loss_pw0.all_years.chn.parquet`
  - `loss_pw0.all_years.class_data.parquet`
  - `loss_pw0.all_years.hill.parquet`
  - `loss_pw0.all_years.out.parquet`
  - `loss_pw0.chn.parquet`
  - `loss_pw0.class_data.parquet`
  - `loss_pw0.hill.parquet`
  - `loss_pw0.out.parquet`
  - `soil_pw0.parquet`
  - `totalwatsed3.parquet`

## Command-surface notes

Ran:
- Legacy baseline command surface: `/workdir/wepp-forest_260430_baseline/release/wepp_260430 --run-name pw0 ...` (resolved in timed file command line).
- openWEPP routed-stage command surface: `target/release/openwepp-cli-watershed --run-dir ... --run-file wshedperf01_openwepp_watershed_hbp_manifest_cond.run --output-dir ... --policy compat --legacy-sidecar-discovery`.
- openWEPP end-to-end exploratory command surface: looped `openwepp-cli-hill` over p-run files then `openwepp-cli-watershed` with `wshedperf01_openwepp_watershed_end2end_*.run` variants.
- openWEPP end-to-end validated command surface (success):
  - `bash -lc 'TS_DIR=\"/tmp/wshedperf01_20260701_102200\"; RUN_DIR=\"$TS_DIR/stage/arboreal-dendrite\"; LOG_DIR=\"$TS_DIR/logs\"; for i in $(seq 1 36); do openwepp-cli-hill --run-file \"runfiles/p${i}_end2end3.run\" --run-dir \"$RUN_DIR\" --output-dir \"$RUN_DIR/output\" ...; done; openwepp-cli-watershed --run-file wshedperf01_openwepp_watershed_end2end_final.run --run-dir \"$RUN_DIR\" --output-dir \"$TS_DIR/outs/openwepp_end_to_end_validated\" --policy compat --legacy-sidecar-discovery'`
