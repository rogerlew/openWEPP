# Static Call-Graph Audit

Evidence class: Static

## Decision

The package preserves the explicit `Compatibility` runtime selector as a replay and
comparator seam. The obsolete skeleton, shadow, and cutover transition modes are
deleted from the public runner API and CLI.

## Initial Findings

- `DefaultCandidate` resolves to `DirectProductionExecutor` under the production
  activation gate.
- `Compatibility` remains reachable only through explicit `--compatibility-runtime`
  or an intentionally disabled default activation in tests.
- `DirectProductionExecutor` already emits direct publication artifacts from the
  retained direct execution frame.
- `HillslopeWritebackSurface` still has setup-time and replay-seam uses outside the
  direct hot-loop. Full type deletion therefore requires a later typed setup-carrier
  package and is not part of this stage.

## Deleted Production-Transition Modes

- Deleted `HillslopeRuntimeSelection` variants:
  `DirectSkeletonNoop`, `DirectSkeletonShadowOnly`,
  `DirectPublicationFrameShadow`, and `DirectPublicationFrameCutover`.
- Deleted CLI flags: `--direct-runtime-skeleton`,
  `--direct-publication-frame-shadow`, and
  `--direct-publication-frame-cutover`.
- Deleted the retained/cutover adapter family:
  `RetainedDirectPublicationRequest`,
  `build_retained_direct_publication_frame`, and
  `build_direct_publication_execution_from_simulation_outputs`.
- Deleted the compatibility-shaped day-input builder family:
  `DirectPublicationDayInputBuilder` and its runtime-surface overlay helpers.
- Deleted stale tests that intentionally exercised the removed selectors and
  adapter-only helper paths.
- Added a source guard in
  `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs`
  that rejects reintroduction of the obsolete selectors, flags, and adapter
  helper names in runner production/API/CLI sources.

## Forbidden Production Reachability Scan

Ran:

```bash
rg -n "DirectSkeletonNoop|DirectSkeletonShadowOnly|DirectPublicationFrameShadow|DirectPublicationFrameCutover|direct-runtime-skeleton|direct-publication-frame-shadow|direct-publication-frame-cutover|DirectPublicationDayInputBuilder|build_retained_direct_publication_frame|RetainedDirectPublicationRequest|build_direct_publication_execution_from_simulation_outputs" crates/openwepp-runner/src crates/openwepp-runner/tests -g '!crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs'
```

Result: no matches.

The full workspace `nextest` run also exercised the source guard and the
runtime-selection tests that prove no-env default and legacy sidecar-discovery
runs select `direct-production-executor` with
`compatibility_edge_invocations = 0`. The multi-OFE/Wave-2 and public per-OFE
WAT tests passed under the same full run.

## Retained Seam / Deferred Deletion

The explicit `Compatibility` selector and `--compatibility-runtime` CLI flag
remain. This is the operator-chosen replay/comparator seam, not a production
fallback.

`HillslopeWritebackSurface` and related symbol-map types still have setup-time,
legacy parser, watershed, and explicit replay uses outside the production direct
hot loop. Full type deletion needs a typed setup-carrier/full replay-seam
deletion package. This package therefore closes as production-transition
runtime deletion, not total symbol-map type deletion.
