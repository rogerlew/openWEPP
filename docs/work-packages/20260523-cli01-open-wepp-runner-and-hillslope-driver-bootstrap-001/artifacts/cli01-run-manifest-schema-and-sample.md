# CLI01 Run-Manifest Schema and Sample Evidence

Status: complete
Evidence mode: Static + Ran

## Static
- Manifest schema id constant:
  - `HILLSLOPE_RUN_MANIFEST_SCHEMA_ID = "openwepp-hillslope-run-manifest-v1"`
- Manifest writer surface:
  - `execute_hillslope_run` in `crates/openwepp-runner/src/lib.rs`
- Deterministic map types:
  - `resolved_sidecars`, `input_checksums`, `output_checksums` use `BTreeMap`
    to guarantee sorted key order in serialized JSON.

## Ran
Produced a manifest through launcher path:

```text
open_wepp_runner run-hillslope \
  --engine openwepp \
  --hillslope-binary /home/workdir/openWEPP/target/debug/openwepp-cli-hill \
  --run-dir /tmp/cli01_runner_run_WA5pGL \
  --run-file case.run \
  --output-dir /tmp/cli01_runner_run_WA5pGL/out \
  --policy strict \
  --manifest-path /tmp/cli01_runner_run_WA5pGL/out/runner_manifest.json
```

Manifest excerpt (`/tmp/cli01_runner_run_WA5pGL/out/runner_manifest.json`):

```json
{
  "schema": "openwepp-hillslope-run-manifest-v1",
  "engine": "openwepp",
  "binary_path": "/home/workdir/openWEPP/target/debug/openwepp-cli-hill",
  "sidecar_policy": "strict",
  "resolved_sidecars": {
    "frost": "/tmp/cli01_runner_run_WA5pGL/frost.txt",
    "pmetpara": "/tmp/cli01_runner_run_WA5pGL/pmetpara.txt",
    "snow": "/tmp/cli01_runner_run_WA5pGL/snow.txt",
    "wepp_ui": "/tmp/cli01_runner_run_WA5pGL/wepp_ui.txt"
  },
  "output_checksums": {
    "/tmp/cli01_runner_run_WA5pGL/out/H5.plot.dat": "51ca38806e7949eca90de1c3f708d7fae5c31e27a67d6bd10e349b9b97218a47",
    "/tmp/cli01_runner_run_WA5pGL/out/H5.wat.dat": "b2679d5e7c07bb2c41bb6b90b0d6c2f24857d23f69522d65c26f2d9451504dca"
  }
}
```

Schema and field evidence:
- schema id matches CLI01 contract value.
- argv vector is captured in invocation order.
- checksum maps are serialized in sorted key order.
