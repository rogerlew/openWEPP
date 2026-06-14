# Watershed CLI Current Characterization

Status: W-A executed

Evidence mode: Ran + Static

## Command Evidence

Ran from `/home/workdir/openWEPP` on 2026-06-13.

Inputs:

- Runfile adapted from
  `/tmp/openwepp_mofe01_mh_final/watershed/run/case.run`.
- HBP shards and manifests retargeted to
  `/tmp/openwepp_mofe01_mi_final/output/H*.hbp` and
  `/tmp/openwepp_mofe01_mi_final/manifests/H*.json`.
- Verified substrate count: `36` HBP shards and `36` manifests.
- Watershed input files from
  `/wc1/runs/ar/arboreal-dendrite/wepp/runs/`.

Command:

```bash
target/debug/openwepp-cli-watershed \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wa/watershed/output \
  --policy compat \
  --legacy-sidecar-discovery
```

Result:

```text
exit=1
stderr:
CLIWAT-E-010 failed parsing watershed impoundment /wc1/runs/ar/arboreal-dendrite/wepp/runs/pw0.imp: IMP-E-004: line 2 invalid domain value '0' for jpond; expected >= 1
output-files=0
```

The run produced no watershed parquet outputs.

## Failure Placement

Static source placement:

- `openwepp-cli-watershed.rs:223-237` parses `pw0.chn` and `pw0.slp`
  before impoundments.
- `openwepp-cli-watershed.rs:239-254` parses `pw0.imp` with
  `expected_structural_count: Some(structure.summary.impoundment_count)` and
  wraps parser errors as `CLIWAT-E-010`.
- `openwepp-cli-watershed.rs:258-285` parses `chan.inp` only after the
  impoundment parse; this was not reached.
- `openwepp-cli-watershed.rs:327-343` parses HBP pass shards only after
  runtime-surface seeding; this was not reached.
- `openwepp-cli-watershed.rs:476-497` executes watershed dispatch and writes
  outputs only after HBP ingestion; this was not reached.

The current run therefore does not characterize routed channel-network output.
It only proves the first hard stop: the no-impoundment parser rejection blocks
the CLI before `chan.inp`, HBP shard consumption, dispatch, or output writing.

## Fixture Notes

Substrate evidence:

- `/wc1/runs/ar/arboreal-dendrite/wepp/runs/pw0.imp` contains `99.1` then
  `0`.
- `/wc1/runs/ar/arboreal-dendrite/wepp/runs/pw0.str` contains only channel
  element rows (`2`) and zeroes in all three impoundment-contributor columns.
- `/wc1/runs/ar/arboreal-dendrite/wepp/runs/chan.inp` is present, but the
  current failure occurs before it is parsed.

## Conclusion

The W-A current-tree behavior is fail-closed before any routed watershed output
exists. W-B must clear this typed no-impoundment parser defect before W-C can
measure routing/output behavior.
