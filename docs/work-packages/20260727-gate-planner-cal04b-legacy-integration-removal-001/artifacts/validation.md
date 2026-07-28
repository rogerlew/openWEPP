# Validation

Evidence class: `Ran + Static`

## Focused tests

Command:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m unittest \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_execute_prefix.py \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_publish_results.py \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_freeze_custody.py \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_external_paths.py
```

Result: `PASS`, 19 tests. The suite uses temporary synthetic files and does not
read Harvard or run CAL.

The retained compatibility test also passed from its tool directory:

```text
PYTHONDONTWRITEBYTECODE=1 /home/workdir/openWEPP/.venv/bin/python \
  -m unittest test_observe.py
```

Result: `PASS`, 1 test.

## Static and documentation checks

- Direct executor validation: `PASS`, 9,261 candidates, 27,783 saturation rows,
  and 18 exact commands.
- Python compilation for all changed prospective tools: `PASS`.
- `direct-execution-plan.json` parse with `jq empty`: `PASS`.
- Prospective-tool scan for planner binaries, external-transition commands,
  Generation-B, and calibration/holdout transaction receipts: zero matches.
- Final Markdown lint for the Order 2 and CAL-04B packages: `PASS`, 59 files, zero
  errors and zero warnings.
- `git diff --check`: `PASS`.

An initial supplementary invocation of `test_observe.py` by repository-relative
file path could not import its sibling module. Running that legacy test from its
own tool directory passed; this was an invocation-context issue, not a product
failure.
