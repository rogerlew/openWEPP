# HPHYS0227 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Canonical Contract Amendments

1. `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
   - Added/linked `INV-SUBHYD-019`.
   - Added `HPHYS0227 WB19 FC/WP + COCA Water-Yield Coupling Addendum`.
   - Updated WB19 branch required symbols to include `thetfc_####`, `thetdr_####`.
   - Updated `avfca` authority to `Σ(thetfc_i * dg_i / fcdep)`.
2. `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
   - Added `HPHYS0227 WB19 FC/WP + COCA Water-Yield Coupling Addendum`.
   - Added WB19 state-input rows for `thetfc_####`, `thetdr_####`.
   - Updated `avfca` lineage text to `thetfc` authority.
3. `docs/specifications/science-contracts/index.md`
   - Updated index summaries for HPHYS0227 authority scope.

## External-authority Surfaces

1. Added suite specification:
   - `docs/specifications/external-authority/suites/cas_l4_subhyd_watyld_fcwp_consistency_001.md`
2. Registered suite:
   - `docs/specifications/external-authority/registry.yaml`
3. Added fixture lock/provenance:
   - `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.sha256`
   - `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/fixtures.provenance.yaml`

## Production Authority Implementation

1. `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
   - Added indexed WB19 theta symbol helpers.
2. `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
   - Enforced WB19 per-layer FC/WP consistency:
     `wb18_perc_fc_#### = (thetfc_####-thetdr_####)*dg_####`.
   - Enforced `thetdr_#### <= thetfc_####`.
   - Corrected `avfca` lineage from FC-store surrogate to `thetfc_####`.

## Closure Measure Mapping

- `MEASURE-HP227-001`: satisfied.  
- `MEASURE-HP227-002`: satisfied.  
- `MEASURE-HP227-003`: satisfied.
