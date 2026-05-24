# CLI01 wepppy Consumer Boundary Note

Status: complete
Evidence mode: Static + Ran

## Static
CLI01 establishes the producer-side launcher contract for wepppy consumption:

- Launcher command:
  - `open_wepp_runner run-hillslope --engine openwepp --hillslope-binary <path> --run-dir <path> --run-file <path> --output-dir <path> [--policy strict|compat] [--manifest-path <path>]`
- Release validation command:
  - `open_wepp_runner release lint --release-dir <path>`
- Required outputs:
  - `H5.wat.dat`
  - `H5.plot.dat`
  - run manifest (`openwepp-hillslope-run-manifest-v1` schema)
- Sidecar policy defaults:
  - strict should remain default for production launches
  - compat is allowed for curated migration/interoperability paths only

No wepppy code was changed in this package. This package is producer-boundary
bootstrap work in openWEPP.

## Ran
- External discovery check:

```text
rg -n "open_wepp_runner|openwepp-cli-hill|run-hillslope|legacy_wepp|openwepp" /workdir/wepppy -g '*.py'
```

Result:
- No existing `open_wepp_runner`/`openwepp-cli-hill` consumer callsites found in
  current wepppy Python surfaces.
