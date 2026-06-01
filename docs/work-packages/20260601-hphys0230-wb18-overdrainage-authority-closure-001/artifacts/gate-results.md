# HPHYS0230 Gate Results

Status: completed  
Evidence mode: Ran

## Commands

1. `cargo test -p openwepp --test wb18_percolation_physics_kernel_contract --test wb11_hydrology_kernel_contract`
2. `cargo test -p openwepp --test auth03_level4_constitutive_gate_contract --test auth05_level4_constitutive_authority_hardening_contract`
3. `cargo fmt --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo test --workspace`
6. `cargo deny check`

## Outcomes

1. pass  
2. pass  
3. pass  
4. pass  
5. pass  
6. pass (`duplicate` / `license-not-encountered` warnings; exit success)
