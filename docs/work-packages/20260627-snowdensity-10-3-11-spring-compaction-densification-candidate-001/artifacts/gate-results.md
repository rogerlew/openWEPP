# Gate Results

Evidence mode: Ran.

## Commands

- `cargo fmt`
- `cargo fmt --check`
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/spring_compaction_densification_candidate.py`
- `cargo test --test snowdensity10_3_11_spring_compaction_densification -- --nocapture`
- `cargo clippy --test snowdensity10_3_11_spring_compaction_densification -- -D warnings`
- `cargo test --test snowdensity06b_coe_bound_density_replay --test snowdensity08_gate_rerun --test snowdensity09_coupled_wat_rerun -- --nocapture`
- `cargo test -p openwepp --test snowdensity03_physics_bulk_offline_contract -- --nocapture`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`
- `rg -n "qwet|frzftp" crates`

## Coupled WAT Summary

- Disposition: `SPRING-DENSIFICATION-NON-PROMOTION`
- Blocker: `COUPLED-SNOW-CONTROL-OR-UNDER-PERSISTENCE-WORSENED`
- Prior 10.3.8 fail count: `761`
- Density baseline fail count: `498`
- Spring candidate fail count: `502`
- Density-minus-candidate fail delta: `-4`
- Paired surfaces worse: `3`
- Candidate under-persistence failures: `128`
- Candidate March/April compaction-only failures: `20`

Primary artifacts:

- `artifacts/spring-compaction-densification-candidate.json`
- `artifacts/spring-compaction-densification-candidate.md`

## Notes

An initial coupled run exposed a runtime density cap roundoff bug in the density
opt-in path (`522.0000000000001` published after reconstructing
`mass/depth`). The fix publishes the stored capped density from the density
state. The rerun completed all `14` WAT executions and generated the final
report.

`cargo test --workspace`, full workspace clippy, and `cargo deny check` passed
after the candidate was dispositioned. The `qwet|frzftp` crate scan returned no
matches.
