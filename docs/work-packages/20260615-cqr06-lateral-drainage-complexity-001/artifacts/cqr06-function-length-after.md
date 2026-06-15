# CQR06 Function Length After

Evidence class: Static

After target file line count: `2527`.

Public function spans after refactor:

- `run_lateral_transfer`: lines `160-171`, `12` lines.
- `run_drainage`: lines `1390-1408`, `19` lines.
- `wb14_ksatadj_flag`: lines `2015-2038`, `24` lines.
- `wb14_load_top_two_layer_ksatadj_metrics`: lines `2039-2050`, `12` lines.
- `resolve_wb14_effective_soil_conductivity`: lines `2309-2334`, `26` lines.

Largest private helper spans after refactor:

- `wb19_lateral_transfer_inputs`: `95` lines.
- `wb19_drainage_inputs`: `94` lines.
- `wb19_lateral_layer_parameters`: `86` lines.
- `wb19_drainage_response`: `78` lines.
- `wb19_run_lateral_substep`: `70` lines.
- `wb19_lateral_summary_state_updates`: `70` lines.

All target-file `#[allow(clippy::too_many_lines)]` suppressions were removed.
