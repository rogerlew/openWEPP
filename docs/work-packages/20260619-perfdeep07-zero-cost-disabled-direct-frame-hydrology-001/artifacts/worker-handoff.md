# PERFDEEP07 Worker Handoff

Status: HOLD.
Evidence mode: Static/Ran.

## Final Disposition

PERFDEEP07 is in `HOLD`. The retained patch reduces the default-disabled H2637
endpoint from PERFDEEP05's `701.95 s` to `685.85 s`, but the P0 threshold is
`<= 676.67 s`.

## What Changed

- Dense-absent request lookup now goes directly to indexed surfaces.
- Hydrology state/flux reads avoid dense lookup when no dense surface exists.
- Hot symbol lookup maps use `HashMap`.
- Runner indexed scheduler resources have fail-closed optional context guards.
- A kernel-contract regression test covers indexed request lookup without dense
  slots.

## Do Not Re-try First

The following experiments were slower and should not be repeated without new
evidence:

- disabling the production indexed scheduler path entirely;
- rebuilding indexed writeback authority after every day;
- propagating indexed surfaces through execution reports;
- direct logical-map bypass whenever hot tables are absent.

## First Actionable Follow-up

Continue from the retained dense-absent/`HashMap` patch and find the remaining
default-path cost before any direct-frame hydrology implementation. The next
candidate should prove a single-run result below `676.67 s` before spending
time on a three-run median gate.

## Validation State

Focused tests and retained-output identity passed. Full workspace/clippy/deny
closure gates were not run because this is a HOLD state, not a completion
claim.
