# Worker Handoff

Status: `executed-hold`

Evidence mode: `Static:` handoff plus `Ran:` command evidence.

## Current State

W7 is held as
`EXECUTED-HOLD-HILLSLOPE-SEDIMENT-PRODUCTION-MISSING`.

Production code change retained:

- `crates/openwepp-runner/src/watershed_supervisor.rs` canonicalizes generated
  hillslope child input paths.

Focused regression retained:

- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
  `wshedw7_watershed_cli_generated_mode_accepts_relative_run_dir`.

## Commands Run

- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw7_watershed_cli_generated_mode_accepts_relative_run_dir -- --nocapture`
- `cargo fmt --check`
- `cargo clippy -p openwepp-runner --test watershed_cli_behavior_contract -- -D warnings`
- `cargo build -p openwepp-runner --release --bins`
- `target/release/openwepp-cli-watershed ... carnivorous-adobo ... --jobs 8`
- `target/release/openwepp-cli-hill` probes for `/wc1/runs/in/insensible-aliquot/wepp` hillslopes `1`, `21`, `172`, `297`, `333`, `390`, and `437`
- `markdown-doc lint --path ...`: `54 files validated, 0 errors, 0 warnings`
- `git diff --check`

## Blocker

Current openWEPP production HBP pass emission has no nonzero sediment signal on
inspected real multi-OFE candidates even when `erod14_wave2_enabled=true`.

## Next Concrete Action

Execute
`docs/work-packages/20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/`
and close defect `WSHED-W7-HOLD-001`.
