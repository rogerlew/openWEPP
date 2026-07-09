# Gate Results

Status: `EXECUTED-COMPLETE`
Evidence: `Ran`

## Focused Gates

- `cargo test -p openwepp --test infile_hbp_parser_contract latest_event_state_represents_no_event_without_synthesizing_event_payload`
  - Result: pass (`1` passed, `24` filtered).
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw9`
  - Result: pass (`3` passed, `26` filtered).

## Workspace Gates

- `cargo check --workspace`
  - Result: pass.
- `cargo fmt --check`
  - Result: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: pass after replacing two float equality assertions in the new
    parser test with epsilon-bound checks.
- `cargo nextest run --workspace --profile quick`
  - Result: pass (`1396` run, `1396` passed, `26` skipped).
- `cargo nextest run --workspace --profile full`
  - Result: pass (`1471` run, `1471` passed, `3` skipped).
- `cargo deny check`
  - Result: pass (`advisories`, `bans`, `licenses`, and `sources` ok).
- `markdown-doc lint --path docs/work-packages/20260709-wshedw9-canonical-noevent-pass-semantics-001 --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - Result: pass (`12` files validated, `0` errors, `0` warnings).
- `git diff --check`
  - Result: pass.
