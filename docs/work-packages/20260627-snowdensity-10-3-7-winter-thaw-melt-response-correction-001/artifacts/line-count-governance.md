# Line-Count Governance

Evidence mode: Ran.

Command for touched Rust files:

```bash
(git diff --name-only; git ls-files --others --exclude-standard) | \
  rg '\.rs$' | sort | xargs wc -l
```

Command for touched Python/contract support files:

```bash
wc -l tools/snowfreeze_observed/winter_thaw_melt_response.py \
  tools/snowfreeze_observed/winter_thaw_melt_response_correction.py \
  tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py \
  tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py \
  docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
```

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs` | 715 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs` | 287 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs` | 1565 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs` | 965 | PASS |
| `crates/openwepp-runner/src/bin/openwepp-snowbench.rs` | 259 | PASS |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 2590 | WARN: below hard threshold after mechanical split |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs` | 4 | PASS |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2555 | WARN: below hard threshold after mechanical split |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs` | 583 | PASS |
| `crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs` | 719 | PASS |
| `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs` | 872 | PASS |
| `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs` | 445 | PASS |
| `tests/integration/snowdensity02_contract_adr_guard.rs` | 88 | PASS |
| `tests/integration/snowdensity05a_melt_contract_guard.rs` | 85 | PASS |
| `tests/integration/snowdensity05b_shortwave_source_contract.rs` | 106 | PASS |
| `tests/integration/snowdensity05c_albedo_state_core.rs` | 164 | PASS |
| `tests/integration/snowdensity05d_opt_in_coe_melt.rs` | 212 | PASS |
| `tests/integration/snowdensity05e_melt_adjudication.rs` | 86 | PASS |
| `tests/integration/snowdensity05f_melt_closure_handoff.rs` | 119 | PASS |
| `tests/integration/snowdensity05g_harness_fidelity_rerun.rs` | 128 | PASS |
| `tests/integration/snowdensity06_density_compaction.rs` | 124 | PASS |
| `tests/integration/snowdensity06b_coe_bound_density_replay.rs` | 177 | PASS |
| `tests/integration/snowdensity07_runtime_opt_in.rs` | 251 | PASS |
| `tests/integration/snowdensity08_gate_rerun.rs` | 116 | PASS |
| `tests/integration/snowdensity09_coupled_wat_rerun.rs` | 151 | PASS |
| `tests/integration/snowdensity10_3_1a_per_day_cancov.rs` | 140 | PASS |
| `tests/integration/snowdensity10_3_5a_meteorology_crate_contract.rs` | 78 | PASS |
| `tests/integration/snowdensity10_3_5b_hourly_partition_jennings_contract.rs` | 115 | PASS |
| `tests/integration/snowdensity10_3_7_winter_thaw_melt_response_correction.rs` | 291 | PASS |
| `tools/snowfreeze_observed/winter_thaw_melt_response.py` | 699 | PASS |
| `tools/snowfreeze_observed/winter_thaw_melt_response_correction.py` | 425 | PASS |
| `tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py` | 446 | PASS |
| `tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py` | 762 | PASS |
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | 2147 | Doc contract, not a Rust line-count trigger |

Disposition: PASS. The only touched Rust files above the 2000-line WARN
threshold are `00_builders_and_authority.rs` and `03_tests.rs`; this package
mechanically moved the contiguous `impl DirectProductionSnowFrostAuthority` tail
into `00a_snow_frost_authority_impl.rs`, and moved the direct-publication
source-guard tests into `tests03/direct_publication_source_guards.rs`. No touched
non-exempt Rust file remains at or above 3000 lines.
