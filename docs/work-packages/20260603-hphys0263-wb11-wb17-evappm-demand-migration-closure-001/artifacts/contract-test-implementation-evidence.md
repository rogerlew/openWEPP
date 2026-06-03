# Contract-Test Implementation Evidence

Status: completed

Evidence mode: static + ran

Static:

- Added `hphys0263_wb11_seed_uses_evappm_branch_when_pmetpara_selects_pmet`
  in `crates/openwepp-runner/src/hillslope/mod.rs`.
- The test builds a one-layer WB11 runtime surface with `pmetpara.mode.iflget=2`,
  selected `kcb=0.95`, selected `rawp=0.80`, climate inputs, PMET root/canopy
  state, and WB18 storage.
- Assertions require:
  - `wb11_et_seed_branch_evappm = 1`
  - `wb11_et_seed_branch_priestley_taylor = 0`
  - `wb11_et_demand = pmet.ep_m`
  - `pmet.etorc_mm` and `pmet.kcbcon` match pinned `evappm.for` reference
    values for the fixture.

Ran:

- `cargo test -p openwepp-runner hphys026 -- --nocapture`
- Result: passed, `5 passed`.
- `cargo test --workspace`
- Result: passed.
