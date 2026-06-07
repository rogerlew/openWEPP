# Snow Conservation Validation Ledger

Status: closed-with-follow-up-postreview

Evidence mode: Ran

## Pre-Fix Evidence

Ran:

- `target/release/openwepp-cli-hill --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs --run-file /tmp/wbval05_j95_perc_20260606T000000Z/generated_runfiles/p7.toml --output-dir /tmp/snowsci_stage1_repro/p7 --policy compat`

Result:

- Failed with `HKERNEL-WB14-RUNOFF-E-003` at `sim_day_index=95`,
  `calendar_year=1990`, `julian_day=95`.
- Diagnostic localization:
  - prior valid state: `snow.runtime_swe=0.007376104224 m`
  - mixed melt terms: `state_loss=0.013547261834 m`,
    `routed_melt=0.001204946614 m`
  - overdraw: `-0.006171157610 m`

## Post-Fix J-95 Publication

Ran:

- `cargo build -p openwepp-runner --bin openwepp-cli-hill --release`
- Release CLI rerun for `p7`, `p11`, `p18`, and `p20` using
  `/tmp/wbval05_j95_perc_20260606T000000Z/generated_runfiles/*.toml`.

Result:

| Hillslope | Result |
|---|---|
| `p7` | published |
| `p11` | published |
| `p18` | published |
| `p20` | published |

## P7 Trace Check

Ran:

- `OPENWEPP_HPHYS0245_TRACE_PATH=/tmp/snowsci_stage1_trace/p7/hphys0245.jsonl`
  with `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=100`.

Key rows:

| Day | Boundary | SWE before m | SWE after m | Melt sum m | Snow S m | Closure error m | WB13 Snow-Water mm | WB13 RM mm |
|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 94 | post_wb13 | 0.046471092328 | 0.022922741699 | 0.023548350629 | 0.023548350629 | 0.0 | 22.922741699 | 23.548350629 |
| 95 | post_wb13 | 0.022922741699 | 0.012031370200 | 0.010891371499 | 0.010891371499 | 0.0 | 12.031370200 | 10.891371499 |

## WBVAL06 Boundary

Ran:

- Fresh isolated WBVAL04 runfile sweep using redirected runfiles in
  `/tmp/snowsci_stage1_wbval06_after_20260607T021725Z/runfiles`.
- Before/after residual recomputation with WBVAL04
  `/tmp/wbval04_rocky_mountain_20260606T000000Z/closure_summary.csv` and
  `/tmp/wbval04_rocky_mountain_20260606T000000Z/closure_ledger.csv` as the
  before source.

Result:

- `22` WAT parquet outputs produced under
  `/tmp/snowsci_stage1_wbval06_after_20260607T021725Z/outputs`.
- This proves the snow fail-closed blocker no longer prevents publication.
- WBVAL06 residual improved but did not close:
  - before, `18/18` WBVAL04 status-valid emitters were conservation-break,
    max annual R `94.433070 mm`, mean prefix max R `58.644141 mm`;
  - after, `22/22` emitters publish WAT and remain conservation-break,
    max annual R `26.790809 mm`, mean prefix max R `20.656270 mm`;
  - after on the before-valid `18` prefixes: max annual R `26.790809 mm`,
    mean prefix max R `22.461195 mm`.
- Complete annual residual closure is not claimed here; the separate WBVAL06
  package retains residual attribution for the remaining above-tolerance
  residual.
