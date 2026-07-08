# Source Guard Evidence

Status: `passed`

Evidence mode: `Ran:` focused tests and `Static:` source review.

Focused W7 guards:

- `wshedw7_watershed_cli_generated_mode_accepts_relative_run_dir`
- `wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`

The W7R guard proves:

- generated p102 HBP parses as schema `1.1`, `nofe=2`, `npart=5`;
- hourly HBP sediment closes to `tdet - tdep`;
- public `totalwatsed3.tdet/tdep` match the generated HBP payload;
- public `sed_del` is nonzero routed sediment yield, not a
  detachment-minus-deposition alias;
- serial and parallel decoded public outputs are identical.

Existing source guards in `watershed_cli_behavior_contract.rs` continue to
forbid old symbol-map/writeback runtime markers in the public watershed path.
