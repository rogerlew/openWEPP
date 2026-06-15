# CRAP After

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/crap_after.json`

Exit code: `0`.

Raw artifact: `crap_after.json`.

Top target-module rows:

| Function | Line | CC | Coverage % | CRAP |
|---|---:|---:|---:|---:|
| `build_initial_seed_projection` | 259 | 17.0 | 91.66666666666666 | 17.16724537037037 |
| `project_yearly_crop_slot_surfaces` | 635 | 15.0 | 95.38461538461539 | 15.02212107419208 |
| `project_perennial_crop_slot` | 992 | 15.0 | 100.0 | 15.0 |
| `project_annual_or_fallow_crop_slot` | 907 | 13.0 | 93.75 | 13.041259765625 |
| `read_primary_initial_live_canopy_inputs` | 1256 | 13.0 | 100.0 | 13.0 |
| `project_initial_seed_surfaces` | 195 | 10.0 | 95.0 | 10.0125 |
| `insert_yearly_schedule_symbols` | 729 | 9.0 | 100.0 | 9.0 |
| `project_primary_drain_controls` | 833 | 9.0 | 100.0 | 9.0 |
| `legacy_initial_residue_depth_m` | 1161 | 7.0 | 89.65517241379311 | 7.054245766534093 |
| `normalized_primary_initial_cancov` | 1340 | 7.0 | 92.3076923076923 | 7.022303140646336 |
| `project_primary_annual_crop_aliases` | 963 | 7.0 | 100.0 | 7.0 |
| `projection_usize_from_surface` | 1509 | 4.0 | 44.44444444444444 | 6.743484224965707 |

Disposition: package CRAP target passed. Maximum target-module CRAP after the
refactor is `17.16724537037037`.
