# Implementation Test Evidence

Evidence class: Static + Ran

Status: complete.

Production edit:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- Function: `legacy_normalize_conductivity_layers_to_200mm`

Implementation:

- Applied the baseline top-200 mm source-layer `ksat` rule for normalized layers
  whose bottom depth is `<= 0.2 m`.
- Replaced arithmetic vertical `weighted_ksat_mm_h` below the top interval with
  `inverse_weighted_ksat_h_per_mm += consumed_thickness / ksat_mm_h`.
- Final vertical conductivity is
  `WB13_PROFILE_LAYER_THICKNESS_M / inverse_weighted_ksat_h_per_mm`.
- Preserved arithmetic horizontal weighting by accumulating the active source
  `ksat * anisotropy` contribution by thickness fraction. For the top-interval
  baseline source-layer `ksat` rule, the source layer's anisotropy ratio remains
  the horizontal multiplier, so `ui_ssh` stays a horizontal surface rather than
  becoming a vertical harmonic alias.
- Changed the helper to return
  `Result<Vec<NormalizedConductivityRuntimeSymbols>, HillslopeRuntimeInputError>`
  so an impossible no-consumed-conductivity normalized layer fails through the
  existing typed runtime normalization error instead of publishing `0.0`.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator \
  runtime_inputs::tests::soil_runtime_surface_contains_canonical_state_symbols \
  -- --nocapture
```

Result:

- 1 passed, 0 failed.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests -- --nocapture
```

Result:

- 79 passed, 0 failed.

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator \
  runtime_inputs::tests::soil_runtime_surface_rejects_non \
  -- --nocapture
```

Result:

- 2 passed, 0 failed.

Purpose:

- Confirmed projection-path guard coverage for non-finite and non-positive
  source `ksat`.

Ran:

```text
cargo test -p openwepp --test parser_runtime_seam_integration -- --nocapture
```

Result:

- 49 passed, 0 failed.

Purpose:

- Refreshed parser/runtime seam expected values after contract amendment.
- Confirmed the projected canonical `ssc` surface now follows the top source
  layer `ksat` rule (`15.0 / 3.6e6`) while lower indexed `ssc_0002` remains
  `8.0 / 3.6e6`.

Ran:

```text
cargo test --workspace
```

Result:

- Passed.
- Includes integration test
  `tests/integration/auth11_required_suite_obligation_guards_contract.rs`.
