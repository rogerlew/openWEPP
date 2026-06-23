# Mode Selection And Consumer Path

Status: executed-held.

## R7E Evidence

- Static: `HillslopeRuntimeSelectionPolicy::default()` requests
  `default-candidate` with default activation `disabled`.
- Static: default-disabled resolution selects `compatibility`, records
  `selection_reason = default-candidate-disabled-compatibility-rollback`, and
  records `fallback_reason = direct-default-candidate-gate-disabled`.
- Static: explicit default-candidate activation selects
  `direct-production-executor` and records
  `default_activation_gate = direct-production-candidate`.
- Static: `--compatibility-runtime` is an explicit CLI rollback path and cannot
  be combined with `--direct-default-candidate`.
- Ran: `cargo test -p openwepp-runner r7e_ -- --nocapture` passed.

## Consumer-Path Proof

- Producer source: runner API and CLI construct a
  `HillslopeRuntimeSelectionPolicy`; `execute_hillslope_run_with_runtime_policy`
  resolves it exactly once before input setup/execution.
- In-memory state/frame object: selected runtime is a
  `HillslopeRuntimeSelection`; production direct still builds a
  `DirectRunFrame` and retained `DirectRunPublicationFrame`.
- Runner handoff:
  `execute_selected_hillslope_days(..., runtime_resolution.selected(), ...)`.
- Downstream consumer call site:
  - default-disabled and explicit compatibility select
    `execute_hillslope_climate_days`;
  - explicit direct and activated default-candidate select
    `execute_hillslope_direct_production_days`;
  - direct publication/cutover modes keep their existing explicit paths.
- Output/API surface:
  - public API exports `HillslopeDefaultRuntimeActivation`,
    `HillslopeRuntimeSelectionPolicy`, and
    `execute_hillslope_run_with_runtime_policy`;
  - CLI help includes `--compatibility-runtime`,
    `--direct-default-candidate`, and `--direct-production-executor`;
  - manifest has top-level `runtime_selection` provenance.
- Negative proof old path is not used:
  - default-disabled test asserts no direct runtime counters and no direct
    frame/executor construction;
  - activated default-candidate test asserts scheduler kernel did not execute;
  - R7F source test still proves the production direct scheduler path excludes
    compatibility scheduler/kernel request entrypoints.
