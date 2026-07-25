# Heavy Attempt 05: Identity-Bound Venv Git Exclude

Evidence class: Ran / Static.

Attempt:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt5-iAfUo6`.

Admission ID:
`62259cf9a80586818692dda0ab5a2ef9a97c94ec39b1113c1344a375cfc2e2a8`.

## Result

- `full`: 2,279 run; 2,276 passed; 3 failed; 31 skipped; `1826.231s`.
- `science-manual`, merge, CRAP, snowbench disposition, publication, and
  terminal verification: not run.
- Published files: 0.

The three failures were:

- `pre_heavy::coverage_tests::exact_planner_output_reconstructs_through_the_public_audit_path`
- `verifier::tests::receipt_verification_reconstructs_identity_dag_inventory_and_artifacts`
- `verifier::tests::verifier_accepts_truthful_fail_and_blocked_receipts`

All returned `GATE-COMMITTED-CHECKOUT-NOT-EXACT`.

## Root Cause

The execution clone's exact Git status contained one entry:

```text
?? .venv
```

The observatory intentionally creates that root symlink and independently
binds its target in `working_tree_identity`. `git clone --local --no-hardlinks`
does not inherit the source checkout's `.git/info/exclude`, so gate-planner
committed-source observation saw the required symlink as ordinary untracked
dirt.

## Correction

Immediately after creating the required symlink, the execution clone replaces
its local `.git/info/exclude` with exact bytes `/.venv\n` and proves the path
is Git-clean. Broad pre-existing rules are removed. The link target and exact
local-exclude bytes remain explicitly hashed. No tracked path, index metadata,
or other nonignored untracked path is excluded.

Focused behavioral self-test proves all properties:

- the required link leaves the clone Git-clean;
- replacing its target changes `working_tree_identity`;
- other untracked drift remains visible and changes identity;
- later local-exclude policy drift is rejected;
- a symlinked `.git/info` directory is rejected.
