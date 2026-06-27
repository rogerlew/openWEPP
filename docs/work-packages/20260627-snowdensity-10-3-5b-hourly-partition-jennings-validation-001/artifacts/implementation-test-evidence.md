# Implementation Test Evidence

Status: complete
Evidence mode: Ran

## Focused Gates

Ran before production implementation:

```text
cargo test --test snowdensity10_3_5b_hourly_partition_jennings_contract --test snowdensity10_3_5a_meteorology_crate_contract
```

Result: pass, `5` tests passed.

Ran after implementation:

```text
cargo test -p openwepp --test hphys0299_hourly_snow_partition_unit_provenance_contract hphys0299_static_openwepp_sources_publish_depth_and_water_equiv_separately
```

Result: pass, `1` test passed. This rerun verifies the source-provenance guard
after the legacy snowfall-depth helper fix.

## Jennings Validation

Ran:

```text
cargo run --release -p openwepp-runner --bin openwepp-snowbench -- jennings-phase --observations tests/fixtures/precip_phase_observed/jennings2018/jennings_et_al_2018_file2_ppt_phase_met_observations.csv --thresholds tests/fixtures/precip_phase_observed/jennings2018/jennings_et_al_2018_file3_temp50_observed_by_station.csv --output-dir docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/artifacts
```

Result: pass. The release run scored `11,711,058` Jennings rows across `6,883`
stations. Harder-Pomeroy hourly accuracy was `0.903141`; legacy `RST` 0 C
accuracy was `0.858331`.

Generated:

- `artifacts/jennings-validation-report.json`
- `artifacts/jennings-validation-report.md`

Note: an earlier debug build run of the same command was intentionally stopped
for runtime; it is not counted as validation evidence.

## Full Gates

Ran:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
bash tools/release/check_authority_suite_antievasion.sh
cargo test --test auth11_required_suite_obligation_guards_contract
wctl doc-lint --path docs/work-packages
```

Result: all pass.
