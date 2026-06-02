# Contract-Test Implementation Evidence

Status: complete

Evidence mode: static

Static: contract-derived tests were added before production code edits.

## Tests Added

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`:
  `hphys0251_management_projection_preserves_crop_pltol` asserts that crop
  `residue_line[3]` (`pltol`) projects into both slot-specific and primary
  runtime state symbols.
- `tests/integration/wb17_et_physics_kernel_contract.rs`:
  `hphys0251_wb17_root_uptake_normalizes_pltol_like_swu_for` asserts the
  baseline `swu.for` normalization branches for raw `pltol <= 0`, `<0.1`, and
  `>0.4`, plus final `Ep`/`Ws` and layer storage mutation.
- `tests/integration/wb17_et_physics_kernel_contract.rs`:
  `hphys0251_wb17_root_uptake_publishes_layer_upi_ui_trace` asserts layer
  `UPi_####`/`Ui_####` publication, aggregate `UPi`/`Ui`, final `Ep=ΣUi`, and
  `Ws=ΣUi/Etp`.

## Expected Pre-Implementation Result

Ran: not yet; the pre-implementation gate is expected to fail until production
projection and root-uptake publication are implemented.
