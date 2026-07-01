# Environment and Input Inventory

Status: `UPDATED`

Record `Static:` and `Ran:` fields here before canonical timing.

## Repository

Ran:
- `git rev-parse HEAD` = `44b1658e0d026d4642f0360e175da1462eb0c3a0`.
- `git status --short` at completion: `M docs/work-packages/README.md` (pointer note) and package artifact updates only.

## Legacy baseline

Ran:
- Baseline path: `/workdir/wepp-forest_260430_baseline/release/wepp_260430`.
- SHA256: `d1447a202a7e32f3bc892a1011787249764d57f61370f004a3d9ccb20aeb8a17`.
- Canonical timed command used: `/tmp/wshedperf01_20260701_081511/timing/legacy_pw0_timed_1.time` (exit `0`).

## openWEPP binaries

Ran:
- Release watershed CLI: `/home/workdir/openWEPP/target/release/openwepp-cli-watershed`.
- SHA256: `209de0a054332c33d9c865c1cf2d898a1cdfef92f3988514fd8fc2ab2aaccc20`.
- Release hillslope CLI: `/home/workdir/openWEPP/target/release/openwepp-cli-hill`.
- SHA256: `ba2366d9251b2db9209bc4c3ae9ca43f3bfafb10449b633fbeffc1bbf2baa504`.
- Build command timing source: `/tmp/wshedperf01_20260701_101739/build_openwepp_cli_repeat.time` (`/usr/bin/time -v cargo build --release -p openwepp-runner --bin openwepp-cli-watershed`, exit `0`).

## Timing tools

Static:
- `/usr/bin/time` = GNU Time (`time (GNU Time) UNKNOWN`).
- `perf` = `perf version 6.8.12`.
- `/usr/bin/time -v` and `perf stat` were used for full-path openWEPP attribution.

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
- Canonical arboreal-dendrite input root (required): `/wc1/runs/ar/arboreal-dendrite/wepp`.
- Timing-stage roots:
  - `/tmp/wshedperf01_20260701_081511/stage/arboreal-dendrite`
  - `/tmp/wshedperf01_20260701_083200/stage/arboreal-dendrite`
  - `/tmp/wshedperf01_20260701_102200/stage/arboreal-dendrite`
  - `/tmp/wshedperf01_20260701_101739/repeat_1/stage/arboreal-dendrite`
  - `/tmp/wshedperf01_20260701_101739/repeat_2/stage/arboreal-dendrite`
  - `/tmp/wshedperf01_20260701_101739/repeat_3/stage/arboreal-dendrite`
  - `/tmp/wshedperf01_20260701_101739/perf_full_e2e/stage/arboreal-dendrite`
- Run files used by timed commands:
  - `wshedperf01_openwepp_watershed_hbp_manifest_cond.run`
  - `wshedperf01_openwepp_watershed_end2end_final.run`
- Hillslope run file families used in full-path loops:
  - `runfiles/p{1..36}.run`
  - `runfiles/p{1..36}_end2end3.run`
- Full end-to-end outputs observed:
  - `/tmp/wshedperf01_20260701_102200/outs/openwepp_end_to_end_validated/interchange`
  - `/tmp/wshedperf01_20260701_101739/repeat_1/outs/openwepp_end_to_end_repeat_1/interchange`
  - `/tmp/wshedperf01_20260701_101739/repeat_2/outs/openwepp_end_to_end_repeat_2/interchange`
  - `/tmp/wshedperf01_20260701_101739/repeat_3/outs/openwepp_end_to_end_repeat_3/interchange`
  - `/tmp/wshedperf01_20260701_101739/perf_full_e2e/outs/openwepp_end_to_end_repeat_perf/interchange`

## Expected output surfaces

Ran:
- Routed/partial output surface (14 files), as emitted by routed runs: `chan.out.parquet`, `chanwb.parquet`, `chnwb.parquet`, `ebe_pw0.parquet`, `loss_pw0.all_years.chn.parquet`, `loss_pw0.all_years.class_data.parquet`, `loss_pw0.all_years.hill.parquet`, `loss_pw0.all_years.out.parquet`, `loss_pw0.chn.parquet`, `loss_pw0.class_data.parquet`, `loss_pw0.hill.parquet`, `loss_pw0.out.parquet`, `soil_pw0.parquet`, `totalwatsed3.parquet`.
- Full end-to-end output surface (14 files) confirmed non-empty in each repeat and perf run.

## Command-surface notes

Ran:
- Canonical legacy surface: `/workdir/wepp-forest_260430_baseline/release/wepp_260430 --run-name pw0 ...` (resolved in timed file command line).
- OpenWEPP routed-stage surface: `target/release/openwepp-cli-watershed --run-dir ... --run-file wshedperf01_openwepp_watershed_hbp_manifest_cond.run --output-dir ... --policy compat --legacy-sidecar-discovery`.
- OpenWEPP full end-to-end stable surface (validated repeats): `bash /tmp/wshedperf01_20260701_101739/repeat_<n>/run_e2e.sh /tmp/wshedperf01_20260701_101739/repeat_<n>/stage/arboreal-dendrite /tmp/wshedperf01_20260701_101739/repeat_<n>/outs/openwepp_end_to_end_repeat_<n> /tmp/wshedperf01_20260701_101739/repeat_<n>/logs`.
- OpenWEPP full end-to-end profile surface: `perf stat ... bash /tmp/wshedperf01_20260701_101739/perf_full_e2e/run_e2e.sh ...`.
