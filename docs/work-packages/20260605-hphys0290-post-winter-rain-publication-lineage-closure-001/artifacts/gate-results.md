# Gate Results

Status: complete
Evidence mode: Ran

## Final Required Gates

Ran:

- Log root: `/tmp/hphys0290_final_gates_20260605T013019Z_after_nan`
- Status table: `/tmp/hphys0290_final_gates_20260605T013019Z_after_nan/status.tsv`

| Gate | Return Code | Seconds | Result |
| --- | ---: | ---: | --- |
| `cargo fmt --check` | 0 | 1 | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | 3 | pass |
| `cargo test --workspace` | 0 | 342 | pass |
| `cargo deny check` | 0 | 1 | pass |
| `bash tools/release/check_authority_suite_antievasion.sh` | 0 | 1 | pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | 0 | 0 | pass |

## Focused Gates

Ran:

- `cargo test -p openwepp-runner hphys0290_wb13_rm_publication -- --nocapture` -> pass (`6 passed`).
- `cargo test --test hphys0290_post_winter_rain_publication_contract -- --nocapture` -> pass (`3 passed`).
- `cargo test --test sim_contract_boundary_unit_registry hphys0290_registry_declares_post_winter_rain_flux_metadata -- --nocapture` -> pass (`1 passed`).
- `cargo test --test sim_contract_boundary_unit_registry canonical_registry_resolves_climate_soil_and_snow_runtime_aliases -- --nocapture` -> pass (`1 passed`).
- `cargo test --test sim_contract_boundary_unit_registry hphys0275_registry_marks_only_migrated_aliases_typed_required -- --nocapture` -> pass (`1 passed`).

## Full H1..H39 Suite

Ran:

- Root: `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix`
- Runtime: `39/39` hillslopes completed.
- Semantic: `0/39` hillslopes passed.
- Summary: `artifacts/full-39-suite-metrics.md`.

Disposition: all required gates passed; semantic parity remains open and keeps
package disposition at `executed-hold`.
