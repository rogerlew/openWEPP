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
| wall time | `3.02 s` | `4.30 s` | `+42.38%` |
| user CPU | `2.96 s` | `4.23 s` | `+42.91%` |
| peak RSS | `44,824 KiB` | `68,240 KiB` | `+52.24%` |
| emitted rows | `0` | `394,488` | — |
| compressed bytes | `0` | `3,526,518` | — |
| bytes per active row | — | `8.9395` | — |
| write throughput | — | about `91,741 rows/s`, `0.782 MiB/s` | — |

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
`/home/workdir/openwepp-wat5-c924aa10-performance`. The run manifests bind
source commit `c924aa109417441b38b04746ddf667bb7fe66697`; the release binary
SHA-256 is
`f391b7b36837c06c6c279a88a2aa94d7b598ce297f810d15d5438671cf6fea88`,
and the 27-column WAT5 file SHA-256 is
`9e9396564005bed1e8427d7efe6101e879bd8c68fac2f9d28ec141195dcc5cce`.
The ordinary HBP, loss, PASS, and WAT hashes are pairwise identical between
the enabled and disabled runs.
