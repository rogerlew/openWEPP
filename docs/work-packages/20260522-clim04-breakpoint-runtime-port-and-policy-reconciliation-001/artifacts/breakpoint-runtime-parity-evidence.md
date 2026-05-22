# Breakpoint Runtime Parity Evidence (CLIM04)

Static:
- Legacy breakpoint authority reviewed: `/workdir/wepp-forest_260430_baseline/src/brkpt.for`.
- Ported equations and symbol continuity reconciled to runtime seam adapters.

Ran:
- `cargo test --test infile_climate_parser_contract`
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::`
- `cargo test -p openwepp-watershed-orchestrator runtime_inputs::tests::`
- `cargo test --workspace`

## Parity Claims and Evidence
1. `stmstr` capture from first breakpoint time is exported on runtime surfaces.
- evidence: passing tests
  - `runtime_inputs::tests::breakpoint_runtime_surface_projects_stmstr_elapsed_timem_and_mxint` (hillslope)
  - `runtime_inputs::tests::breakpoint_runtime_surface_projects_stmstr_elapsed_timem_and_mxint` (watershed)

2. Breakpoint `timem` is normalized to elapsed storm seconds.
- evidence: same tests assert `timem_0001 == 0` and elapsed second values from non-zero `stmstr` fixture.

3. Interval intensities and event summaries match port contract shape.
- evidence: same tests assert `prcp`, `stmdur`, `mxint`, and terminal `intsty_last == 0` behavior.

4. Parser/runtime breakpoint policy closure implemented.
- evidence: passing tests
  - `strict_mode_accepts_breakpoint_cardinality_at_1500_boundary`
  - `strict_mode_rejects_breakpoint_cardinality_over_1500`
  - `strict_mode_rejects_duplicate_breakpoint_times`
  - runtime seam duplicate-time rejection tests in both orchestrators.

## Confidence Notes
- Confidence tier for this package is unit/integration parity confidence against static legacy contract mapping.
- No unresolved high-severity breakpoint parity gap remains in the CLIM04 scoped write set.
