# Gate Results

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Required non-doc gates from package exit criteria were executed.
- All required gates passed.
- `cargo deny check` emitted warnings only (no failing advisory/ban/license/source result).

## Ran
1. `cargo fmt --check`
   - result: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass
3. `cargo test --workspace`
   - result: pass
4. `cargo deny check`
   - result: pass (warnings only)

Additional rerun lane commands:
- `cargo test -p openwepp --test pl14_tier_a_candidate_replay_contract` -> pass
- `cargo test -p openwepp --test pl14r_tier_a_replay_rerun_contract` -> pass
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract` -> pass
- `cargo test -p openwepp --test pl15_tier_a_delta_closeout_contract` -> pass
- `cargo test -p openwepp --test pl15r_tier_a_delta_recloseout_contract` -> pass

Workspace status capture:
- `git status --short`
