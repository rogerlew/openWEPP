# Fixture Matrix

Evidence class: Static plus Ran.

Status: executed-held.

| Fixture family | Coverage source | R7G status | Residual risk |
|---|---|---|---|
| snow/frost active days | Full H2637 with `snow.txt`; focused R7/R6 historical fixtures | `FAIL/BLOCKED` for direct full H2637 | First direct blocker is active snow partition authority. Frost may be next but was not reached; default manifest final coupling vectors still show `frsoil.active = true`. |
| breakpoint climate | Existing runtime-input tests, not rerun in R7G | `NOT RUN` | Needs direct production fixture after active snow authority lands. |
| PMET branches | Focused R7 test `r7d_direct_day_two_pmet_seed_keeps_direct_wb14_lineage_boundary` | `PASS focused / BLOCKED full H2637 direct` | Full-H2637 PMET direct evidence blocked before first active-snow direct day. |
| irrigation when enabled | Existing parser/runtime surfaces only | `NOT RUN` | Needs explicit enabled-irrigation direct fixture; no R7G direct evidence. |
| multi-OFE transfer ratios | H2637 compatibility/rollback; R7D4/R7D8 historical direct 5-day evidence | `PASS compatibility / BLOCKED full direct` | Direct full-H2637 transfer evidence blocked before output. |
| nonzero erosion | R7D8 H2637 5-day nonzero HBP sediment export evidence | `PASS focused / BLOCKED full direct` | Full direct nonzero erosion parity blocked by active snow before output. |
| sidecar absence/presence | H2637 sidecar presence (`pmetpara`, `snow`) | `PASS presence compatibility / BLOCKED direct / NOT RUN absence` | Direct presence fails at snow authority; absence fixture still needed. |
| management transitions | Existing constructor/phase fixtures, not rerun in R7G | `NOT RUN full direct` | Needs post-snow-authority direct fixture coverage. |

## Notes

R7G documents the required fixture families but cannot harden them to passing
full-direct evidence until the active snow authority blocker is lifted. The
next package must start with snow/frost because it blocks the full H2637
direct endpoint before timing, parity, or downstream fixture expansion can
execute.
