# Implementation and Test Evidence

Status: `executed-hold`

Evidence mode: `Ran:`

## Implementation

W7 discovered and fixed a public watershed supervisor path-resolution defect:
generated hillslope runfiles now canonicalize input file paths before child CLI
execution. Without this, a relative public `--run-dir` caused child hillslope
CLIs to resolve committed fixture inputs relative to isolated job output
directories.

Changed files:

- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

Focused regression:

- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw7_watershed_cli_generated_mode_accepts_relative_run_dir -- --nocapture`
- Result: `1 passed; 0 failed; 24 filtered out`

Validation:

- `cargo fmt --check`: PASS
- `cargo clippy -p openwepp-runner --test watershed_cli_behavior_contract -- -D warnings`: PASS
- `cargo build -p openwepp-runner --release --bins`: PASS

## Fixture Probes

- `/tmp/wshedw7_probe_carn/out`: committed `carnivorous-adobo` public
  watershed run, `--jobs 8`, completed after the path fix; sediment remained
  zero.
- `/tmp/wshedw7_probe_insensible_h1`: local multi-OFE hillslope probe,
  `erod14_wave2_enabled=true`, pass parquet aggregate all zero.
- `/tmp/wshedw7_probe_multi/H{21,172,297,333,390,437}`: additional multi-OFE
  probes, all pass sediment aggregates zero.
