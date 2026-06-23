# R7 Blocker Ledger

Status: executed-held.

## Ledger Protocol

Each blocker entry must include defect ID, observed command/output or static
evidence, mechanism, owner surface, classification, correction, validation,
and next gate. Classifications are `in-envelope`, `out-of-envelope`,
`invalid-input`, `authority-missing`, and `evidence-unavailable`.

## Known Starting Blockers

- `R7E-DEFAULT-ACTIVATION-CANDIDATE-ABSENT`: default direct runtime selection,
  rollback policy, and manifest semantics are incomplete.
- `R7F-HOT-COMPATIBILITY-RUNTIME-NOT-ISOLATED`: production direct no-compat
  hot-loop proof is incomplete.
- `R7G-PERFORMANCE-AND-FIXTURE-GATES-UNPROVEN`: fresh H2637 performance and
  fixture matrix evidence are incomplete after R7D8.
- `R7H-RELEASE-CUTOVER-READINESS-ABSENT`: release checklist, docs, and
  anti-evasion gates are incomplete.

## Iterations

### Iteration 1 - R7E Selection And Manifest Policy

- Defect ID: `R7E-DEFAULT-ACTIVATION-CANDIDATE-ABSENT`
- Evidence:
  - Static: `crates/openwepp-runner/src/api.rs` now defines
    `HillslopeRuntimeSelection::DefaultCandidate`,
    `HillslopeRuntimeSelectionPolicy`, and
    `HillslopeRuntimeSelectionResolution`.
  - Static: `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` defaults to
    `DefaultCandidate`, adds `--direct-default-candidate`, and adds explicit
    `--compatibility-runtime` rollback.
  - Static: the run manifest now includes top-level `runtime_selection`
    provenance with requested, selected, selection reason, default activation
    gate, fallback reason, output policy, rollback runtime, and rollback
    availability.
  - Ran: `cargo test -p openwepp-runner r7e_ -- --nocapture` passed.
  - Ran: `cargo test -p openwepp-runner r7 -- --nocapture` passed.
- Mechanism: mode is resolved once before execution; default-candidate with
  disabled activation resolves to compatibility rollback, while explicit
  activation selects production direct and remains manifest-visible.
- Owner surface: runner API, CLI, manifest assembly, focused runner tests.
- Classification: closed in-envelope.
- Correction: implemented runtime-selection policy and manifest provenance.
- Validation: focused R7E tests pass and default-disabled audit counters remain
  zero.
- Next gate: R7F no-hot-compatibility proof.

### Iteration 2 - R7F Compatibility Edge Accounting

- Defect ID: `R7F-HOT-COMPATIBILITY-RUNTIME-NOT-ISOLATED`
- Evidence:
  - Static: `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
    still routes production direct away from `execute_hillslope_climate_days`,
    symbol-registry audit, indexed-shadow audit, and scheduler kernel
    execution.
  - Static: `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`
    still keeps `seed_surfaces: Vec<HillslopeWritebackSurface>` and builds
    production direct day inputs by cloning/merging compatibility-shaped
    surfaces and reading symbols inside the interleaved day/OFE loop.
  - Static: this package added production-direct compatibility-edge accounting
    through `record_direct_runtime_compatibility_edge_invocation()`.
  - Ran: `cargo test -p openwepp-runner r7 -- --nocapture` passed, including
    `r7c_direct_production_executor_reports_interleaved_day_input_compatibility_edges`
    and `r7f_remaining_direct_day_input_builder_compatibility_edge_is_accounted`.
- Mechanism: production direct no longer enters the compatibility scheduler,
  but the direct day-input builder remains a hot compatibility surface edge
  before each direct day frame executes.
- Owner surface: runner direct publication day-input builder and direct
  production execution setup.
- Classification: unresolved architecture blocker.
- Correction completed in this package: false zero-edge evidence was removed;
  production direct manifests now count the remaining compatibility edge.
- Required correction not completed: replace the interleaved
  `DirectPublicationDayInputBuilder` with typed direct day-input/state
  projection that does not construct `HillslopeWritebackSurface`,
  `BoundarySymbol`, or `BoundaryValue` in the production direct day/OFE loop.
- Validation: focused tests prove the edge is now counted. R7F no-compatibility
  acceptance remains failed.
- Next gate: close
  `HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`.

### Blocked Gates

- `R7G-PERFORMANCE-AND-FIXTURE-GATES-UNPROVEN`: blocked because performance
  closure would be invalid while production direct still has a counted hot
  compatibility surface edge.
- `R7H-RELEASE-CUTOVER-READINESS-ABSENT`: blocked because release readiness
  cannot claim direct mode as normal while R7F is red.
