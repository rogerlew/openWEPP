# R3A Default-Disabled Regression Gate

Status: complete.
Evidence mode: Ran.

R2A closed with default-disabled H2637 reps:
`634.06 s`, `636.01 s`, `640.93 s`; median `636.01 s`.

R3A must preserve:

- all direct-runtime opt-ins disabled by default;
- H2637 protected identity;
- final default-disabled H2637 median `<= 676.67 s`.

## R3A Gate Execution

Ran:

- Build command:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Build result: PASS; elapsed build time `56.38 s`.
- Source commit before this package execution commit:
  `9a1bd293f94f37bafa473b5a60bb882f5c2e85f9`.
- Release binary SHA-256:
  `d55aa166376ccaeec51db3ef507a0fa15ffbb0b7f73a74c651e203a04d48b60e`.
- Final release sidecar SHA-256:
  `bb177b4f8000a79dbabd41a12fa66fde867e02887ba9bf86bf94c5e48806bb32`.
- Runtime opt-ins unset:
  `OPENWEPP_PERFDEEP02_FRAME_ISLAND`,
  `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`,
  `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`,
  `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`,
  `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`,
  `OPENWEPP_HPHYS0245_TRACE_PATH`.
- CLI shape:
  `target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run --output-dir /tmp/r3a-h2637-final/default/repN/h2637_same --policy compat --legacy-sidecar-discovery`.

| Rep | Seconds | RSS KB | Manifest SHA-256 |
|---|---:|---:|---|
| 1 | 630.31 | 228600 | `7cf0ef1d6dcaaed55e2d51a24f65dc173a45be372631cf9ead3cf8b1b4c0000c` |
| 2 | 640.85 | 228416 | `91717e2fb85b7700527233f15f98bf46118a163188d827aed0eec7955c614cf6` |
| 3 | 632.08 | 228660 | `a81d2e987556a18bb47f70917746fb4bb8110d487ddccd7085fc157969950fea` |

Min/median/max: `630.31 / 632.08 / 640.85 s`.

Threshold result: PASS, median `632.08 s <= 676.67 s`.

## Protected Output Identity

Manifest output checksums were stable across all three reps for:

- HBP:
  `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8`
- loss:
  `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021`
- WAT:
  `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`
- plot:
  `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6`

PASS parquet raw container hashes differed, consistent with established
Parquet container nondeterminism:

- rep 1:
  `ed4ae695b8c27ad85fc620b6b5391cfd62aa5ed4814cd1a798bd96c07c4bbaf0`
- rep 2:
  `b0f8d72fafbee68f98213b68b33cea6f1a6bd0a35dcb22049f759f5c6487b93e`
- rep 3:
  `afe47482b8cf1aaca7b4b5cbe72d1ad43177e2514ae9bb403f9665cd2ed27aeb`

DuckDB row/schema equivalence against the established H2637 anchor passed:

- schema: matching 17-column `DESCRIBE` output;
- baseline rows: `12419`;
- candidate rows: `12419`;
- `left_minus_right = 0`;
- `right_minus_left = 0`.
