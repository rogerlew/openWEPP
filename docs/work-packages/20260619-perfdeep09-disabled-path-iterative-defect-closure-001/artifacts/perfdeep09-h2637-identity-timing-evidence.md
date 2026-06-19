# PERFDEEP09 H2637 Identity and Timing Evidence

Status: complete.
Evidence class: Ran.

## Control

Release command:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

No-edit same-machine control:

```text
perfdeep09_baseline_rep1  682.65  228924
binary_sha256 beae925662febe47a741ce9e9e5bdd905f088a0601c70c8f4967b04c912b3c09
manifest_sha256 d2636205e554abea1fd184db831e000a18aa576113a5a40f31d743520d0b5eee
```

Control output checksums recorded by manifest:

```text
H2637.hbp          44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8
H2637.loss.json    4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021
H2637.pass.parquet 05b29ca38d31ca6546f2ba889a8f4486509106d99aae31f9fd1b9ac0bf920d95
H2637.plot.parquet 1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6
H2637.wat.parquet  c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474
```

## Rejected Candidate

```text
perfdeep09_hash_registry_rep1  689.30  229352
```

Disposition: rejected and reverted.

## Retained Candidate

Binary:

```text
target/release/openwepp-cli-hill      42900b26106649f3dd89d3ae0ba436e25148336ca97fb8074912227415593032
target/release/openwepp-cli-hill.json 5ab834e0060d480afefb2c30d666814bdc6547bc0daab5820e18f656feff19c9
```

Final timing:

| Rep | Seconds | RSS KB | Manifest SHA-256 |
|---|---:|---:|---|
| 1 | `634.61` | `228856` | `b49b596b3fddfdd75ebf0ea0f21f6c7034b63a92b8c5cd3ceb3f49b6e2104a39` |
| 2 | `635.65` | `228280` | `57a3324f6e9d8ec7e56e4d2f4baf122949bcd50dc674315b592f594a54e77286` |
| 3 | `636.58` | `228168` | `3fd933bda47bf393fafe42ee6522c752f8cac5d7c4cebecaa74eb45598acb128` |

Min/median/max: `634.61 / 635.65 / 636.58 s`.

Gate: `PASS` (`635.65 s <= 676.67 s`).

Final output checksums:

```text
H2637.hbp          44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8
H2637.loss.json    4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021
H2637.wat.parquet  c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474
H2637.plot.parquet 1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6
H2637.pass.parquet e545c97931aa3e43fb03bfb980380d2e4fa5db5609fbec546345821793aefd1e
```

PASS parquet row identity against `/tmp/perfdeep07/default/rep1/h2637_same/H2637.pass.parquet`:

```text
schema_equal=True
baseline_shape=12419x17
candidate_shape=12419x17
left_minus_right 0
right_minus_left 0
```

PASS raw bytes drift across runs, consistent with prior PERFDEEP03/05 identity
policy; row/schema equivalence is the protected PASS identity lane.
