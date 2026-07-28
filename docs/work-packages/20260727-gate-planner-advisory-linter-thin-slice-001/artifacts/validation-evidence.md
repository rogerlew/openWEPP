# Validation Evidence

Evidence class: Ran.

## Focused Tests

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python \
  -m unittest tools/validation/test_workplan_lint.py
........................
----------------------------------------------------------------------
Ran 25 tests in 3.738s

OK
```

The suite covers all three modes, deterministic output, findings with exit
zero, partial/unavailable/misuse output, argument and operand validation,
timeout and output bounds, exact Git argv/environment/stdin confinement,
pre-launch refusal for every prohibited configuration and attribute class,
helper and network canaries, file/index/object byte-and-metadata stability,
symlink refusal, wrapper behavior, exact dirty rename/conflict parsing,
identity-conflict and detached-HEAD findings, full Git-tree metadata snapshots,
live fixed-bound capture, OS-level absence of IP network calls in every mode,
and absence of legacy imports or secondary subprocess surfaces.

## Documentation And Static Checks

```text
.venv/bin/python -m py_compile \
  tools/validation/workplan_lint.py \
  tools/validation/test_workplan_lint.py
PASS

markdown-doc lint --path tools/validation
1 files validated, 0 errors, 0 warnings

markdown-doc lint --path \
  docs/work-packages/20260727-gate-planner-advisory-linter-thin-slice-001
11 files validated, 0 errors, 0 warnings

git diff --check
PASS
```

## Native Repository Exercise

All three native-repository modes produced one canonical JSON document,
retained package and policy-input analysis, reported `analysis_status` as
`partial`, and exited 3 before Git launch because openWEPP declares prohibited
Git LFS clean, smudge, process, and attribute drivers. Each invocation
completed in less than one second. This is the required frozen-contract
refusal behavior, not a lifecycle verdict.

## Line Count

```text
 1001 tools/validation/workplan_lint.py
   10 tools/validation/workplan-lint
1,011 production lines
```
