# Verification

Evidence mode: Static + Ran.

## CRAP After

Ran:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path lcov.info && cargo crap --workspace --lcov lcov.info --min 0 --format json --output /tmp/openwepp-crap-row7-after-final.json
```

Result:

- Command exited 0.
- `cargo llvm-cov` wrote `lcov.info`.
- `cargo llvm-cov` warned that 132 source files had no matching LCOV entry; this
  warning is unchanged in kind from the baseline coverage workflow and includes
  many test-source paths.
- Row #7 owned production offender count above CRAP 30: `0`.

Original row #7 offenders after the final run:

```text
file	function	cc	coverage	crap
00a_snow_frost_authority_impl.rs	layered_snow_frost_insulation_depth_density	18.0	82.8125	19.645065307617188
01_publication.rs	DirectFrostLayerCarryProjection::validate_for_layer	13.0	100.0	13.0
00a_snow_frost_authority_impl.rs	invert_sturm1997_snow_density_kg_m3	10.0	89.65517241379311	10.110705645987945
00_builders_and_authority.rs	direct_production_sturm1995_climate_normals	9.0	80.95238095238095	9.559766763848396
01_wb12_wb16_wb19_projection.rs	project_typed_wb11_frozen_depth_refresh	14.0	98.63013698630137	14.000503834022677
04_direct_publication.rs	validate_retained_direct_publication_frame	8.0	100.0	8.0
00a_snow_frost_authority_impl.rs	sturm1997_snow_conductivity_w_m_k	8.0	88.0	8.110592
00_builders_and_authority.rs	parse_snowdensity1015_default_snow_density_model	7.0	73.91304347826086	7.869893975507521
00_builders_and_authority.rs	snowdensity1037_diagnostic_snow_melt_model	4.0	0.0	20.0
01_frost_and_layer_helpers.rs	rebalance_direct_production_no_final_frost_layers_to_storage	18.0	76.47058823529412	22.220639120700184
00_builders_and_authority.rs	parse_snowdensity1015_default_snow_melt_model	6.0	90.47619047619048	6.031098153547133
00_builders_and_authority.rs	maybe_write_r7h_direct_production_wb15_trace	4.0	16.176470588235293	13.423621005495624
00_builders_and_authority.rs	DirectProductionPriestleyTaylorAuthority::compute_demand	6.0	92.3076923076923	6.0163859808830225
00_builders_and_authority.rs	DirectProductionDayInputBuilder::build	29.0	92.2279792746114	29.3948188197701
00_builders_and_authority.rs	direct_production_surface_litter_projection	11.0	64.91228070175438	16.226979421468414
01_wb12_wb16_wb19_projection.rs	wb16_equivalent_plane_alpha	7.0	82.85714285714286	7.246857142857142
00_builders_and_authority.rs	direct_production_typed_growth_crop_authority	25.0	80.37383177570094	29.724834148678866
```

Extraction:

```text
jq -r '[.entries[] | select(.file | test("(direct_runtime/01_publication|direct_publication|direct_seed_projections)")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row7-after-final.json
```

Output:

```text
0
```

## Focused Tests

Ran:

```text
cargo nextest run -p openwepp-runner -p openwepp-hillslope-orchestrator
```

Result:

- `207` tests run: `207` passed, `0` skipped.

Ran marker-sensitive snow-density contracts after keeping selector parser helper
markers in `00_builders_and_authority.rs`:

```text
cargo nextest run -p openwepp --no-fail-fast --test paradigm2_stage1_layered_snow_density --test snowdensity03_physics_bulk_offline_contract --test snowdensity05f_melt_closure_handoff --test snowdensity07_runtime_opt_in --test snowdensity08_gate_rerun --test snowdensity09_coupled_wat_rerun --test snowdensity10_3_11_spring_compaction_densification --test snowdensity10_3_15_default_activation_active_cap --test snowdensity10_3_16_open_surface_ablation_stage_a --test snowdensity10_3_17_shallow_pack_compaction_guard --test snowdensity10_3_20_sublimation_stage_b_unlock --test snowdensity10_3_22_climate_class_density_specialization --test snowdensity10_3_7_winter_thaw_melt_response_correction --test snowdensity10_3_8_liquid_holding_capacity
```

Result:

- `58` tests run: `58` passed, `0` skipped.

## Full Gates

Ran:

```text
cargo fmt --check
git diff --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
markdown-doc lint --path docs/work-packages/20260701-kernel-boundary-cqr-row7-wb-publication-001 --path docs/work-packages/README.md
markdown-doc validate --path docs/work-packages/20260701-kernel-boundary-cqr-row7-wb-publication-001 --path docs/work-packages/README.md
```

Results:

- `cargo fmt --check`: exited 0.
- `git diff --check`: exited 0.
- `cargo clippy --workspace --all-targets -- -D warnings`: exited 0.
- `cargo nextest run --workspace --profile full`: `1239` tests run, `1239`
  passed, `1` skipped; slow tests were
  `snowdensity05e_melt_adjudication::coe_melt_snowbench_runs_both_models_as_diagnostic_only`
  and
  `snowfrost_fidelity_g0_pysnobal_bridge_contract::g0_exporter_emits_pysnobal_schema_and_required_anti_alias_lineage`.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
- Authority anti-evasion script: `PASS: authority suite anti-evasion checks passed.`
- Auth11 obligation guard: `2` tests run, `2` passed.
- `markdown-doc lint`: 9 files, 0 errors, 0 warnings.
- `markdown-doc validate`: 9 files, 0 errors.

## H2637 Endpoint

Ran:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
/usr/bin/time -v target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/stage1-seed-authority/after-1b/h2637.run \
  --output-dir /tmp/kernel-boundary-cqr-row7-h2637/output \
  --manifest-path /tmp/kernel-boundary-cqr-row7-h2637/output/manifest.json
```

Results:

- Release build exited 0.
- H2637 exited 0; wall `1:07.39`; max RSS `79588 KiB`.
- Manifest selected `direct-production-executor`.
- `compatibility_edge_invocations`: `0`.
- `execution_provenance.scheduler_kernel_executed`: `false`.
- `wb13_publication.row_count`: `235961`.
- CLI output files were emitted through the runfile output path
  `/tmp/stage1-seed-authority/after-1b/output`; the explicit row #7 output
  directory contains the manifest.

Protected output comparison against
`/tmp/typed-direct-carrier-identity/base/output`:

```text
PASS H2637.hbp
PASS H2637.loss.json
PASS H2637.plot.parquet
PASS H2637.wat.parquet
PASS H2637.pass.parquet
```

Output hashes:

```text
18c7ddcd8b5b4205876e47e82eaa3931d56db0b98d37f96d5dcebb50b7f85c2e  H2637.hbp
73d588ee03c1316a75743dc6f33225282e8ac82e6647018b395ea66e0d03dcd6  H2637.loss.json
cb1259dda3b5113e58e6fe94ddc10ea8968589ea356a12fe3a358852cce3d223  H2637.plot.parquet
26d4b9415820e6da2e16869f2f926a8b5ddd39c565dfff612a0551477b7e09f6  H2637.wat.parquet
f4de3e5c2224556e6c913d6ca12d807415da56a07b182d4e3238fec1879a6e22  H2637.pass.parquet
```

## Line Counts

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs crates/openwepp-runner/src/hillslope/direct_seed_projections/01_wb12_wb16_wb19_projection.rs
```

Result:

```text
302 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
1235 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
1716 crates/openwepp-runner/src/hillslope/03_tests.rs
4137 crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
726 crates/openwepp-runner/src/hillslope/direct_seed_projections/01_wb12_wb16_wb19_projection.rs
8116 total
```

Disposition: see `line-count-governance.md`.
