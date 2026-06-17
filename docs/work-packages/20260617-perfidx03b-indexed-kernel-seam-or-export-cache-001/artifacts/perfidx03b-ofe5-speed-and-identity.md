# PERFIDX03B OFE5 Speed And Identity

Ran: release OFE5 timing and same-run-name identity checks on 2026-06-17.

## Perf Availability

- `cat /proc/sys/kernel/perf_event_paranoid` returned `0`.
- `perf stat -e task-clock true` passed.

## Timing Fixture

- Run dir: `/tmp/perfho01/run-dirs/ofe5`
- Baseline binary:
  `/tmp/perfidx03/baseline_src/target/release/openwepp-cli-hill`
- Current binary: `target/release/openwepp-cli-hill`
- Current release build:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`

## Timing Results

Latest baseline rerun:

| Run | Elapsed s | User s | Sys s | Max RSS KB |
| --- | ---: | ---: | ---: | ---: |
| baseline_rerun1 | 26.86 | 26.83 | 0.02 | 25344 |
| baseline_rerun2 | 26.71 | 26.68 | 0.02 | 25728 |
| baseline_rerun3 | 26.88 | 26.84 | 0.02 | 25344 |

Baseline mean: `26.82s`.

Final current active indexed/export-cache path:

| Run | Elapsed s | User s | Sys s | Max RSS KB |
| --- | ---: | ---: | ---: | ---: |
| current_merge_run1 | 24.56 | 24.48 | 0.03 | 25808 |
| current_merge_run2 | 25.78 | 25.74 | 0.02 | 25324 |
| current_merge_run3 | 26.00 | 25.94 | 0.05 | 25424 |

Current mean: `25.45s`.

Disposition: PASS. Current is faster than the latest baseline rerun by about
`1.37s` mean elapsed (`5.1%` faster) and closes the held PERFIDX03 `38.34s`
active-flip regression.

## Same-Run-Name Identity

Run name for both: `perfidx03-ofe5-bitid`.

Byte-stable output hashes:

| Output | Baseline SHA256 | Current SHA256 | Result |
| --- | --- | --- | --- |
| `H1.hbp` | `1eca3b506fb5c4ebcd6dd560617833b5aed08bd98314684cd7c325e1228de43b` | `1eca3b506fb5c4ebcd6dd560617833b5aed08bd98314684cd7c325e1228de43b` | PASS |
| `H1.loss.json` | `a40b0bc0c8a86fc72afe966d2ec1bb17e34d7f9b47f108cfc4b6c86d1793f727` | `a40b0bc0c8a86fc72afe966d2ec1bb17e34d7f9b47f108cfc4b6c86d1793f727` | PASS |
| `H1.wat.parquet` | `64ac87f3042532db1f83e896863f957b0bdf9693fd7de8138e85b695b5edf3ed` | `64ac87f3042532db1f83e896863f957b0bdf9693fd7de8138e85b695b5edf3ed` | PASS |
| `H1.plot.parquet` | `7a9a5ed8e1d3f56960ab579dee4bac6ad87c9ba30f2911be1d55471b4e408516` | `7a9a5ed8e1d3f56960ab579dee4bac6ad87c9ba30f2911be1d55471b4e408516` | PASS |

`H1.pass.parquet` bytes differed:

- baseline: `df232534a0be0dbed0743b9df23efa0aa8a83d4d35ee9e97b62bc1119824760f`
- current: `380429ecea40f2315f57ebee7f83de1d421a6c8a91dcabd395c8bc7351d83463`

DuckDB row comparison:

```text
side,rows
only_a,0
only_b,0
```

Disposition: PASS. Required byte-stable outputs match byte-for-byte, and pass
parquet logical rows compare equal.

