# Full 39 Suite Metrics

Status: complete

Evidence mode: Ran

Ran:

Command:

```sh
RUN_ROOT=/tmp/hphys0320_stmtim_start_time_source_line_20260606T000000Z
RUNS=/tmp/hphys0305_paired_melt_terms_20260605T000000Z/runs
OUT="$RUN_ROOT/full39_hillslope_output"
STATUS="$RUN_ROOT/full39_hillslope_status.tsv"
for hill in $(seq 1 39); do
  OPENWEPP_HPHYS0245_TRACE_PATH="$OUT/H${hill}.hphys0320.trace.jsonl" \
  OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=1800 \
  target/release/openwepp-cli-hill \
    --run-dir "$RUNS" \
    --run-file "p${hill}_openwepp.run" \
    --output-dir "$OUT" \
    --policy compat
done
```

Status:

- Batch status:
  `/tmp/hphys0320_stmtim_start_time_source_line_20260606T000000Z/full39_hillslope_status.tsv`
- H1..H39 release-binary hillslope runtime: `39/39` exited `0`.
- Full trace directory:
  `/tmp/hphys0320_stmtim_start_time_source_line_20260606T000000Z/full39_hillslope_output/`

Semantic comparator status:

- HPHYS0320 changed production timing projection, so same-runtime static
  carry-forward alone was not used.
- A local H1..H39 release-binary runtime batch with trace capture was run and
  passed.
- No broader semantic comparator target change is claimed here; ADR0017 still
  treats comparator rows as investigation flags, and this package closes only
  the `stmtim` timing-seam source ownership for the combined `57` carried rows.
