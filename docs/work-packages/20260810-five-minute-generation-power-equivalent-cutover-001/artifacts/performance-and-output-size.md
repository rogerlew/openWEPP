# Performance And Output Size

Status: `PASS — default-path limits retained; WAT5-enabled cost measured`

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

The reopened package also measured the enabled product on a representative
45-year Topanga p1 hillslope geometry and calendar. The frozen p1
soil/management/slope files were retained, while every one of the 16,437 days
used the same source-complete 80 mm, two-hour warm-rain event. This is a
conservative continuous-activity performance workload, not a scientific
Topanga outcome. The historical Topanga climate itself fails closed on day 8
because positive melt lacks 300-second timing and therefore cannot measure a
complete enabled run.

| Metric | WAT5 disabled | WAT5 enabled | Delta |
|---|---:|---:|---:|
| wall time | `2.63 s` | `4.11 s` | `+56.27%` |
| user CPU | `2.59 s` | `4.05 s` | `+55.89%` |
| peak RSS | `44,440 KiB` | `68,248 KiB` | `+53.57%` |
| emitted rows | `0` | `394,488` | — |
| compressed bytes | `0` | `3,526,518` | — |
| bytes per active row | — | `8.9395` | — |
| write throughput | — | about `95,982 rows/s`, `0.818 MiB/s` | — |

All 16,437 days emitted an active event span. The sparse file contains 24 of
288 possible intervals per day, so 8.333% of daily intervals were emitted.
The measured output is dramatically smaller than the earlier small-file
upper projection of about 2.15 GB per continuously active 45-year hillslope
and 301 GB for 140 hillslopes, but those upper figures remain the operational
planning warning. The writer buffers at most 8,192 rows and has no per-run row
accumulation.

The enabled overhead is acceptable only for an explicitly requested
diagnostic and is not a production-default performance claim. Presence of
`outputs.wat_subhourly` in the run file is the sole selector; absence retains
the previously measured default path. Raw timing receipts, inputs, output,
and checksums are retained under
`/home/workdir/openwepp-wat5-reopen-performance`.
