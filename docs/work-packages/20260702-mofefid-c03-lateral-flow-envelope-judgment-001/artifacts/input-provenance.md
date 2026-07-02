# C03 Input Provenance

Evidence class: **Ran**. Analysis inputs, so a reviewer can reproduce.

## Judged run

`H2637.wat.parquet` post-DC01, produced by:
- Binary: `openwepp-cli-hill` built from the DC01 branch
  (`worktree-mofefid-dc01`, merged to main `91273392`).
- Command: `openwepp-cli-hill --run-dir <h2637/runs> --run-file <h2637.run>
  --output-dir <out> --policy compat --legacy-sidecar-discovery`
  (default path, no env selectors; runon re-infiltration is production
  default post-DC01).
- H2637 fixture: 19 OFE × 34 yr = 235,961 OFE-days; staged WB05A replay
  inputs (`p2637.{sol,man,slp,cli}`, `pmetpara.txt`).
- Baseline comparand `postmerge/out` = pre-DC01 main (`db301bcd` era),
  same fixture/command.

The scratch output dirs (`dc01-m3/out`, `postmerge/out`) are regenerable
from the committed binary + fixture by the command above; they are not
vendored (large parquet). The three analysis scripts in this directory are
committed and deterministic given that WAT.

## Scripts

- `c03_storm_decomp.py` — total-export decomposition (shows the baseflow trap).
- `c03_quickflow.py` — quickflow-separated + surface-only event tiers, ENV-T bins.
- `c03_sensitivity.py` — separation-slope sweep + parameter-free surface anchor.
