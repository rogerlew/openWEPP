# Low Final Metrics

Evidence class: **Ran + Static**

Status: `PASS`

## Source And Protocol

- Instrumented workspace commit:
  `9145d288809935a79ec78143758a0d8de1c2ffd7`.
- Corrected ordinary-gate commit:
  `8e0f7367fad57a9ec03e8855727a6bfd64560ca0`.
- Literal slug/phase: `low` / `final`.
- The only source delta between those commits is test-only: four exact float
  comparisons became epsilon comparisons and one exhaustive PMET test received
  a narrow `clippy::too_many_lines` allowance. Production source is unchanged,
  so the single instrumented profile was not rerun after the lint correction.
- The 18-package report-only expansion recorded in
  `final/report-packages.txt` emitted LCOV and JSON from the same 315 profiles;
  it did not rerun or clear coverage.

## Results And Integrity

| Step | Exit | Elapsed | Max RSS |
| --- | ---: | ---: | ---: |
| Instrumented workspace run | 0 | `36:13.15` | 833,760 KB |
| LCOV report | 0 | `0:03.30` | 155,868 KB |
| JSON report | 0 | `0:04.29` | 396,828 KB |
| CRAP report | 0 | `0:01.09` | 206,832 KB |

The JSON contains 110,035 instrumented lines and 97,163 covered lines, for
88.301904% workspace line coverage. Relative to Low start, coverage rises
0.466305 percentage points, with 651 more covered and 157 more instrumented
lines.

| Durable artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `final/final.lcov` | 4,552,212 | `acf5635539695b70d82593d908549b0d2c89b470c8bd13a3aaba434dfb64faad` |
| `final/final.json` | 20,005,639 | `df7493ddfc4c62e75c011d249f64efaf919c2ff6d8ab5f493faca2d04dc086df` |
| `final/final-crap.json` | 2,957,059 | `0f66b37412fbaa7b692f831b3aa1f39fe77f69a0523ddddb5ae1d360c9558a3a` |
| `final/final-production-over30.json` | 380 | `a9c356cb7109e7253d7770b22557216f22c0cf593984147daeeb24f8f81c6f26` |
| `final/final-actionable-over30.json` | 3 | `37517e5f3dc66819f61f5a7bb8ace1921282415f10551d2defa5c3eb0985b570` |
| `final/report-packages.txt` | — | `773e707aa9a39077a4efb4479d1a52ac253d3ce156e4f8b277f8d4e70844a690` |

## Instrumented-Failure Attribution

The ignored-run command reproduced exactly four known parallel-instrumentation
failures: the three H2637 selector tests and the process-global R3C audit
counter assertion at `direct_runtime_r3c_r4b.rs:780`. No new failure identity
appeared. The ordinary full profile subsequently passed all 1,944 selected
tests.

## Census And Ratchet

Low start contained 13 rows across 12 modules. Final contains two raw rows
across two modules: `MeteorologyError::fmt` at CRAP 56 and
`SymbolAliasRegistryError::fmt` at CRAP 90. Their source hashes match the
current dual-reviewed `R-OBSERVABILITY` records L-08 and L-11. The actionable
filter is the empty JSON array.

The ratchet therefore removes all 11 eligible identities, adds zero, retains
two explicitly dispositioned denominator rows, and leaves zero actionable
production row above CRAP 30. Disposition: `PASS`.
