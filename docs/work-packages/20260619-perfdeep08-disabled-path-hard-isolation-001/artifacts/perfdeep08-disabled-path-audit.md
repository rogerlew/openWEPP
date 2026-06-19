# PERFDEEP08 Disabled-Path Audit

Status: complete.
Evidence mode: Static/Ran.

## Static Findings

PERFDEEP07 already removed the dense-absent request lookup tax and changed hot
symbol lookup maps to `HashMap`. PERFDEEP08 inspected the remaining disabled
path and found:

- production indexed scheduler runtime is still active by design on the default
  MOFE path;
- removing production indexed runtime entirely was already rejected in
  PERFDEEP07 as slower (`753.38 s`, `755.48 s`);
- `maybe_record_perfdeep02_frame_roundtrip` still checked
  `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH` at each call site;
- `indexed_shadow_surface::observe_clone_source_surface` and
  `validate_shadow_surface` still entered thread-local state even when
  `OPENWEPP_INDEXED_SHADOW_REPORT_PATH` was unset;
- a scheduler flag-hoist candidate touched `scheduler.rs`, which remains over
  3000 lines and would require line-count closure before retention.

## Candidate Tried

Disabled diagnostic hook caching:

- cache PERFDEEP02 roundtrip path lookup with `OnceLock<Option<PathBuf>>`;
- short-circuit indexed-shadow observe/validate when the report env var is
  unset;
- do not change physics, output, scheduler logic, or direct-frame execution.

The `scheduler.rs` flag-hoist micro-change was reverted before timing to avoid
retaining a 3000+ line touched file without a split/closure plan.

## Result

The hook-cache candidate measured `691.93 s`, RSS `229444 KB`. It was rejected
and reverted.
