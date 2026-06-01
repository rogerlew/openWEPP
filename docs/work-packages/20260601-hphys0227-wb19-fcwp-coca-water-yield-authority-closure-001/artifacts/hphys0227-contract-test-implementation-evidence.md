# HPHYS0227 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Added/Updated Test Surfaces

1. `tests/integration/hphys0227_wb19_fcwp_coca_watyld_authority_contract.rs`
   - verifies registry/suite/contract linkage for HPHYS0227.
   - executes paired WB19 FC/WP theta-lineage cases.
   - enforces fixed-threshold `q` consistency and expected `watyld/fcdep/unsdep`.
2. `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
   - includes HPHYS0227 suite doc/root in fixture-integrity obligations.
3. `Cargo.toml`
   - registers HPHYS0227 integration test target.
4. Follow-on regression adjustments to keep workspace suites aligned with new
   required WB19 indexed symbols:
   - `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
   - `tests/integration/wb15_canopy_interception_kernel_contract.rs`
   - `crates/openwepp-runner/src/hillslope/mod.rs` (HPHYS0213 test seeds)
   - additional impacted integration seeds (`clim05`, `clim06`, `erod13`,
     `erod14`, `wb11`, `wb12`, `wb16`, `wb17`, `wb20`, `irrig10`,
     `wb19_lateral_drainage`).

## Fixture Additions

- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/wb19_fcwp_coca_watyld_cases.json`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.sha256`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.provenance.yaml`

## Executed Evidence

- Ran:
  - targeted HPHYS/AUTH suites (see `gate-results.md`, command #1),
  - `cargo test -p openwepp --test wb14_infiltration_hyetograph_kernel_contract`,
  - `cargo test -p openwepp --test wb15_canopy_interception_kernel_contract`,
  - `cargo test -p openwepp-runner --lib`,
  - `cargo test --workspace`.
- Result:
  - pass.

## Closure Measure Mapping

- `MEASURE-HP227-004`: satisfied.  
- `MEASURE-HP227-005`: satisfied.
