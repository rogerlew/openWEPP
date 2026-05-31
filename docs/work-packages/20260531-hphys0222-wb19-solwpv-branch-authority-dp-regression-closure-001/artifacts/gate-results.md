# HPHYS0222 Gate Results

Status: completed
Evidence mode: Ran

## Commands and outcomes
1. `cargo test --test auth08_wb19_solwpv_fcdep_branch_constitutive_contract`
   - pre-fix expected fail captured (`solwpv_9002` fcdep mismatch).
2. `cargo test --test auth06_fixture_provenance_hash_enforcement_contract --test auth08_wb19_solwpv_fcdep_branch_constitutive_contract --test hphys0221_wb19_water_yield_fcdep_coupling_contract`
   - pass.
3. `cargo test --test hphys0219_wb19_coca_threshold_contract --test wb19_lateral_drainage_physics_kernel_contract`
   - pass.
4. `cargo fmt --check`
   - initial fail (format diff in new AUTH08 test),
   - after `cargo fmt`, pass.
5. `cargo clippy --workspace --all-targets -- -D warnings`
   - initial fail (test-only cast warnings),
   - after updates (`auth07`, `auth08`), pass.
6. `cargo test --workspace`
   - pass.
7. `cargo deny check`
   - pass with warning-only duplicate crate entries and unmatched license
     allow-list entries.

## Gate decision
- `MEASURE-HP222-005`: pass.
