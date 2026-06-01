# HPHYS0228 Gate Results

Status: completed  
Evidence mode: Ran

## Commands

1. `cargo test -p openwepp --test wb14_infiltration_hyetograph_kernel_contract`
2. `cargo fmt --check`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo test --workspace`
5. `cargo deny check`

## Outcomes

1. pass  
2. pass  
3. pass  
4. pass  
5. pass (`duplicate` and `license-not-encountered` warnings only; exit success)
