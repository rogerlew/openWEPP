# Default-Disabled Regression Gate

Status: complete.
Evidence mode: Ran.

Gate:

- H2637 default-disabled median wall time must be `<= 676.67 s`.
- Protected HBP, loss, WAT, plot, and PASS row identity must pass.

R3B reference:

- Final default-disabled reps: `640.67 s`, `643.05 s`, `639.21 s`.
- Median: `640.67 s`.

Ran:

- Build command:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Build result: PASS; elapsed build time `57.73 s`, RSS `1104412 KB`.
- Source commit before this package execution commit:
  `5d177389761e816095286f81f813df37f9a7d5f0`.
- Release binary SHA-256:
  `ad4df663c8b34f907b2e3e54bd4c695d5d175a2f6497385d4af01b63d404c671`.
- Final release sidecar SHA-256:
  `e06094508cbdbe948a28fc0728689d66286ed866ad3b00d4b2a4257dfc585c8e`.
- Runtime opt-ins unset:
  `OPENWEPP_PERFDEEP02_FRAME_ISLAND`,
  `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`,
  `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`,
  `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`,
  `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`,
  `OPENWEPP_HPHYS0245_TRACE_PATH`.
- CLI shape:
  `target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run --output-dir /tmp/r3c-h2637-final2/default/repN/h2637_same --policy compat --legacy-sidecar-discovery`.

| Rep | Seconds | RSS KB | Manifest SHA-256 |
|---|---:|---:|---|
| 1 | 640.85 | 228972 | `e677c620480e8c280eded99b6264ab404162693b580f694b1946b3e4f0d96aee` |
| 2 | 643.41 | 228352 | `0583ad00295f3b361bb7990fb04702a154f7dadacc4c94acded82cd4f4c1d087` |
| 3 | 644.07 | 229152 | `3dfe01612e6b64c61551c70ef1b32820991994d217fd83677941a29c726ceff1` |

Min/median/max: `640.85 / 643.41 / 644.07 s`.

Threshold result: PASS, median `643.41 s <= 676.67 s`.

Protected output identity:

- HBP:
  `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8`
- loss:
  `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021`
- WAT:
  `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`
- plot:
  `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6`

PASS parquet raw hashes differed across reps, consistent with established
container nondeterminism:

- rep 1:
  `e2d08f95865e798e34ad93b56a44ecf621e07dbcf8a8da1efc4dee6d70a44a04`
- rep 2:
  `312ea4fd61ed6e9da407c1059c69c083119848b7955dbb00911c824bc008bf3e`
- rep 3:
  `eae5f048249f338b34175bfcb929ae5b943ac8936d2fc54372045d8f99541cfd`

DuckDB row/schema equivalence against the established H2637 anchor passed:

- baseline rows: `12419`;
- candidate rows: `12419`;
- `left_minus_right = 0`;
- `right_minus_left = 0`;
- schema: 17 columns.
