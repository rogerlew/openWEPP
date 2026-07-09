# Gate Results

Status: `EXECUTED-COMPLETE`
Evidence: `Ran`

## Focused Gates

- `cargo check -p openwepp-watershed-orchestrator -p openwepp-runner -p openwepp-input-contract`
  - passed.
- `cargo test -p openwepp-runner hbp_latest_event_payload_exposes_groundwater_baseflow_and_deep_seepage --test watershed_cli_behavior_contract -- --nocapture`
  - passed: `1` passed, `26` filtered out.
- `cargo test --test wshedw5_typed_watershed_runtime_contract gwbaseflow -- --nocapture`
  - passed: `3` passed, `8` filtered out.
- `cargo test -p openwepp-runner r6a_direct_hbp_writer_serializes_groundwater_payload_operands -- --nocapture`
  - passed: `1` passed in `openwepp-runner` library tests.

## Workspace Gates

- `cargo fmt --check`
  - passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - passed.
- `cargo nextest run --workspace --profile quick`
  - passed: `1392` tests run, `1392` passed, `26` skipped, `153.619s`.
- `cargo nextest run --workspace --profile full`
  - passed: `1467` tests run, `1467` passed, `3` skipped, `584.642s`.
- `cargo deny check`
  - passed: advisories, bans, licenses, and sources ok.

## Documentation And Diff Gates

- `markdown-doc lint --path docs/work-packages/20260709-laned-active-baseflow-export-closure-001 --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md --path docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
  - passed: `11` files validated, `0` errors, `0` warnings.
- `git diff --check`
  - passed.
