# HPHYS0232 Gate Results

Status: completed  
Evidence mode: Ran

## Commands

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace -q`
4. `cargo deny check`
5. `cargo test -p openwepp --test wb18_percolation_physics_kernel_contract`
6. `cargo test -p openwepp-runner hphys0232_wb11_seed`
7. `cargo test -p openwepp --test auth03_level4_constitutive_gate_contract --test auth05_level4_constitutive_authority_hardening_contract`

## Outcomes

1. pass  
2. pass  
3. pass  
4. pass (`duplicate` / `license-not-encountered` warnings; exit success)  
5. pass  
6. pass  
7. pass
