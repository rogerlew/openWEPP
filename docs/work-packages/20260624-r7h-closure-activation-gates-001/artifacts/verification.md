# Verification

Evidence class: Ran.

## Release Build

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: passed.

## H2637 Direct Gate Loop

Fixture:

- Run dir: `/tmp/openwepp_farpoint01_h2637/without_ui/runs`
- Run file: `h2637.run`

Runs:

- Initial direct default-candidate: failed at lane `1`, day `10`,
  `hydrology_projection.aggregate_storage_delta_m`.
- After no-material frost storage-delta fix: failed at lane `2`, day `25`,
  `hydrology_projection.frozen_layer_storage_m`.
- After stale coarse frozen-layer clear:
  `r7h_direct_default_after_stale_frozen_clear 112.82 1083256`.
- After no-material clear fast path:
  `r7h_direct_default_after_no_material_fast_path 113.53 1083636`.

Final direct manifest:

- `/tmp/r7h-closure-activation-gates/direct-default-after-no-material-fast-path/owepp_output/H2637.manifest.json`
- `requested=default-candidate`
- `selected=direct-production-executor`
- `compatibility_edge_invocations=0`
- `day_frame_commits=235961`
- `scheduler_kernel_executed=false`
- `publication_source=direct-publication-frame`

Performance disposition:

- Failed. `113.53 s > 91.2 s`.

## Protected Output Characterization

Compared current direct output to retained compatibility capture under
`/tmp/r7g-cont-h2637/capture/compat`.

This is not a current-code parity pass because current compatibility was not
rerun after the direct performance gate failed.

| Output | Retained compat vs current direct |
| --- | --- |
| HBP | differ |
| WAT | differ |
| PASS | differ |
| loss | differ |
| plot | differ |

## Focused Rust Gates

- `cargo test -p openwepp-hillslope-orchestrator r7g_ -- --nocapture`: passed.
- `cargo test -p openwepp-runner r7g_ -- --nocapture`: passed.
- `cargo test -p openwepp-runner r7e_ -- --nocapture`: passed.
- `cargo fmt --check`: passed.

Workspace gates are recorded in `gate-results.md`.
