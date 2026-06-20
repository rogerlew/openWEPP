# R3B Default-Disabled Regression Gate

Status: complete.
Evidence mode: Ran.

R3B must preserve the default-disabled H2637 median gate `<= 676.67 s` and
protected output identity.

Ran:

- Build command:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Build result: PASS; elapsed build time `56.60 s`, RSS `1090172 KB`.
- Source commit before this package execution commit:
  `f2504f0b727a9cd46a5a5916480991eb8c88762e`.
- Release binary SHA-256:
  `4b3371e5b3eba72bb223fdfacfa093417622fcde0e096a68318df32302c353e6`.
- Final release sidecar SHA-256:
  `440bfafd165f32df47f32c2ee99b50f95d85409f7af5ebb429d6c1d694c487c9`.
- Runtime opt-ins unset:
  `OPENWEPP_PERFDEEP02_FRAME_ISLAND`,
  `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`,
  `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`,
  `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`,
  `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`,
  `OPENWEPP_HPHYS0245_TRACE_PATH`.
- CLI shape:
  `target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run --output-dir /tmp/r3b-h2637-final/default/repN/h2637_same --policy compat --legacy-sidecar-discovery`.

| Rep | Seconds | RSS KB | Manifest SHA-256 |
|---|---:|---:|---|
| 1 | 640.67 | 227412 | `7b1afd41526d12e7303057f154e5fefc4ef2419740096b8ce69c6033ac742d51` |
| 2 | 643.05 | 228640 | `b33a888c047f29dbd219b9fc5695640079905bc81f3731c371153bfd93cba901` |
| 3 | 639.21 | 228324 | `bbe481e6d323a6f9d00fda7c70a47176172f1d96a60fa867094bc7de15e53e65` |

Min/median/max: `639.21 / 640.67 / 643.05 s`.

Threshold result: PASS, median `640.67 s <= 676.67 s`.

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
  `ec5c1a84bbc7bcffa46615c7fd769f5ac4a51864e15241196d699730635d02fb`
- rep 2:
  `b0a877891054c70fde6f1929bcfff79ebd8f0caf3e633529e12131468c115832`
- rep 3:
  `873bc0564f2ef1a4d911c9c2160715560d6a9d5e679b0159fdff49c06cf0f0a7`

DuckDB row/schema equivalence against the established H2637 anchor passed:

- baseline rows: `12419`;
- candidate rows: `12419`;
- `left_minus_right = 0`;
- `right_minus_left = 0`;
- schema: 17 columns.
