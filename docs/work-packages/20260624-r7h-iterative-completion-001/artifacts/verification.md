# Verification

Evidence class: Ran.

Focused gates:

- `cargo test -p openwepp-hillslope-orchestrator r7h_ -- --nocapture`:
  passed.
- `cargo test -p openwepp-hillslope-orchestrator
  r7b_constructor_type_size_layout_is_bounded -- --nocapture`: passed.
  Reported sizes:
  `DirectRunConstructorInputs=72`,
  `DirectLaneConstructorInputs=952`,
  `DirectDayConstructorInputs=3912`,
  `DirectRunFrame=256`,
  `DirectLaneFrame=1168`,
  `DirectDayFrame=11704`.
- `cargo test -p openwepp --test
  hphys0319_fixed_baseline_stmtim_observe_contract --test
  hphys0320_stmtim_start_time_source_line_contract`: passed.

Workspace gates:

- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed.

Documentation gates:

- Work-package closeout evidence refreshed after final gate run.
- R7H remains closed `OPT-IN`; direct default activation remains deferred.
