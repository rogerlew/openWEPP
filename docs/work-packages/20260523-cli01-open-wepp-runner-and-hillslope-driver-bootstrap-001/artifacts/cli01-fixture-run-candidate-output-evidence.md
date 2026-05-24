# CLI01 Fixture Run Candidate Output Evidence

Status: complete
Evidence mode: Static + Ran

## Static
- Required candidate outputs for CLI01 hillslope bootstrap:
  - `H5.wat.dat`
  - `H5.plot.dat`
- Output generation surface:
  - `execute_hillslope_run` in `crates/openwepp-runner/src/lib.rs`

## Ran
1. Direct hillslope binary execution (`strict` fixture):

```text
/tmp/cli01_strict_success_sClCCQ/out/H5.wat.dat
/tmp/cli01_strict_success_sClCCQ/out/H5.plot.dat
/tmp/cli01_strict_success_sClCCQ/out/openwepp_hillslope_run_manifest.json
```

Checksums:

```text
b2679d5e7c07bb2c41bb6b90b0d6c2f24857d23f69522d65c26f2d9451504dca  H5.wat.dat
51ca38806e7949eca90de1c3f708d7fae5c31e27a67d6bd10e349b9b97218a47  H5.plot.dat
```

2. Launcher-boundary execution (`open_wepp_runner run-hillslope`):

```text
/tmp/cli01_runner_run_WA5pGL/out/H5.wat.dat
/tmp/cli01_runner_run_WA5pGL/out/H5.plot.dat
/tmp/cli01_runner_run_WA5pGL/out/runner_manifest.json
```

Checksums (match direct execution):

```text
b2679d5e7c07bb2c41bb6b90b0d6c2f24857d23f69522d65c26f2d9451504dca  H5.wat.dat
51ca38806e7949eca90de1c3f708d7fae5c31e27a67d6bd10e349b9b97218a47  H5.plot.dat
```

Sample content snippets:

```text
H5.wat.dat header:
DAILY WATER BALANCE - HOURLY SEEPAGE UPDATE FROM UI

H5.plot.dat header:
PLOT SUMMARY
Y J OFE P NLAYERS
```
