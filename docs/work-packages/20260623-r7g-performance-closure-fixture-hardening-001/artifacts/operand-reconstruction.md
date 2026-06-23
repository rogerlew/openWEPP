# Operand Reconstruction Evidence

Evidence class: Static plus Ran.

Status: executed-held.

## Conservation-Sensitive Output Families

| Family | Operand authority | Reconstruction evidence | Anti-alias status | Disposition |
|---|---|---|---|---|
| HBP runoff/sediment | R6J/R7D8 direct publication frame for focused fixtures; compatibility scheduler for R7G full H2637 | Existing focused reconstruction only | Focused nonzero sediment anti-alias exists from R7D8 | `BLOCKED` for full direct H2637 because no direct output was produced. |
| WAT storage/ET/snow/frost | Direct publication rows for focused fixtures; compatibility WB13 for R7G full H2637 | Existing R6J focused reconstruction only | Active snow/frost fixture now fails before direct output | `BLOCKED` for full direct H2637. |
| PASS runoff/sediment | Direct publication rows for focused fixtures; compatibility scheduler for R7G full H2637 | Existing R6J/R7D focused reconstruction only | Focused PASS aliases covered historically | `BLOCKED` for full direct H2637. |
| loss JSON | Direct publication metadata for focused fixtures; compatibility output for R7G full H2637 | Existing R6J focused reconstruction only | Sidecar presence exercised by H2637 compatibility | `BLOCKED` for full direct H2637; absence not run. |
| manifest counters/checksums | R7G compatibility/rollback manifests | Output checksum map independently compared with `jq -S .output_checksums` and `cmp=0` | Direct counter manifest absent due fail-closed direct run | `PASS` for compatibility rollback; `BLOCKED` for full direct. |

## Notes

No new conservation-sensitive production correction landed in R7G. R7G
therefore does not add new reconstruction claims beyond checking that
compatibility rollback preserves the protected output checksum map. Direct
full-H2637 reconstruction remains blocked until typed active snow partition
authority produces direct outputs.
