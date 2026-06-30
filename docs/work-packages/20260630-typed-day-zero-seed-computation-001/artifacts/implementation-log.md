# Implementation Log

Evidence mode: Static/Ran.

## Scaffold

Static:

- Created `20260630-typed-day-zero-seed-computation-001/`.
- Added package, active prompt, and artifacts.
- Updated `docs/work-packages/README.md`.

## Phase 1 Partial Implementation

Static:

- Factored WB18/WB19 lane substep calculation into
  `TypedWb11LaneSubstepProjection` and
  `project_typed_wb11_lane_substeps`.
- Factored rainfall/hyetograph normalization into typed interval/projection
  structs and `project_typed_wb11_hyetograph`.
- Factored WB11 initial storage math into typed layer input/projection/total
  structs and `project_typed_wb11_initial_storage`.
- Factored fine-frost frozen-depth refresh into typed layer input/projection
  structs and `project_typed_wb11_frozen_depth_refresh`.
- Factored residue interception and `Ws` defaults into
  `project_typed_wb11_optional_defaults`.
- Factored WB12 reconciliation seed defaults into
  `TypedWb12ReconciliationSeedProjection` and
  `project_typed_wb12_reconciliation_seed`.
- Factored ET-demand seed into typed Priestley-Taylor and EVAPPM/PMET input
  structs and projections. The surface reader now builds typed ET inputs before
  calling the projections, and the existing publisher remains the writer
  adapter.
- Factored `efflen` and default `m` into
  `project_typed_wb11_efflen_and_m`.
- Factored the WB16 `ealpha` compatibility-default decision into
  `project_typed_wb16_ealpha_compatibility`.
- Kept existing `HillslopeWritebackSurface` writers as adapters. No consumers
  were cut over to a typed carrier.

Ran:

```text
cargo fmt --check
cargo check -p openwepp-runner
cargo nextest run -p openwepp-runner publication_wb11_seed
cargo clippy -p openwepp-runner --all-targets -- -D warnings
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-day-zero-seed-computation-001 --format json
markdown-doc validate --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-day-zero-seed-computation-001
git diff --check
```

Results:

- `cargo fmt --check`: pass.
- `cargo check -p openwepp-runner`: pass.
- `publication_wb11_seed` slice: `29` run, `29` passed.
- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`: pass.
- `markdown-doc lint`: `10` files scanned, `0` errors, `0` warnings.
- `markdown-doc validate`: `10` files, `0` errors.
- `git diff --check`: pass.

## Hold Boundary

Static:

- The full day-zero carrier still cannot be assembled without the remaining
  typed sub-computations and typed static parsed-input projection.
- H2637 + multi-OFE + Wave-2 fixture-level seed identity was not run because
  the full typed carrier still has unsourced consumer reads.
- Phase 2 cutover, Phase 3 deletion, Phase 4 no-compatibility proof, and full
  workspace gates remain blocked by incomplete Phase 1 coverage.

## Autonomous Completion Resume

Static:

```text
rg -n "require_runtime_surface_scalar|runtime_surface_symbol_value|direct_publication_optional|direct_publication_required|DirectProduction.*from_seed|from_seed\\(|direct_publication_layer_states|direct_publication_profile_inputs|direct_publication_percolation_inputs|direct_publication_subsurface_inputs" crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs | wc -l
```

Result: `207`.

```text
rg -n "build_hillslope_runtime_surface_from_|HillslopeWritebackSurface::default\\(|merge_runtime_surfaces|BoundarySymbol::from" crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/intake_lane_setup crates/openwepp-hillslope-orchestrator/src/runtime_inputs | wc -l
```

Result: `499`.

This first broad helper count included retained transition/test-only seed
surface adapters, not just production consumer reads. The clarified Gate 1
consumer groups were then cut over to the typed carrier:

- production direct execution calls `DirectProductionSeedAuthority::from_typed_inputs`;
- snowbench diagnostic replay calls the same typed authority;
- lane constructor, day-input builder, coupling metadata, Wave-2 flag, and
  winter hourly geometry read typed carrier values.

Ran:

```text
cargo fmt --check
cargo check -p openwepp-runner
cargo nextest run -p openwepp-runner publication_wb11_seed publication_wb19_wb12_wb16
cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection -- --nocapture
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
bash tools/release/check_authority_suite_antievasion.sh
cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-day-zero-seed-computation-001 --format json
markdown-doc validate --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-day-zero-seed-computation-001 --format json
git diff --check
```

Results:

- H2637 HBP/loss/PASS/WAT/plot byte-identical against clean `5b139058`.
- cli01 HBP/loss/WAT/plot byte-identical.
- H2637 current run: `1:08.62`, `91692 KiB`,
  `compatibility_edge_invocations=0`.
- H2637 clean baseline: `1:09.02`, `113268 KiB`.
- Focused seed tests: `41` passed.
- Focused multi-OFE/Wave-2 test: passed.
- Full nextest: `1879` passed, `1` skipped, `2` slow, `671.206s`.
- Remaining release gates: passed.

Gate result: `GATE1-PASSED-PHASE3-PENDING`.
