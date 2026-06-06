# Run Manifest

Status: complete

Evidence mode: Ran

Ran:

Run root:

- Source inputs: `/wc1/runs/in/indispensable-presenter/wepp/runs/`
- Execution scratch root: `/tmp/wbval01_rocky_mountain_20260606T000000Z/`
- Generated TOML wrappers:
  `/tmp/wbval01_rocky_mountain_20260606T000000Z/generated_runfiles_nodiscovery/`
- Canonical output root:
  `/tmp/wbval01_rocky_mountain_20260606T000000Z/nodiscovery/`
- Batch status TSV:
  `/tmp/wbval01_rocky_mountain_20260606T000000Z/run_status_nodiscovery.tsv`

Binary:

- Build command:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- Build result: pass
- Binary: `target/release/openwepp-cli-hill`
- Binary SHA-256:
  `ae6395705279f764fb8e851392834ab99b7a069450967021c2bc161fcb5d37b4`
- Source commit:
  `30794db1ce5031aa9a8639a246bd61ce440ee801`

Wrapper pattern:

```text
/home/workdir/openWEPP/target/release/openwepp-cli-hill \
  --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs \
  --run-file /tmp/wbval01_rocky_mountain_20260606T000000Z/generated_runfiles_nodiscovery/<prefix>.toml \
  --output-dir /tmp/wbval01_rocky_mountain_20260606T000000Z/nodiscovery/<prefix> \
  --policy compat
```

The generated wrappers reference the authoritative `/wc1` `.sol`, `.man`,
`.slp`, `.cli`, and `pmetpara.txt` inputs, set `wepp_ui = true`, and include the
`snow.txt` values inline (`rst = 0.0`, `newsnw = 100.0`, `ssd = 250.0`).
This avoided broad shared-directory legacy sidecar scanning while preserving
the hourly `wepp_ui.txt` lane.

Initial compatibility check:

- Direct legacy `.run` command:
  `target/release/openwepp-cli-hill --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs --run-file p1.run --output-dir /tmp/wbval01_rocky_mountain_20260606T000000Z/hillslopes/p1 --policy compat`
- Result: fail closed with `CLIHILL-E-010` because the current
  `openwepp-cli-hill` front door requires TOML runfiles; legacy WEPP text
  `.run` files are not parsed by this binary.

Inventory and execution:

| Prefix | OFEs | Role | RC | Output | Classification/blocker |
|---|---:|---|---:|---|---|
| p1 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p2 | 1 | single-OFE | 1 | no WAT | CLIM-RUNTIME-E-017 |
| p3 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p4 | 1 | single-OFE | 1 | no WAT | CLIM-RUNTIME-E-017 |
| p5 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p6 | 1 | single-OFE | 1 | no WAT | CLIM-RUNTIME-E-017 |
| p7 | 1 | single-OFE | 1 | no WAT | HKERNEL-WB11-PERC-E-003 |
| p8 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p9 | 1 | single-OFE | 1 | no WAT | CLIM-RUNTIME-E-017 |
| p10 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p11 | 1 | single-OFE | 1 | no WAT | HKERNEL-WB11-PERC-E-003 |
| p12 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p13 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p14 | 1 | single-OFE | 1 | no WAT | CLIM-RUNTIME-E-017 |
| p15 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p16 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p17 | 1 | single-OFE | 1 | no WAT | CLIM-RUNTIME-E-017 |
| p18 | 1 | single-OFE | 1 | no WAT | HKERNEL-WB11-PERC-E-003 |
| p19 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p20 | 1 | single-OFE | 1 | no WAT | HKERNEL-WB11-PERC-E-003 |
| p21 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| p22 | 1 | single-OFE | 0 | WAT emitted | conservation-break |
| pw0 | 9 | observed-only watershed/multi-OFE | n/a | not run | multi-OFE outside WBVAL01 single-OFE closure |

Fail-closed blockers:

- `CLIM-RUNTIME-E-017`: `p2`, `p4`, `p6`, `p9`, `p14`, `p17`
- `HKERNEL-WB11-PERC-E-003`: `p7`, `p11`, `p18`, `p20`
