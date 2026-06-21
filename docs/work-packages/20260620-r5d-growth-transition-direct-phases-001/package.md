# 20260620-r5d-growth-transition-direct-phases-001

## Objective
Implement the R5D array-native direct runtime growth-transition slice for annual and perennial plant growth phases without changing public outputs or default activation.

## Scope
- Add typed direct runtime inputs, direct compute, state mutation, downstream operands, and shadow projection for `AnnualGrowthTransition`.
- Add typed direct runtime inputs, direct compute, state mutation, downstream operands, and shadow projection for `PerennialGrowthTransition`.
- Make R4N evapotranspiration/root-uptake consume direct growth context when required by the direct executor and fail closed when that context is absent.
- Add focused tests for active crop slot resolution, annual/perennial/fallow/pre-plant/reset/cut/grazing paths, alias-sensitive plant state, nonfinite/domain failures, and phase-span identity.
- Preserve default-disabled output identity and H2637 default-disabled runtime threshold.

## Out Of Scope
- Public WB13 ET/WAT plant metadata/PASS/loss/manifest cutover.
- Default activation of array-native direct runtime.
- Replacement of legacy publication authority.
- Broad crop-parameter import or scheduler/API changes beyond direct-runtime shadow state.

## Authority
- `docs/work-packages/r5-burndown-execplan.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDROLOGY-001.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

## Required Evidence
- Static review proving no public scheduler/API output cutover.
- Focused R5D tests.
- Direct runtime aggregate tests.
- Runner no-compatibility and default-disabled tests affected by phase count.
- `cargo fmt --check`.
- `git diff --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo test --workspace`.
- `cargo deny check`.
- Markdown lint for touched package/docs.
- H2637 default-disabled runtime median `<= 676.67 s`.
- Protected-output identity/equivalence review against default-disabled baseline.

## Status
Complete. Pushed commit: `2fbd3802`.

Final verdict: `COMPLETE-R5D-GROWTH-TRANSITION-DIRECT-PHASES`.
