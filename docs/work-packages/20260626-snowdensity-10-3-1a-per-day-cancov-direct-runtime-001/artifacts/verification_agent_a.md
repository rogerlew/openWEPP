# Verification Agent A

Evidence class: Ran.

## Verified Commands

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass.

## Verified Behavior

- New package integration test passed in workspace.
- G0 PySnobal bridge test passed in workspace and confirms
  `canopy_series.csv` emission.
- 05G representative CoE replay passed in workspace and confirms the new canopy
  source remains representative for conifer `cancov ~= 0.9`.
- 06B CoE-bound density replay passed in workspace, confirming CoE boundary CSV
  compatibility.

## Verdict

Verified complete.
