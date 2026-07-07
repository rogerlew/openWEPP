# Timing And Fidelity

Status: EXECUTED. Evidence mode: Static + Ran.

## Binary Provenance

Baseline release binary:

- Build log: `artifacts/runner-build.log`.
- SHA256: `628486b358b94bf87f09880c0e3b687a924b33502967e08fba5145b0a8e72f51`.
- Timing log: `artifacts/h2637-active-hybrid-time.log`.

After first raw-zero attempt:

- SHA256: `aad4db3177d6ca227ccf62209c83b97a18377ed4f32ef29509f316a9d5589379`.
- Timing log: `artifacts/h2637-active-hybrid-after-time.log`.
- Result: no map-evaluation reduction; this proved the raw-zero guard was too
  strict for the active fixture and was superseded by exact active-addend
  absence plus validation.

After final exact active-addend implementation:

- Build log: `artifacts/runner-build-after-effective.log`.
- Provenance: `artifacts/binary-after-effective-provenance.txt`.
- SHA256: `ac4e883c19d39efd0280963b80a1c44b0edf2e520a026c36dcb4b7a9b71e9228`.
- Timing log: `artifacts/h2637-active-hybrid-after-effective-time.log`.

## H2637 Timing And Counters

| Metric | Baseline | After exact evaluator | Delta |
|---|---:|---:|---:|
| User time | `38.39 s` | `33.37 s` | `-5.02 s` (`-13.08 %`) |
| Wall time | `0:38.41` | `0:33.43` | `-4.98 s` (`-12.97 %`) |
| System time | `0.01 s` | `0.05 s` | `+0.04 s` |
| Solver runs | `17898` | `17898` | `0` |
| Solver steps | `7381407` | `7381405` | `-2` |
| Homogeneous steps | `381501` | `381501` | `0` |
| Source-free steps | `1739149` | `1739149` | `0` |
| Implicit steps | `980804` | `980804` | `0` |
| Equilibrium map evaluations | `151435969` | `0` | `-100 %` |
| Branch evaluations | `20110816` | `20110873` | `+57` |
| Alpha evaluations | `119746485` | `119746445` | `-40` |

The solve-cost bottleneck is removed for the H2637 source-memory hybrid vector:
the endpoint improvement is material and the nested map counter is exactly
zero.

## Output Delta Audit

Output hashes:

- `H2637.loss.json`: unchanged
  (`725f57233fd60df097a824a2c20f26992a58b3a457594245a9ac91d2278f3cfb`).
- `H2637.hbp`: changed
  `939e37a7352c0f7a75c4004829a7a3886ee0f1b91820164e36fe7d734cde5fa5`
  -> `bfb2b002f8b67cd3c4b42504ae9cbc02189c13651f658b0c035c51cd23f50621`;
  `cmp -l | wc -l = 54`.
- `H2637.pass.parquet`: changed
  `a26ddd09729b960d8fbed6bbb351d37f5307b21eb8cdb3c0003500f59d4fec04`
  -> `44e3da28ed5a2c4b310507d8d2f03e65c3a902e2f01e59f08e11e732d80e1f34`.

Pass parquet audit:

- Shape: `(731, 17)` both.
- Columns and index: identical.
- Changed numeric columns only:
  - `tdet`: 1 row, max absolute `3.4790161862474633e-09`, max relative
    `1.538378910835884e-10`.
  - `sedcon_1`: 3 rows, max absolute `2.137607582261558e-13`, max relative
    `3.835458595657556e-10`.
  - `sedcon_2`: 3 rows, max absolute `3.1241884079769022e-12`, max relative
    `3.835449647009613e-10`.
  - `sedcon_3`: 3 rows, max absolute `1.4798808879723513e-12`, max relative
    `3.835455266933441e-10`.
  - `sedcon_4`: 3 rows, max absolute `3.713276214289962e-12`, max relative
    `3.835455149064e-10`.
  - `sedcon_5`: 3 rows, max absolute `1.8874062035489914e-11`, max relative
    `3.8354494105819043e-10`.

Unchanged pass surfaces include `runvol`, `sbrunv`, `peakro`, `tdep`, calendar,
and identifiers. The output movement is therefore sparse numerical dust from
the exact branch-equilibrium solve, not an ownership/schema change.

## Closure Evidence

Manifest active-lane closure remains at the existing machine-precision level:

- `max_day_cascade_residual_rel`: baseline
  `4.598323211290479e-13`, after `4.579976970630865e-13`.
- `max_day_seam_residual_rel`: baseline and after
  `4.082921815102614e-14`.
- `max_day_identity_residual_rel`: baseline
  `4.454598907017495e-13`, after `4.443101533649709e-13`.
- WB13 identity maxima: `0.0` before and after.

Case-4 full-hybrid oracle ladder remains passing in the focused `ofe_routing`
suite and final full-suite gate logs.
