# FQ3 Run Manifest

Evidence mode: Ran

## Scope

- Run root: `/wc1/runs/al/algebraic-radium/wepp/runs`
- Population under validation: 42 single-OFE hillslopes (`p1..p43`, excluding `p11`)
- openWEPP source outputs: reused post-FQ1 validated corpus at `/tmp/fq1_after/outputs`
- Legacy comparator outputs: regenerated in this package at `/tmp/fq3_exec/legacy/outputs`

## Binaries

- openWEPP: `/workdir/openWEPP/target/release/openwepp-cli-hill`
  - sha256: `0d87032deaa62d96a941ad3aaa9d02520369a7e1f457c68a40174eaaab7868dd`
- Legacy comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
  - sha256: `1fb763b013329d205ec92cece827f8e8d424cf3e8427bd5b3f961b672e9610d8`

## Commands Run

1. Legacy rerun from authoritative input directory so relative files (`p*.man`, `p*.cli`, etc.) resolve:

```bash
BASE=/tmp/fq3_exec/legacy
RUNROOT=/wc1/runs/al/algebraic-radium/wepp/runs
BIN=/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill
for rf in "$BASE"/runfiles/p*.run; do
  p=$(basename "$rf" .run)
  outdir="$BASE/outputs/$p"
  mkdir -p "$outdir"
  (cd "$RUNROOT" && "$BIN" < "$rf" > "$outdir/stdout.log" 2> "$outdir/stderr.log")
  echo -e "$p\t$?" >> "$BASE/run_status.tsv"
done
```

2. Legacy WAT interchange and group/term characterization using `/workdir/wepppy/.venv/bin/python` with `PYTHONPATH=/workdir/wepppy`.

## Run Outcomes

- Legacy rerun status: 42/42 prefixes `rc=0`
- Legacy WAT interchange rows: 107,394
- openWEPP WAT rows consumed (from `/tmp/fq1_after/outputs`): 107,394
- Generated evidence outputs:
  - `/tmp/fq3_exec/reports/management_group_map.csv`
  - `/tmp/fq3_exec/reports/per_prefix_term_classification.csv`
  - `/tmp/fq3_exec/reports/group_term_aggregate.csv`
  - `/tmp/fq3_exec/reports/fq3_summary.json`

## Notes

- Initial legacy execution attempt was invalid because it ran outside `RUNROOT` and failed to resolve `p*.man`; this package reran comparator cases correctly and used only corrected outputs for classification.
- Legacy WAT schema does not include `Interception`; those rows are classified as `legacy-term-unavailable`.
