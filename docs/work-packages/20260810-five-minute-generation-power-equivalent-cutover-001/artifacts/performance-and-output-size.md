# Performance And Output Size

Status: `PASS — default-path limits; optional output sizing reported`

Evidence mode: `Ran + Projection`

Fifteen release-mode amplified-p61 repetitions compared the frozen runner with
the diagnostics-disabled implementation runner under identical inputs and output
targets. Raw receipts are `perf-baseline-final.tsv` and
`perf-current-final.tsv` in the external evidence root.

| Metric | Frozen median | Current median | Delta | Gate |
|---|---:|---:|---:|---:|
| elapsed | `0.73 s` | `0.73 s` | `0.0%` | `<=1%` |
| peak RSS | about `27.9 MB` | about `27.9 MB` | about `0.2%` | `<=1%` |

An earlier implementation filled 288 bins unconditionally and measured about
4% overhead. It was rejected and replaced by an opt-in-only recomputation;
the final default path does not allocate or fill WAT5 arrays.

The post-review two-hour p61 warm-rain example wrote 24 rows and an 11,866-byte small
Parquet file. A deliberately conservative uncompressed/small-file linear upper
projection is 4,733,640 rows and about 2.15 GB for a continuously active
45-year hillslope, or 662,709,600 rows and about 301 GB for 140 hillslopes.
Real sparse support and large row-group compression should be much smaller;
these upper figures make the optional storage cost explicit rather than
promising an unmeasured compression ratio. The writer buffers at most 8,192
rows and has no per-run row accumulation.

The later review fixes are confined to the explicitly requested WAT5 path,
except that boxing the optional event reduced `DirectDayFrame` resident size;
they do not restore the rejected unconditional 288-bin work. The default-path
performance receipt is therefore reused with an explicit structural
applicability check, while the post-review file size is freshly measured.
