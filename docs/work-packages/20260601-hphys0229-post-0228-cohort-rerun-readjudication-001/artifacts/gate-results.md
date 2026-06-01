# HPHYS0229 Gate Results

Status: completed  
Evidence mode: Ran

## Commands

1. `cargo test -p openwepp --test wb14_infiltration_hyetograph_kernel_contract --test hphys0224_wb19_withdrawal_soilwater_cap_contract --test hphys0225_wb19_layer_pool_withdrawal_cap_contract --test hphys0226_wb19_lateral_saturated_thickness_response_contract --test hphys0227_wb19_fcwp_coca_watyld_authority_contract`
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
