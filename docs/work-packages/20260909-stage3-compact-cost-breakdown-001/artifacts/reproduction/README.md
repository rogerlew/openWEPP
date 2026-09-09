# Reproduction

Run `run_series.py` from the isolated P source directory, after building the
frozen release test executable. The collector creates a new output directory,
pins every fresh child process to CPU0, retains full logs and hashes, checks
protected scientific identity, and rejects incomplete additive accounting.

```sh
python /workdir/openWEPP/docs/work-packages/20260909-stage3-compact-cost-breakdown-001/artifacts/reproduction/run_series.py \
  --binary /workdir/.cache/openwepp/targets/compact-cost-P/release/deps/openwepp_runner-682944f41aa5eb67 \
  --environment /workdir/openWEPP/docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/reproduction/runtime-environment.json \
  --identity /tmp/openwepp-compact-cost-P-vKI6gb/p-final-admission-identity.json \
  --out /tmp/openwepp-compact-cost-P-vKI6gb/results/series-03 \
  --cwd /tmp/openwepp-compact-cost-P-vKI6gb
```
