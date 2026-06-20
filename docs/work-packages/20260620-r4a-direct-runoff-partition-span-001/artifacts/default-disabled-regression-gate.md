# Default-Disabled Regression Gate

Status: complete.
Evidence mode: Ran.

Gate:

- H2637 default-disabled median wall time must be `<= 676.67 s`.
- Protected HBP, loss, WAT, plot, and PASS row identity must pass.

R3C reference:

- Final default-disabled reps: `640.85 s`, `643.41 s`, `644.07 s`.
- Median: `643.41 s`.

Ran:

- Build command:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Build result: PASS; elapsed build time `57.96 s`, RSS `1088060 KB`.
- Source commit before this package execution commit:
  `80c78cd5f98aac71bc919c3afbd2f732607e075f`.
- Release binary SHA-256:
  `3ff8b1ad6658f0a69b43025d4ba81839eb4be8fa938b9e71469abfdad1002455`.
- Final release sidecar SHA-256:
  `7297bd5bf0ba0896a6e4b380d8167066c1f010f21bea1a000cf8b590f700cccf`.
- Runtime opt-ins unset:
  `OPENWEPP_PERFDEEP02_FRAME_ISLAND`,
  `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`,
  `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`,
  `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`,
  `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`,
  `OPENWEPP_HPHYS0245_TRACE_PATH`.
- CLI shape:
  `target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run --output-dir /tmp/r4a-h2637-final/default/repN/h2637_same --policy compat --legacy-sidecar-discovery`.

Each rep emitted only the known MOFE01 M-G warning:

```text
MOFE01-MG-W-001 EROD14 Wave-2 qin is seeded from water-transfer provenance only
```

| Rep | Seconds | RSS KB | Manifest SHA-256 |
|---|---:|---:|---|
| 1 | 644.01 | 228688 | `e14e48b66fa1134269f6abe0a47dfc346a7ee9c3072b8b6c4c85fb80ed7e6ee0` |
| 2 | 646.84 | 228916 | `b8694444ee53fd5122a4c10697ce2a431f01f703e892269b0b51f34298c5c975` |
| 3 | 643.66 | 228948 | `f813481b5325eae4e916c2d250a7313684939e976070f520c82b0ee27eecee23` |

Min/median/max: `643.66 / 644.01 / 646.84 s`.

Threshold result: PASS, median `644.01 s <= 676.67 s`.

Protected output identity:

- HBP:
  `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8`
- loss:
  `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021`
- WAT:
  `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`
- plot:
  `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6`

PASS parquet raw hashes varied across reps, consistent with established
container nondeterminism. Final raw PASS hash:

- `8d0f9ab7edf8990a8055cdc5f6dcaede68b4c110d4403a65eedd53c3452a5b75`

DuckDB row/schema equivalence against the established H2637 anchor passed:

- baseline rows: `12419`;
- candidate rows: `12419`;
- `left_minus_right = 0`;
- `right_minus_left = 0`;
- schema: 17 columns.
