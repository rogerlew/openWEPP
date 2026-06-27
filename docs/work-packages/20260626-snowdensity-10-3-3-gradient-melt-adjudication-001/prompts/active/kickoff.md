# Kickoff: SNOWDENSITY-10.3.3 Gradient Melt Adjudication

Execution mode: package-end-to-end.

Autonomy: execute the package from scaffold through disposition without further
operator intervention unless a hard blocker prevents evidence generation.

## Required Reading

### Core

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3
- `docs/work-packages/20260626-snowdensity-10-3-1a-per-day-cancov-direct-runtime-001/package.md`
- `docs/work-packages/20260626-snowdensity-10-3-2-canopy-stratum-correspondence-001/package.md`
- `tests/fixtures/cancov_forest/README.md`

### Conditional

- `docs/specifications/science-contracts/AGENTS.md` if editing a science
  contract or production kernel/routing path.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` if a
  contract marker or invariant is cited beyond existing package authority.

### On-Demand

- `tools/snowfreeze_observed/coe_melt_adjudication.py`
- `tools/snowfreeze_observed/snotel_density_three_way.py`
- `tests/fixtures/cancov_forest/observations/README.md`
- `tests/fixtures/cancov_forest/observations/manifest.json`

## Execution

1. Confirm the working tree state.
2. Implement a package-local canopy-gradient CoE melt adjudication tool.
3. Keep exact stratum bindings verdict-bearing and mixed aggregate comparisons
   diagnostic-only unless a package-local aggregate rule is explicitly recorded.
4. Run `legacy_coe` and `coe_shortwave_albedo_v1` with
   `openwepp-snowbench coe-melt`.
5. Produce JSON and Markdown reports with regime summaries for conifer, mixed,
   deciduous, and open/pasture regimes.
6. Record validation gates and close the package with dual review and
   verification artifacts.

## Required-Reading Budget

Estimated local pre-read is below the `REQUIRES-JUSTIFICATION` threshold because
large source/reference files are on-demand and the core set is limited to package
authority, the fixture map, and prior closure packages.
