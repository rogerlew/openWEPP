# Publication Identity

Status: `PARTIAL-PRE-FIX-PASS`

Evidence class: **Ran + Static** test binding; release closure blocked.

The W7R production CLI test runs the same committed p102 watershed with
`--jobs 1` and `--jobs 4`. Its `assert_watershed_outputs_row_equivalent`
comparison reads every required Parquet output and compares decoded schema and
rows, including value/null posture independent of file container bytes. It also
checks nonzero HBP sediment and public `totalwatsed3`/EBE consumption. Command
10 passed this binding at the frozen source. Command 16 then failed before
release closure, so this result is partial pre-fix evidence and cannot be
reused by the required campaign restart.
