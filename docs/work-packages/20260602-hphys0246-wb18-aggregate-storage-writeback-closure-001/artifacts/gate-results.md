# HPHYS0246 Gate Results

Status: completed
Evidence mode: Ran

## Rust Gates
- `cargo fmt --check`
  - Pass.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Pass.
- `cargo test --workspace`
  - Pass.
- `cargo deny check`
  - Pass with warnings:
    - unmatched license allowances: `ISC`, `Unicode-DFS-2016`
    - duplicate dependency versions: `getrandom`, `hashbrown`, `twox-hash`

## Focused Gates
- `cargo test -p openwepp-hillslope-orchestrator hphys0246_wb18 -- --nocapture`
  - Pass, `2 passed`.
- `cargo test -p openwepp-hillslope-orchestrator`
  - Pass, `77 passed`.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - Pass.
- H1/H7/H39 telemetry run:
  - H1: pass, return code `0`, `480` trace rows.
  - H7: pass, return code `0`, `480` trace rows.
  - H39: pass, return code `0`, `480` trace rows.

## Authority Guards
- `bash tools/release/check_authority_suite_antievasion.sh`
  - Pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract`
  - Pass, `2 passed`.

## Not Satisfied
- Independent dual review artifacts are not independently authored.
- Independent dual verification artifacts are not independently authored.
