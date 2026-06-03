# Contract-Test Implementation Evidence

Status: completed

Evidence mode: static + ran

Static:

- Added contract-derived test
  `hphys0262_trace_row_captures_pmet_demand_seeding_lineage`.
- Added contract-derived test
  `hphys0262_projects_pmetpara_selected_crop_coefficients`.

Ran:

- `cargo test -p openwepp-runner hphys0262_trace_row_captures_pmet_demand_seeding_lineage -- --nocapture`
  before implementation failed as expected because PMET trace fields were absent.
- `cargo test -p openwepp-runner hphys0262 -- --nocapture` passed after
  implementation.
