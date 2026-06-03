# Verification Agent A

Status: completed

Evidence mode: ran

## Commands

Ran:

```text
cargo fmt --check
cargo test --test hphys0256_wb19_latqcc_lane_branch_contract --test wb19_lateral_drainage_physics_kernel_contract --test hphys0221_wb19_water_yield_fcdep_coupling_contract --test hphys0226_wb19_lateral_saturated_thickness_response_contract --test hphys0227_wb19_fcwp_coca_watyld_authority_contract -- --nocapture
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Result

- Ran: all commands passed.
