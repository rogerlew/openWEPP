# Direct Execution Procedure

Status: `PROSPECTIVE`

Evidence class: `Static`

`direct-execution-plan.json` contains the literal argv arrays and fixed path
placeholders. The package-local executor runs the calibration phase directly:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/execute-prefix.py \
  --execution-root <fresh-attempt>/objects
```

The attempt directory must already exist and the execution root must not. The
executor creates `objects`, `publication`, `cargo-target`, and
`direct-evidence`. Each command receives a literal argument vector without
shell evaluation. Standard output, standard error, source identity, argv,
environment delta, timestamps, exit state, and declared outputs are recorded
under `direct-evidence`.

On the first failure, `primary-failure.json` and the command log are flushed and
fsynced before return. No cleanup or publication path can delete that directory.
Successful completion creates `calibration-complete.json`.

Freeze consumes that completion record and the calibration artifacts directly.
Two independent read-only verifier invocations create `verifier_a.csv` and
`verifier_b.csv`; the barrier validates both against the same freeze digest and
publishes their summary. No capability, dispatch, attestation, workflow, runner,
or transaction identity is involved.

Holdout requires an existing empty output root separate from calibration:

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/holdout.py \
  --execution-root <attempt>/objects \
  --custody-root <custody> \
  --holdout-output-root <empty-holdout-output>
```

The launcher fails if `bubblewrap` is unavailable. Inside the sandbox, the
repository and calibration attempt are read-only; only custody and the separate
holdout-output root are writable. The `OPENED_ONCE` token is created and fsynced
before the first Harvard content read. A post-open failure forbids rerun.
