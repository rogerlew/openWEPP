# Implementation Log

Evidence mode: Static/Ran.

## Package Scaffold

Static:

- Created package
  `docs/work-packages/20260630-typed-seed-authority-carrier-rearchitecture-001/`.
- Added `package.md`, active kickoff prompt, and artifacts for pipeline map,
  gates, verification, review, and disposition.
- Updated `docs/work-packages/README.md` with the package pointer.

## Required Reading

Static:

- Read the three held packages:
  `20260630-typed-direct-setup-symbol-map-elimination-001/`,
  `20260630-typed-direct-setup-symbol-map-carrier-deletion-001/`, and
  `20260630-stage1-seed-authority-migration-001/`.
- Read the seed pipeline and consumer code:
  `00_wb11_runtime_seed.rs`, `05_runner_execution_and_outputs.rs`,
  `00_runner_intake_and_lane_setup.rs`, and direct publication day-input
  helpers.

## Static Execution

Static:

- Confirmed production direct still seeds from a day-zero
  `HillslopeWritebackSurface`.
- Confirmed the day-zero seed authority is computed by merging day-one climate
  and mutating the surface with `seed_wb11_runtime_surface_inputs`.
- Confirmed the next remaining surfaces are not independent reads:
  constructor seed state, day-input authority, coupling metadata, Wave-2 flag,
  and winter hourly geometry all consume the computed day-zero authority.
- Confirmed no complete typed parse-derived projection API exists for the
  ordered seed pipeline. Existing typed structs are downstream consumers; the
  authoritative setup projection still emits and mutates symbol-map surfaces.

Ran:

```text
rg -n "require_runtime_surface_scalar|runtime_surface_symbol_value|direct_publication_optional|direct_publication_required|DirectProduction.*from_seed|from_seed\(|direct_publication_layer_states|direct_publication_profile_inputs|direct_publication_percolation_inputs|direct_publication_subsurface_inputs" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs | wc -l
```

Result: `206` for the narrower scalar/helper pattern after Stage 1B. The held
Stage 1 package's broader seed-read metric remains `207`.

## Phase 1 Decision

Result: HOLD.

The package does not implement a transient surface-backed "typed" carrier. That
would preserve the symbol-map as the seed authority and would not satisfy the
single-authority requirement. Phase 1 is blocked on factoring typed projection
APIs for the ordered seed pipeline.
