# Disposition

Evidence class: Ran + Static

Result: `EXECUTED-HOLD-PARTIAL-RSS-REDUCTION`

## Summary

This package delivered a large identity-preserving RSS reduction, but it did not
close the full run-length-flat RSS gate.

Implemented:

- Removed production direct preallocation of
  `Vec<DirectDayConstructorInputs>`.
- Moved retained direct publication execution instead of cloning it.
- Built WAT/PASS projection vectors only when requested.
- Preserved fail-closed optional-output validation.

Measured:

- H2637 full-output RSS: `1159672 KiB` -> `316212 KiB`.
- H2637 HBP/loss-only RSS: `1159296 KiB` -> `184644 KiB`.
- H2637 full-output HBP/WAT/PASS/loss/plot bytes: unchanged.
- H2637 minimized HBP/loss bytes: unchanged.
- Direct runtime counters: `compatibility_edge_invocations=0`.

## Hold Reason

The requested acceptance gate was run-length-flat RSS. That is not yet true.

Remaining blockers:

- `BLOCKED-BY-RETAINED-DIRECT-PUBLICATION-FRAME`: the direct publication frame
  still retains all `DirectPublicationDayRow` values for the full run.
- `BLOCKED-BY-FULL-OUTPUT-PROJECTION-BUFFERS`: WAT/PASS projection vectors and
  parquet/Arrow writer buffers still scale with requested output row count.
- `BLOCKED-BY-FULL-NEXTEST-ENV`: the full nextest profile failed because this
  worktree lacks `.venv/bin/python` for Python-backed harness tests.

## Recommended Follow-On

Run a follow-on package focused on streaming direct publication:

1. Replace whole-run `DirectRunPublicationFrame.rows` retention with a sink that
   validates, summarizes, and emits day rows as they are produced.
2. Preserve byte identity for HBP/loss and, where possible, WAT/PASS/plot.
3. If parquet row-group chunking is required, decide whether the gate remains
   byte identity or is amended to semantic row identity with explicit parquet
   metadata evidence.
4. Re-run H2637, a shorter fixture, and a longer fixture to prove the RSS slope
   is flat enough for the direct publication endpoint.
5. Resume typed setup/symbol-map carrier deletion only after the publication RSS
   blocker is closed.

No physics, output schema, runtime policy, default, or compatibility fallback
change was made by this package.
