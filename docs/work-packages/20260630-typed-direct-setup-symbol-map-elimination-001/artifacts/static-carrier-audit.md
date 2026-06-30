# Static Carrier Audit

Evidence class: Static + Ran

## Production Symbol-Map Carriers Still Present

`rg` over `crates/openwepp-hillslope-orchestrator/src` found `620` references to
`HillslopeWritebackSurface`, `HillslopeKernelRequest`,
`KernelWritebackPayload`, or `SymbolRegistry` across `41` Rust files. The large
carriers remain rooted in:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`, including
  `HillslopeWritebackSurface` and the legacy scheduler machinery.
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`, including
  `SymbolRegistry`-mediated day-frame apply/commit helpers.
- Runtime input modules that still publish parsed input into
  `HillslopeWritebackSurface`.
- Kernel support and coupling helpers that still expose symbol-map request or
  payload entrypoints for compatibility tests and the replay seam.

## Production Direct Setup Sites

The direct runner still starts from symbol-map surfaces:

- `StaticRuntimeSurfaceParts` stores `runtime_surface`, `snow_surface`, and
  `frost_surface` as `HillslopeWritebackSurface`
  (`00_runner_intake_and_lane_setup.rs:882`).
- `HillslopeClimateExecutionState` stores the production setup
  `runtime_surface`, `symbol_registry`, and `hot_symbol_tables`
  (`00_runner_intake_and_lane_setup.rs:895`).
- `direct_production_lane_seed_surfaces` clones lane seed authority from
  `HillslopeWritebackSurface` (`05_runner_execution_and_outputs.rs:361`).
- `direct_production_execution_runtime_surface` produces another
  `HillslopeWritebackSurface` for execution provenance and coupling
  (`05_runner_execution_and_outputs.rs:387`).
- `DirectProductionRunFrameBuildInputs` still passes
  `climate_context_surface: &HillslopeWritebackSurface` and
  `lane_seed_surfaces: &[HillslopeWritebackSurface]`
  (`05_runner_execution_and_outputs.rs:201`).
- `seed_direct_production_lane_constructor_inputs` reads seed state from a
  `day_zero_seed_surface` (`05_runner_execution_and_outputs.rs:415`).

These sites are the correct target for typed setup once the RSS-first retained
publication work is separated.

## Retained Publication And Ledger Sites

The RSS profile points to these retained structures as the current memory driver:

- `DirectRunPublicationFrame` stores all direct publication rows in
  `rows: Vec<DirectPublicationDayRow>` with `Vec::with_capacity(expected_row_count)`
  (`direct_runtime/01_publication.rs:186` and `:197`).
- `DirectPublicationExecution` owns the full `DirectRunPublicationFrame`
  (`direct_runtime/02_state_reports.rs:430`).
- `execute_hillslope_direct_production_days` stores
  `retained_direct_publication: Some(direct_execution)`
  (`05_runner_execution_and_outputs.rs:157` and `:196`).
- `HillslopeClimateExecution` retains `wb13_rows`, `pass_rows`,
  `retained_direct_publication`, and `direct_publication`
  (`00_runner_intake_and_lane_setup.rs:909`).
- `ClimateExecutionAccumulator` preallocates `wb13_rows` for
  `day_count * contributor_ofe_count` and `pass_rows` for `day_count`
  (`05_runner_execution_and_outputs.rs:557`).
- `build_direct_publication_artifacts` clones
  `execution.retained_direct_publication`, then unconditionally builds HBP,
  WAT rows, PASS rows, loss text, and manifest text
  (`04_direct_publication.rs:31`).
- `write_hillslope_direct_publication_outputs` writes from those retained
  artifacts (`05_runner_execution_and_outputs.rs:931`).

The minimized-output H2637 run is the key discriminator: WAT/PASS/plot were not
requested, but `build_direct_publication_artifacts` still constructs WAT and PASS
projection rows before output gating. That explains why output selection did not
move RSS.

## Reordered Implementation Recommendation

Do not start with symbol-map setup deletion as the RSS lever. Start with a
retained-publication package that:

1. Avoids cloning `DirectPublicationExecution` during output assembly.
2. Builds WAT/PASS projection rows only when those outputs are requested.
3. Streams HBP/loss/manifest/WAT/PASS projections from the direct publication
   rows or writes them incrementally, instead of retaining every projection at
   once.
4. Drops publication rows as soon as all required downstream consumers have read
   them, or replaces the full retained frame with compact publication summaries
   plus optional output-specific streaming writers.

After that RSS reduction lands and is identity-gated, resume typed setup and
delete the symbol-map carriers from the production direct path.
