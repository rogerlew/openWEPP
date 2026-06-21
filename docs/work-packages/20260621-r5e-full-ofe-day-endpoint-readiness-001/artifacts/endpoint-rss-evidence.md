# Endpoint RSS Evidence

Status: complete.
Evidence mode: Ran.

Required endpoint evidence:

- default-disabled compatibility H2637 reps;
- opt-in direct-skeleton H2637 rep or reps;
- RSS for each run;
- protected-output comparison against the accepted baseline.

R5E does not introduce a direct-only/projection-only public output mode. No such
mode exists to benchmark in R5E; R6 owns direct public-output projection cutover.

Default-disabled compatibility endpoint:

```text
r5e_h2637_default_rep1  641.37  228724
r5e_h2637_default_rep2  642.02  227956
r5e_h2637_default_rep3  635.47  228940
```

Opt-in direct-skeleton endpoint:

```text
r5e_h2637_direct_skeleton_rep1  638.33  229260
```

Default comparison rerun used for protected output comparison:

```text
r5e_h2637_default_compare  628.65  228684
```

Known warning:

- `MOFE01-MG-W-001` emitted in H2637 endpoint runs; unchanged classification.

Protected output hashes:

| Artifact | Direct skeleton SHA-256 | Default comparison SHA-256 | Verdict |
|---|---|---|---|
| HBP | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` | byte-identical |
| WAT | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` | byte-identical |
| loss | `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021` | `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021` | byte-identical |
| plot | `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6` | `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6` | byte-identical |
| PASS | `613730e5fb9597144d9ef962dfa52c3fef23eff2286b58e215ba2ce9efd2196c` | `62ac888d12c56378ebafe2ca3e9ffe7f9e691f27b214be314a5f60ee07091cf0` | DuckDB row-equivalent |

PASS DuckDB row equivalence:

```text
baseline_rows  12419
candidate_rows 12419
left_minus_right 0
right_minus_left 0
```

PASS schema column counts:

```text
default 17
direct  17
```
