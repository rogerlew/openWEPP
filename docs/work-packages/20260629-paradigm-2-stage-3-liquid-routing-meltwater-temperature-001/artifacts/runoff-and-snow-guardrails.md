# Runoff and Snow Guardrails

Evidence class: Static + Ran.

Default/rollback:

- Default selector is disabled when `OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL` is
  absent or empty.
- Unsupported non-empty selector values fail closed through the direct
  production authority parser.
- No runfile, `.run`, public CLI, parser, fixture, cap, frost, or public output
  schema selector was added.
- Runner binary sources are scanned by
  `stage3_internal_selector_not_user_cli_exposed`; the test passed.

Runoff mass boundary:

- Stage 3 does not replace CoE melt or route an energy-balance melt mass.
- The focused opt-in test asserts
  `partition.routed_melt_m == diagnostics.incoming_liquid_m` within tolerance:
  diagnostic routing starts from the existing authoritative routed liquid.
- Refreeze/retention are diagnostic Stage 3 partitions; public downstream liquid
  forcing remains the existing CoE mass path in this package.

Snow-density guardrail:

- Stage 3 requires the Stage 1 multilayer density model only to consume a real
  layer stack; it does not change density compaction parameters or the density
  cap.
- The deferred observed-data guardrail was run with
  `tools/snowfreeze_observed/paradigm2_stage3_liquid_routing_meltwater_temperature.py`
  across the cross-SNOTEL + cancov corpus.
- Current no-env default remains the better full-arm snow reference:
  `15` robust fails / `179` ordinal score.
- The Stage 1 rollback and Stage 3 candidate both scored `16` robust fails /
  `177` ordinal score. Stage 3 introduced `0` worse robust cells versus the
  Stage 1 rollback baseline, so the liquid/temperature increment is snow-neutral.
- Because the full opt-in arm depends on Stage 1 layered density, it still
  inherits Stage 1's non-promoted snow profile. No default or promotion claim is
  authorized from this package.
- The adjacent Stage 1 and Stage 2 regression tests passed:

```bash
cargo test \
  --test paradigm2_stage0_surface_energy_balance_contract \
  --test paradigm2_stage1_layered_snow_density \
  --test paradigm2_stage2_snow_frost_insulation_profile \
  --test paradigm2_stage3_liquid_routing_meltwater_temperature \
  --test snowdensity05d_opt_in_coe_melt \
  --test snowdensity07_runtime_opt_in \
  --test snowdensity10_3_7_winter_thaw_melt_response_correction \
  --test snowdensity10_3_8_liquid_holding_capacity \
  --test snowdensity10_3_11_spring_compaction_densification \
  --test snowdensity10_3_16_open_surface_ablation_stage_a \
  --test snowdensity10_3_20_sublimation_stage_b_unlock
```

Result: passed.

Stage 0 guard update:

- The Stage 0 no-production-wiring test was revised to allow only the Stage 3
  opt-in boundary authorized by `INV-SNOWFREEZE-080`.
- Runtime references to Stage 0 surface-energy primitives outside the Stage 3
  opt-in files remain forbidden by that test.

Runoff/timing observed guardrail:

- The deferred observed-gate wrapper scored the timing/runoff cells available in
  the daily corpus: `40` cells were compared, with `0` better, `40` equal, and
  `0` worse for Stage 3 versus Stage 1 rollback.
- `event_rain_on_snow_response` remains unavailable in the daily observed
  corpus, so no event-scale rain-on-snow timing verdict is claimed.
- The real run elapsed `167.666 s`; all direct-runtime model arms completed
  after the stale-cold-content cap fix.
