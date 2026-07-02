# Source Guard Evidence

Status: `passed-for-hold`

Evidence mode: `Ran:` focused test and `Static:` source review.

Focused regression added:

- `wshedw7_watershed_cli_generated_mode_accepts_relative_run_dir`

The test prevents regression to child input paths that only work when the
public `--run-dir` is absolute. This protects committed fixture execution and
strict fixture auditability.

Existing source guards in `watershed_cli_behavior_contract.rs` continue to
forbid deleted watershed runtime markers such as
`WatershedWritebackSurface`, `execute_watershed_dispatch_with_kernel`, and
`compatibility_writeback_surface` in production routing/publication paths.

No nonzero-sediment fixture guard was added because W7 holds before fixture
adoption. Adding a zero-only guard would weaken the package objective.
