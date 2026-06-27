# Kickoff: SNOWDENSITY-10.3.4 Maritime Over-Accumulation Diagnosis

Execution mode: package-end-to-end.

Autonomy: execute the package from scaffold through disposition without further
operator intervention unless a hard blocker prevents evidence generation.

## Required Reading

### Core

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3
- `docs/work-packages/20260626-snowdensity-10-3-3-gradient-melt-adjudication-001/package.md`
- `tests/fixtures/cancov_forest/README.md`
- `tests/fixtures/snowfreeze_observed/README.md`

### Conditional

- `docs/specifications/science-contracts/AGENTS.md` if editing a science
  contract or production kernel/routing path.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` if a
  contract marker or invariant is cited beyond existing package authority.

### On-Demand

- `tools/snowfreeze_observed/cancov_gradient_melt_adjudication.py`
- `tools/snowfreeze_observed/non_snotel_rubric_baseline.py`
- `tools/snowfreeze_observed/snotel_density_three_way.py`
- `tests/fixtures/cancov_forest/observations/README.md`
- `tests/fixtures/snowfreeze_observed/observations/manifest.json`

## Execution

1. Confirm the working tree state.
2. Implement a diagnostic maritime over-accumulation tool.
3. Run legacy-CoE snowbench surfaces for HJ Andrews, Sleepers, Harvard, and
   Hubbard Brook fixtures.
4. Pair Harvard and Sleepers against installed observations; mark HJ Andrews and
   Hubbard Brook observation-blocked unless paired snow tables are installed.
5. Rank candidate mechanisms with explicit `DEFECT-ELIGIBLE`,
   `FORCING-LIMITED`, `OBSERVATION-BLOCKED`, `LOW-PRIORITY`, or
   `NOT-SUPPORTED` dispositions.
6. Record validation gates and close the package with dual review and
   verification artifacts.

## Required-Reading Budget

Estimated local pre-read is below the `REQUIRES-JUSTIFICATION` threshold because
large source/reference files are on-demand and the core set is limited to
package authority, fixture maps, and prior closure packages.
