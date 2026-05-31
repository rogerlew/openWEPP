# HPHYS0216 Gate Results

Status: completed
Evidence mode: Ran

## Required gates
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Results
1. `cargo fmt --check`: pass.
2. `cargo clippy --workspace --all-targets -- -D warnings`: pass.
3. `cargo test --workspace`: pass after one contract-test correction
   (`tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`) to align
   with HPHYS0216 authority.
4. `cargo deny check`: pass with warnings only.

`cargo deny check` warnings observed:
- duplicate transitive versions (`getrandom`, `hashbrown`, `twox-hash`)
- unmatched allowlist entries in `deny.toml` (`ISC`, `Unicode-DFS-2016`)

No gate output files were persisted under a package-local run root for this
execution; results are from direct command invocations in workspace shell
session.
