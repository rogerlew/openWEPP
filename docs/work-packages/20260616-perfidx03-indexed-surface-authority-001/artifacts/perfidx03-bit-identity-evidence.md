# PERFIDX03 Bit-Identity Evidence

Status: PARTIAL 2026-06-17
Evidence mode: **Ran**

## Scope Actually Run

The full package anchor requirement was H2637 both `wepp_ui` variants plus the
1-5 OFE ladder. PERFIDX03 stopped before that expensive full anchor because the
active indexed authority path failed the realized performance gate on OFE5.

The exercised output-identity evidence is still useful:

- Synthetic multi-OFE current-vs-baseline pair: strict hashes matched for
  `H1.hbp`, `H1.loss.json`, and `H1.wat.parquet`.
- OFE5 active-flip attempt: `H1.hbp`, `H1.wat.parquet`, and `H1.plot.parquet`
  hashes matched across baseline/current; `H1.loss.json` differed only when the
  timing runfiles intentionally used different `run_name` values.
- OFE5 active-flip same-run-name pass parquet bytes were not stable, but DuckDB
  row-set comparison showed zero logical row differences. Baseline pass parquet
  bytes were also non-deterministic between baseline repeats, so the byte mismatch
  was container metadata/churn, not row-data divergence.
- Final no-flip current tree: strict hashes match baseline for `H1.hbp`,
  `H1.loss.json`, `H1.wat.parquet`, and `H1.plot.parquet`; pass parquet logical
  rows compare equal.

## Synthetic Multi-OFE Hashes

Paths:

- Baseline: `/tmp/perfidx03/bitid/baseline_run/output/`
- Current: `/tmp/perfidx03/bitid/current_run/output/`

```text
467a13b1de5e5e558e84be2822f116be7b2c59fb98e531f5c1c002a0790d42b3  H1.hbp
b316e8753317375d4cbb39f1923fa0b5ab2eb42ec8ca8a7807f5523292c79087  H1.loss.json
166c07b57e4e59db672854ae1f07fc1465e8168fed1d9c1b32920216c846cf61  H1.wat.parquet
```

The same hashes were observed for baseline and current.

## OFE5 Final No-Flip Pair

Paths:

- Baseline: `/tmp/perfidx03/speed/bitid_baseline/ofe5/`
- Current no-flip: `/tmp/perfidx03/speed/bitid_current/ofe5/`

```text
1eca3b506fb5c4ebcd6dd560617833b5aed08bd98314684cd7c325e1228de43b  H1.hbp
a40b0bc0c8a86fc72afe966d2ec1bb17e34d7f9b47f108cfc4b6c86d1793f727  H1.loss.json
64ac87f3042532db1f83e896863f957b0bdf9693fd7de8138e85b695b5edf3ed  H1.wat.parquet
7a9a5ed8e1d3f56960ab579dee4bac6ad87c9ba30f2911be1d55471b4e408516  H1.plot.parquet
```

DuckDB row comparison for `H1.pass.parquet`:

```text
baseline_minus_noflip_current 0
noflip_current_minus_baseline 0
```

## Disposition

No logical output divergence was found in the exercised cases. The full
load-bearing H2637 + ladder anchor remains **NOT RUN** because the authority flip
failed the performance gate first. This prevents PASS disposition.
