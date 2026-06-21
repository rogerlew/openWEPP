# Implementation Test Evidence

Status: in progress.
Evidence mode: Static + Ran.

Implemented code paths:

- `DirectRunPublicationFrame` and typed row/group operand structs in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`.
- `DirectFrameExecutor::run_publication_capture`, which records one
  run/lane/day row from typed direct day frames during direct span execution.
- `HillslopeRuntimeSelection::DirectPublicationFrameShadow`, with CLI flag
  `--direct-publication-frame-shadow`.
- Runner-side direct publication shadow builder
  `build_direct_publication_shadow`, which seeds lane area from parsed slope
  OFEs and calendar from the existing climate span.
- Direct projection consumers:
  `build_hbp_output_from_direct_publication`,
  `build_hillslope_wat_rows_from_direct_publication`,
  `build_hillslope_pass_rows_from_direct_publication`,
  `build_loss_output_json_from_direct_publication`, and
  `build_manifest_text_from_direct_publication`.

Focused tests run:

| Command | Result | Evidence |
|---|---|---|
| `cargo test -p openwepp-hillslope-orchestrator r6a_publication_capture_records_run_bound_rows_without_publication_alias -- --nocapture` | PASS | Capture emits four run/lane/day rows for a 2-lane/2-day frame, records `publication_capture_runs = 1`, records `skeleton_runs = 0`, and proves `publication.runoff_m` is not projected as direct runoff. |
| `cargo test -p openwepp-runner r6a_ -- --nocapture` | PASS | `r6a_direct_publication_frame_shadow_runs_without_skeleton_counter` proves opt-in runner capture on the fixture with `publication_capture_runs = 1`, `skeleton_runs = 0`, and `compatibility_edge_invocations = 0`; `r6a_direct_projection_consumers_read_publication_frame_operands` proves WAT/PASS/loss/manifest consumers read direct frame operands. |

Residual work outside R6A:

- independent reconstruction beyond the direct projection fixture;
- production public writer cutover;
- byte/Arrow identity and metadata/checksum parity.
