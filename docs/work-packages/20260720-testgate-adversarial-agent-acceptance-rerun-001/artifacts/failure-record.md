# Fail-Closed TESTGATE Attempt

Evidence class: `Ran`.

## Command

```text
python tools/local_ci/testgate.py --binary target/debug/openwepp-gate-plan --base HEAD --artifact-root /tmp/openwepp-testgate-adversarial-QKvyqa --intent-package docs/work-packages/20260720-testgate-adversarial-agent-acceptance-rerun-001/package.md --dirty --execute
```

The fresh root was empty before invocation. The controller had rehashed the
sentinel successfully and removed only that known file before this command.

## Result

The one admitted invocation stopped before intent planning:

```text
ERROR: fatal: path 'docs/work-packages/20260720-testgate-adversarial-agent-acceptance-rerun-001/package.md' exists on disk, but not in '98613275bed9eb07ec77bf1975b712f7a13d2892'
```

The helper created only `/tmp/openwepp-testgate-adversarial-QKvyqa/execution/`.
There is no authorization record, intent plan, terminal plan, observation, or
receipt. This is an authorization rejection before the helper's receipt-writing
phase, not a missing artifact to recreate or relabel.

## Root Cause And Disposition

`tools/local_ci/testgate.py` reads an explicitly requested intent package from
the selected base commit before planning. This package is untracked in the
current worktree, so it cannot authorize its own dirty increment at `--base
HEAD`. Static source inspection also found a second independent schema defect:
the helper requires the exact heading `## Declared Write Set`, while this
package uses `## Intended Write Set`.

The package requires one local TESTGATE execution and directs the controller to
stop immediately if planning differs from the contract. No retry, alternate
base, staging, commit, broad test, or receipt fabrication is authorized.

Disposition: `FAIL — TESTGATE-INTENT-PACKAGE-BASE-AUTHORIZATION`.
