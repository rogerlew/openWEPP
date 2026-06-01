# HPHYS0227 Gate Results

Status: completed  
Evidence mode: Ran

## Commands

1. `cargo test --test hphys0227_wb19_fcwp_coca_watyld_authority_contract --test auth06_fixture_provenance_hash_enforcement_contract --test auth08_wb19_solwpv_fcdep_branch_constitutive_contract --test hphys0219_wb19_coca_threshold_contract --test hphys0221_wb19_water_yield_fcdep_coupling_contract --test hphys0224_wb19_withdrawal_soilwater_cap_contract --test hphys0225_wb19_layer_pool_withdrawal_cap_contract --test hphys0226_wb19_lateral_saturated_thickness_response_contract --test wb19_lateral_drainage_physics_kernel_contract`
2. `cargo test -p openwepp --test wb14_infiltration_hyetograph_kernel_contract`
3. `cargo test -p openwepp --test wb15_canopy_interception_kernel_contract`
4. `cargo test -p openwepp-runner --lib`
5. `cargo test --workspace`
6. `cargo fmt`
7. `cargo fmt --check`
8. `cargo clippy --workspace --all-targets -- -D warnings`
9. `cargo deny check`

## Outcomes

1. pass  
2. pass  
3. pass  
4. pass  
5. pass  
6. pass  
7. pass  
8. pass  
9. pass (`duplicate`/`license-not-encountered` warnings only; exit success)
