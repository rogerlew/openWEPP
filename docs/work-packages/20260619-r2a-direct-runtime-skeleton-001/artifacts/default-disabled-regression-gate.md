# R2A Default-Disabled Regression Gate

Status: complete.
Evidence mode: Ran.

PERFDEEP09 closed the default-disabled blocker with final H2637 reps:
`634.61 s`, `635.65 s`, `636.58 s`; median `635.65 s`.

R2A must preserve:

- all PERFDEEP/direct-runtime opt-ins disabled by default;
- H2637 protected identity under PERFDEEP09 policy;
- final default-disabled H2637 median `<= 676.67 s`.

## R2A Gate Execution

Ran:

- Build command:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Build result: pass; elapsed build time `56.76 s`.
- Source commit before this package commit:
  `8d60d7db6d268d8179eab7b2d3cfa93e58416d42`.
- Release binary checksum:
  `fe91cdce61636de56422ea18fdba7bbc6525ffee9b342236c9cc3a225cbdf45c`.
- Runtime opt-ins unset:
  `OPENWEPP_PERFDEEP02_FRAME_ISLAND`,
  `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`,
  `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`,
  `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`,
  `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`,
  `OPENWEPP_HPHYS0245_TRACE_PATH`.
- CLI shape:
  `target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run --output-dir /tmp/r2a-h2637/default/repN/h2637_same --policy compat --legacy-sidecar-discovery`.

| Rep | Seconds | RSS KB | Manifest SHA-256 |
|---|---:|---:|---|
| 1 | 634.06 | 228740 | `0853f5986294d390642ed18d54c8beb6accca7cf20d10fa9a55ceaefcfec5777` |
| 2 | 636.01 | 228508 | `d96fee85cfc506167f5f5427a319ea5373c3617d386a2192771abafe58830451` |
| 3 | 640.93 | 228192 | `96acfef9164dd550da7ed864c93d9f724f4a39f61b42b5a6958b597458fc172b` |

Min/median/max: `634.06 / 636.01 / 640.93 s`.

Threshold result: PASS, median `636.01 s <= 676.67 s`.

## Protected Output Identity

Ran:

- Manifest output checksums were stable across all three reps for:
  HBP `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8`,
  loss `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021`,
  WAT `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`,
  and plot `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6`.
- PASS parquet raw container hashes differed, consistent with known parquet
  container nondeterminism:
  rep 1 `3c783306c4c5b4efdb460a1d6a0b206a1a43d2c3e8db95d817eeb4466d1bb49a`;
  rep 2 `3252c099b39d4f25dd8ec8f835c94925d109779f2249918cf51c58600c9b9571`;
  rep 3 `d118aa7919b193eae62f284715392e9973f4842686134342fc4ea13075607da0`.
- DuckDB row/schema equivalence against the established H2637 anchor passed:
  `schema_equal=True`, baseline shape `12419x17`, candidate shape `12419x17`,
  `left_minus_right=0`, `right_minus_left=0`.
