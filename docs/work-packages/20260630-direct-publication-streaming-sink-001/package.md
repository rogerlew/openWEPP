# Direct Publication Streaming Sink

Status: EXECUTED-COMPLETE-STREAMING-RSS-REDUCTION

Package id: `20260630-direct-publication-streaming-sink-001`

## Objective

Flatten the residual direct-publication RSS slope by streaming direct
publication rows and requested parquet outputs instead of retaining all
run-length-scaled rows in memory.

## Required Reading

- `docs/work-packages/20260630-direct-publication-rss-reduction-001/package.md`.
- `docs/work-packages/20260630-direct-publication-rss-reduction-001/artifacts/stage-a-heap-attribution.md`.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`.
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`.
- Direct publication helpers under
  `crates/openwepp-runner/src/hillslope/direct_publication/`.
- HBP/WAT/PASS writer paths in `crates/openwepp-hillslope-output/src/`.

## Scope

Stage 1 streams direct publication rows through a production sink and drops each
row after publication consumers have seen it. Retained-frame APIs stay available
for tests and diagnostic callers.

Stage 2 writes requested WAT/PASS parquet outputs incrementally in bounded row
groups. Parquet row-group layout may differ; column schema, row count, and values
must not.

## Non-Scope

- No typed setup or symbol-map carrier deletion.
- No physics, output-schema column, default-policy, snow, frost, hydrology, or
  erosion behavior change.
- No change to non-direct compatibility/replay output behavior.

## Gates

- RSS measured for H2637, a materially shorter run, and a longer or equal stress
  run at each relevant step.
- RSS-vs-run-length slope is materially flattened versus the prior held package.
- HBP/loss/manifest and non-parquet required outputs remain byte-identical.
- WAT/PASS parquet outputs preserve schema, row count, and values after
  incremental row groups.
- Direct manifests continue to select `direct-production-executor` with
  `compatibility_edge_invocations=0`.
- Full closure loop: `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`
  in a `.venv`-capable environment, `cargo deny check`, authority anti-evasion,
  required-suite obligation guard, and scoped Markdown lint/validate.

## Disposition

Complete. Direct production now streams publication rows into bounded
publication consumers and writes WAT/PASS parquet through incremental row-group
writers. The old retained-frame API remains available for tests/diagnostic
capture, but the production direct endpoint no longer accumulates
`DirectRunPublicationFrame.rows` or whole-run WAT/PASS projection vectors.

H2637 full-output RSS dropped from the prior held package's `316212 KiB` to
`112652 KiB`; H2637 required-output RSS dropped from `184644 KiB` to
`52228 KiB`. The required-output result is nearly flat relative to the
`16437`-day W9 single-OFE observed fixture (`47856 KiB`) despite H2637 emitting
`235961` publication rows. H2637 HBP/loss/plot/WAT/PASS outputs are
byte-identical to the retained-row baseline.
