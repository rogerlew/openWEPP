# Run Manifest

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Static:

- Source inputs: `/wc1/runs/in/indispensable-presenter/wepp/runs/`.
- Climate artifacts passed the precondition audit before any openWEPP run.
- `pw0.slp` declares `9` OFEs and remains observed-only / outside this
  single-OFE closure package.
- All `p1` through `p22` slope files declare `1` OFE.
- Wrapper pattern kept the WBVAL01 no-discovery TOML shape: authoritative
  `/wc1` `.sol`, `.man`, `.slp`, `.cli`, and `pmetpara.txt` paths,
  `wepp_ui = true`, and inline `snow.txt` values (`rst = 0.0`,
  `newsnw = 100.0`, `ssd = 250.0`).

Ran:

- Build command:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Build result: pass.
- Binary: `target/release/openwepp-cli-hill`.
- Binary SHA-256:
  `6aa3a88c6acfb6b57fa409a7073c755ca7ee866f449df0111a194f0e01435628`.
- Source commit:
  `5b23ef27d398e69bf754be730d28fce63a38c131`.
- Execution scratch root:
  `/tmp/wbval04_rocky_mountain_20260606T000000Z/`.
- Generated TOML wrappers:
  `/tmp/wbval04_rocky_mountain_20260606T000000Z/generated_runfiles/`.
- Output root:
  `/tmp/wbval04_rocky_mountain_20260606T000000Z/outputs/`.
- Batch status TSV:
  `/tmp/wbval04_rocky_mountain_20260606T000000Z/run_status.tsv`.

Wrapper command pattern:

```text
/home/workdir/openWEPP/target/release/openwepp-cli-hill \
  --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs \
  --run-file /tmp/wbval04_rocky_mountain_20260606T000000Z/generated_runfiles/<prefix>.toml \
  --output-dir /tmp/wbval04_rocky_mountain_20260606T000000Z/outputs/<prefix> \
  --policy compat
```

Inventory:

| Prefix | OFEs | Role |
|---|---:|---|
| p1 | 1 | single-OFE |
| p2 | 1 | single-OFE |
| p3 | 1 | single-OFE |
| p4 | 1 | single-OFE |
| p5 | 1 | single-OFE |
| p6 | 1 | single-OFE |
| p7 | 1 | single-OFE |
| p8 | 1 | single-OFE |
| p9 | 1 | single-OFE |
| p10 | 1 | single-OFE |
| p11 | 1 | single-OFE |
| p12 | 1 | single-OFE |
| p13 | 1 | single-OFE |
| p14 | 1 | single-OFE |
| p15 | 1 | single-OFE |
| p16 | 1 | single-OFE |
| p17 | 1 | single-OFE |
| p18 | 1 | single-OFE |
| p19 | 1 | single-OFE |
| p20 | 1 | single-OFE |
| p21 | 1 | single-OFE |
| p22 | 1 | single-OFE |
| pw0 | 9 | observed-only multi-OFE |

Execution results:

| Prefix | OFEs | RC | Output | First typed failure / classification |
|---|---:|---:|---|---|
| p1 | 1 | 0 | WAT emitted | conservation-break |
| p2 | 1 | 0 | WAT emitted | conservation-break |
| p3 | 1 | 0 | WAT emitted | conservation-break |
| p4 | 1 | 0 | WAT emitted | conservation-break |
| p5 | 1 | 0 | WAT emitted | conservation-break |
| p6 | 1 | 0 | WAT emitted | conservation-break |
| p7 | 1 | 1 | no WAT | HKERNEL-WB11-PERC-E-003 |
| p8 | 1 | 0 | WAT emitted | conservation-break |
| p9 | 1 | 0 | WAT emitted | conservation-break |
| p10 | 1 | 0 | WAT emitted | conservation-break |
| p11 | 1 | 1 | no WAT | HKERNEL-WB11-PERC-E-003 |
| p12 | 1 | 0 | WAT emitted | conservation-break |
| p13 | 1 | 0 | WAT emitted | conservation-break |
| p14 | 1 | 0 | WAT emitted | conservation-break |
| p15 | 1 | 0 | WAT emitted | conservation-break |
| p16 | 1 | 0 | WAT emitted | conservation-break |
| p17 | 1 | 0 | WAT emitted | conservation-break |
| p18 | 1 | 1 | no WAT | HKERNEL-WB11-PERC-E-003 |
| p19 | 1 | 0 | WAT emitted | conservation-break |
| p20 | 1 | 1 | no WAT | HKERNEL-WB11-PERC-E-003 |
| p21 | 1 | 0 | WAT emitted | conservation-break |
| p22 | 1 | 0 | WAT emitted | conservation-break |
| pw0 | 9 | n/a | not run | multi-OFE outside WBVAL04 single-OFE closure |

Fail-closed detail for `p7`, `p11`, `p18`, and `p20`:

- CLI wrapper error: `CLIHILL-E-011`.
- Kernel message: `HKERNEL-WB11-PERC-E-003`.
- Scheduler detail: `boundary_class=DOMAIN_VIOLATION`,
  `last_phase=percolation_deep_seepage`,
  `last_decision_outcome=Reject`,
  `wb18_guard_terms={layer_count=8,invalid_ratio_layers=none}`.
- Date: `sim_day_index=95`, `calendar_year=1990`, `julian_day=95`.
