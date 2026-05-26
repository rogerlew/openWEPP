# MOFE13 `ksatadj` Regime Alignment Report

Status: complete
Evidence mode: mixed (Static + Ran)

Objective closure:
- openWEPP WB14 runoff reconciliation now executes baseline-authoritative
  `ksatadj(iplane)=1` conductivity adjustment regimes for `solwpv` 9001/9002/9003.

Static implementation summary:
- Runtime seam publication in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs` now projects:
  - `solwpv` (`soil.datver_raw`)
  - OFE-scoped `ksatadj`, `ksatfac`, `ksatrec`, `lkeff`
  - primary aliases `ksatadj`, `ksatfac`, `ksatrec`, `lkeff` for OFE1
- WB14 kernel implementation in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`:
  - `solwpv==9001`:
    - `Ke = ((Ke_u-Ke_l)/(exp(1/ksatrec)-1))*(exp(sat_frac/ksatrec)-1)+Ke_l`,
      `Ke_l = Ke_u/ksatfac`
  - `solwpv>=9002`:
    - `Ke = Ks * sat_frac^(2*lambda+3)`
    - `lambda = 1/psi`,
      `psi = ln(1500/33) / ln(theta_fc/theta_dr)`
  - `solwpv==9003`:
    - applies floor `Ke >= lkeff` when `lkeff > 0`
- Typed guard posture preserved for non-finite/out-of-domain active inputs;
  no silent fallback introduced for active regime violations.

Ran:
- Contract-derived runtime and WB14 regime tests pass post-implementation.
