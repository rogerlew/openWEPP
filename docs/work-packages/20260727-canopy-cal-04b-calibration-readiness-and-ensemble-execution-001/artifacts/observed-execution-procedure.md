# Observed Execution Procedure

Status: `FROZEN PROSPECTIVE CONTROL`

Evidence class: `Static`

All commands run from `/home/workdir/openWEPP`. The text after `--` must be
copied byte-for-byte from the matching `argv` field in
`executor-command-plan.csv`; the runner tokenizes both values and refuses any
difference. Do not invoke a planned command directly.

## Comparator executor

For command IDs `prepare` through `summarize_pre_freeze`, in plan order, run:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/observe.py run --command-id <id> -- <frozen argv>
```

The wrapper refuses a missing/non-PASS prerequisite or any existing receipt,
log, or output manifest. After `summarize_pre_freeze` succeeds, create the
immutable snapshot:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/observe.py render --through summarize_pre_freeze --snapshot pre-freeze
```

Then execute `freeze` through the same `observe.py run` form. Do not run the
freeze script directly. The freeze command validates and checksum-binds the
snapshot and every observed receipt/log/output manifest it names.

The comparator may run the exact same bounded prefix without shell evaluation
using:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/execute-prefix.py
```

That coordinator reads the frozen plan, invokes each row through `observe.py`
with its exact tokenized argv, creates only the pre-freeze snapshot, and has no
freeze or holdout path.

## Independent freeze verifiers

Verifier A runs only `freeze_verify_a`; after its PASS receipt exists, verifier
B independently runs only `freeze_verify_b`. Each uses the same observed runner
form and exact frozen argv. Each verifier internally runs
`validate_preopen.py`, which checks the observed ledger through `freeze` and
the Harvard-free semantic state before issuing its immutable verifier receipt.

Verifier A:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/observe.py run --command-id freeze_verify_a -- PYTHONDONTWRITEBYTECODE=1 .venv/bin/python docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py --verifier-id verifier_a
```

Verifier B:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/observe.py run --command-id freeze_verify_b -- PYTHONDONTWRITEBYTECODE=1 .venv/bin/python docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py --verifier-id verifier_b
```

After both independent agents return PASS, the coordinator runs
`freeze_barrier` through the observed runner. No verifier or coordinator reads
Harvard content.

## Holdout and terminal derivation

The holdout agent runs `holdout` through the observed runner exactly once.
Holdout preflight reruns `validate_preopen.py`, validates the observed ledger
through `freeze_barrier`, then validates the transitive freeze and external
verifier receipts before the durable opening token.

Run `summarize_post_holdout` through the observed runner, then derive the
pre-terminal artifacts:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/observe.py render --through summarize_post_holdout
```

Run `terminal_validate` through the observed runner. Finally rerun `render`
with `--through terminal_validate`. This last control-plane render replaces the
terminal `command-log.csv` and `execution-inventory.csv` solely from validated
append-only receipts; it does not synthesize PASS from the prospective plan.
