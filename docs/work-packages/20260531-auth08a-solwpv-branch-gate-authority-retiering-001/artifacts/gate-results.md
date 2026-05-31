# AUTH08A Gate Results

Status: completed
Evidence mode: Ran

## Commands
1. `cargo test --test auth08_wb19_solwpv_fcdep_branch_constitutive_contract --test auth06_fixture_provenance_hash_enforcement_contract` (pass)
2. `cargo fmt --check` (pass)
3. `cargo clippy --workspace --all-targets -- -D warnings` (pass)
4. `cargo test --workspace` (pass)
5. `cargo deny check` (pass; warning-only duplicate crates and unmatched
   license allow-list entries)

## Decision
- pass
