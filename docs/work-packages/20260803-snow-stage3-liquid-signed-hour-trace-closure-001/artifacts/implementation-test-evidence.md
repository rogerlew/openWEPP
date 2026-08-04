# Implementation And Test Evidence

Status: `PASS`

Evidence mode: `Static + Ran`

## Behavior-Neutral Publication

- `DirectSnowAccumulationMeltDiagnostics` now carries exact daily wind,
  dewpoint, and canopy; hourly forcing; authoritative hourly routed melt;
  hourly CoE capacity/store/release/sublimation; and exact hourly pack
  depth/density snapshots.
- `SnowHourlyState` snapshots geometry before and after the existing hourly
  transition. `SnowHourlyTrace` projects those values without changing the
  transition, redistribution, storage, or daily-closure arithmetic.
- The direct trace formatter advances from v3 to v4, names retained liquid as a
  delta, publishes all five Stage-3 liquid values, and publishes existing
  duration-weighted active/lower thermal arrays plus lower present fraction.
- No equation, constant, guard, branch, selector, default, state mutation,
  fixture, observation, or protected WAT/HBP/PASS formatter changed.

## Direct Focused Evidence

- `cargo nextest run --no-fail-fast --test snow_surface_eb03_contract --test snow_surface_eb04v_density_process_diagnostics_contract --test snow_surface_eb04w_accumulation_melt_diagnostics_contract --test paradigm2_stage3_liquid_routing_meltwater_temperature --test paradigm2_stage3_decouple_water_temperature --test hphys0296_snow_rm_acceptance_authority_contract` — `34/34` passed.
- Post-refactor Stage-3 subset — `18/18` passed.
- `cargo test -p openwepp-runner formatter_preserves -- --nocapture` — both
  deliberately distinct formatter tests passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed after
  extracting the diagnostic-only hourly carrier/formatter helpers.
- `cargo fmt --all -- --check` and `git diff --check` — passed.
- `cargo test --workspace --doc` — passed for all workspace crates.
- `markdown-doc lint` and `markdown-doc validate` — passed for the package,
  canonical contract, index, catalog, and roadmap.
- In-memory Python source compilation of `tools/trace_closure.py` — passed.

## Generated Assurance Identity

The first quick profile failed closed because the intentional v123 contract
bytes no longer matched the generated assurance identity. The exact release
`openwepp-assurance` binary was built, then the bounded transaction was checked
and applied:

```text
target/release/openwepp-assurance amend adopt-report-source \
  --report snow-and-frozen-soil-process-evaluation \
  --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md \
  --check
target/release/openwepp-assurance amend adopt-report-source \
  --report snow-and-frozen-soil-process-evaluation \
  --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md \
  --apply
```

Receipt `ac9ae76f...` records `scientific-full`, generation
`9e64c4c7... -> 12bddac7...`, and no invalidated authority. The report remains
`DRAFT` with no active events or approval/publication authority.
`openwepp-assurance validate --all` passed after adoption.

The contract target's pre-implementation result was `7 passed / 2 expected
failed`; after implementation it is `9/9` passed.
