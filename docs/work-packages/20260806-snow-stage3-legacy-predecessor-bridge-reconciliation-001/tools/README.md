# Predecessor Bridge Tools

These package-local tools implement the prospectively frozen endpoint phase.
They write only below
`target/snow_stage3_legacy_predecessor_bridge_reconciliation/`.

`run_predecessor_bridge_matrix.py` owns source checkout, offline release builds,
the four endpoint cells and their controls, runtime-manifest custody, protected
output checks, and read-only verification. It performs no science reduction.

`reconstruct_predecessor_bridge.py` independently parses schema-v4 aggregates
and schema-v6 tuple primitives, reduces the 35 frozen windows, computes the
per-water-year factorial contrasts before medians, and applies the frozen
checkpoint predicates. It does not import the runner.

From a clean admitted HEAD, execute:

```bash
.venv/bin/python \
  docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/tools/run_predecessor_bridge_matrix.py \
  --execute --expected-head <full-sha>
.venv/bin/python \
  docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/tools/reconstruct_predecessor_bridge.py \
  --reconstruct
```

Always execute the frozen checkpoint decision after endpoint reconstruction.
When neither source gate triggers, this writes an explicit no-op receipt. When
one or both trigger, it builds and runs all 14 frozen build-input checkpoints
for exactly those forcing lanes:

```bash
.venv/bin/python \
  docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/tools/run_predecessor_bridge_matrix.py \
  --execute-checkpoints --expected-head <full-sha>
.venv/bin/python \
  docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/tools/reconstruct_predecessor_bridge.py \
  --reconstruct-checkpoints
```

Then verify every retained byte without model execution:

```bash
.venv/bin/python \
  docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/tools/run_predecessor_bridge_matrix.py \
  --verify-existing
.venv/bin/python \
  docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/tools/reconstruct_predecessor_bridge.py \
  --verify-existing
```

Run model-free tests with:

```bash
.venv/bin/python -m unittest \
  docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/tools/test_run_predecessor_bridge_matrix.py \
  docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/tools/test_reconstruct_predecessor_bridge.py
```

If a checkpoint lane identifies a first divergent build-input transition, stop
causal disposition until the package's contract-first twin instrumentation has
localized that transition. A no-trigger receipt permits endpoint disposition
without checkpoint builds.
